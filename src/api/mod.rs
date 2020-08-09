use actix_web::web;

pub mod dir;
pub mod file;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/dir")
      .route("/", web::post().to(dir::create_dir))
      .route("/tree", web::get().to(dir::dir_tree))
      .service(web::resource("/list")
        .route(web::get().to(dir::list_dirs))
      )
      .service(web::resource("/{id}")
        .route(web::get().to(dir::get_dir))
        .route(web::put().to(dir::update_dir_name))
        .route(web::delete().to(dir::delete_dir))
      )
    )
    .service(web::scope("/file")
      .route("/", web::post().to(file::create_file))
    );
}