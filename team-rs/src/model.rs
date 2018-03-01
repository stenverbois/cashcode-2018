#![allow(unused_parens)]

use nom::*;

use std::str;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Input {
    pub nrows: u32,
    pub ncolumns: u32,
    pub nvehicles: u32,
    pub nrides: u32,
    pub bonus: u32,
    pub max_steps: u32,
    pub rides: Vec<Ride>,
}

#[derive(Debug)]
pub struct Output {

}

#[derive(Clone, Debug)]
pub struct Ride {
    pub start: (u32, u32),
    pub end: (u32, u32),
    pub tbegin: u32,
    pub tend: u32,
}

#[derive(Clone, Debug)]
pub struct Rides {
    pub starts: Vec<(u32, u32)>,
    pub ends: Vec<(u32, u32)>,
    pub tbegins: Vec<u32>,
    pub tends: Vec<u32>,
}

pub fn to_alt(rides: &Vec<Ride>) -> Rides {
    let mut alt_rides = Rides {
        starts: Vec::new(),
        ends: Vec::new(),
        tbegins: Vec::new(),
        tends: Vec::new(),
    };

    for ride in rides.iter() {
        alt_rides.starts.push(ride.start);
        alt_rides.ends.push(ride.end);
        alt_rides.tbegins.push(ride.tbegin);
        alt_rides.tends.push(ride.tend);
    }

    alt_rides
}

named!(pub input<Input>, do_parse!(
    nrows: integer    >>
    ncolumns: integer >>
    nvehicles: integer  >>
    nrides: integer >>
    bonus: integer >>
    max_steps: terminated!(integer, newline)   >>
    rides: rides >>
    ( Input {
        nrows,
        ncolumns,
        nvehicles,
        nrides,
        bonus,
        max_steps,
        rides,
    })
));

named!(rides<Vec<Ride>>,
    many1!(
        do_parse!(
            ints: lineofints >>
            ( 
                Ride {
                    start: (ints[0], ints[1]),
                    end: (ints[2], ints[3]),
                    tbegin: ints[4],
                    tend: ints[5],
                }
            )
        )
    )
);

named!(lineofints<Vec<u32>>, terminated!(many1!(integer), newline));
named!(skipline, terminated!(take_till!(|c| c as char == '\n'), newline));

named!(integer<u32>,
    map_res!(
      map_res!(terminated!(digit, opt!(char!(' '))), str::from_utf8),
      FromStr::from_str
));