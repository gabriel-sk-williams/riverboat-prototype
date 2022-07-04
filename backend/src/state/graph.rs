//! Shared application state.
use neo4jrs::*;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct State {
    pub client: Arc<Graph>,
}

impl State {
    pub(crate) async fn new(uri: String) -> tide::Result<Self> {
        let user = "neo4j";
        let pass = "localwebmaster";
        let graph = Arc::new(Graph::new(&uri, user, pass).await.unwrap());
        Ok(Self { client: graph })
    }
    
    /// Access the neo4j client.
    pub fn neo(&self) -> &Arc<Graph> {
        &self.client
    }
    
}