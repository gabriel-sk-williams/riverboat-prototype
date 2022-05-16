import axios from "axios";
import React, { Component } from "react";
import SwitchTable from './SwitchTable.js';
import Orb from './Orb.js';
import Minus from './Minus.js';
import StickyFooter from './StickyFooter.js';

import { Switch, Grid, Typography, Container } from '@mui/material/';
import { AddCircleOutline } from '@mui/icons-material';
import { defaultHeader, defaultEvent, defaultCircle, defaultSpace } from './util/defaults.js';
import { success, grey } from './theme.js';

export default class Riverboat extends Component {
	constructor(props) {
        super(props);
        this.state = {
			viewPayouts: false,
			activeEvent: defaultEvent,
			activeCircle: defaultCircle,
			activeSpace: defaultSpace,
			joined: [], // player -> circle
			models: {}, // player -> model -> space
			payouts: {} // models -> calc
        };
    }

	componentDidMount() {
		let urlEvent = "http://localhost:7878/event/" + defaultSpace.uuid;
		let urlJoined = "http://localhost:7878/joined/" + defaultCircle.uuid;
		let urlModels = "http://localhost:7878/models/" + defaultSpace.uuid;
		let urlSpace = "http://localhost:7878/space/" + defaultSpace.uuid;
		let urlPayouts = "http://localhost:7878/payouts/" + defaultSpace.uuid;

		Promise.all([
			axios.get(urlEvent, { headers: defaultHeader }),
			axios.get(urlJoined, { headers: defaultHeader }),
			axios.get(urlModels, { headers: defaultHeader }),
			axios.get(urlSpace, { headers: defaultHeader }),
			axios.get(urlPayouts, { headers: defaultHeader }),
		]).then(([resEvent, resJoined, resModels, resSpace, resPayout]) => {
			this.setState({ 
				activeEvent: JSON.parse(resEvent.data),
				joined: JSON.parse(resJoined.data),
				models: JSON.parse(resModels.data),
				activeSpace: JSON.parse(resSpace.data),
				payouts: JSON.parse(resPayout.data),
			});
		})
	}

	deleteModel = (name) => { // when clicking orb
		let { activeSpace } = this.state;
		let { uuid } = activeSpace;
		let nameSpace = { name, suuid: uuid };
		let urlDelete = "http://localhost:7878/delete_model";
		let urlCalc = "http://localhost:7878/calc";
		let urlModels = "http://localhost:7878/models/" + uuid;
		let urlPayouts ="http://localhost:7878/payouts/" + uuid;

		axios.post(urlDelete, nameSpace, { headers: defaultHeader })
		.then(res => { 
			return axios.post(urlCalc, activeSpace, { headers: defaultHeader })
		})
		.then(res => { 
			return Promise.all([
				axios.get(urlModels, { headers: defaultHeader }),
				axios.get(urlPayouts, { headers: defaultHeader }),
			]).then(([resModels, resPayouts]) => {
				this.setState({ 
					models: JSON.parse(resModels.data),
					payouts: JSON.parse(resPayouts.data),
				});
			})
		})
	}

	genRandomModel = (name, risk) => {
		let { activeSpace } = this.state;
		let { uuid, pattern } = activeSpace;
		let params = { name, risk, uuid, pattern };
		let urlGen = "http://localhost:7878/gen_random";
		let urlCalc = "http://localhost:7878/calc";
		let urlModels = "http://localhost:7878/models/" + uuid;
		let urlPayouts ="http://localhost:7878/payouts/" + uuid;

		axios.post(urlGen, params, { headers: defaultHeader })
		.then(res => { 
			return axios.post(urlCalc, activeSpace, { headers: defaultHeader })
		})
		.then(res => { 
			return Promise.all([
				axios.get(urlModels, { headers: defaultHeader }),
				axios.get(urlPayouts, { headers: defaultHeader }),
			]).then(([resModels, resPayouts]) => {
				this.setState({ 
					models: JSON.parse(resModels.data),
					payouts: JSON.parse(resPayouts.data),
				});
			})
		})
		
	}

	postModel = (model, puuid, suuid) => { // use form

	}
	
	// writes connection to graph
	toggleJoined = (name, joined) => {
		let cuuid = this.state.activeCircle.uuid;
		let urlToggle = joined
			? "http://localhost:7878/leave" 
			: "http://localhost:7878/join";

		let link = { name, cuuid };
		let urlJoined = "http://localhost:7878/joined/" + cuuid;

		axios.post(urlToggle, link, { headers: defaultHeader })
		.then(res => { 
			return axios.get(urlJoined, { headers: defaultHeader })
		})
		.then(res => {
			this.setState({ joined: JSON.parse(res.data) });
		})
	}

	removePlayer = (name) => {
		let cuuid = this.state.activeCircle.uuid;
		let link = { name, cuuid };
		let urlLeave = "http://localhost:7878/leave"
		let urlJoined = "http://localhost:7878/joined/" + cuuid;

		axios.post(urlLeave, link, { headers: defaultHeader })
		.then(res => { 
			return axios.get(urlJoined, { headers: defaultHeader })
		})
		.then(res => {
			this.setState({ joined: JSON.parse(res.data) });
		})
	}

	// add random player
	addRandom = () =>  {
		let cuuid = this.state.activeCircle.uuid;
		let urlAdd = "http://localhost:7878/add_random/" + cuuid;
		let urlJoined = "http://localhost:7878/joined/" + cuuid;

		axios.post(urlAdd, { headers: defaultHeader })
		.then(res => { 
			return axios.get(urlJoined, { headers: defaultHeader })
		})
		.then(res => {
			this.setState({ joined: JSON.parse(res.data) });
		})
	}

	switchTable = () => {
		this.setState(prevState => ({ viewPayouts: !prevState.viewPayouts }));
	}

	calculate = () => {
		let { activeSpace } = this.state;
		let urlCalc = "http://localhost:7878/calc";
		let urlPayouts ="http://localhost:7878/payouts/" + activeSpace.uuid;

		axios.post(urlCalc, activeSpace, { headers: defaultHeader })
		.then(res => { 
			return axios.get(urlPayouts, { headers: defaultHeader })
		})
		.then(res => {
			this.setState({ payouts: JSON.parse(res.data),});
		})
	}

	// probably best moved to Rust during Space creation
	rm = (event, field) => {

		const capitalize = (str) => { return str.charAt(0).toUpperCase() + str.slice(1); }

		const deal = (word) => { 
			return ( 
				word === "a" 
				? event.fighter_a
				: word === "b" 
				? event.fighter_b
				: word === "ko" 
				? "KO"
				: word === "dec" 
				? "Decision"
				: word === "draw" 
				? "Draw /"
				: word === "nc" 
				? "No Contest"
				: capitalize(word)
			);
		};

        let words = field.split("_");
		let capitals = words.map(word => deal(word));

		let top = capitals[0];
		let bottom = capitals.slice(1).join(" ");

        return { top, bottom };
    }

	render() {
		const { joined, models, payouts, activeSpace, activeEvent, viewPayouts } = this.state;
		const modNames = Object.keys(models);
		const players = joined.map(p => ({ ...p, hasModel: modNames.includes(p.name) }));
		const headers = activeSpace ? activeSpace.fields.map(f => this.rm(activeEvent, f)) : [];

		return (
			<div>
				<Container component="main" sx={{ mt: 8, mb: 2 }} maxWidth="md">
					<Typography align="center" variant="h2" component="h1" gutterBottom>
					{'(riverboat)'}
					</Typography>
					<Typography align="center" variant="h5" component="h2" gutterBottom>
					{'prediction aggregator '} <br/><br/>
					{'Add and remove players from the prediction circle. '} <br/>
					{'Create or generate certainties for each outcome to see payouts.'}
					</Typography>
				</Container>

				<Container component="main" sx={{ mt: 6, mb: 2 }} maxWidth="md">
					<Grid container direction="row" justifyContent="center">	
						{players.reverse().map((p, i) => (
							<Grid key={i} item xs={2} md={1.8}>
								<Orb 
									id={i} 
									deleteModel={this.deleteModel}
									genModel={this.genRandomModel}
									postModel={this.postModel}
									name={p.name}
									risk={p.risk}
									hasModel={p.hasModel}
								/>
								<Minus
									id={i} 
									removePlayer={this.removePlayer}
									name={p.name} 
									hasModel={p.hasModel}
								/>
							</Grid>
						))}
						<AddCircleOutline 
							onClick={this.addRandom} 
							sx={{
								"&:hover": {cursor: 'pointer', color: success.main}, 
								color: grey[300], mt: 5
							}} 
						/>
					</Grid>
          		</Container>

				<Container component="main" maxWidth="md">
					<Grid container direction="row" justifyContent="center">
						<Typography sx={{ mt: 0.9, mr: 3 }}>Models</Typography>
						<Switch onClick={this.switchTable} />
						<Typography sx={{ mt: 0.9, ml: 3 }}>Payouts</Typography>	
					</Grid>
				</Container>

				<SwitchTable
					viewPayouts={viewPayouts}
					headers={headers}
					players={players} 
					models={models} 
					payouts={payouts}
					randomize={this.genRandomModel}
			  	/>;
				
				<StickyFooter />
			</div>
		);
	}
}