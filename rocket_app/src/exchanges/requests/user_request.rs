use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::users;


#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct UserRequest {
    name: String,
    email: String,
}
