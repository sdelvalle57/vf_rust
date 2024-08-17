use actix_web::{get, HttpResponse, Responder, web, route};
use juniper::http::GraphQLRequest;
use juniper::http::graphiql::graphiql_source;
use actix_web_lab::respond::Html;
use crate::graphql::schema::{Schema, create_schema};
use crate::graphql::handler;

use std::{io, sync::Arc};
use actix_web::{App, HttpServer};
use actix_cors::Cors;
use actix_web::web::Data;

/// GraphiQL playground UI
/// check example here https://github.com/actix/examples/blob/master/graphql/juniper/README.md
#[get("/graphiql")]
pub async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}


#[actix_web::main]
pub async fn start_server() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port 7878");
    log::info!("GraphiQL playground: http://localhost:7878/graphiql");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        App::new()
            .wrap(cors)
            .app_data(Data::from(schema.clone()))
            .service(handler::graphql)
            .service(handler::graphql_playground)
    })
    .workers(5)
    .bind(("127.0.0.1", 7878))
    .expect("unable to start server")
    .run()
    .await
}