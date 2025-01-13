use crate::entity::user_schema::users;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable};
use diesel::AsChangeset;
use validator_derive::Validate; // Importa la macro derive


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
#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 20))]
    pub password: String
}

#[derive(AsChangeset, Deserialize, Serialize, Validate)]
#[diesel(table_name = users)]
pub struct UpdateUser {

    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,   // Campos opcionales
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8, max = 20))]
    pub password: Option<String>
}
