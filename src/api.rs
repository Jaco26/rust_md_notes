use actix_web::HttpResponse;
use actix_web::web::{Path, Json, Data};

use crate::util::Markdown;
use crate::db::DbV2;
use crate::model::api_model::receive::{
  CreateDirItemParams,
  GetDirParams
};


pub async fn create_dir(db: Data<DbV2>, item: Json<CreateDirItemParams>) -> HttpResponse {
  let CreateDirItemParams { name, parent_dir_id } = &item.0;
  match db.create_dir(name, parent_dir_id) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn create_file(db: Data<DbV2>, item: Json<CreateDirItemParams>) -> HttpResponse {
  let CreateDirItemParams { name, parent_dir_id } = &item.0;
  match db.create_file(name, parent_dir_id) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn get_dir(db: Data<DbV2>, params: Path<GetDirParams>) -> HttpResponse {
  let GetDirParams { id } = &params.into_inner();
  match db.get_dir(id) {
    Ok(dir) => HttpResponse::Ok().json(dir),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}


pub async fn get_dirnames(db: Data<DbV2>) -> HttpResponse {
  debug!("Hello from get_dirnames!");
  match db.get_dirnames() {
    Ok(dirnames) => HttpResponse::Ok().json(dirnames),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}