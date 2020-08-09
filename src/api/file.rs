use actix_web::HttpResponse;
use actix_web::web::{Json, Data};

use crate::db::Db;
use crate::model::api_model::receive::{
  CreateDirItemParams,

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
