
use serde::{Serialize, Deserialize};
use tide::{Request, Response};
use crate::State;
use crate::state::handle;
use crate::models;
use crate::calc;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameCircle {
    pub name: String,
    pub cuuid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NameSpace {
    pub name: String,
    pub suuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gens {
    pub name: String,
    pub risk: i64,
    pub uuid: String, 
    pub pattern: String,
}

//
// GET LIST - JSON
//

pub(crate) async fn list_joined(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let cuuid: &str = req.param("cuuid")?;
    let graph = req.state().neo();
    let json = models::list_joined(String::from(cuuid), graph);

    handle::response(json)
}

// need match pattern -> list_wbm
pub(crate) async fn list_models(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let suuid: &str = req.param("suuid")?;
    let graph = req.state().neo();
    let models = models::wbm::list_models(String::from(suuid), graph);

    handle::response(models)
}

// need match pattern -> list_wbm
pub(crate) async fn list_payouts(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let suuid: &str = req.param("suuid")?;
    let graph = req.state().neo();
    let payouts = models::wbm::list_payouts(String::from(suuid), graph);

    handle::response(payouts)
}

//
// GET SINGLE NODE
//

pub(crate) async fn get_player(req: Request<State>) -> tide::Result {
    let name: &str = req.param("name")?;
    let graph = req.state().neo();
    let profile = models::get_player(String::from(name), graph);

    handle::result(profile)

}

pub(crate) async fn get_circle(req: Request<State>) -> tide::Result {
    let cuuid: &str = req.param("cuuid")?;
    let graph = req.state().neo();
    let profile = models::get_circle(String::from(cuuid), graph);

    handle::result(profile)
}

pub(crate) async fn get_space(req: Request<State>) -> tide::Result {
    let suuid: &str = req.param("suuid")?;
    let graph = req.state().neo();
    let profile = models::get_space(String::from(suuid), graph);

    handle::result(profile)
}

pub(crate) async fn get_event(req: Request<State>) -> tide::Result {
    let suuid: &str = req.param("suuid")?;
    let graph = req.state().neo();
    let profile = models::get_event(String::from(suuid), graph);
    
    handle::result(profile)
}

//
// POSTS
//

// calc and merge all payouts from existing space -> models
pub(crate) async fn calc_payouts(mut req: Request<State>) -> tide::Result {

    let models::Space { uuid, pattern, fields, stake } = req.body_json().await?;
    let graph = req.state().neo();

    let models = match pattern.as_str() {
        "Win By Method" => models::wbm::map(uuid.clone(), graph).unwrap(),
        _ => HashMap::new() // use Error?
    };

    let payouts = calc::payouts(models, fields, stake);

    for (name, map) in payouts.iter() {
        let payout = models::WinByMethod::new_payout(map.clone());
        let _posted = models::wbm::post_payout(String::from(name), uuid.clone(), payout, graph);
    }

    handle::post()
}

pub(crate) async fn join_circle(mut req: Request<State>) -> tide::Result {

    let NameCircle { name, cuuid } = req.body_json().await?;
    let graph = req.state().neo();
    let _json = models::join(String::from(name), String::from(cuuid), graph);

    handle::post()
}

pub(crate) async fn leave_circle(mut req: Request<State>) -> tide::Result {

    let NameCircle { name, cuuid } = req.body_json().await?;
    let graph = req.state().neo();
    let _json = models::leave(String::from(name), String::from(cuuid), graph);

    handle::post()
}

pub(crate) async fn delete_model(mut req: Request<State>) -> tide::Result {

    let NameSpace { name, suuid } = req.body_json().await?;
    let graph = req.state().neo();
    let _prt = models::delete_model(String::from(name), String::from(suuid), graph);

    handle::post()
}

//
// RANDOM MODEL GENERATION
//

pub(crate) async fn add_random(req: Request<State>) -> tide::Result {

    let cuuid: &str = req.param("circle")?;
    let graph = req.state().neo();
    let _json = models::add_random(String::from(cuuid), graph);

    handle::post()
}

pub(crate) async fn gen_random(mut req: Request<State>) -> tide::Result {

    let Gens { name, risk, uuid, pattern } = req.body_json().await?;
    let graph = req.state().neo();

    let outcomes = match pattern.as_str() {
        "Win By Method" => models::WinByMethod::outcomes(),
        _ => 2 // use Error?
    };

    let certs = calc::gen_random(risk, outcomes);
    let model = models::WinByMethod::new_model(certs);
    let _prt = models::wbm::post_model(String::from(name), String::from(uuid), model, graph);

    handle::post()
}