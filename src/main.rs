#[macro_use]
extern crate log;

extern crate rusqlite;

use actix_files as fs;
use actix_web::{web, error, HttpServer, HttpResponse, App, Error};
use actix_web::middleware::Logger;
use tera::{Tera, Context};
use comrak;

mod util;
mod db;
mod model;
mod api;

use db::DbV2;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "markdown_editor=debug,actix_web=info");
    env_logger::init();

    let db = DbV2::new("./db.sql").unwrap_or_else(|err| {
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
            .service(web::resource("/api/file")
                .route(web::post().to(api::create_file))
            )
            .service(web::resource("/api/dir/{id}")
                .route(web::get().to(api::get_dir))
            )
            .service(web::resource("/api/dir")
                .route(web::post().to(api::create_dir))
            )
            .service(web::resource("/api/dirnames")
                .route(web::get().to(api::get_dirnames))
            )

            .service(fs::Files::new("/js", "./static/js/"))
            .service(fs::Files::new("/css", "./static/css/"))
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}

// async fn editor_page(file: web::Path<model::NewMdFile>, tmpl: web::Data<Tera>, db: web::Data<Db>) -> actix_web::Result<HttpResponse, Error> {
//     let filenames = db.get_all_file_names().expect("[editor_page] Couldn't load filenames");
//     let file = db.get_file(&file.name).unwrap().unwrap();
//     let mut ctx = Context::new();
//     ctx.insert("filenames", &filenames);
//     ctx.insert("file", &file);
//     let html = tmpl.render("page/editor.html", &ctx).map_err(|err| error::ErrorInternalServerError(err))?;
//     Ok(HttpResponse::Ok().content_type("text/html").body(html))
// }

// async fn home_page(tmpl: web::Data<Tera>, db: web::Data<Db>) -> actix_web::Result<HttpResponse, Error> {
//     let filenames = db.get_all_file_names().expect("[home_page] Couldn't load filenames");
//     let mut ctx = Context::new();
//     ctx.insert("filenames", &filenames);
//     let html = tmpl.render("page/home.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template Error"))?;
//     Ok(HttpResponse::Ok().content_type("text/html").body(html))
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let db = DbV2::new("./db.sql").expect("Couldn't create database");
//     HttpServer::new(move || {
//         let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
//         App::new()
//             .data(tera)
//             .data(db.clone())
//             .service(web::resource("/api/file")
//                 .route(web::post().to(api::create_file))
//                 .route(web::put().to(api::update_file))
//             )
//             .service(web::resource("/api/file/{name}")
//                 .route(web::get().to(api::get_file))
//             )
//             .service(web::resource("/").route(web::get().to(home_page)))
//             .service(web::resource("/page/{name}").route(web::get().to(editor_page)))
//             .service(fs::Files::new("/js", "./static/js/"))
//             .service(fs::Files::new("/css", "./static/css/"))

//     })
//     .bind("0.0.0.0:3000")?
//     .run()
//     .await
// }
