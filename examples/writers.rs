// Copyright 2018-2020 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate argmin;
extern crate ndarray;
use argmin::prelude::*;
use argmin::solver::linesearch::MoreThuenteLineSearch;
use argmin::solver::quasinewton::BFGS;
use argmin::testfunctions::rosenbrock;
use argmin_core::finitediff::*;
use ndarray::{array, Array1, Array2};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

#[cfg_attr(serde1, derive(Serialize, Deserialize))]
#[derive(Clone, Default)]
struct Rosenbrock {
    a: f64,
    b: f64,
}

impl ArgminOp for Rosenbrock {
    type Param = Array1<f64>;
    type Output = f64;
    type Hessian = Array2<f64>;
    type Jacobian = ();

    fn apply(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        Ok(rosenbrock(&p.to_vec(), self.a, self.b))
    }

    fn gradient(&self, p: &Self::Param) -> Result<Self::Param, Error> {
        Ok((*p).forward_diff(&|x| rosenbrock(&x.to_vec(), self.a, self.b)))
    }
}

fn run() -> Result<(), Error> {
    // Define cost function
    let cost = Rosenbrock { a: 1.0, b: 100.0 };

    // Define initial parameter vector
    // let init_param: Array1<f64> = array![-1.2, 1.0];
    let init_param: Array1<f64> = array![-1.2, 1.0, -10.0, 2.0, 3.0, 2.0, 4.0, 10.0];
    let init_hessian: Array2<f64> = Array2::eye(8);

    // set up a line search
    let linesearch = MoreThuenteLineSearch::new();

    // Set up solver
    let solver = BFGS::new(init_hessian, linesearch);

    // Create writer
    #[cfg(serde1)]
    let writer = WriteToFile::new("params", "param")
        // Set serializer to JSON
        .serializer(WriteToFileSerializer::JSON);

    // Create writer which only saves new best ones
    #[cfg(serde1)]
    let writer2 = WriteToFile::new("params", "best")
        // Set serializer to JSON
        .serializer(WriteToFileSerializer::JSON);

    #[cfg(serde1)]
    let res = Executor::new(cost, solver, init_param)
        .max_iters(10)
        .add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
        .add_observer(writer, ObserverMode::Every(3))
        .add_observer(writer2, ObserverMode::NewBest)
        .run()?;

    #[cfg(not(serde1))]
    let res = Executor::new(cost, solver, init_param)
        .max_iters(10)
        .add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
        .run()?;

    // Wait a second (lets the logger flush everything before printing again)
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Print result
    println!("{}", res);
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("{} {}", e.as_fail(), e.backtrace());
        std::process::exit(1);
    }
}
