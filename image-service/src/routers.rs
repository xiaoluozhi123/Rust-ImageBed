pub mod config {
    use actix_web::{get, HttpResponse, Responder};

    #[get("/")]
    pub async fn home_config() -> impl Responder {
        HttpResponse::Ok().body("Hello World")
    }
}
