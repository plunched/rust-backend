use crate::model::Movie;
use std::{fs, u8};

static MOVIES_DB: &str = "data/movies.json";

fn _movies() -> Result<Vec<Movie>, serde_json::Error> {
    let data = fs::read_to_string(MOVIES_DB).expect("Error reading from file");
    let movies: Result<Vec<Movie>, serde_json::Error> = serde_json::from_str(&data);
    movies
}

pub fn read_movies() -> Option<Vec<Movie>> {
    match _movies() {
        Ok(movies) => Some(movies),
        Err(_) => None,
    }
}

fn read_movie(id: u8) -> Option<Movie> {
   match _movies() {
       Ok(movies) => {
           let res = movies.get(id as usize);

           match res {
               Some(x) => Some(movies[x].clone()),
               None => None,
           }
       }
       Err(_) => None,
   }
}
