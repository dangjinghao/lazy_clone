use actix_web::{App, HttpServer, web, web::Data};

use clap::Parser;
use lazy_clone::{args::Args, service::catch_all};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let bind_addr = args.bind.clone();
    println!("Starting server at {}", bind_addr);
    let args_data = Data::new(args);
    HttpServer::new(move || {
        App::new()
            .app_data(args_data.clone())
            .default_service(web::to(catch_all))
    })
    .bind(bind_addr)?
    .run()
    .await
}
