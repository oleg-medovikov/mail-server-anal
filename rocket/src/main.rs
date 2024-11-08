use std::env;
use sqlx::PgPool;


use rocket::fs::{FileServer, relative};
use rocket::routes;

mod models;

mod api;
use api::read_files::read_files;

use dotenv::dotenv;


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
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("No DB CONNECT!");

    rocket::build()
        .mount("/", routes![manual::second])
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api",  routes![read_files,]).manage(pool)
}
