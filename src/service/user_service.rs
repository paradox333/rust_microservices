use crate::entity::user_schema::users::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::QueryResult;
use crate::entity::user_entity::User;

pub struct UserService;

impl UserService {
    pub fn get_all_users(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
}

}

