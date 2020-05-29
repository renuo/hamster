#![feature(proc_macro_hygiene, decl_macro)]

use hamster::ham::{HamImage};
use image::{DynamicImage};

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
use rocket::request::{Form};
use rocket::response::NamedFile;
use rocket::response::{Redirect};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

#[derive(Debug, FromForm)]
struct FormInput {
    file_name: String,
}

#[derive(Serialize)]
struct HamImageContext {
    file_name: String,
}

fn create_ham_image(image: DynamicImage, file_name: String) {
    let ham_image = HamImage::from_rgb(image.to_rgb());
    let output_path = format!("data_out/{}", file_name);
    ham_image.to_rgb().save(output_path).unwrap();
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("templates/index.html").ok()
}

#[post("/image", data = "<image_form>")]
fn image(image_form: Form<FormInput>) -> Redirect {
    let form_input = image_form.into_inner();
    let input_file_path = format!("data_in/{file_name}", file_name = &form_input.file_name);
    let img = image::open(input_file_path).unwrap();
    create_ham_image(img, form_input.file_name.clone());

    Redirect::to(uri!(ham_image: file_name = form_input.file_name))
}

#[get("/ham_image/<file_name>")]
fn ham_image(file_name: String) -> Template {
    let context = HamImageContext { file_name };
    Template::render("ham_image", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, image, ham_image])
        .mount("/", StaticFiles::from("data_out"))
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}