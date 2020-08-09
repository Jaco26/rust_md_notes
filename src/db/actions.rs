use rusqlite::{Connection, Error, NO_PARAMS};

use crate::model::api_model::{
  receive::{
    UpdateFileContentJson
  },
  send::{
    MdFile,
    MdFileHtml,
    Directory,
    DirectoryItem,
  }
};

pub fn update_file_content(
  conn: &Connection,
  file_id: &str,
  markdown: &str,
  html: &str
) -> Result<(), Error> {
  let sql = "UPDATE file SET markdown = ?2, html = ?1 WHERE id = ?3";
  let params = &[&html, &markdown, &file_id];
  conn.execute(sql, params)?;
  Ok(())
}

pub fn get_file(conn: &Connection, file_id: &str) -> Result<MdFile, Error> {
  let mut stmt = conn.prepare("SELECT * FROM file WHERE id = ?1")?;
  let row = stmt.query_row(&[file_id], |row| {
    Ok(MdFile {
      id: row.get(0)?,
      item_id: row.get(1)?,
      name: row.get(2)?,
      markdown: row.get(3)?,
      html: row.get(4)?,
    })
  })?;
  Ok(row)
}

pub fn get_file_html(conn: &Connection, file_id: &str) -> Result<MdFileHtml, Error> {
  let mut stmt = conn.prepare("SELECT html FROM file WHERE id = ?1")?;
  let row = stmt.query_row(&[file_id], |row| {
    Ok(MdFileHtml {
      html: row.get(0)?
    })
  })?;
  Ok(row)
}

pub fn delete_file(conn: &Connection, file_id: &str) -> Result<(), Error> {
  conn.execute("DELETE FROM file WHERE id = ?1", &[file_id])?;
  Ok(())
}

