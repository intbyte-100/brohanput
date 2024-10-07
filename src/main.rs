pub mod enigutil;

#[macro_use]
extern crate rocket;

use enigutil::BrKeyboard;
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { name: "Rust Developer" })
}

#[derive(FromForm)]
struct InputForm {
    input: String,
}

#[derive(FromForm)]
struct Arrow {
    arrow: String,
}

#[derive(FromForm)]
struct KeyStay {
    state: String,
}

#[post("/submit", data = "<input>")]
fn submit(input: Form<InputForm>) {
    BrKeyboard::get().text(&input.input)
}
#[post("/backspace")]
fn backspace() {
    BrKeyboard::get().backspace();
}

#[post("/enter")]
fn enter() {
    BrKeyboard::get().enter();
}

#[post("/arrow", data = "<arrow>")]
fn arrow(arrow: Form<Arrow>) {
    BrKeyboard::get().arrow(&arrow.arrow);
}

#[post("/tab")]
fn tab() {
    BrKeyboard::get().tab();
}

#[post("/shift", data = "<state>")]
fn press_shift(state: Form<KeyStay>) {
    BrKeyboard::get().shift(&state.state);
}

#[post("/control", data = "<state>")]
fn press_control(state: Form<KeyStay>) {
    BrKeyboard::get().control(&state.state);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, submit, backspace, enter, arrow, tab, press_shift, press_control],
        )
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing())
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 8000,
            ..Default::default()
        })
}
