use crate::entity::user_schema::users::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::QueryResult;
use crate::entity::user_entity::User;
use crate::entity::user_entity::NewUser;
use crate::entity::user_entity::UpdateUser;

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

    pub fn update_user(
        conn: &mut PgConnection,
        user_id: i32,
        updated_user: &UpdateUser,
    ) -> QueryResult<User> {
        diesel::update(users.filter(id.eq(user_id)))
            .set(updated_user)
            .get_result(conn)
    }

    pub fn get_user_by_id(conn: &mut PgConnection, user_id: i32) -> QueryResult<User> {
        users.filter(id.eq(user_id)).first::<User>(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<(), diesel::result::Error> {
        diesel::delete(users.filter(id.eq(user_id)))
            .execute(conn)
            .map(|rows_affected| {
                if rows_affected == 0 {
                    Err(diesel::result::Error::NotFound)
                } else {
                    Ok(())
                }
            })?
    }
}

