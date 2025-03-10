use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

async fn greet(req: HttpRequest) -> impl Responder {

    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)

}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new( || {
        App::new()
            .route("/",       web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")
    .expect("REASON")
    .run()
    .await

}