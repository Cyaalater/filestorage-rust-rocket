#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use chrono::Local;
use rocket::http::ext::IntoCollection;
use crate::schema::users::hashed_password;
use self::models::*;
use self::diesel::prelude::*;

pub fn db_establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn db_add_file<'a>(conn: &SqliteConnection, name: &'a str, description: &'a str, path: &'a str, uploader: &'a str){
    use schema::files;
    let current_time = Local::now().to_string();
    let new_file = NewFile {
        name,
        description,
        path,
        uploader,
        date : &current_time.as_str().clone()
    };
    diesel::insert_into(files::table)
        .values(&new_file)
        .execute(conn)
        .expect("Error saving a post");
}

pub fn db_search_file(conn: &SqliteConnection, input_name: String) {
    use schema::files::dsl::*;
    let results = files.filter(name.like(format!("%{}%",input_name)))
        .limit(5)
        .load::<Files>(conn);
    println!("{:?}",results.unwrap())
}

pub fn db_find_file(conn: &SqliteConnection,input_id: i32) -> Files{
    use schema::files::dsl::*;
    // use schema::files;
    files.find(input_id)
        .first(conn)
        .expect("Error: Searching in query")
}

pub fn db_show_files(conn: &SqliteConnection) -> Vec<Files>{
    use schema::files::dsl::*;
    let results = files.load::<Files>(conn);
    results.unwrap()
}

// TODO: Add the index function to files

pub fn db_add_user<'a>(conn: &SqliteConnection, username: &'a str, user_hashed_password: &'a str){
    use schema::users;
    let new_user = NewUser {
        username,
        hashed_password: user_hashed_password,
        permissions: &1
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving a user");
}