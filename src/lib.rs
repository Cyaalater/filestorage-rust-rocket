#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;
pub mod models;


use std::borrow::Borrow;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::ops::Add;
use chrono::{Local, Duration, DateTime};
use rocket::http::ext::IntoCollection;
use crate::schema::users::hashed_password;
use self::models::*;
use self::diesel::prelude::*;
use hex_literal::hex;
use bcrypt::*;
use multipart::server::SaveResult::Error;
use crate::schema::sessions::session_id;

pub fn db_establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn key_get() -> String{
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY")
        .expect("SECRET_KEY MUST BE SET");
    secret_key
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
    let hashed = hash(user_hashed_password,DEFAULT_COST)
        .expect("Failed hashing the password");
    // let mut hasher = Sha256::new();
    // hasher.update(user_hashed_password);
    // let hash_result = hasher.finalize();
    let new_user = NewUser {
        username,
        hashed_password: hashed.as_str(),
        permissions: &1
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error inserting user");

}

pub fn db_search_user(conn: &SqliteConnection, input_username: String) -> QueryResult<Users> {
    use schema::users::dsl::*;
    users.filter(username.eq(input_username))
        .first(conn)
}

pub fn db_create_session<'a>(conn: &SqliteConnection, user_session_id: &'a str, uuid: &'a i32)
{
    println!("NEW SESSION CREATED: {}",&user_session_id);
    use schema::sessions;
    let current_time = Local::now() + Duration::minutes(15);
    let new_session = NewSession {
        session_id: user_session_id,
        expire_at: &current_time.to_string(),
        user_id: &uuid
    };
    diesel::insert_into(sessions::table)
        .values(&new_session)
        .execute(conn)
        .expect("FAILED");
}

pub fn db_check_session(conn: &SqliteConnection, user_session_id: String) -> Result<i32,()>
{
    use schema::sessions;
    use schema::sessions::dsl::*;
    let session: QueryResult<Session> = sessions.find(user_session_id.as_str())
        .first(conn);
    if session.is_err(){
        return Err(());
    }

    let session_open = session.as_ref().unwrap();

    if session_open.expire_at.parse::<DateTime<Local>>().unwrap().le(&Local::now())
    {
        println!("EXPIRED SESSION");
        diesel::delete(sessions.find(user_session_id)).execute(conn);
        return Err(());
    }
    Ok(session.unwrap().user_id)
}


pub fn compare_hash(un_hashed: String, hashed: String) -> bool
{
    bcrypt::verify(un_hashed,hashed.as_str())
        .expect("Error verifying the hash on a user")
}