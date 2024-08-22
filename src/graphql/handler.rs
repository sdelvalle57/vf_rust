use actix_web::{get, HttpResponse, Responder, web, route};
use juniper::http::GraphQLRequest;
use juniper::http::graphiql::graphiql_source;
use actix_web_lab::respond::Html;
use crate::graphql::schema::{Schema, create_schema};
use crate::graphql::context::Context;
use crate::db::conn::Pool;

use std::{io, sync::Arc};
use actix_web::{App, HttpServer};
use actix_cors::Cors;
use actix_web::web::Data;

#[get("/graphiql")]
pub async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    schema: web::Data<Schema>,
    pool: web::Data<Arc<Pool>>,
    req: web::Json<GraphQLRequest>
) -> impl Responder {
    let ctx = Context {
        pool: pool.get_ref().clone(),
    };

    let res = req.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(res)
}

pub async fn start_server(pool: Arc<Pool>) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port 7878");
    log::info!("GraphiQL playground: http://localhost:7878/graphiql");

    HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(Data::from(schema.clone()))
            .app_data(Data::new(pool.clone()))
            .service(crate::graphql::handler::graphql)
            .service(crate::graphql::handler::graphql_playground)
    })
    .workers(5)
    .bind(("127.0.0.1", 7878))?
    .run()
    .await  // The key change: returning the result of `.await`
}