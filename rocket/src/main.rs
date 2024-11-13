use std::env;
use sqlx::PgPool;
use dotenv::dotenv;

use rocket::fs::{FileServer, relative};
use rocket::routes;

mod base;
use base::create_tables::create_tables;

mod models;

mod api;
use api::read_files::read_files;
use api::get_messages::get_messages;
use api::get_senders::get_senders;



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

    let _ =  create_tables(&pool).await.expect("Не удалось создать таблицы");

    rocket::build()
        .mount("/", routes![manual::second])
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api",  routes![read_files,get_messages, get_senders]).manage(pool)
}
