use crate::model::Movie;
use std::fs;

static MOVIES_DB: &str = "data/movies.json";

fn _movies() -> Result<Vec<Movie>, serde_json::Error> {
    let data = fs::read_to_string(MOVIES_DB).expect("Error reading from file");
    let movies: Result<Vec<Movie>, serde_json::Error> = serde_json::from_str(&data);
    movies
}

fn _write_movies(movies: Vec<Movie>) {
    let data = serde_json::to_string(&movies).expect("couldn't convert the json into a string!");
    fs::write(MOVIES_DB, data).expect("couldn't write to the json database!")
}

pub fn read_movies() -> Option<Vec<Movie>> {
    match _movies() {
        Ok(movies) => Some(movies),
        Err(_) => None,
    }
}

pub fn read_movie(id: u8) -> Option<Movie> {
    match _movies() {
        Ok(movies) => {
            let res = movies.get(id as usize);
            res.cloned()
        }
        Err(_) => None,
    }
}

pub fn create_movie(movie: Movie) -> Option<Movie> {
    match _movies() {
        Ok(mut movies) => {
            movies.push(movie.clone());
            _write_movies(movies);
            Some(movie)
        }
        Err(_) => None,
    }
}

pub fn delete_movie(id: u8) -> bool {
    match _movies() {
        Ok(mut movies) => {
            movies.remove(id as usize);
            _write_movies(movies);
            true
        }
        Err(_) => false,
    }
}
