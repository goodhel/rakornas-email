use tide::{Response, Body};
use std::env;
use dotenv::dotenv;
use tide::utils::After;
// use sqlx::postgres::PgPoolOptions;

mod paths;
mod handler;

#[derive(serde::Serialize)]
struct WebServiceResponse {
    status: String,
    info: String,
}

fn ws_response(status: &str, info: &str) -> tide::Result<Response>{
    let mut res = Response::new(200);
    let data = WebServiceResponse {
        status: status.into(), info: info.into()
    };
    res.set_body(Body::from_json(&data)?);
    Ok(res)
}

// fn to_json(data: &impl serde::Serialize) -> tide::Result<Response> {
//     let mut res = Response::new(200);
//     res.set_body(Body::from_json(data)?);
//     Ok(res)
// }

#[async_std::main]
async fn main() -> tide::Result<()> {
    // println!("Hello, world!");
    dotenv().ok();
    
    // let conn_str =
    //     env::var("DATABASE_URL").expect("Env var DATABASE_URL is required for this example.");

    let pool = sqlx::PgPool::connect(&env::var("DATABASE_URL")?).await?;
    // let pg_pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&env::var("DATABASE_URL")?)
    //     .await?;

    tide::log::start();
    let mut app =  tide::with_state(pool.clone());

    app.with(After(|mut res: Response| async {
        if let Some(err) = res.downcast_error::<sqlx_core::error::Error>() {
            println!("ada error terkait database: {}", err.to_string());
            res = ws_response("Error", "terkait DB")?;
        }
        else if let Some(err) = res.downcast_error::<serde_json::error::Error>() {
            println!("ada error parse json: {}", err.to_string());
            res = ws_response("Error", "error di data JSON")?;
        }
        Ok(res)
    }));

    let _res = paths::set(&mut app);
    app.listen("127.0.0.1:18081").await?;
    Ok(())
}
