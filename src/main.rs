#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate rocket_multipart_form_data;


use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Write;
use rocket::Data;
use rocket::response::Redirect;
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataOptions, MultipartFormDataField, RawField};
use rocket::http::ContentType;
use rocket_contrib::templates::Template;
use tera::Context;


fn main() {
    rocket::ignite()
    .mount("/", routes![index, file_up, file_down, file_del])
    .attach(Template::fairing())
    .launch();
}

fn get_folder_contents() -> Vec<String> {
    let mut files = Vec::new();

    let paths = std::fs::read_dir("/home/cwtbt/Documents/RustProjects/descend_receptacle/Receptacle").unwrap();

    for file in paths {
        files.push(file.unwrap().path().file_name().unwrap().to_string_lossy().into_owned());
    }

    files
}

fn pass_folder_contents(context: &mut Context) {
    let contents_vec = get_folder_contents();
    let contents_array = contents_vec.as_slice();
    context.insert("folder_contents", contents_array);
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("file_contents", "Please upload a file.");

    pass_folder_contents(&mut context);
    Template::render("layout", &context)
}

#[post("/download/<file_name>")]
fn file_down(file_name: String) -> File {
    let path = format!("/home/cwtbt/Documents/RustProjects/descend_receptacle/Receptacle/{}", file_name);
    let target_file = File::open(path).unwrap();
    target_file
}

#[post("/delete/<file_name>")]
fn file_del(file_name: String) -> Redirect {
    let path = format!("/home/cwtbt/Documents/RustProjects/descend_receptacle/Receptacle/{}", file_name);

    match std::fs::remove_file(path) {
        Ok(_) => {},
        Err(e) => {println!("Failed to delete file ({})", e)}
    }
    Redirect::to("/")

}

#[post("/", data = "<data>")]
fn file_up(content_type: &ContentType, data: Data) -> Redirect {
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

    Redirect::to("/")
}

fn save_file(file_name: String, contents: &[u8]) {
    let path = format!("/home/cwtbt/Documents/RustProjects/descend_receptacle/Receptacle/{}", file_name);

    let mut saved_file = File::create(path).unwrap();
    match saved_file.write_all(contents) {
        Ok(_) => {},
        Err(e) => println!("Failed writing: {}", e)
    }
}
