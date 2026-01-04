use diesel::SqliteConnection;
use rocket_sync_db_pools::database;

#[database("sqlite")]
pub struct DBConn(SqliteConnection);