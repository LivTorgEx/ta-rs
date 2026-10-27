use crate::errors::Result;
use crate::indicators::{ExponentialMovingAverage, FastStochastic};
use crate::{Close, High, Low, Next, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Stochastic oscillator with separately configurable %K and %D smoothing.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct StochasticOscillator {
    fast: FastStochastic,
    slow_k: ExponentialMovingAverage,
    slow_d: ExponentialMovingAverage,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StochasticOscillatorOutput {
    pub k: f64,
    pub d: f64,
}

impl StochasticOscillator {
    pub fn new(fast_period: usize, slow_k_period: usize, slow_d_period: usize) -> Result<Self> {
        Ok(Self {
            fast: FastStochastic::new(fast_period)?,
            slow_k: ExponentialMovingAverage::new(slow_k_period)?,
            slow_d: ExponentialMovingAverage::new(slow_d_period)?,
        })
    }
}

impl<T: High + Low + Close> Next<&T> for StochasticOscillator {
    type Output = StochasticOscillatorOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        let k = self.slow_k.next(self.fast.next(input));
        let d = self.slow_d.next(k);
        StochasticOscillatorOutput { k, d }
    }
}

impl Reset for StochasticOscillator {
    fn reset(&mut self) {
        self.fast.reset();
        self.slow_k.reset();
        self.slow_d.reset();
    }
}
