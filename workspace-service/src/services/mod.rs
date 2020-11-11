use sqlx::{Executor, Postgres};
use std::sync::Arc;

pub mod team;
pub mod user;
pub mod workspace;

pub struct DB<'c> {
    pub connection: Arc<dyn Executor<'c, Database = Postgres>>,
}

impl<'c> DB<'c> {
    pub fn new(connection: Arc<dyn Executor<'c, Database = Postgres>>) -> Self {
        Self { connection }
    }
}
