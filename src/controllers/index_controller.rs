use actix_web::{web, HttpRequest, HttpResponse};

pub async fn greet(info: HttpRequest) -> HttpResponse {
    // Implement login logic here...
    HttpResponse::Ok().json("Logged in")
}
