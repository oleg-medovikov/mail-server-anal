use rocket::routes;

mod read_files; 
mod get_messages;
mod get_senders;
mod check_token;
mod user_login;
mod drop_token;
mod read_known_users;
mod get_fuck_emails;

pub fn api_routes() -> Vec<rocket::Route> {
    routes![
        read_files::read_files,
        get_messages::get_messages,
        get_senders::get_senders,
        check_token::check_token,
        user_login::user_login,
        drop_token::drop_token,
        read_known_users::read_known_users,
        get_fuck_emails::get_fuck_emails,
    ]
}
