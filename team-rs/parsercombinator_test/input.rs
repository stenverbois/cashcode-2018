#[derive(Clone, Debug)]
pub struct Warehouse {
    loc: (u32, u32),
    inventory: Vec<u32>,
}

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

#[derive(Clone, Debug)]
pub struct Order {
    loc: (u32, u32),
    types: Vec<u32>,
}

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
