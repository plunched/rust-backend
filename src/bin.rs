#![feature(proc_macro_hygiene, decl_macro)]

use std::u8;

use rocket_contrib::json::Json;

use lib::db;
use lib::model::Movie;

#[macro_use]
extern crate rocket;

fn main() {
    rocket().launch();
}

#[get("/")]
fn get_movies() -> Json<Option<Vec<Movie>>> {
    Json(db::read_movies())
}

#[get("/<id>")]
fn get_movie(id: Index<u8>) -> Json<Option<Movie>> {
    Json(db::read_movie(
        id,
    ))
}



fn rocket() -> Rocket {
    rocket::ignite().mount("/movies", routes![get_movies, get_movie])
}
