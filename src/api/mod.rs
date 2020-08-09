use actix_web::web;

mod dir;
mod file;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/dir")
    .route("/", web::post().to(dir::create_dir))
    .route("/{id}", web::get().to(dir::get_dir))
    .route("/list", web::get().to(dir::list_dirs))
    .route("/tree", web::get().to(dir::dir_tree))
  );
  cfg.service(web::scope("/file")
    .route("/", web::post().to(file::create_file))
  );
}