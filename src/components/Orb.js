import React, { Component } from "react";
import Card from '@mui/material/Card';
import Typography from '@mui/material/Typography';
import CardActionArea from '@mui/material/CardActionArea';
import { primary, info, grey } from '../util/theme.js';

export default class Orb extends Component {
	
	handleClick = () => {
		return ( 
			this.props.hasModel 
			? this.props.deleteModel(this.props.name) 
			: this.props.genModel(this.props.name, this.props.risk)
		);
	}

    render() {
		const { hasModel, name } = this.props;
		const initial = name.charAt(0);
		const bwidth = hasModel ? 2 : 1;
		const bcolor = hasModel ? primary.main : grey['500'];
		const tycolor = hasModel ? info.dark : grey['900'];
        return (
			<Card
				onClick={this.handleClick}
				variant="outlined" 
				sx={{
					mt: 2,
					width: 70, 
					height: 70,
					borderRadius: 50,
					borderWidth: bwidth,
					borderColor: bcolor
				}}
			>
				<CardActionArea sx={{ width:'100%', height:'100%' }}>
					<Typography 
						align="center" 
						variant="h5" 
						sx={{
							mt:0.25,
							color: tycolor,
						}}>
					{initial}
					</Typography>
				</CardActionArea>
			</Card>
        );
    }
}