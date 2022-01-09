use crate::args::Args;

use log::{error, info};

use mysql::prelude::Queryable;
use mysql::{Opts, Pool};

/** Connect to given remote database and process tables
 *
 * # Arguments
 *
 * * `args` - User command lines arguments
 */
pub fn process_mysql_connection(args: &mut Args, opts: Opts) -> Result<(), mysql::Error> {
    let mut tables: Vec<String> = vec![];
    let mut file: String = String::new();
    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn().unwrap();
    info!("Connection successfull with remote database");
    conn.query_map(r"SHOW TABLES;", |table_name: String| {
        tables.push(table_name)
    })
    .unwrap();
    for table in tables.iter() {
        if let Err(e) = conn.query_map(
            format!("SHOW CREATE TABLE {0};", table),
            |(_, script): (String, String)| file.push_str(format!("{};\n", script).as_str()),
        ) {
            error!("An error happened while querying remote database");
            return Err(e);
        }
    }
    info!(
        "Query made successfully with remote database, {} tables found",
        tables.len()
    );

    args.set_data(file);
    Ok(())
}
