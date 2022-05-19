# (riverboat)
## a Rust server that scores predictions against each other and automatically calculates payouts for each player.

React / MUI

Rust

Neo4j / Cypher

![Demo](/../media/gif/hero.gif?raw=true "Demo")

For this demo there are five possible outcomes in a hypothetical boxing match between Mike Tyson and Muhammad Ali. 
Each player stakes $10, submits a model of their certainties, and the server calculates the payouts for everyone by outcome.

Players added to and removed from the circle with direct database queries run through a Rust backend. Payouts are calculated with Rust and passed backed to the frontend as JSON objects.

![Graph](/../media/jpg/graph.jpg?raw=true "Graph")
