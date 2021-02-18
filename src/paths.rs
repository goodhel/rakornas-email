use sqlx::{pool::Pool, Postgres};
use tide::Server;
use crate::handler;

pub fn set(app: &mut Server<Pool<Postgres>>) -> Result<(), std::io::Error> {

    // PEMBAHASAN 01 : list record, output ke json
    app.at("/kantor")
        .get(handler::kantor::list)
        .post(handler::kantor::tambah);

    Ok(())
}