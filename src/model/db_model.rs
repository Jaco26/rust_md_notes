use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Dir {
  pub id: String,
  pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DirItem {
  pub id: String,
  pub parent_dir_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MdFile {
  id: String,
  item_id: String,
  name: String,
  markdown: String,
  html: String,
}
