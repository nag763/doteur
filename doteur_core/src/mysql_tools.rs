use log::{error, info};

use mysql::prelude::Queryable;
use mysql::{Opts, OptsBuilder, Pool};

/** Connect to given remote database and process tables
 *
 * # Arguments
 *
 * * `args` - User command lines arguments
 */
fn process_mysql_connection(opts: Opts) -> Result<String, mysql::Error> {
    let mut tables: Vec<String> = vec![];
    let mut data: String = String::new();
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
            |(_, script): (String, String)| data.push_str(format!("{};\n", script).as_str()),
        ) {
            error!("An error happened while querying remote database");
            return Err(e);
        }
    }
    info!(
        "Query made successfully with remote database, {} tables found",
        tables.len()
    );

    Ok(data)
}

pub fn process_mysql_connection_from_url(url: &str) -> Result<String, mysql::Error> {
    let opts: Opts = Opts::from_url(url)?;
    process_mysql_connection(opts)
}

/// Connect to the db and return the data from the given parameters if the connection is
/// succesful
///
/// # Arguments
///
/// * `db_url` - Database url or ip
/// * `db_port` - Database remote port
/// * `db_name` - Database remote schema name
/// * `db_user` - Database remote user
/// * `db_password` - Database remote user's password
pub fn process_mysql_connection_from_params(
    db_url: String,
    db_port: u16,
    db_name: String,
    db_user: String,
    db_password: String,
) -> Result<String, mysql::Error> {
    let opts_builder: OptsBuilder = OptsBuilder::new()
        .ip_or_hostname(Some(db_url))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password));
    process_mysql_connection(Opts::from(opts_builder))
}
