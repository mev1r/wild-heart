use serde::de::DeserializeOwned;
use serde::Serialize;
use sled::Db;
use uuid::Uuid;

pub struct PersistenceLayer {
    db: Db,
    prefix: String,
}

impl PersistenceLayer {
    pub fn new(db: Db, prefix: String) -> Self {
        Self { db, prefix }
    }

    pub fn save<T: Serialize>(&self, id: Uuid, item: &T) -> Result<(), String> {
        let key = format!("{}:{}", self.prefix, id);
        let bytes = bincode::serialize(item)
            .map_err(|e| e.to_string())?;

        self.db.insert(key, bytes)
            .map_err(|e| e.to_string())?;

        self.db.flush()
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn load_all<T: DeserializeOwned>(&self) -> Result<Vec<(Uuid, T)>, String> {
        let prefix = format!("{}:", self.prefix);
        let mut items = Vec::new();

        for entry in self.db.scan_prefix(&prefix) {
            let (key, value) = entry.map_err(|e| e.to_string())?;

            let key_str = String::from_utf8_lossy(&key);
            let uuid_str = key_str.strip_prefix(&prefix)
                .ok_or("Invalid key format")?;
            let id = Uuid::parse_str(uuid_str)
                .map_err(|e| e.to_string())?;

            let item: T = bincode::deserialize(&value)
                .map_err(|e| e.to_string())?;

            items.push((id, item));
        }

        Ok(items)
    }
}