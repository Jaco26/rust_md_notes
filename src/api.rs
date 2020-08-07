use actix_web::HttpResponse;
use actix_web::web::{Path, Json, Data};

use comrak::{markdown_to_html, ComrakOptions};

use crate::db::Db;
use crate::model::{MdFile, NewMdFile, UpdateMdFile};


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

pub async fn update_file(file: Json<UpdateMdFile>, db: Data<Db>) -> HttpResponse {

  let updated_file = MdFile {
    name: file.name.clone(),
    markdown: file.markdown.clone(),
    html: markdown_to_html(&file.markdown, &ComrakOptions::default())
  };

  match db.update_file_content(updated_file) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    } 
  }
}