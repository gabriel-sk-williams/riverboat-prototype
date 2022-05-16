//! Handles routine Results from server
use tide::{Body, Response};

pub fn post() -> tide::Result {
    let mut res = Response::new(200);
    res.set_body("model posted".to_string());
    Ok(res)
}

pub fn response(json: serde_json::Result<String>) -> tide::Result<impl Into<Response>> {
    match json {
        Ok(json) => {
            let mut res = Response::new(200); // OK
            res.set_body(Body::from_json(&json)?);
            Ok(res)
        }
        Err(e) => {
            let mut res = Response::new(204); // No Content
            res.set_body(e.to_string());
            Ok(res)
        }
    }
}

pub fn result(profile: serde_json::Result<String>) -> tide::Result {
    match profile {
        Ok(profile) => {
            let mut res = Response::new(200); // OK
            res.set_body(Body::from_json(&profile)?);
            Ok(res)
        }
        Err(e) => {
            let mut res = Response::new(204); // No Content
            res.set_body(e.to_string());
            Ok(res)
        }
    }
}