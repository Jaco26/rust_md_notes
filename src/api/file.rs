use actix_web::HttpResponse;
use actix_web::web::{Json, Data, Path};

use crate::util;
use crate::db::{self, Db};
use crate::model::api_model::receive::{
  CreateDirItemParams,
  UpdateFileContentJson,
};


pub async fn create_file(db: Data<Db>, item: Json<CreateDirItemParams>) -> HttpResponse {
  let CreateDirItemParams { name, parent_dir_id } = &item.0;
  match db.create_file(name, parent_dir_id) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}


pub async fn delete_file(db: Data<Db>, params: Path<(String,)>) -> HttpResponse {
  let (file_id,) = params.into_inner();
  let conn = db.conn().expect("connection error");
  match db::actions::delete_file(&conn, &file_id) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{:?}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn update_file_content(
  db: Data<Db>,
  md: Data<util::Markdown>,
  params: Path<(String,)>,
  body: Json<UpdateFileContentJson>
) -> HttpResponse {
  let (file_id,) = params.into_inner();
  let UpdateFileContentJson { markdown } = body.0;
  let html = md.html(&markdown);
  let conn = db.conn().expect("connection error");
  match db::actions::update_file_content(&conn, &file_id, &markdown, &html) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{:?}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn get_file(db: Data<Db>, params: Path<(String,)>) -> HttpResponse {
  let (file_id,) = params.into_inner();
  let conn = db.conn().expect("connection error");
  match db::actions::get_file(&conn, &file_id) {
    Ok(file) => HttpResponse::Ok().json(file),
    Err(error) => {
      eprintln!("{:?}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn get_file_html(db: Data<Db>, params: Path<(String,)>) -> HttpResponse {
  let (file_id,) = params.into_inner();
  let conn = db.conn().expect("connection error");
  match db::actions::get_file_html(&conn, &file_id) {
    Ok(file) => HttpResponse::Ok().json(file),
    Err(error) => {
      eprintln!("{:?}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}



