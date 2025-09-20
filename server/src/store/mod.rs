mod events;
mod persistence;

pub use events::Change;

use crate::models::Model;
use crate::store::persistence::PersistenceLayer;
use dashmap::DashMap;
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct Store<T: Model> {
    pub(crate) data: DashMap<Uuid, T>,
    events: broadcast::Sender<Change<T>>,
    persistence: Option<PersistenceLayer>,
}

impl<T: Model> Store<T> {
    pub fn with_persistence(db: sled::Db, collection_name: &str) -> Result<Self, String> {
        let (tx, _) = broadcast::channel(100);
        let persistence = PersistenceLayer::new(db, collection_name.to_string());

        let mut store = Self {
            data: DashMap::new(),
            events: tx,
            persistence: Some(persistence),
        };

        store.load_from_disk().map_err(|e| {
            eprintln!("Failed to load {} from disk: {}", collection_name, e);
            e
        })?;

        Ok(store)
    }

    fn load_from_disk(&mut self) -> Result<(), String> {
        if let Some(ref persistence) = self.persistence {
            let items = persistence.load_all::<T>()?;
            for (id, item) in items {
                self.data.insert(id, item);
            }
        }
        Ok(())
    }

    pub fn insert(&self, item: T) -> Result<T, String> {
        let id = item.id();

        if let Some(ref persistence) = self.persistence {
            persistence.save(id, &item)?;
        }

        self.data.insert(id, item.clone());
        let _ = self.events.send(Change::Created(item.clone()));

        Ok(item)
    }

    pub fn update<F>(&self, id: &Uuid, f: F) -> Result<T, String>
    where
        F: FnOnce(&mut T),
    {
        let mut entry = self.data.get_mut(id)
            .ok_or_else(|| "Item not found".to_string())?;

        f(&mut *entry);
        let updated = entry.clone();

        if let Some(ref persistence) = self.persistence {
            persistence.save(*id, &updated)?;
        }

        let _ = self.events.send(Change::Updated(updated.clone()));

        Ok(updated)
    }
    
    pub fn find_by<F>(&self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        self.data.iter()
            .find(|entry| predicate(entry.value()))
            .map(|entry| entry.value().clone())
    }

    pub fn find_all_by<F>(&self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        self.data.iter()
            .filter(|entry| predicate(entry.value()))
            .map(|entry| entry.value().clone())
            .collect()
    }
}