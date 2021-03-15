use sqlx::{pool::Pool, Postgres};
use tide::Server;
use crate::handler;

pub fn set(app: &mut Server<Pool<Postgres>>) -> Result<(), std::io::Error> {
   
    app.at("/email").post(handler::email::send_email);

    Ok(())
}