use engine::graphql::{Schema, Query, Mutation, Context, create_schema};

use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

async fn graphiql() -> HttpResponse {
  let html = graphiql_source("http://127.0.0.1:8080/graphql", None);
  HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(html)
}

async fn graphql(
  st: web::Data<Arc<Schema>>,
  data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
  let user = web::block(move || {
      let context = Context{};
      let res = data.execute_sync(&st, &context);
      serde_json::to_string(&res)
  })
  .await?;
  Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(user))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/*
curl -g \
-X POST \
-H "Content-Type: application/json" \
-d '{"query":"query{speciesByID(id: 1) { name }}"}' \
http://localhost:8080/graphql

*/