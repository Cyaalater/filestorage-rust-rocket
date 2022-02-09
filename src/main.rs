use std::fs::OpenOptions;
use std::error::Error;
use rocket::data::FromData;
use rocket::form::{Form, FromForm};
use rocket::serde::de::Unexpected::Str;
use multipart::server::Multipart; // 0.16.1, default-features = false, features = ["server"]
use rocket::Data;
use rocket::http::{ContentType, Status};
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};
use rocket::serde::json::Json;
use serde::{Serialize,Deserialize};
use dieseldb::{compare_hash, db_add_file, db_add_user, db_create_session, db_establish_connection, db_find_file, db_search_file, db_search_user, db_show_files, key_get};
use dieseldb::models::Files;
use dotenv::dotenv;
use std::env;
#[macro_use] extern crate rocket;

// TODO: Remove all csv components, functions and change them to diesel


#[derive(FromForm)]
struct UserInput {
    name: String,
    path: String
}

#[derive(FromForm)]
struct UserForm {
    user: String,
    password: String
}

#[derive(Serialize,Deserialize)]
struct RegisterResult {
    success: bool,
    data: String
}

#[derive(Serialize,Deserialize)]
struct LoginResult {
    success: bool,
    session: String
}

extern crate rocket_multipart_form_data;





#[get("/")]
async fn bash() -> String {
    let file_data = std::fs::read_to_string("./bashscript.sh").unwrap();
    file_data
}

#[get("/download/<file_index>/<secret>")]
async fn bash_download(file_index: i32, secret: String) -> Vec<u8> {
    if secret != key_get() {
        return vec![4,0,4]
    }
    let conn = db_establish_connection();
    // TODO: Add the index function to files and add here or remove it entirely and change it to search (Which is kinda weak)
    let database_data: Files = db_find_file(&conn,file_index);
    let file_name = database_data.path;
    let file = std::fs::read(format!("./Uploads/{}",file_name)).unwrap();
    file
}

#[get("/get")]
async fn bash_get() -> String {
    use dieseldb::models::Files;
    let db = db_establish_connection();
    let db_data = db_show_files(&db);
    let mut result : String = "Current database:\n-------------------------------------------------------------\n".to_string();
    for i in (std::ops::Range{ start: 0, end: (db_data.len())}) {
        let item = &db_data[i];
        result.push_str(format!("{}] Uploader = {} || Name = {}\n{}\n-------------------------------------------------------------\n",item.id,item.uploader,item.name,item.description).as_str());
    }
    result
}


#[post("/upload/<secret>", data = "<data>")]
async fn post_file(content_type: &ContentType, secret: String, data: Data<'_>) -> Status {
    if secret != key_get(){
        return Status::Unauthorized
    }
    println!("{:?}",content_type);

    // Set allowed form data in the post request
    let mut options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            MultipartFormDataField::file("file"),
            MultipartFormDataField::text("name"),
            MultipartFormDataField::text("description")
        ]
    );
    // Pulling out the data
    let mut multipart_form_data = MultipartFormData::parse(content_type,data,options).await.unwrap();
    let uploaded_file = multipart_form_data.files.get("file");
    let description_vec = multipart_form_data.texts.get("description").unwrap();
    let description_text = &description_vec[0].text;
    let name_vec = multipart_form_data.texts.get("name").expect("Error parsing name");
    let name_text = &name_vec[0].text;
    if let Some(mut file_fields) = uploaded_file {
        let file_field = &file_fields[0];
        // let file_data = std::fs::read(&file_field.path);
        let transfer_status = std::fs::copy(&file_field.path,format!("./Uploads/{}",&file_field.file_name.as_ref().unwrap())).is_ok();
        if transfer_status == true {
            let db = db_establish_connection();
            db_add_file(&db,name_text,description_text,&file_field.file_name.as_ref().unwrap(),"Placeholder");
        }else {
            return Status::BadRequest
        }


    }
    Status::Ok
}

#[post("/register", data="<form>")]
async fn register_user(form: Form<UserForm>) -> Json<RegisterResult>{
    let db = db_establish_connection();
    let name = form.user.clone();
    let password = form.password.clone();
    let query_result = db_search_user(&db,name.clone());
    if query_result.is_ok()
    {
        return Json(RegisterResult {
            success: false,
            data: String::from("User with same name exists")
        })
    }
    db_add_user(&db,name.as_str(),password.as_str());
    Json(RegisterResult {
        success: true,
        data: String::from("User has been registered with low level permissions")
    })
}

#[post("/login", data = "<form>")]
async fn login_user(form: Form<UserForm>) -> Json<LoginResult>
{
    let db = db_establish_connection();
    let name = form.user.clone();
    let password = form.password.clone();
    let user = db_search_user(&db,name);
    if user.is_err()
    {
        return Json(LoginResult {
            success: false,
            session: String::from("Error")
        });
    }
    let user_data = user.unwrap();
    if compare_hash(password, user_data.hashed_password)
    {
        let uuid = uuid::Uuid::new_v4();
        db_create_session(&db, &uuid.to_owned().to_string(),&user_data.id);
        return Json(LoginResult {
            success: true,
            session: uuid.to_string()
        });
    }
    Json(LoginResult {
        success: false,
        session: String::from("Error")
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![bash])
        .mount("/bash",routes![bash_get,bash_download])
        .mount("/api",routes![post_file,register_user,login_user])

}

