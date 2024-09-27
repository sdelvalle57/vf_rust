mod db;
mod graphql;
mod recipe;
mod common;
mod templates;

use crate::db::conn::establish_connection_pool;
use crate::graphql::handler;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish the database connection pool
    let pool = Arc::new(establish_connection_pool());

    // Start the GraphQL server with the connection pool
    handler::start_server(pool).await

    
}