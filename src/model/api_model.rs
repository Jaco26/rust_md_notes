
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

}




/*

{
  "root": ["dir_item_id", "dir_item_id"]
  ""
}

enum DirItemKind {
  Dir,
  File,
}

struct DirectoryItem {
  id: String,
  name: String,
  kind: DirItemKind,
}

struct Directory {
  id: String,
  items: Vec<DirectoryItem>
}

*/