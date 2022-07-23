use actix_web::{web , get , App, HttpServer , HttpResponse , web::ServiceConfig , Responder , middleware::Logger};
use actix_files as fs;
use ownlinkmemo_domain as domain;
//use ownlinkmemo_domain::link::LinkRepository;
use ownlinkmemo_api_server as api;
use ownlinkmemo_infra_testmemory as infra;
use env_logger::Env;

fn configure_static(cfg: &mut web::ServiceConfig){
    cfg.service(fs::Files::new("pages", "./static/").show_files_listing());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move ||{
        App::new()
        .app_data(web::Data::new(
            api::link::Repository{
                repository: std::sync::Mutex::new(infra::init())
            }
        ))
        .configure(api::server::configure_api)
        .configure(configure_static)
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
}