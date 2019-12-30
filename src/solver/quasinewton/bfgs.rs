// Copyright 2018-2020 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # References:
//!
//! [0] Jorge Nocedal and Stephen J. Wright (2006). Numerical Optimization.
//! Springer. ISBN 0-387-30303-0.

use crate::prelude::*;
#[cfg(feature = "serde1")]
use serde::de::DeserializeOwned;
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// BFGS method
///
/// [Example](https://github.com/argmin-rs/argmin/blob/master/examples/bfgs.rs)
///
/// # References:
///
/// [0] Jorge Nocedal and Stephen J. Wright (2006). Numerical Optimization.
/// Springer. ISBN 0-387-30303-0.
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct BFGS<L, H> {
    /// Inverse Hessian
    inv_hessian: H,
    /// line search
    linesearch: L,
}

impl<L, H> BFGS<L, H> {
    /// Constructor
    pub fn new(init_inverse_hessian: H, linesearch: L) -> Self {
        BFGS {
            inv_hessian: init_inverse_hessian,
            linesearch,
        }
    }
}

impl<O, L, H> Solver<O> for BFGS<L, H>
where
    O: ArgminOp<Output = f64, Hessian = H>,
    O::Param: Debug
        + Default
        + ArgminSub<O::Param, O::Param>
        + ArgminDot<O::Param, f64>
        + ArgminDot<O::Param, O::Hessian>
        + ArgminScaledAdd<O::Param, f64, O::Param>
        + ArgminNorm<f64>
        + ArgminMul<f64, O::Param>,
    O::Hessian: Clone
        + Default
        + Debug
        + ArgminSub<O::Hessian, O::Hessian>
        + ArgminDot<O::Param, O::Param>
        + ArgminDot<O::Hessian, O::Hessian>
        + ArgminAdd<O::Hessian, O::Hessian>
        + ArgminMul<f64, O::Hessian>
        + ArgminTranspose
        + ArgminEye,
    L: Clone + ArgminLineSearch<O::Param> + Solver<OpWrapper<O>>,
{
    const NAME: &'static str = "BFGS";

    fn init(
        &mut self,
        op: &mut OpWrapper<O>,
        state: &IterState<O>,
    ) -> Result<Option<ArgminIterData<O>>, Error> {
        let param = state.get_param();
        let cost = op.apply(&param)?;
        let grad = op.gradient(&param)?;
        Ok(Some(
            ArgminIterData::new().param(param).cost(cost).grad(grad),
        ))
    }

    fn next_iter(
        &mut self,
        op: &mut OpWrapper<O>,
        state: &IterState<O>,
    ) -> Result<ArgminIterData<O>, Error> {
        let param = state.get_param();
        let cur_cost = state.get_cost();
        let prev_grad = state.get_grad().unwrap();
        // let prev_grad = if let Some(grad) = state.get_grad() {
        //     grad
        // } else {
        //     op.gradient(&param)?
        // };

        let p = self.inv_hessian.dot(&prev_grad).mul(&(-1.0));

        self.linesearch.set_search_direction(p);

        // Run solver
        let ArgminResult {
            operator: line_op,
            state:
                IterState {
                    param: xk1,
                    cost: next_cost,
                    ..
                },
        } = Executor::new(
            OpWrapper::new_from_op(&op),
            self.linesearch.clone(),
            param.clone(),
        )
        .grad(prev_grad.clone())
        .cost(cur_cost)
        .ctrlc(false)
        .run()?;

        // take care of function eval counts
        op.consume_op(line_op);

        let grad = op.gradient(&xk1)?;
        // let next_cost = op.apply(&xk1)?;

        let yk = grad.sub(&prev_grad);

        let sk = xk1.sub(&param);

        let yksk: f64 = yk.dot(&sk);
        let rhok = 1.0 / yksk;

        let e = self.inv_hessian.eye_like();
        let mat1: O::Hessian = sk.dot(&yk);
        let mat1 = mat1.mul(&rhok);

        let mat2 = mat1.clone().t();

        let tmp1 = e.sub(&mat1);
        let tmp2 = e.sub(&mat2);

        let sksk: O::Hessian = sk.dot(&sk);
        let sksk = sksk.mul(&rhok);

        // if state.get_iter() == 0 {
        //     let ykyk: f64 = yk.dot(&yk);
        //     self.inv_hessian = self.inv_hessian.eye_like().mul(&(yksk / ykyk));
        //     println!("{:?}", self.inv_hessian);
        // }

        self.inv_hessian = tmp1.dot(&self.inv_hessian.dot(&tmp2)).add(&sksk);

        Ok(ArgminIterData::new().param(xk1).cost(next_cost).grad(grad))
    }

    fn terminate(&mut self, state: &IterState<O>) -> TerminationReason {
        if state.get_grad().unwrap().norm() < std::f64::EPSILON.sqrt() {
            return TerminationReason::TargetPrecisionReached;
        }
        if (state.get_prev_cost() - state.get_cost()).abs() < std::f64::EPSILON {
            return TerminationReason::NoChangeInCost;
        }
        TerminationReason::NotTerminated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::linesearch::MoreThuenteLineSearch;
    use crate::test_trait_impl;

    type Operator = MinimalNoOperator;

    test_trait_impl!(bfgs, BFGS<Operator, MoreThuenteLineSearch<Operator>>);
}
