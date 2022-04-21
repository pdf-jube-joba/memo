use ownlinkmemo_domain as domain;
use actix_web::{get , post , web , App , HttpResponse , Responder};

pub async fn link_listup<T: domain::link::LinkRepository>(repo: web::Data<T>) -> HttpResponse {
  HttpResponse::Ok()
    .content_type("application/json")
    .json(repo.listup())
}

/*
#[get("/links/{id}")]
pub async fn link_get(config: web::Data<&lib::config::Config> , path: web::Path<String>) -> impl Responder {
  match lib::link::search(&config, &path.into_inner()) {
    Ok(obj) => HttpResponse::Ok()
      .content_type("application/json")
      .json(obj),
    Err(_err) => HttpResponse::NotFound().finish()
  }
}
*/