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
use serde::{Deserialize, Serialize};

/// The Dogleg method computes the intersection of the trust region boundary with a path given by
/// the unconstraind minimum along the steepest descent direction and the optimum of the quadratic
/// approximation of the cost function at the current point.
///
/// # References:
///
/// [0] Jorge Nocedal and Stephen J. Wright (2006). Numerical Optimization.
/// Springer. ISBN 0-387-30303-0.
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Default)]
pub struct Dogleg {
    /// Radius
    radius: f64,
}

impl Dogleg {
    /// Constructor
    pub fn new() -> Self {
        Dogleg {
            radius: std::f64::NAN,
        }
    }
}

impl<O> Solver<O> for Dogleg
where
    O: ArgminOp<Output = f64>,
    O::Param: std::fmt::Debug
        + ArgminMul<f64, O::Param>
        + ArgminWeightedDot<O::Param, f64, O::Hessian>
        + ArgminNorm<f64>
        + ArgminDot<O::Param, f64>
        + ArgminAdd<O::Param, O::Param>
        + ArgminSub<O::Param, O::Param>,
    O::Hessian: ArgminInv<O::Hessian> + ArgminDot<O::Param, O::Param>,
{
    const NAME: &'static str = "Dogleg";

    fn next_iter(
        &mut self,
        op: &mut OpWrapper<O>,
        state: &IterState<O>,
    ) -> Result<ArgminIterData<O>, Error> {
        let param = state.get_param();
        let g = state
            .get_grad()
            .unwrap_or_else(|| op.gradient(&param).unwrap());
        let h = state
            .get_hessian()
            .unwrap_or_else(|| op.hessian(&param).unwrap());
        let pstar;

        // pb = -H^-1g
        let pb = (h.inv()?).dot(&g).mul(&(-1.0));

        if pb.norm() <= self.radius {
            pstar = pb;
        } else {
            // pu = - (g^Tg)/(g^THg) * g
            let pu = g.mul(&(-g.dot(&g) / g.weighted_dot(&h, &g)));
            // println!("pb: {:?}, pu: {:?}", pb, pu);

            let utu = pu.dot(&pu);
            let btb = pb.dot(&pb);
            let utb = pu.dot(&pb);

            // compute tau
            let delta = self.radius.powi(2);
            let t1 = 3.0 * utb - btb - 2.0 * utu;
            let t2 =
                (utb.powi(2) - 2.0 * utb * delta + delta * btb - btb * utu + delta * utu).sqrt();
            let t3 = -2.0 * utb + btb + utu;
            let tau1: f64 = -(t1 + t2) / t3;
            let tau2: f64 = -(t1 - t2) / t3;

            // pick maximum value of both -- not sure if this is the proper way
            let mut tau = tau1.max(tau2);

            // if calculation failed because t3 is too small, use the third option
            // println!("t1: {:?}", tau);
            if tau.is_nan() || tau.is_infinite() {
                tau = (delta + btb - 2.0 * utu) / (btb - utu);
                // println!("btb: {:?}", btb);
                // println!("utu: {:?}", utu);
            }
            // println!("t2: {:?}", tau);

            if tau >= 0.0 && tau < 1.0 {
                pstar = pu.mul(&tau);
            } else if tau >= 1.0 && tau <= 2.0 {
                // pstar = pu + (tau - 1.0) * (pb - pu)
                pstar = pu.add(&pb.sub(&pu).mul(&(tau - 1.0)));
            } else {
                // println!("{:?}", tau);
                return Err(ArgminError::ImpossibleError {
                    text: "tau is bigger than 2, this is not supposed to happen.".to_string(),
                }
                .into());
            }
        }
        let out = ArgminIterData::new().param(pstar);
        Ok(out)
    }

    fn terminate(&mut self, state: &IterState<O>) -> TerminationReason {
        if state.get_iter() >= 1 {
            TerminationReason::MaxItersReached
        } else {
            TerminationReason::NotTerminated
        }
    }
}

impl ArgminTrustRegion for Dogleg {
    fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_trait_impl;

    test_trait_impl!(dogleg, Dogleg);
}
