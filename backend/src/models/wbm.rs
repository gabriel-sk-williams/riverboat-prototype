
use neo4jrs::*; // modified, uses "NeoGraph"
use serde::{Serialize, Deserialize};
use serde_json::Map;
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct WinByMethod {
    pub a_by_dec: f64,
    pub a_by_ko: f64,
    pub b_by_dec: f64,
    pub b_by_ko: f64,
    pub draw_nc: f64,
}

impl WinByMethod {
    pub fn outcomes() -> i64 { 5 }

    pub fn new_model(certs: Vec<i64>) -> WinByMethod {
        WinByMethod {
            a_by_dec: certs[0] as f64,
            a_by_ko: certs[1] as f64,
            b_by_dec: certs[2] as f64,
            b_by_ko: certs[3] as f64,
            draw_nc: certs[4] as f64,
        }
    }

    pub fn new_payout(mut payouts: HashMap<String, f64>) -> WinByMethod {
        WinByMethod {
            a_by_dec: payouts.remove("a_by_dec").expect(""),
            a_by_ko: payouts.remove("a_by_ko").expect(""),
            b_by_dec: payouts.remove("b_by_dec").expect(""),
            b_by_ko: payouts.remove("b_by_ko").expect(""),
            draw_nc: payouts.remove("draw_nc").expect(""),
        }
    }
}

#[tokio::main]
pub async fn map(suuid: String, 
                 graph: &Graph) 
                 -> serde_json::Result<HashMap<String, HashMap<String, f64>>> {

    let mut cursor = graph.execute(
        Query::new("
            MATCH (player:Player)-->(model:Model)-->(s:Space {uuid: $uuid})
            RETURN player, model
        ")
        .param("uuid", suuid)
    ).await.unwrap();

    let mut map = HashMap::new();
    while let Ok(Some(row)) = cursor.next().await {
        let player: Node = row.get("player").unwrap();
        let name: String = player.get("name").unwrap();
        let model: Node = row.get("model").unwrap();

        let hmap = HashMap::from([
            (String::from("a_by_dec"), model.get("a_by_dec").unwrap()),
            (String::from("a_by_ko"), model.get("a_by_ko").unwrap()),
            (String::from("b_by_dec"), model.get("b_by_dec").unwrap()),
            (String::from("b_by_ko"), model.get("b_by_ko").unwrap()),
            (String::from("draw_nc"), model.get("draw_nc").unwrap()),
        ]);
        
        map.insert(name, hmap);
    }

    Ok(map)
}

#[tokio::main]
pub async fn list_models(suuid: String, graph: &Graph) -> serde_json::Result<String> {

    let mut cursor = graph.execute(
        Query::new("
            MATCH (player:Player)-->(model:Model)-->(s:Space {uuid: $uuid})
            RETURN player, model
        ")
        .param("uuid", suuid)
    ).await.unwrap();

    let mut map = Map::new();
    while let Ok(Some(row)) = cursor.next().await {
        let player: Node = row.get("player").unwrap();
        let name: String = player.get("name").unwrap();

        let model: Node = row.get("model").unwrap();
        let wbm = WinByMethod {
            a_by_dec: model.get("a_by_dec").unwrap(),
            a_by_ko: model.get("a_by_ko").unwrap(),
            b_by_dec: model.get("b_by_dec").unwrap(),
            b_by_ko: model.get("b_by_ko").unwrap(),
            draw_nc: model.get("draw_nc").unwrap()
        };
        
        map.insert(name, json!(wbm));
    }

    let json = serde_json::to_string(&map)?;
    Ok(json)
}

// prob switch to list_wbm, and just grab models / payouts together (or separate)
#[tokio::main]
pub async fn list_payouts(suuid: String, graph: &Graph) -> serde_json::Result<String> {

    // query p:Payout
    let mut cursor = graph.execute(
        Query::new("
            MATCH (player:Player)<--(payout:Payout)<--(s:Space {uuid: $uuid})
            RETURN player, payout
        ")
        .param("uuid", suuid)
    ).await.unwrap();

    let mut map = Map::new();
    while let Ok(Some(row)) = cursor.next().await {
        let player: Node = row.get("player").unwrap();
        let name: String = player.get("name").unwrap();

        let model: Node = row.get("payout").unwrap();
        let payout = WinByMethod {
            a_by_dec: model.get("a_by_dec").unwrap(),
            a_by_ko: model.get("a_by_ko").unwrap(),
            b_by_dec: model.get("b_by_dec").unwrap(),
            b_by_ko: model.get("b_by_ko").unwrap(),
            draw_nc: model.get("draw_nc").unwrap()
        };
        map.insert(name, json!(payout));
    }

    let json = serde_json::to_string(&map)?;
    Ok(json)
}

#[tokio::main]
pub async fn post_model(name: String, suuid: String, 
                        model: WinByMethod, graph: &Graph) 
                        -> tide::Result<()> {
    
    let WinByMethod { a_by_dec, a_by_ko, b_by_dec, b_by_ko, draw_nc } = model;

    graph.run(
        Query::new("
            MATCH (player:Player {name: $name})-->(circle)-->(space:Space {uuid: $suuid})
            WITH player, space
            MERGE (player)-[:SETS]->(model:Model)-[:FOR]->(space) SET model = {
                a_by_dec: $adec,
                a_by_ko: $ako,
                b_by_dec: $bdec,
                b_by_ko: $bko,
                draw_nc: $draw
            }
            RETURN model
        ")
        .param("name", name)
        .param("suuid", suuid)
        .param("adec", a_by_dec)
        .param("ako", a_by_ko)
        .param("bdec", b_by_dec)
        .param("bko", b_by_ko)
        .param("draw", draw_nc)
        ).await.unwrap();
    
    Ok(())
}

#[tokio::main]
pub async fn post_payout(name: String, suuid: String, 
                         payout: WinByMethod, graph: &Graph) 
                         -> tide::Result<()> {
    
    let WinByMethod { a_by_dec, a_by_ko, b_by_dec, b_by_ko, draw_nc } = payout;

    graph.run(
        Query::new("
            MATCH (player:Player {name: $name})-->(circle)-->(space:Space {uuid: $suuid})
            WITH player, space
            MERGE (space)-[:SETS]->(payout:Payout)-[:FOR]->(player) SET payout = {
                a_by_dec: $adec,
                a_by_ko: $ako,
                b_by_dec: $bdec,
                b_by_ko: $bko,
                draw_nc: $draw
            }
            RETURN payout
        ")
        .param("name", name)
        .param("suuid", suuid)
        .param("adec", a_by_dec)
        .param("ako", a_by_ko)
        .param("bdec", b_by_dec)
        .param("bko", b_by_ko)
        .param("draw", draw_nc)
        ).await.unwrap();
    
    Ok(())
}