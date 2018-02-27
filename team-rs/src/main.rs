#[macro_use]
extern crate nom;

use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

mod model;
use model::*;

mod alg;
use alg::exec_alg;

fn do_all_files() {
    let paths = fs::read_dir("./input").unwrap();
    
    for path in paths {
        do_single_file(path.unwrap().path().to_str().unwrap());
    }
}

fn do_single_file(path: &str) {
    println!("Parsing '{}'...", path);
    let mut ifs = File::open(path).expect("file not found");
    let mut contents = Vec::new();
    ifs.read_to_end(&mut contents).unwrap();

    let data_in = match input(&contents).unwrap() {
        (i, ref data) if i.is_empty() => data.clone(),
        _ => panic!("Could not finish parsing")
    };
    
    println!("Running algorithm '{}'...", path);

    let data_out = exec_alg(data_in);

    println!("Done. Generating output for '{}'...", path);

    output(&data_out, Path::new(path).file_name().unwrap().to_str().unwrap());
}

fn output(out: &Output, out_path: &str) {
    let mut ofs = File::create(format!("./output/{}.out", out_path)).expect("unable to open output file");

    let _ = ofs.write(format!("something {}\n", 5).as_bytes());
    let _ = ofs.write("something else\n".as_bytes());
}

fn main() {
    // Quick and dirty argument parsing
    let args: Vec<String> = env::args().skip(1).collect();
    let first = args.first().map(|f| f.to_owned());
    match first {
        None => do_all_files(), 
        Some(ref arg) if arg.starts_with("--all") => do_all_files(),
        _ => for arg in args.into_iter() {
            do_single_file(&arg)
        }
    }
}
