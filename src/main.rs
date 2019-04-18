#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate rocket_multipart_form_data;


use std::fs::File;
use std::io::Write;
use rocket::Data;
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataOptions, MultipartFormDataField, RawField};
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
    options.allowed_fields.push(MultipartFormDataField::raw("file"));
    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => match err {
            _ => panic!("{:?}", err)
        }
    };


    let target_file = multipart_form_data.raw.remove(&"file".to_string());

    if let Some(target_file) = target_file {
        match target_file {
            RawField::Single(file) => {
                let file_name = file.file_name.unwrap();
                let raw_data = file.raw.as_slice();
                save_file(file_name, raw_data);
            }

            RawField::Multiple(_files) => {}
        }
    }

    context.insert("file_contents", "Upload complete.");
    Template::render("layout", &context)
}

fn save_file(file_name: String, contents: &[u8]) {
    let path = format!("/home/cwtbt/Documents/RustProjects/descend_receptacle/Receptacle/{}", file_name);

    let mut saved_file = File::create(path).unwrap();
    match saved_file.write_all(contents) {
        Ok(_) => {},
        Err(e) => println!("Failed writing: {}", e)
    }
}
