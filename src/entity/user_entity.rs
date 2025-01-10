use crate::entity::user_schema::users;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable};

// Struct para representar un usuario que se obtiene de la base de datos
#[derive(Queryable, Identifiable, Serialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}

// Struct para representar un nuevo usuario que se insertará en la base de datos
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String
}

