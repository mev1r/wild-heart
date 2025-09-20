use crate::models::Model;

#[derive(Clone, Debug)]
pub enum Change<T: Model> {
    Created(T),
    Updated(T),
}