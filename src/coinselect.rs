// Wallet Library for RGB smart contracts
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2019-2025 by Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
// Written in 2024-2025 by Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2019-2024 LNP/BP Standards Association, Switzerland.
// Copyright (C) 2024-2025 LNP/BP Laboratories,
//                         Institute for Distributed and Cognitive Systems (InDCS), Switzerland.
// Copyright (C) 2025 RGB Consortium, Switzerland.
// Copyright (C) 2019-2025 Dr Maxim Orlovsky.
// All rights under the above copyrights are reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

use core::str::FromStr;

use rgb::popls::bp::Coinselect;
use rgb::{CellAddr, StateCalc};
use strict_types::StrictVal;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display, Default)]
#[display(lowercase)]
pub enum CoinselectStrategy {
    /// Collect them most small outputs unless the invoiced value if reached
    #[default]
    Aggregate,

    /// Collect the minimum number of outputs (with the large value) to reduce the resulting input
    /// count
    SmallSize,
}

impl FromStr for CoinselectStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aggregate" => Ok(CoinselectStrategy::Aggregate),
            "smallsize" => Ok(CoinselectStrategy::SmallSize),
            s => Err(s.to_string()),
        }
    }
}

impl Coinselect for CoinselectStrategy {
    fn coinselect(
        &mut self,
        invoiced_state: &StrictVal,
        calc: &mut (impl StateCalc + ?Sized),
        owned_state: Vec<(CellAddr, &StrictVal)>,
    ) -> Option<Vec<CellAddr>> {
        let res = match self {
            CoinselectStrategy::Aggregate => owned_state
                .into_iter()
                .take_while(|(_, val)| {
                    if calc.is_satisfied(invoiced_state) {
                        return false;
                    }
                    calc.accumulate(*val).is_ok()
                })
                .map(|(addr, _)| addr)
                .collect(),
            CoinselectStrategy::SmallSize => owned_state
                .into_iter()
                .rev()
                .take_while(|(_, val)| {
                    if calc.is_satisfied(invoiced_state) {
                        return false;
                    }
                    calc.accumulate(*val).is_ok()
                })
                .map(|(addr, _)| addr)
                .collect(),
        };
        if !calc.is_satisfied(invoiced_state) {
            return None;
        };
        Some(res)
    }
}