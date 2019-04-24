#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::{Json, Value};

mod hero;
use hero::Hero;

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[get("/")]
fn read() -> Json<Value> {
    Json(json!(["hero 1", "hero 2"]))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
    Json(json!({"status": "ok"}))
}

// embed_migrations!();

fn main() {
    // let connection = establish_connection();
    // // This will run the necessary migrations.
    // embedded_migrations::run(&connection);
    // // By default the output is thrown out. If you want to redirect it to stdout, you
    // // should call embedded_migrations::run_with_output.
    // embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    rocket::ignite()
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}
