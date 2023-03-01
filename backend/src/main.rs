use actix_web::{App, HttpServer, HttpResponse, Responder, web};

use mysql::{Pool};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let pool = Pool::new("mysql://jpheckles:jph@localhost/inventorydb").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

