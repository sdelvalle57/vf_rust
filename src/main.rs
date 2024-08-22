mod db;
mod graphql;
mod agent;
mod resource_specification;

use crate::db::conn::establish_connection_pool;
use crate::graphql::handler::start_server;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish the database connection pool
    let pool = Arc::new(establish_connection_pool());

    // Start the GraphQL server with the connection pool
    start_server(pool).await
}