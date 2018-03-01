#![allow(dead_code)]

use model::{Input, Output, Ride, Rides, to_alt};

pub fn exec_alg(input: Input) -> Vec<Vec<u32>> {
    
    // println!("{:?}", input);

    let alt_rides = to_alt(&input.rides);
    let mut gen = Generation::new(&input);
    let ctx = Context {
        Ps: alt_rides.starts,
        Pe: alt_rides.ends,
        Ts: alt_rides.tbegins,
        Te: alt_rides.tends,
        bonus: input.bonus,
    };

    let mut result: Vec<Vec<u32>> = vec![Vec::new(); input.nvehicles as usize];
    while gen.available.iter().any(|&a| a) {
        // println!("AVAILABLE {:?}", gen.available);
        // println!("GENERATION");
        let assignment = make_assignment(&gen, &ctx);


        if assignment.iter().all(|&a| a == -1) {
            break;
        }

        // println!("{:?}", assignment);
        // println!("{:?}", assignment.len());
        gen = next_gen(&gen, &ctx, &assignment);

        let max_time = gen.Tc.iter().max().unwrap().clone();

        use std::cmp;
        let new_ctx = Context {
            Ps: ctx.Ps.clone(),
            Pe: ctx.Pe.clone(),
            Ts: ctx.Ts.clone(),
            Te: ctx.Te.iter().map(|&te| cmp::min(te, max_time)).collect(),
            bonus: ctx.bonus,
            
        };

        for (idx, &ass) in assignment.iter().enumerate() {
            if ass != -1 {
                result[idx].push(ass as u32);
            }
        }

        loop {
            let new_assignment = make_assignment(&gen, &new_ctx);
            // println!("{:?}", new_assignment);
            if new_assignment.iter().all(|&a| a == -1) {
                break;
            }

            gen = next_gen(&gen, &new_ctx, &new_assignment);

            for (idx, &ass) in new_assignment.iter().enumerate() {
                if ass != -1 {
                    result[idx].push(ass as u32);
                }
            }
        }
    }

    // println!("{:?}", result);

    return result;
}

#[derive(Default, Clone, Debug)]
struct Generation {
    Tc: Vec<u32>,
    Pc: Vec<(u32, u32)>,
    available: Vec<bool>,
}

impl Generation {
    pub fn new(input: &Input) -> Self {
        Generation {
            Tc: vec![0; input.nvehicles as usize],
            Pc: vec![(0, 0); input.nvehicles as usize],
            available: vec![true; input.nrides as usize],
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Context {
    Ps: Vec<(u32, u32)>,
    Pe: Vec<(u32, u32)>,
    Ts: Vec<u32>,
    Te: Vec<u32>,
    bonus: u32,
}

fn next_gen(gen: &Generation,  ctx: &Context, assignments: &Vec<i32>) ->
    Generation
{
    let mut available = gen.available.clone();
    let mut new_Tc = vec![0; gen.Tc.len()];
    let mut new_Pc = vec![(0, 0); gen.Pc.len()];

    // println!("NEXT GEN assignments len {:?}", assignments.len());
    // println!("NEXT GEN old gen {:?}", gen);

    for (car, &ride) in assignments.iter().enumerate().filter(|&(_, &a)| a != -1) {
        if ride != -1 {
            available[ride as usize] = false;
        }

        new_Tc[car] = gen.Tc[car] + dist(gen.Pc[car], ctx.Ps[ride as usize]) + dist(ctx.Ps[ride as usize], ctx.Pe[ride as usize]);
        new_Pc[car] = ctx.Pe[ride as usize];
    }

    Generation {
        Tc: new_Tc,
        Pc: new_Pc,
        // @TODO
        available: available,
    }
}

fn simple_cost(gen: &Generation, ctx: &Context) -> Vec<Vec<i32>> {
    // println!("GENERATION IN COST FUNC {:?}", gen);
    let ncars = gen.Pc.len();
    let nrides = ctx.Pe.len();
    let mut res = vec![vec![0i32; nrides]; ncars];
    // @TODO: Filter onmogelijke ritten

    let cars_4_ride = cars_for_ride(gen, ctx);
    for c in 0..ncars {
        for r in 0..nrides {
            let dead_dist = dist(gen.Pc[c], ctx.Ps[r]);
            // @TODO: Dit zo laten meetellen? 
            let wait = ctx.Ts[r] as i32 - (gen.Tc[c] as i32 + dist(gen.Pc[c], ctx.Ps[r]) as i32);
        
            let bonus = if wait >= 0 {
                -(ctx.bonus as i32)
            } else {
                0
            };

            res[c][r] = (dead_dist as i32 + wait as i32 + bonus) * (cars_4_ride[r] << 1) as i32;
        }
    }

    res
}

fn cars_for_ride(gen: &Generation, ctx: &Context) -> Vec<u32> {
    let mut res = vec![0; ctx.Ps.len()];
    for ride in 0..ctx.Ps.len() {
        for car in 0..gen.Pc.len() {
            if doable(gen, ctx, car as u32, ride as u32) {
                res[ride] += 1;
            }
        }
    }

    res
}

fn make_assignment(gen: &Generation, ctx: &Context) -> Vec<i32> {
    let costs_for_all_cars = simple_cost(gen, ctx);

    let mut res = vec![0; costs_for_all_cars.len()];
    let mut available = gen.available.clone();

    let mut todo: Vec<usize> = (0..costs_for_all_cars.len()).collect();

    while !todo.is_empty() {
        let mut min_ride_costs = vec![i32::max_value(); costs_for_all_cars.len()];
        let mut min_ride_idxs = vec![-1; costs_for_all_cars.len()];

        for (car, cost_for_one_car) in costs_for_all_cars.iter().enumerate().filter(|&(idx, _)| todo.contains(&idx)) {
            // @TODO: optimize
            // @TODO Haalbare rides
            {
                let available_rides = cost_for_one_car.iter().enumerate().filter(|&(ride, _)| {
                    let doable = doable(gen, ctx, car as u32, ride as u32);
                    // println!("DOABLE {:?}", doable);
                    available[ride] && doable
                });
                for (ride_idx, &cost) in available_rides {
                    if cost < min_ride_costs[car] {
                        min_ride_idxs[car] = ride_idx as i32;
                    }
                }
            }
        }

        let (best_car, _) = min_ride_costs.iter().enumerate().filter(|&(idx, _)| todo.contains(&idx)).min_by_key(|&(_, &v)| v).unwrap();
        res[best_car] = min_ride_idxs[best_car];
        
        todo.remove_item(&best_car);

        if min_ride_idxs[best_car] != -1 {
            available[min_ride_idxs[best_car] as usize] = false;
        }
    }
    
    res
}

fn doable(gen: &Generation, ctx: &Context, car: u32, ride: u32) -> bool {
    (gen.Tc[car as usize] + dist(gen.Pc[car as usize], ctx.Ps[ride as usize]) + dist(ctx.Ps[ride as usize], ctx.Pe[ride as usize])) < ctx.Te[ride as usize]
}

fn dist(start: (u32, u32), end: (u32, u32)) -> u32 {
    ((end.0 as i32 - start.0 as i32).abs() + (end.1 as i32 - start.1 as i32).abs()) as u32
}

impl Ride {
    fn len(&self) -> u32 {
        ((self.end.0 as i32 - self.start.0 as i32).abs() + (self.end.1 as i32 - self.start.1 as i32).abs()) as u32
    }
}