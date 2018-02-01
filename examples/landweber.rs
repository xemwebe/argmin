// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(unused_imports)]
#![allow(non_snake_case)]
extern crate argmin;
extern crate ndarray;
use ndarray::{arr1, arr2};
use ndarray::prelude::*;
use argmin::ArgminSolver;
use argmin::operator::ArgminOperator;
use argmin::landweber::Landweber;

fn run() -> Result<(), Box<std::error::Error>> {
    // Set up problem
    let A = arr2(&[[4., 1.], [1., 3.]]);
    let y = arr1(&[1., 2.]);
    let prob = ArgminOperator::new(&A, &y);

    // Set up Newton solver
    let mut solver = Landweber::new(0.01);

    // Initialize the solver
    let init_param = arr1(&[0., 0.]);
    solver.init(&prob, &init_param)?;

    let mut par;
    loop {
        par = solver.next_iter()?;
        // println!("{:?}", par);
        if par.iters >= 1000 {
            break;
        };
    }

    println!("{:?}", par);

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
    }
}