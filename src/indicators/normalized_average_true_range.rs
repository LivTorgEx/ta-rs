use std::fmt;

use crate::errors::Result;
use crate::indicators::AverageTrueRange;
use crate::{Close, High, Low, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Normalized average true range, expressed as a percentage of the close.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct NormalizedAverageTrueRange {
    atr: AverageTrueRange,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalizedAverageTrueRangeOutput {
    pub value: f64,
    pub atr: f64,
}

impl NormalizedAverageTrueRange {
    pub fn new(period: usize) -> Result<Self> {
        Ok(Self {
            atr: AverageTrueRange::new(period)?,
        })
    }
}

impl Period for NormalizedAverageTrueRange {
    fn period(&self) -> usize {
        self.atr.period()
    }
}

impl<T: High + Low + Close> Next<&T> for NormalizedAverageTrueRange {
    type Output = NormalizedAverageTrueRangeOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        let atr = self.atr.next(input);
        let close = input.close().abs();
        NormalizedAverageTrueRangeOutput {
            value: if close == 0.0 {
                0.0
            } else {
                atr / close * 100.0
            },
            atr,
        }
    }
}

impl Reset for NormalizedAverageTrueRange {
    fn reset(&mut self) {
        self.atr.reset();
    }
}

impl Default for NormalizedAverageTrueRange {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for NormalizedAverageTrueRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NATR({})", self.period())
    }
}
