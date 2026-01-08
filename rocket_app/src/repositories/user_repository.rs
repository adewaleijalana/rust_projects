use crate::{
    exchanges::requests::user_request::UserRequest,
    models::user::User,
    repositories::{base_create_repository::BaseCreateRepository, base_repository::BaseRepository},
    schema::users,
};
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

    fn delete_by_id(id: i32, conn: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::delete(users::table)
            .filter(users::id.eq(id))
            .execute(conn)
    }
}

impl BaseCreateRepository<User, UserRequest, users::table> for UserRepository {
    fn save(conn: &mut SqliteConnection, new_entity: UserRequest) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_entity)
            .execute(conn)?;

        let id = Self::last_inserted_id(conn, users::table)?;

        Self::find_by_id(id, conn)
    }

    fn last_inserted_id(conn: &mut SqliteConnection, entity: users::table) -> QueryResult<i32> {
        entity.select(users::id).order(users::id.desc()).first(conn)
    }
}
