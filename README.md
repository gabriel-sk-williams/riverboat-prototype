# (riverboat)
## A decentralized prediction aggregator that calculates payouts on a Rust server.

React / MUI Frontend

Rust / Neo4j Backend

[Pitch Demo](https://vimeo.com/manage/videos/728569619/10ce03d124)

![Demo](/../media/gif/hero.gif?raw=true "Demo")

For this demo there are five possible outcomes in a hypothetical boxing match between Mike Tyson and Muhammad Ali. 
Each player stakes $10, submits a model of their certainties, and the server calculates the payouts for everyone by outcome.

Players are added and removed from the circle with direct database queries through the Rust backend. Payouts are calculated with Rust and passed backed to the frontend as JSON objects.

![Graph](/../media/jpg/graph.jpg?raw=true "Graph")
