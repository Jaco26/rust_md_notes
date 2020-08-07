extern crate rusqlite;

use actix_files as fs;
use actix_web::{web, error, HttpServer, HttpResponse, App, Error};
use tera::{Tera, Context};

mod db;
mod model;
mod api;

use db::Db;

async fn editor_page(file: web::Path<model::NewMdFile>, tmpl: web::Data<Tera>, db: web::Data<Db>) -> actix_web::Result<HttpResponse, Error> {
    let filenames = db.get_all_file_names().expect("[editor_page] Couldn't load filenames");
    let file = db.get_file(&file.name).unwrap().unwrap();
    let mut ctx = Context::new();
    ctx.insert("filenames", &filenames);
    ctx.insert("file", &file);
    let html = tmpl.render("page/editor.html", &ctx).map_err(|err| error::ErrorInternalServerError(err))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn home_page(tmpl: web::Data<Tera>, db: web::Data<Db>) -> actix_web::Result<HttpResponse, Error> {
    let filenames = db.get_all_file_names().expect("[home_page] Couldn't load filenames");
    let mut ctx = Context::new();
    ctx.insert("filenames", &filenames);
    let html = tmpl.render("page/home.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template Error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = Db::new("./db.sql").expect("Couldn't create database");
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .data(tera)
            .data(db.clone())
            .service(web::resource("/api/file")
                .route(web::post().to(api::create_file))
            )
            .service(web::resource("/").route(web::get().to(home_page)))
            .service(web::resource("/page/{name}").route(web::get().to(editor_page)))
            .service(fs::Files::new("/js", "./static/js/"))
            .service(fs::Files::new("/css", "./static/css/"))

    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
