use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct NewMdFile {
  pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MdFile {
  pub name: String,
  pub markdown: String,
  pub html: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateMdFile {
  pub name: String,
  pub markdown: String,
}
