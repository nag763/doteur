// Copyright ⓒ 2021-2024 LABEYE Loïc
// This tool is distributed under the MIT License, check out [here](https://github.com/nag763/doteur/blob/main/LICENCE.MD).

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
    info!("Connection successfull to remote database");
    conn.query_map(r"SHOW TABLES;", |table_name: String| {
        tables.push(table_name)
    })
    .unwrap();
    for table in tables.iter() {
        if let Err(e) = conn.query_map(
            format!("SHOW CREATE TABLE {0};", table),
            |(_, script): (String, String)| data.push_str(format!("{};\n", script).as_str()),
        ) {
            error!("An error happened while querying remote database : {}", e);
            return Err(e);
        }
    }
    info!(
        "Query made successfully to remote database, {} tables found",
        tables.len()
    );

    Ok(data)
}

/// Connect to the db and return the data from the given parameters if the connection is
/// successfull
///
/// # Arguments
///
/// * `url` - The url to connect to. It has to be in the format `mysql://usr:password@localhost:3306/database`
///
/// # Example
///
/// ```
/// use doteur_core::mysql_tools::get_schemas_from_mysql_params;
/// assert_eq!(
///     get_schemas_from_mysql_url("mysql://usr:password@localhost:3306/database",
///     "CREATE TABLE ...")
/// );
/// ```
///
pub fn get_schemas_from_mysql_url(url: &str) -> Result<String, mysql::Error> {
    let opts: Opts = Opts::from_url(url)?;
    process_mysql_connection(opts)
}

/// Connect to the db and return the data from the given parameters if the connection is
/// successfull
///
/// # Arguments
///
/// * `db_url` - Database url or ip
/// * `db_port` - Database remote port
/// * `db_name` - Database remote schema name
/// * `db_user` - Database remote user
/// * `db_password` - Database remote user's password
///
/// # Example
///
/// ```
/// use doteur_core::mysql_tools::get_schemas_from_mysql_params;
/// assert_eq!(
///     get_schemas_from_mysql_params(
///         "localhost",
///         3306,
///         "schemaname",
///         "usr",
///         "password",
///     ), "CREATE TABLE foo [...]"
/// );
/// ```
pub fn get_schemas_from_mysql_params(
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
