use rusqlite::{self, Connection, Error, NO_PARAMS};

use crate::util;
use crate::model::api_model::send::{
  Directory,
  DirectoryItem,
};



#[derive(Clone)]
pub struct DbV2 {
  location: String,
}

impl DbV2 {
  pub fn new(location: &str) -> Result<DbV2, Error> {
    let sql_create_tables = "
      CREATE TABLE IF NOT EXISTS dir (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL
      );
      CREATE TABLE IF NOT EXISTS dir_child_directory (
        id TEXT PRIMARY KEY,
        parent_dir_id TEXT,
        FOREIGN KEY (parent_dir_id) REFERENCES dir (id)
      );
      CREATE TABLE IF NOT EXISTS dir_file (
        id TEXT PRIMARY KEY,
        parent_dir_id TEXT,
        FOREIGN KEY (parent_dir_id) REFERENCES dir (id)
      );
      CREATE TABLE IF NOT EXISTS file (
        id TEXT PRIMARY KEY,
        item_id TEXT, 
        name TEXT NOT NULL,
        markdown TEXT NOT NULL,
        html TEXT NOT NULL,
        FOREIGN KEY (item_id) REFERENCES dir_file (id)
      );  
    ";

    let sql_seed = "INSERT INTO dir (id, name) VALUES (?1, 'root');";
  
    let db = DbV2 { location: location.to_string() };
    let conn = db.open_conn()?;
    conn.execute_batch(sql_create_tables)?;
    if !db.exists(&conn, "SELECT * FROM dir WHERE name = 'root'", NO_PARAMS)? {
      conn.execute(sql_seed, &[&util::uuid()])?; 
    }
    Ok(db)
  }
  fn open_conn(&self) -> Result<Connection, Error> {
    rusqlite::Connection::open(&self.location)
  }
  fn connect(&self) -> Result<Connection, Error> {
    rusqlite::Connection::open(&self.location)
  }
  fn exists(&self, conn: &rusqlite::Connection, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<bool, Error> {
    conn.prepare(sql)?.exists(params)
  }
  pub fn create_file(&self, name: &str, parent_dir_id: &str) -> Result<(), Error> {
    let sql_dir_file = "INSERT INTO dir_file (id, parent_dir_id) VALUES (?1, ?2);";
    let sql_file = "INSERT INTO file (id, item_id, name, markdown, html)
                    VALUES (?1, ?2, ?3, ?4, ?5);";
    let dir_item_id = util::uuid();
    let conn = self.connect()?;
    conn.execute(sql_dir_file, &[&dir_item_id, parent_dir_id])?;
    conn.execute(sql_file, &[&util::uuid(), &dir_item_id, name, "", ""])?;
    Ok(())
  }
  pub fn create_dir(&self, name: &str, parent_dir_id: &str) -> Result<(), Error> {
    let sql_dir = "INSERT INTO dir (id, name) VALUES (?1, ?2);";
    let sql_dir_child_directory = "INSERT INTO dir_child_directory (id, parent_dir_id) VALUES (?1, ?2);";
    let dir_id = util::uuid();
    let conn = self.connect()?;
    conn.execute(sql_dir, &[&dir_id, name])?;
    conn.execute(sql_dir_child_directory, &[&dir_id, parent_dir_id])?;
    Ok(())
  }
  pub fn get_dir(&self, dir_id: &str) -> Result<Directory, Error> {
    let conn = self.connect()?;

    let mut stmt_dir = conn.prepare("SELECT * FROM dir WHERE id = ?1;")?;
    let mut dir = stmt_dir.query_row(&[dir_id], |row| {
      Ok(Directory {
        id: dir_id.to_string(),
        name: row.get(1)?,
        files: Vec::new(),
        child_dirs: Vec::new(),
      })
    })?;

    let mut stmt_dir_files = conn.prepare("
      SELECT dir_file.id, file.name
      FROM dir_file INNER JOIN file
        ON dir_file.id = file.item_id
      WHERE dir_file.parent_dir_id = :id;
    ")?;
    let dir_files: Vec<DirectoryItem> = stmt_dir_files.query_map_named(&[(":id", &dir_id)], |row| {
      Ok(DirectoryItem {
        id: row.get(0)?,
        name: row.get(1)?,
      })
    })?.map(|x| x.unwrap()).collect();

    dir.files = dir_files;

    let mut stmt_dir_child_dirs = conn.prepare("
      SELECT dcd.id, (SELECT name FROM dir WHERE id = dcd.id)
      FROM dir_child_directory dcd
      WHERE dcd.parent_dir_id = :id
    ")?;
    let dir_child_dirs: Vec<DirectoryItem> = stmt_dir_child_dirs.query_map_named(&[(":id", &dir_id)], |row| {
      Ok(DirectoryItem {
        id: row.get(0)?,
        name: row.get(1)?,
      })
    })?.map(|x| x.unwrap()).collect();

    dir.child_dirs = dir_child_dirs;

    Ok(dir)
  }
  pub fn get_dirnames(&self) -> Result<Vec<DirectoryItem>, Error> {
    let conn = self.connect()?;
    let mut stmt = conn.prepare("SELECT * FROM dir;")?;
    let rows: Vec<DirectoryItem> = stmt.query_map(NO_PARAMS, |row| {
      Ok(DirectoryItem {
        id: row.get(0)?,
        name: row.get(1)?,
      })
    })?.map(|x| x.unwrap()).collect();
    Ok(rows)
  }
}
