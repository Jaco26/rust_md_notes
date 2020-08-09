use actix_web::web;

pub mod dir;
pub mod file;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/api")
      .service(web::scope("/dir")
        .route("/", web::post().to(dir::create_dir))
        .route("/list", web::get().to(dir::list_dirs))
        .route("/tree", web::get().to(dir::dir_tree))
      )
      // .route("/dir/list", web::get().to(dir::list_dirs))
      // .service(web::resource("/dir/list")
      //   .route(web::get().to(dir::list_dirs))
      // )
    );
  //   .route("/dir/", web::post().to(dir::create_dir))
  //   .route("/dir/{id}", web::get().to(dir::get_dir))
  //   .service(web::resource("/dir/list")
  //     .route(web::get().to(dir::list_dirs))
  //   ) 
  //   // .route("/dir/list", web::get().to(dir::list_dirs))
  //   .route("/dir/tree", web::get().to(dir::dir_tree))
  //   .route("/file/", web::post().to(file::create_file));
  // cfg.service(web::scope("/file")
  //   .route("/", web::post().to(file::create_file))
  // );
}