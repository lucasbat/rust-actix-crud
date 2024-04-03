use actix_web::{get, App, web, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
mod databases {
    pub mod postgres_connection;
}
mod services;
#[derive(Clone)]
pub struct AppState {
    postgres_client: Pool<Postgres>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    dotenv().ok();
    let _pool = databases::postgres_connection::start_connection().await;
    HttpServer::new(move|| {
        App::new()
            .app_data(
                web::Data::new(AppState {
                    postgres_client: _pool.clone(),
                })
            )
            .service(index)
            .configure(services::users::services::users_routes)
    }).bind(("127.0.0.1", 8080))?.run().await
}
