
pub mod receive {
  use serde::Deserialize;

  #[derive(Deserialize)]
  pub struct CreateDirItemParams {
    pub name: String,
    pub parent_dir_id: String,
  }

  #[derive(Deserialize)]
  pub struct GetDirParams {
    pub id: String,
  }

  #[derive(Deserialize)]
  pub struct UpdateDirNameJson {
    pub name: String,
  }

  #[derive(Deserialize)]
  pub struct UpdateFileContentJson {
    pub markdown: String,
  }
}



pub mod send {
  use serde::Serialize;

  #[derive(Debug, Serialize)]
  pub struct DirectoryItem {
    pub id: String,
    pub name: String,
  }

  #[derive(Serialize)]
  pub struct Directory {
    pub id: String,
    pub name: String,
    pub files: Vec<DirectoryItem>,
    pub child_dirs: Vec<DirectoryItem>
  }

  #[derive(Serialize)]
  pub struct MdFile {
    pub id: String,
    pub item_id: String,
    pub name: String,
    pub markdown: String,
    pub html: String,
  }

  #[derive(Serialize)]
  pub struct MdFileHtml {
    pub html: String,
  }

}
