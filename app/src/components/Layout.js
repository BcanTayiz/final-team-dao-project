import React from 'react';

import Navbar from './navbar';
import Createdata from './createData';
import CreateTeam from './createTeam';
import getVotes from './getVotes';
import joinToTeam from './joinToTeam';
import Leavetoteam from './leaveToTeam';
import prizeCalculate from './prizeCalculate';
import votesCalculate from './votesCalculate';


const Layout = (props) => {
    return (
        <div>
            <Navbar/>
            {props.children}
            <Createdata />
            <CreateTeam/>
            <getVotes />
            <joinToTeam />
            <Leavetoteam />
            <prizeCalculate />
            <votesCalculate />
        </div>
    );
}

export default Layout;
