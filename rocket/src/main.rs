use rocket::fs::{FileServer, relative};
//use rocket::serde::Serialize;
use serde::Serialize;
use rocket::{get, http::Status, serde::json::Json};

#[macro_use] extern crate rocket;

// Define a struct for the response data
#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

// Create a handler function that returns a JSON response
#[get("/hello")]
fn hello() -> Result<Json<HelloResponse>, Status> {
    let response = HelloResponse {
        message: String::from("Hello, Rocket!"),
    };

    Ok(Json(response))
}


mod manual {
    use std::path::{PathBuf, Path};
    use rocket::fs::NamedFile;

    #[rocket::get("/second/<path..>")]
    pub async fn second(path: PathBuf) -> Option<NamedFile> {
        let mut path = Path::new(super::relative!("static")).join(path);
        if path.is_dir() {
            path.push("login.html");
        }

        NamedFile::open(path).await.ok()
    }
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![manual::second])
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api",  routes![hello,])
}
