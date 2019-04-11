#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate rocket_multipart_form_data;

use std::io::Read;
use std::fs::File;
use rocket::Data;
use rocket_multipart_form_data::{mime, MultipartFormData, MultipartFormDataOptions, MultipartFormDataField, FileField};
use rocket::http::ContentType;
use rocket_contrib::templates::Template;
use tera::Context;


fn main() {
    rocket::ignite()
    .mount("/", routes![index, file_up])
    .attach(Template::fairing())
    .launch();
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("file_contents", "Please upload a file.");
    Template::render("layout", &context)
}

#[post("/", data = "<data>")]
fn file_up(content_type: &ContentType, data: Data) -> Template {
    let mut context = Context::new();

    let mut options = MultipartFormDataOptions::new();
    //options.allowed_fields.push(MultipartFormDataField::file("target_file"));
    options.allowed_fields.push(MultipartFormDataField::file("target_file").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap());
    println!("SIZE: {}", options.allowed_fields.len());

    let multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let target_file = multipart_form_data.files.get(&"target_file".to_string());

    if let Some(target_file) = target_file {
        match target_file {
            FileField::Single(file) => {
                let file_path = &file.path;

                let mut f = File::open(file_path).unwrap();
                //let mut buf = [0; 500];
                let mut buf = String::new();

                match f.read_to_string(&mut buf) {
                    Ok(_) => {},
                    Err(e) => {println!("Error reading file: {}", e);}
                }
                println!("!!!!!FILE CONTENTS!!!!! {}", buf);
            }

            FileField::Multiple(_files) => {}
        }
    }

    context.insert("file_contents", "Upload complete.");
    Template::render("layout", &context)
}
