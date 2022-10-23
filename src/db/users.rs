use crate::db::connection::establish_connection;
use crate::db::models::users::*;
use crate::schema;
use diesel::prelude::*;

pub fn get_user(q_email: &str) -> Result<Vec<User>, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(email.eq(&q_email))
        .load::<User>(connection)
        .expect("Error loading user");

    Ok(results)
}

pub fn create_user(
    name: &String,
    username: &String,
    email: &String,
    salt: &String,
    password: &String,
    created_on: &diesel::dsl::now,
) -> Result<Vec<User>, diesel::result::Error> {
    use schema::users;
    let connection = &mut establish_connection();

    let new_user: NewUser = NewUser {
        name,
        username,
        email,
        salt,
        password,
        created_on,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_results::<User>(connection)

}
