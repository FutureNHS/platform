use sqlx::{Executor, Postgres};
use std::sync::Arc;

pub mod team;
pub mod user;
pub mod workspace;

pub struct DB<'c> {
    connection: Arc<dyn Executor<'c, Database = Postgres>>,
}
