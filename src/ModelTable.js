import React, { Component } from "react";
import Container from '@mui/material/Container';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import CasinoIcon from '@mui/icons-material/Casino';
import { primary, grey } from './theme.js';

export default class ModelTable extends Component {

    rp = (val) => { return ( typeof val === 'number' ? val + " %" : val ); }

    randomize = (name, risk) => { this.props.randomize(name, risk); }

    render() {
        const { headers, players, models } = this.props;

        const entries = (name) => {
            let model = models[name] ? models[name] : { a:"-", b:"-", c:"-", d: "-", e:"-" };
            return Object.keys(model).map((cell, i) => (
                <TableCell key={i} align="center">
                    {this.rp(model[cell])}
                </TableCell>
            ));
        }

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
                                <TableCell align="left" sx={{width: 50}}></TableCell>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {players.map((row, i) => (
                                <TableRow key={i}>
                                    <TableCell component="th" scope="row">
                                        {row.name}
                                    </TableCell>
                                    {entries(row.name)}
                                    <TableCell align="right">
                                        <CasinoIcon
                                            onClick={() => this.randomize(row.name, row.risk)} 
                                            sx={{
                                                "&:hover":{cursor:'pointer', color:primary.main}, 
                                                color: grey[300], 
                                                mt: 0.5
                                            }} 
                                        />
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