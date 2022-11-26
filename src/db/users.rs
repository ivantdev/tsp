use crate::db::connection::{establish_connection};
use crate::db::models::users::*;
use crate::schema;
use diesel::prelude::*;

pub fn get_user(query: &str) -> Result<Vec<User>, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(email.eq(&query).or(username.eq(&query)))
        .load::<User>(connection)
        .expect("Error loading user");

    Ok(results)
}

pub fn get_user_by_id(user_id: &i32) -> Result<User, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let result = users
        .find(user_id)
        .get_result::<User>(connection)
        .expect("Error loading user");

    Ok(result)
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
