use diesel::{QueryResult, SqliteConnection};

pub trait BaseRepository<T> {
    fn find_by_id(id: i32, conn: &mut SqliteConnection) -> QueryResult<T>;
}