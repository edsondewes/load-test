use repository::DB;
use std::convert::Infallible;
use warp::{Filter, Rejection};

mod auth;
mod error;
mod handler;
mod repository;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let messages_routes = warp::path::end()
        .and(warp::post())
        .and(warp::header("Authorization"))
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::post_message_handler);

    let routes = messages_routes.recover(error::handle_rejection);

    println!("Started on port 3000");
    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
