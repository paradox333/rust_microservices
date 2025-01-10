use crate::entity::user_schema::users::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::QueryResult;
use crate::entity::user_entity::User;
use crate::entity::user_entity::NewUser;

pub struct UserService;

impl UserService {
    pub fn get_all_users(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
}

    pub fn create_user(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users)
        .values(new_user)
        .get_result(conn)
    }

}

