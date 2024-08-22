use std::sync::Arc;
use crate::db::conn::Pool;

pub struct Context {
    pub pool: Arc<Pool>,
}

impl juniper::Context for Context {}
