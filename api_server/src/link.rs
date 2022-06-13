use ownlinkmemo_domain as domain;
use actix_web::{web , HttpResponse};
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

pub struct Repository {
  pub repository: Mutex<domain::repository::Repository>
}

pub async fn link_search(repo: web::Data<Repository>) -> HttpResponse {
  println!("hello");
  HttpResponse::Ok()
    .content_type("application/json")
    .json(repo.repository.lock().unwrap().link_repository.search())
}

pub async fn link_pick(repo: web::Data::<Repository>, path: web::Path::<u64>) -> HttpResponse {
  let id: domain::setting::Id = domain::setting::Id::from(path.into_inner());
  match repo.repository.lock().unwrap().link_repository.pick(id) {
    Ok(v) => HttpResponse::Ok().content_type("application/json").json(v),
    Err(_) => HttpResponse::NotFound().finish()
  }
}

#[derive(Serialize, Deserialize)]
pub struct NeededJson {
  user: domain::setting::User,
  info: domain::link::InfoUser,
  body: domain::link::Body
}

pub async fn link_post(repo: web::Data::<Repository>, content: web::Json::<NeededJson>) -> HttpResponse {
  let input: NeededJson = content.into_inner();
  match repo.repository.lock().unwrap().link_repository.post(input.user, input.info, input.body) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().finish()
  }
}