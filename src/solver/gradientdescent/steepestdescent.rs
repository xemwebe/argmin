// Copyright 2018-2020 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Steepest Descent method
//!
//! [SteepestDescent](struct.SteepestDescent.html)
//!
//! # References:
//!
//! [0] Jorge Nocedal and Stephen J. Wright (2006). Numerical Optimization.
//! Springer. ISBN 0-387-30303-0.

use crate::prelude::*;
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// Steepest descent iteratively takes steps in the direction of the strongest negative gradient.
/// In each iteration, a line search is employed to obtain an appropriate step length.
///
/// [Example](https://github.com/argmin-rs/argmin/blob/master/examples/steepestdescent.rs)
///
/// # References:
///
/// [0] Jorge Nocedal and Stephen J. Wright (2006). Numerical Optimization.
/// Springer. ISBN 0-387-30303-0.
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct SteepestDescent<L> {
    /// line search
    linesearch: L,
}

impl<L> SteepestDescent<L> {
    /// Constructor
    pub fn new(linesearch: L) -> Self {
        SteepestDescent { linesearch }
    }
}

impl<O, L> Solver<O> for SteepestDescent<L>
where
    O: ArgminOp<Output = f64>,
    O::Param: Clone
        + Default
        + SerializeAlias
        + ArgminSub<O::Param, O::Param>
        + ArgminDot<O::Param, f64>
        + ArgminScaledAdd<O::Param, f64, O::Param>
        + ArgminMul<f64, O::Param>
        + ArgminSub<O::Param, O::Param>
        + ArgminNorm<f64>,
    O::Hessian: Default,
    L: Clone + ArgminLineSearch<O::Param> + Solver<OpWrapper<O>>,
{
    const NAME: &'static str = "Steepest Descent";

    fn next_iter(
        &mut self,
        op: &mut OpWrapper<O>,
        state: &IterState<O>,
    ) -> Result<ArgminIterData<O>, Error> {
        let param_new = state.get_param();
        let new_cost = op.apply(&param_new)?;
        let new_grad = op.gradient(&param_new)?;

        self.linesearch.set_search_direction(new_grad.mul(&(-1.0)));

        // Run solver
        let ArgminResult {
            operator: line_op,
            state:
                IterState {
                    param: next_param,
                    cost: next_cost,
                    ..
                },
        } = Executor::new(
            OpWrapper::new_from_op(&op),
            self.linesearch.clone(),
            param_new,
        )
        .grad(new_grad)
        .cost(new_cost)
        .ctrlc(false)
        .run()?;

        // hack
        op.consume_op(line_op);

        Ok(ArgminIterData::new().param(next_param).cost(next_cost))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::linesearch::MoreThuenteLineSearch;
    use crate::test_trait_impl;

    test_trait_impl!(
        steepest_descent,
        SteepestDescent<MoreThuenteLineSearch<Vec<f64>>>
    );
}
