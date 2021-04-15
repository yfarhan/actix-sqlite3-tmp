mod configs;
use configs::Settings;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn server_start_print(addr: &String) {
    use colored::*;

    println!("----------------------------------------------");
    println!("{} {} !", "Starting server on ".green(), &addr.blue().bold());
    println!("----------------------------------------------");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new();
    let server = settings.unwrap().server;

    let addr = format!("127.0.0.1:{}", &server.port);
    server_start_print(&addr);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(addr)?
    .run()
    .await
}
