#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate rocket_multipart_form_data;

use rocket::Data;
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataOptions, MultipartFormDataField, FileField};
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
    options.allowed_fields.push(MultipartFormDataField::file("target_file"));

    let multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let up_file = multipart_form_data.files.get(&"target_file".to_string());

    if let Some(up_file) = up_file {
        match up_file {
            FileField::Single(file) => {
                let _file_path = &file.path;
            }

            FileField::Multiple(_files) => {}
        }
    }

    context.insert("file_contents", "Upload complete.");
    Template::render("layout", &context)
}
