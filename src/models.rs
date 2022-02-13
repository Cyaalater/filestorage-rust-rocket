use super::schema::files;
use super::schema::users;
use super::schema::sessions;
use diesel::prelude::*;
use serde_derive::Serialize;

#[derive(Queryable,Debug,Serialize)]
pub struct Files {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub path: String,
    pub uploader: String,
    pub date: String,
}

#[derive(Insertable)]
#[table_name="files"]
pub struct NewFile<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub path: &'a str,
    pub uploader: &'a str,
    pub date: &'a str,
}

#[derive(Queryable,Debug)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub permissions: i32
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hashed_password: &'a str,
    pub permissions: &'a i32
}

#[derive(Queryable,Debug)]
pub struct Session {
    pub session_id: String,
    pub expire_at: String,
    pub user_id: i32
}
#[derive(Insertable)]
#[table_name="sessions"]
pub struct NewSession<'a> {
    pub session_id: &'a str,
    pub expire_at: &'a str,
    pub user_id: &'a i32
}