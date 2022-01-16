// Copyright ⓒ 2021-2022 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).

use log::info;

use rusqlite::{Connection, Result};

/// Connect to sqlite database and process tables
///
/// # Arguments
///
/// * `path` - Relative or absolute path to the database
///
/// # Example
///
/// ```
/// assert_eq!(get_schemas_from_sqlite_instance("sqlite3.db"), "CREATE TABLE [MYTABLE] ...");
/// ```
pub fn get_schemas_from_sqlite_instance(path: &str) -> Result<String, rusqlite::Error> {
    let conn = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    info!("Connection with database {} done", "foo");
    let mut stmt = conn.prepare("SELECT sql FROM sqlite_master WHERE sql IS NOT NULL;")?;
    let mut rows = stmt.query([])?;
    info!("Query to sqlite db done");
    let mut schemas: Vec<String> = vec![];
    while let Some(row) = rows.next()? {
        schemas.push(row.get(0)?);
    }
    info!("Schema parsed successfully from sqlite3 database");
    Ok(schemas.join(";\n"))
}
