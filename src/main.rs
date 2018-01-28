#![allow(unused_parens)]

#[macro_use]
extern crate nom;
use nom::*;

use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Input {
    nrows: u32,
    ncolumns: u32,
    ndrones: u32,
    deadline: u32,
    maxload: u32,
    weights: Vec<u32>,
    warehouses: Vec<Warehouse>,
    orders: Vec<Order>,
}

#[derive(Clone, Debug)]
struct Warehouse {
    loc: (u32, u32),
    inventory: Vec<u32>,
}

#[derive(Clone, Debug)]
struct Order {
    loc: (u32, u32),
    types: Vec<u32>,
}

named!(input<Input>, do_parse!(
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

fn main() {
    let mut f = File::open("input.txt").expect("file not found");
    let mut contents = Vec::new();
    f.read_to_end(&mut contents).unwrap();

    let result = input(&contents);
    println!("{:?}", result);
}
