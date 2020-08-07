use actix_web::HttpResponse;
use actix_web::web::{Path, Json, Data};

use crate::db::Db;
use crate::model::NewMdFile;


pub async fn create_file(new_file: Json<NewMdFile>, db: Data<Db>) -> HttpResponse {
  match db.create_file(&new_file.name) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn get_file(params: Path<(String,)>, db: Data<Db>) -> HttpResponse {
  match db.get_file(&params.0) {
    Ok(option) => match option {
      Some(file) => HttpResponse::Ok().json(file),
      None => HttpResponse::NotFound().finish()
    }
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

