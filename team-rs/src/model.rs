#![allow(unused_parens)]

use nom::*;

use std::str;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Input {
    nrows: u32,
    ncolumns: u32,
    ndrones: u32,
    deadline: u32,
    maxload: u32,
    weights: Vec<u32>,
    warehouses: Vec<Warehouse>,
    orders: Vec<Order>,
}

#[derive(Debug)]
pub struct Output {

}

#[derive(Clone, Debug)]
pub struct Warehouse {
    loc: (u32, u32),
    inventory: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Order {
    loc: (u32, u32),
    types: Vec<u32>,
}

named!(pub input<Input>, do_parse!(
    nrows: integer    >>
    ncolumns: integer >>
    ndrones: integer  >>
    deadline: integer >>
    maxload: terminated!(integer, newline)   >>
    weights: preceded!(skipline, lineofints) >>
    warehouses: warehouses >>
    orders: orders >>
    ( Input {
        nrows,
        ncolumns,
        ndrones,
        deadline,
        maxload,
        weights,
        warehouses,
        orders,
    })
));

named!(warehouses<Vec<Warehouse>>,
    length_count!(
        map!(lineofints, |v| v[0]),
        do_parse!(
            loc: map!(lineofints, |t| (t[0], t[1])) >>
            inventory: lineofints                   >>
            ( Warehouse {loc, inventory} )
        )
    )
);

named!(orders<Vec<Order>>,
    length_count!(
        map!(lineofints, |v| v[0]),
        do_parse!(
            loc: map!(lineofints, |t| (t[0], t[1])) >>
            skipline                                >>
            types: lineofints                       >>
            ( Order {loc, types} )
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