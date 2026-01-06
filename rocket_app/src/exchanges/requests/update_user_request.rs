use diesel::prelude::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUserRequest {
    name: String,
    email: String,
}
