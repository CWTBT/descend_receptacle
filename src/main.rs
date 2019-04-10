#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

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

#[post("/?<file_name>")]
fn file_up(file_name: Option<String>) -> Template {
    let mut context = Context::new();

    match file_name {
        None => context.insert("file_contents", "no file found"),
        Some(file_name) =>
        {
            context.insert("file_contents", &file_name);
            println!("it worked!");
        }
    }

    Template::render("layout", &context)
}
