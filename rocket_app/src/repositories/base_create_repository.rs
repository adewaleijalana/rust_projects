use diesel::{QueryResult, SqliteConnection};

pub trait BaseCreateRepository<T, U, V> {
    fn save(conn: &mut SqliteConnection, new_entity: U) -> QueryResult<T>;

    fn last_inserted_id(conn: &mut SqliteConnection, entity: V) -> QueryResult<i32>;
}
