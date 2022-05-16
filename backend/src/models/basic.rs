// use c4rs::*;
use neo4jrs::*; // modified, uses "NeoGraph"
use serde::{Serialize, Deserialize};

// Use Graph::run for cases where you just want a write operation
// use Graph::execute when you are interested in the result stream
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    name: String,
    uuid: String,
    money: f64, // BoltType: Float
    risk: i64, // BoltType: Integer
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Circle {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Space {
    pub uuid: String,
    pub pattern: String,
    pub fields: Vec<String>,
    pub stake: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventFight {
    pub uuid: String,
    pub name: String,
    pub fighter_a: String,
    pub fighter_b: String,
}

//
// GETS
// 

#[tokio::main]
pub async fn list_joined(cuuid: String, graph: &Graph) -> serde_json::Result<String> {

    let mut cursor = graph.execute(
        Query::new("
            MATCH (player:Player)-[:JOINED]->(c:Circle {uuid: $uuid})
            RETURN player
        ")
        .param("uuid", cuuid)
    ).await.unwrap();

    let mut vec = Vec::new();
    while let Ok(Some(row)) = cursor.next().await {
        let node: Node = row.get("player").unwrap();
        let player = Player {
            name: node.get("name").unwrap(),
            uuid: node.get("uuid").unwrap(),
            money: node.get("money").unwrap(),
            risk: node.get("risk").unwrap()
        };
        vec.push(player);
    }

    let json = serde_json::to_string(&vec)?;
    Ok(json)
}

#[tokio::main]
pub async fn get_player(name: String, graph: &Graph) -> serde_json::Result<String> {

    let mut cursor = graph.execute(
        Query::new("
            MATCH (player:Player {name: $name}) 
            RETURN player
        ")
        .param("name", name)
    ).await.unwrap();

    let row = cursor.next().await.unwrap().unwrap();
    let node: Node = row.get("player").unwrap();
    let player = Player {
        name: node.get("name").unwrap(),
        uuid: node.get("uuid").unwrap(),
        money: node.get("money").unwrap(),
        risk: node.get("risk").unwrap(),
    };
    let conv = serde_json::to_string(&player)?;
    Ok(conv)
}

#[tokio::main]
pub async fn get_circle(cuuid: String, graph: &Graph) -> serde_json::Result<String> {

    let mut cursor = graph.execute(
        Query::new("
            MATCH (circle:Circle {uuid: $uuid}) 
            RETURN circle
        ")
        .param("uuid", cuuid)
    ).await.unwrap();

    let row = cursor.next().await.unwrap().unwrap();
    let node: Node = row.get("circle").unwrap();
    let circle = Circle {
        name: node.get("name").unwrap(),
    };
    let conv = serde_json::to_string(&circle)?;
    Ok(conv)
}

#[tokio::main]
pub async fn get_space(suuid: String, graph: &Graph) -> serde_json::Result<String> {

    let mut cursor = graph.execute(
        Query::new("
            MATCH (space:Space {uuid: $uuid}) 
            RETURN space
        ")
        .param("uuid", suuid)
    ).await.unwrap();

    let row = cursor.next().await.unwrap().unwrap();
    let node: Node = row.get("space").unwrap();
    let space = Space {
        uuid: node.get("uuid").unwrap(),
        pattern: node.get("pattern").unwrap(),
        fields: node.get("fields").unwrap(),
        stake: node.get("stake").unwrap(),
    };
    let conv = serde_json::to_string(&space)?;
    Ok(conv)
}

#[tokio::main]
pub async fn get_event(suuid: String, graph: &Graph) -> serde_json::Result<String> {

    let mut cursor = graph.execute(
        Query::new(
            "MATCH (event:Event)-[:INFORMS]->(s:Space {uuid: $uuid}) RETURN event
        ")
        .param("uuid", suuid)
    ).await.unwrap();

    let row = cursor.next().await.unwrap().unwrap();
    let node: Node = row.get("event").unwrap(); // could do match operation for event type
    let event = EventFight {
        uuid: node.get("uuid").unwrap(),
        name: node.get("name").unwrap(),
        fighter_a: node.get("fighter_a").unwrap(),
        fighter_b: node.get("fighter_b").unwrap(),
    };
    let conv = serde_json::to_string(&event)?;
    Ok(conv)
}

//
// POSTS
//

#[tokio::main]
pub async fn add_random(cuuid: String, graph: &Graph) -> tide::Result<()> {

    graph.run(
        Query::new("
            MATCH (c:Circle {uuid: $cuuid}) WITH c
            MATCH (p:Player) WHERE NOT (p)-[:JOINED]->(c) 
            WITH c, p, rand() as r ORDER BY r LIMIT 1
            MERGE (p)-[:JOINED]->(c)
            RETURN p
        ")
        .param("cuuid", cuuid)
    ).await.unwrap();

    Ok(())
}

#[tokio::main]
pub async fn join(name: String, cuuid: String, graph: &Graph) -> tide::Result<()> {

    graph.run(
        Query::new("
            MATCH (p:Player {name: $name})
            WITH p MATCH (c:Circle {uuid: $cuuid})
            MERGE (p)-[:JOINED]->(c)
            RETURN p
        ")
        .param("name", name)
        .param("cuuid", cuuid)
    ).await.unwrap();

    Ok(())
}

#[tokio::main]
pub async fn leave(name: String, cuuid: String, graph: &Graph) -> tide::Result<()> {

    graph.run(
        Query::new("
            MATCH (p:Player {name: $name})-[r:JOINED]->(c:Circle {uuid: $cuuid})
            DELETE r
        ")
        .param("name", name)
        .param("cuuid", cuuid)
    ).await.unwrap();

    Ok(())
}

// delete both models and payouts
#[tokio::main]
pub async fn delete_model(name: String, suuid: String, graph: &Graph) -> tide::Result<()> {
    
    graph.run(
        Query::new("
            MATCH (p:Player {name: $name})--(n)--(s:Space {uuid: $suuid})
            WHERE n:Model OR n:Payout
            DETACH DELETE n
        ")
        .param("name", name)
        .param("suuid", suuid)
        ).await.unwrap();
    
    Ok(())
}