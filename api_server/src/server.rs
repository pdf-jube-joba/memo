use actix_web::{web , App , HttpServer};
use ownlinkmemo_domain as domain;
use crate::link;

pub async fn create_server<T: domain::link::LinkRepository + Clone + Send + Sync + 'static>(repo: T) -> std::io::Result<()> {
  println!("start");
  HttpServer::new(move || {
      App::new()
          .app_data(web::Data::new(repo.clone()))
          .route("/links" , web::get().to(link::link_listup::<T>))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}