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

#[macro_use] extern crate rocket;

#[derive(Deserialize,Serialize)]
struct DatabaseItem {
    name : String,
    description : String
}

#[derive(Serialize)]
struct FullDatabase {
    items : Vec<DatabaseItem>,
    count : usize
}

#[derive(FromForm)]
struct UserInput {
    name: String,
    path: String
}

async fn create_row_cvs(name : String, path : String) -> Result<(), Box<dyn Error>>{
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("src/index.csv")
        .unwrap();

    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&[name,path]);
    wtr.flush();

    Ok(())
}

async fn read_all_rows_cvs() -> Result<FullDatabase, Box<dyn Error>> {
    let mut data_vec: Vec<DatabaseItem> = Vec::new();
    let file = OpenOptions::new()
        .read(true)
        .open("src/index.csv")
        .unwrap();
    let mut count = 0;
    let mut rdr = csv::Reader::from_reader(file);
    for i in rdr.records(){
        let result_1 = i?;
        // let mut json_stream = json!({
        //     "name": result_1.get(0).unwrap(),
        //     "description": result_1.get(1).unwrap()
        // });
        let json_stream_result = DatabaseItem {
            name: result_1.get(0).unwrap().to_string(),
            description: result_1.get(1).unwrap().to_string()
        };
        data_vec.push(json_stream_result);
        count += 1;
    }
    let result = FullDatabase {
        items : data_vec,
        count
    };
    Ok(result)
}


extern crate rocket_multipart_form_data;





#[get("/")]
async fn bash() -> String {
    let file_data = std::fs::read_to_string("./bashscript.sh").unwrap();
    file_data
}

#[get("/download/<file_index>")]
async fn bash_download(file_index: usize) -> Vec<u8> {
    let database_data: FullDatabase = read_all_rows_cvs().await.unwrap();
    let file_name = &database_data.items[file_index].name;
    let file = std::fs::read(format!("./Uploads/{}",file_name)).unwrap();
    file
}

#[get("/get")]
async fn bash_get() -> String {
    let database_data: FullDatabase = read_all_rows_cvs().await.unwrap();
    let mut result : String = "Current database:\n-------------------------------------------------------------\n".to_string();
    for i in (std::ops::Range{ start: 0, end: (database_data.count-1)}) {
        let item = &database_data.items[i];
        result.push_str(format!("{} || {} || {}\n-------------------------------------------------------------\n",i,item.name,item.description).as_str());
    }
    result
}


#[get("/get")]
async fn index() -> Json<FullDatabase>{Json(read_all_rows_cvs().await.unwrap())}
// #[post("/", format="application/x-www-form-urlencoded" ,data = "<user>")]
// async fn post_index(user : Form<UserInput> ) -> String {
//     println!("{} || {}",&user.name,&user.path);
//     create_row_cvs(user.name.clone(), user.path.clone()).await;
//     return String::from("hey")
// }

#[post("/upload", data = "<data>")]
async fn post_file(content_type: &ContentType, data: Data<'_>) -> Status {
    println!("{:?}",content_type);
    let mut options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            MultipartFormDataField::file("file"),
            MultipartFormDataField::text("description")
        ]
    );
    let mut multipart_form_data = MultipartFormData::parse(content_type,data,options).await.unwrap();
    let uploaded_file = multipart_form_data.files.get("file");
    let description_file = multipart_form_data.texts.get("description").unwrap();
    let description_text = &description_file[0].text;
    if let Some(mut file_fields) = uploaded_file {
        let file_field = &file_fields[0];
        // let file_data = std::fs::read(&file_field.path);
        let transfer_status = std::fs::copy(&file_field.path,format!("./Uploads/{}",&file_field.file_name.as_ref().unwrap())).is_ok();
        if transfer_status == true {
            create_row_cvs(format!("{}",&file_field.file_name.as_ref().unwrap()),description_text.to_string()).await;
        }else {
            Status::BadRequest
        }


    }
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![bash])
        .mount("/bash",routes![bash_get,bash_download])
        .mount("/api",routes![index,post_file])

}

