use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/gae")]
async fn gae() -> impl Responder {
        HttpResponse::Ok().body("Your very gae!")
}

#[get("/randomint")]
async fn randomint() -> impl Responder {
    let int = rand::thread_rng().gen_range(0..100);
    println!("Requested number: {}", int.to_string());
    HttpResponse::Ok().body(int.to_string())

}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}



async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(gae)
            .service(randomint)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

