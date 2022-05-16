import React, { Component } from "react";
import Card from '@mui/material/Card';
import { RemoveCircleOutline } from '@mui/icons-material';
import { error, grey } from './theme.js';

export default class Minus extends Component {

	handleClick = () => {
		return this.props.removePlayer(this.props.name);
	}

    render() {
		const minus = this.props.hasModel 
            ? <div /> 
            : <RemoveCircleOutline 
                onClick={() => this.handleClick()}
                sx={{
                    "&:hover": {cursor: 'pointer', color: error.light}, 
                    color: grey[300],
                    mt: 1,
                    ml: 3
                }} 
            />;
        return (
			<Card
				onClick={this.handleClick}
				sx={{
                    border: 'none',
                    boxShadow: 'none',
					width: 70, 
					height: 40,
				}}
			>
				{minus}
			</Card>
        );
    }
}