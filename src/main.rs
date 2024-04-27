use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use webdocker::random_fruit;

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello World Random Fruit!")
}

#[get("/fruit")]
async fn fruit() -> impl Responder {
    println!("Randow Fruit: {}", random_fruit());
    HttpResponse::Ok().body(random_fruit())
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/version")]
async fn version() -> impl Responder {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    return HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"));
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
 println!("Running the service");
     HttpServer::new(|| {
       App::new()
           .service(hello)
           .service(fruit)
           .service(health)
           .service(version)

     })
         .bind("0.0.0.0:8081")
         .unwrap().run()
         .await
}