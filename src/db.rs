use rusqlite::{self, Connection, Error, NO_PARAMS};

use crate::model::MdFile;

#[derive(Clone)]
pub struct Db {
    location: String,
}

impl Db {
    pub fn new(location: &str) -> Result<Db, Error> {
        let db = Db { location: location.to_string() };
        let conn = db.open_conn()?;
        let sql = "CREATE TABLE IF NOT EXISTS files (
                    name TEXT PRIMARY KEY,
                    markdown TEXT NOT NULL,
                    html TEXT NOT NULL
                  )";
        conn.execute(sql, NO_PARAMS)?; 
        Ok(db)
    }
    fn open_conn(&self) -> Result<Connection, Error> {
        rusqlite::Connection::open(&self.location)
    }
    fn connect(&self) -> Result<Connection, Error> {
      rusqlite::Connection::open(&self.location)
    }
    pub fn create_file(&self, name: &str) -> Result<(), Error>  {
      let sql = "INSERT INTO files (name, markdown, html)
                VALUES (?1, ?2, ?3)";
      let params = &[&name, "", ""];
      self.connect()?.execute(sql, params)?;
      Ok(())
    }
    pub fn get_file(&self, name: &str) -> Result<Option<MdFile>, Error> {
      let sql = "SELECT * FROM files WHERE name = :name";
      let conn = self.connect()?;
      let mut stmt = conn.prepare(sql)?;
      let rows: Vec<MdFile> = stmt.query_map_named(&[(":name", &name)], |row| {
        Ok(MdFile {
          name: row.get(0)?,
          markdown: row.get(1)?,
          html: row.get(2)?,
        })
      })?.map(|x| x.unwrap()).collect();
      if let Some(first) = rows.first() {
        return Ok(Some(first.clone()));
      }
      Ok(None)
    }
    pub fn get_all_files(&self) -> Result<Vec<MdFile>, Error> {
      let sql = "SELECT * FROM files";
      let conn = self.connect()?;
      let mut stmt = conn.prepare(sql)?;
      let rows = stmt.query_map(NO_PARAMS, |row| {
        Ok(MdFile {
          name: row.get(0)?,
          markdown: row.get(1)?,
          html: row.get(2)?,
        })
      })?;
      Ok(rows.map(|x| x.unwrap()).collect())
    }
    pub fn get_all_file_names(&self) -> Result<Vec<String>, Error> {
      let sql = "SELECT name FROM files";
      let conn = self.connect()?;
      let mut stmt = conn.prepare(sql)?;
      let file_names = stmt
        .query_map(NO_PARAMS, |row| row.get(0))?
        .map(|n| n.unwrap())
        .collect();
      Ok(file_names)
    }
    pub fn update_file_name(&self, old_name: &str, new_name: &str) -> Result<(), Error> {
      let sql = "UPDATE files SET name = ?1 WHERE name = ?2";
      let params = &[new_name, old_name];
      self.connect()?.execute(sql, params)?;
      Ok(())
    }
    pub fn update_file_content(&self, file: MdFile) -> Result<(), Error> {
      let sql = "UPDATE files SET markdown = $1, html = $2 WHERE name = ?3";
      let params = &[&file.markdown, &file.html, &file.name];
      self.connect()?.execute(sql, params)?;
      Ok(())
    }
    pub fn delete_file(&self, name: &str) -> Result<(), Error>{
      let sql = "DELETE * FROM files WHERE name = ?1";
      let params = &[name];
      self.connect()?.execute(sql, params)?;
      Ok(())
    }  
}