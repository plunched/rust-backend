#![feature(proc_macro_hygiene, decl_macro)]

use std::u8;

use rocket_contrib::json::Json;

use lib::db;
use lib::model::Movie;
use rocket::Rocket;

#[macro_use]
extern crate rocket;

fn main() {
    rocket().launch();
}

#[get("/")]
fn hello_world() -> String {
    format!("hello world")
}

#[get("/")]
fn get_movies() -> Json<Option<Vec<Movie>>> {
    Json(db::read_movies())
}

#[get("/<id>")]
fn get_movie(id: u8) -> Json<Option<Movie>> {
    Json(db::read_movie(id))
}

#[post("/", data = "<movie>")]
fn create_movie(movie: Json<Movie>) -> Json<Option<Movie>> {
    Json(db::create_movie(movie.0))
}

#[delete("/<id>")]
fn delete_movie(id: u8) -> Json<bool> {
    Json(db::delete_movie(id))
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes!(hello_world)).mount(
        "/movies",
        routes![get_movies, get_movie, create_movie, delete_movie],
    )
}
