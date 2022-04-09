use ownlinkmemo_infra_file as lib;
use actix_web::{get , post , web , App , HttpResponse , HttpServer , Responder , Result};

#[get("/links/list/")]
pub async fn listup(config: web::Data<&lib::config::Config>) -> std::io::Result<impl Responder> {
  let list = lib::links::listup(&config)?;
  Ok(web::Json(list))
}