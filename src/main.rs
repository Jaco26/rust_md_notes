#[macro_use]
extern crate log;

extern crate rusqlite;

use actix_files as fs;
use actix_web::{web, HttpServer, App};
use actix_web::middleware::Logger;
use tera::Tera;

mod util;
mod db;
mod model;
mod api;

use db::Db;

async fn index() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/index.html")?)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "markdown_editor=debug,actix_web=info");
    env_logger::init();

    let db = Db::new("./data.sqlite").unwrap_or_else(|err| {
        panic!("{:?}", err);
    });
    let md = util::Markdown::new();
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .data(md.clone())
            .data(tera.clone())
            .wrap(Logger::new("%r %s"))
            .service(web::scope("/api").configure(api::register_routes))
            .service(fs::Files::new("/js", "./static/js/"))
            .service(fs::Files::new("/css", "./static/css/"))
            .service(web::resource("/*").route(web::get().to(index)))
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}

