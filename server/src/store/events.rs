use uuid::Uuid;
use crate::models::Model;

#[derive(Clone, Debug)]
pub enum Change<T: Model> {
    Created(T),
    Updated(T),
    Deleted(Uuid),
}

impl<T: Model> Change<T> {
    pub fn id(&self) -> Uuid {
        match self {
            Change::Created(item) => item.id(),
            Change::Updated(item) => item.id(),
            Change::Deleted(id) => id.clone(),
        }
    }
}