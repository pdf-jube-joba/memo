use actix_web::{web , App , HttpServer , HttpResponse};
use ownlinkmemo_domain as domain;
use crate::link;

// configuring to simple api server

pub async fn test() -> HttpResponse {
  println!("test");
  HttpResponse::Ok().finish()
}

pub fn configure_api(cfg: &mut web::ServiceConfig) {
  cfg
  .service(
    web::resource("links")
    .route(web::get().to(link::link_search))
  )
  .service(
    web::resource("test")
    .route(web::get().to(test))
  );
}