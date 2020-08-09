#[macro_use]
extern crate log;

extern crate rusqlite;

use actix_files as fs;
use actix_web::{web, error, HttpServer, HttpResponse, App, Error};
use actix_web::middleware::Logger;
use tera::{Tera, Context};

mod util;
mod db;
mod model;
mod api;

use db::Db;


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
            // .service(web::resource("/api/dir/list")
            //     .route(web::get().to(api::dir::list_dirs)))
            // .route("/api/dir/", web::post().to(api::dir::create_dir))
            // .route("/api/dir/{id}", web::get().to(api::dir::get_dir))
            // .route("/api/dir/tree", web::get().to(api::dir::dir_tree))
            // .route("/api/file/", web::post().to(api::file::create_file))
            .configure(api::register_routes)
            // .service(
            //     web::scope("/api")
            //         .configure(api::register_routes)
            // )
            .service(fs::Files::new("/js", "./static/js/"))
            .service(fs::Files::new("/css", "./static/css/"))
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}

