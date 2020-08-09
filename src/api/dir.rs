use actix_web::HttpResponse;
use actix_web::web::{Path, Json, Data};

use crate::db::{self, Db};
use crate::model::api_model::receive::{
  CreateDirItemParams,
  GetDirParams,
  UpdateDirNameJson
};

pub async fn delete_dir(db: Data<Db>, params: Path<(String,)>) -> HttpResponse {
  let (dir_id,) = params.into_inner();
  match db.delete_dir(&dir_id) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn update_dir_name(
  db: Data<Db>,
  params: Path<(String,)>,
  body: Json<UpdateDirNameJson>
) -> HttpResponse {
  let (dir_id,) = params.into_inner();
  let UpdateDirNameJson { name } = &body.0;
  match db.update_dir_name(&dir_id, &name) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}


pub async fn create_dir(db: Data<Db>, item: Json<CreateDirItemParams>) -> HttpResponse {
  let CreateDirItemParams { name, parent_dir_id } = &item.0;
  match db.create_dir(name, parent_dir_id) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

/// Return a flat list of all directory names and ids
pub async fn list_dirs(db: Data<Db>) -> HttpResponse {
  match db.list_dirs() {
    Ok(dirnames) => HttpResponse::Ok().json(dirnames),
    Err(error) => {
      eprintln!("{:?}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

/// Return a full tree of all directorys and files. Top is `"root"`
pub async fn dir_tree(db: Data<Db>) -> HttpResponse {
  HttpResponse::Ok().body("not implemented")
}

pub async fn get_dir(db: Data<Db>, params: Path<GetDirParams>) -> HttpResponse {
  let GetDirParams { id } = &params.into_inner();
  match db.get_dir(id) {
    Ok(dir) => HttpResponse::Ok().json(dir),
    Err(error) => {
      eprintln!("{}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

