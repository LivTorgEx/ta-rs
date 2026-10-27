use crate::errors::Result;
use crate::indicators::ExponentialMovingAverage;
use crate::{Close, Next, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Difference between two exponential moving averages.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ExponentialMovingAverageCross {
    fast: ExponentialMovingAverage,
    slow: ExponentialMovingAverage,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExponentialMovingAverageCrossOutput {
    pub value: f64,
    pub fast: f64,
    pub slow: f64,
}

impl ExponentialMovingAverageCross {
    pub fn new(fast_period: usize, slow_period: usize) -> Result<Self> {
        Ok(Self {
            fast: ExponentialMovingAverage::new(fast_period)?,
            slow: ExponentialMovingAverage::new(slow_period)?,
        })
    }
}

impl Next<f64> for ExponentialMovingAverageCross {
    type Output = ExponentialMovingAverageCrossOutput;

    fn next(&mut self, input: f64) -> Self::Output {
        let fast = self.fast.next(input);
        let slow = self.slow.next(input);
        ExponentialMovingAverageCrossOutput {
            value: fast - slow,
            fast,
            slow,
        }
    }
}

impl<T: Close> Next<&T> for ExponentialMovingAverageCross {
    type Output = ExponentialMovingAverageCrossOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for ExponentialMovingAverageCross {
    fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
    }
}
