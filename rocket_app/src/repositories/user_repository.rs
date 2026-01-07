use crate::{models::user::User, repositories::base_repository::BaseRepository, schema::users};
use diesel::prelude::*;

pub struct UserRepository;

impl BaseRepository<User> for UserRepository {
    fn find_by_id(id: i32, conn: &mut SqliteConnection) -> QueryResult<User> {
        users::table.filter(users::id.eq(id)).first::<User>(conn)
    }

    fn find_all(conn: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table
            .order(users::id.asc())
            .limit(limit)
            .load::<User>(conn)
    }
}
