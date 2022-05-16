
extern crate neo4jrs;
mod models;
mod routes;
mod state;
mod calc;

use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use state::State;

#[async_std::main]
async fn main() -> tide::Result<()> {

    let cors = CorsMiddleware::new()
    .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
    .allow_origin(Origin::from("*"))
    .allow_credentials(true);

    let state = state::State::new(String::from("127.0.0.1:7687")).await?;
    let mut app = tide::with_state(state);
    app.with(cors);
    app.at("/joined/:cuuid").get(routes::list_joined);
    app.at("/models/:suuid").get(routes::list_models);
    app.at("/payouts/:suuid").get(routes::list_payouts);

    app.at("/player/:name").get(routes::get_player); // not in use
    app.at("/circle/:cuuid").get(routes::get_circle); // not in use
    app.at("/space/:suuid").get(routes::get_space);
    app.at("/event/:suuid").get(routes::get_event); // event informs space
    
    app.at("/calc").post(routes::calc_payouts); // Space
    app.at("/join").post(routes::join_circle); // NameCircle
    app.at("/leave").post(routes::leave_circle); // NameCircle
    app.at("/delete_model").post(routes::delete_model); // NameSpace

    app.at("/add_random/:circle").post(routes::add_random); // cuuid
    app.at("/gen_random").post(routes::gen_random); // Gens

    println!("Listening for connections on port {}", 7878);
    app.listen("127.0.0.1:7878").await?;
    Ok(())
}