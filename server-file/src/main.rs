use actix_web::{web, App, HttpServer};
use ownlinkmemo_infra_file as lib;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: lib::config::Config = lib::config::Config {
        target_dir: String::from("../"),
        data_path: String::from(".memoData"),
        links_path: String::from("Links")
      };
    
    HttpServer::new(move || {
        let config = config.clone();
        App::new()
            .app_data(web::Data::new(move ||{config}))
            .service(api::listup)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}