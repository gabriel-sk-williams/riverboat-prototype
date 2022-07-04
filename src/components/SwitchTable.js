import React, { Component } from "react";
import { Container, Table, TableBody, 
    TableCell, TableContainer, TableHead, 
    TableRow, Typography, Paper } from '@mui/material';
import CasinoIcon from '@mui/icons-material/Casino';
import { primary } from '../util/theme.js';
import { grey, pink, blueGrey } from '@mui/material/colors';

export default class SwitchTable extends Component {

    payments = (val) => {
        if (typeof val === 'number') {
            let sign  = Math.sign(val) > 0 ? " +" : " -";
            return sign + Math.abs(val).toFixed(2);
        } else {
            return val;
        }
    }

    certainties = (val) => { return ( typeof val === 'number' ? val + " %" : val ); }

    randomize = (name, risk) => { this.props.randomize(name, risk); }

    // "high certainty" depends on the number of results
    // for five possible outcomes, 20 is median
    cc = (num) => { // certainty color
        return (
            num > 60
            ? {color:grey[900]}
            : num > 48
            ? {color:grey[800]}
            : num > 32
            ? {color:grey[700]}
            : num > 16
            ? {color:grey[600]}
            : {color:blueGrey[500]}
        );
    }

    pc = (num) => { // payout color
        return (
            num >= 0.0
            ? {color:grey[700]}
            : {color:pink[500]}
        );
    }

    render() {
        const { viewPayouts, headers, players, models, payouts } = this.props;

        const modelTable = (name) => {
            let model = models[name] ? models[name] : { a:"-", b:"-", c:"-", d: "-", e:"-" };
            return Object.keys(model).map((cell, i) => (
                <TableCell key={i} align="center">
                    <Typography sx={this.cc(model[cell])}>
                        {this.certainties(model[cell])}
                    </Typography>
                </TableCell>
            ));
        }

        const payoutTable = (name) => {
            let payout = payouts[name] ? payouts[name] : { a:"-", b:"-", c:"-", d: "-", e:"-" };

            return Object.keys(payout).map((cell, i) => (
                <TableCell key={i} align="center">
                    <Typography sx={this.pc(payout[cell])}>
                        {this.payments(payout[cell])}
                    </Typography>
                </TableCell>
            ));
        }

        const entries = viewPayouts ? payoutTable : modelTable;

        return (
            <Container component="main" sx={{ mt: 4, mb: 2 }} maxWidth="md">
                <TableContainer component={Paper} elevation={0}>
                    <Table sx={{ minWidth: 650 }} aria-label="simple table">
                        <TableHead>
                            <TableRow>
                                <TableCell></TableCell>
                                    {headers.map((hd, i) => (
                                        <TableCell key={i} align="center">
                                            {hd.top}<br/>{hd.bottom}
                                        </TableCell>
                                    ))}
                                <TableCell sx={{width: 50}}align="left"></TableCell>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {players.map((row, i) => (
                                <TableRow key={i}>
                                    <TableCell component="th" scope="row" >
                                        {row.name}
                                    </TableCell>
                                    {entries(row.name)}
                                    <TableCell align="left">
                                        {viewPayouts 
                                            ? <CasinoIcon sx={{ opacity: 0, mt: 0.5 }} />
                                            : <CasinoIcon
                                                onClick={() => this.randomize(row.name, row.risk)} 
                                                sx={{
                                                    "&:hover":{cursor:'pointer', color:primary.main}, 
                                                    color: grey[300], 
                                                    mt: 0.5
                                                }} 
                                            />
                                        }
                                    </TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                </TableContainer>
            </Container>
        );
    }
}