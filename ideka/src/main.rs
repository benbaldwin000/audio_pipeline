use actix_files;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_file))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/listen")]
pub async fn get_file(req: HttpRequest) -> HttpResponse {
    let file = actix_files::NamedFile::open_async("./public/electric_prince.mp3")
        .await
        .unwrap();

    file.into_response(&req)
}
