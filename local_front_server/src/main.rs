//use actix_web::{web , get , App, HttpServer , HttpResponse , web::ServiceConfig};
//use ownlinkmemo_domain as domain;
//use ownlinkmemo_domain::link::LinkRepository;
use ownlinkmemo_api_server as api;
use ownlinkmemo_infra_testmemory as infra;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    api::server::create_server::<infra::TestLinkRepository>(infra::TestLinkRepository::init()).await
}