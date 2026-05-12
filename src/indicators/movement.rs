use std::fmt;

use crate::{Buy, High, Low, Next, Reset, Sell};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Direction of a movement block.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum MovementDirection {
    Long,
    Short,
    None,
}

/// Output produced by [`Movement`] for each candle.
///
/// `block_start` is `true` on the first candle of a new directional block.
/// `block_high` and `block_low` are the price extremes of the current block.
/// `direction` is `None` until at least one candle has been processed.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct MovementOutput {
    pub direction: MovementDirection,
    pub block_start: bool,
    pub block_high: f64,
    pub block_low: f64,
}

/// Simple directional-block indicator.
///
/// Groups consecutive candles by dominant volume direction:
/// if `buy_vol > sell_vol` the candle is Long, otherwise Short.
/// When the direction flips a new block begins.
///
/// No qtym, no percentile, no state machine — pure directional grouping.
///
/// # Example
///
/// ```
/// use ta::indicators::{Movement, MovementDirection};
/// use ta::{Next, DataItem};
///
/// let mut mov = Movement::new();
///
/// let d1 = DataItem::builder()
///     .open(100.0).high(102.0).low(99.0).close(101.0)
///     .volume(1000.0).buy(700.0).sell(300.0)
///     .build().unwrap();
///
/// let out = mov.next(&d1);
/// assert_eq!(out.direction, MovementDirection::Long);
/// assert!(out.block_start);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Movement {
    current_direction: MovementDirection,
    block_high: f64,
    block_low: f64,
    is_new: bool,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            current_direction: MovementDirection::None,
            block_high: 0.0,
            block_low: f64::MAX,
            is_new: true,
        }
    }
}

impl Default for Movement {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Buy + Sell + High + Low> Next<&T> for Movement {
    type Output = MovementOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        let candle_direction = if input.buy() > input.sell() {
            MovementDirection::Long
        } else {
            MovementDirection::Short
        };

        let block_start = if self.is_new {
            self.is_new = false;
            self.current_direction = candle_direction.clone();
            self.block_high = input.high();
            self.block_low = input.low();
            true
        } else if candle_direction != self.current_direction {
            // Direction flipped — start a new block
            self.current_direction = candle_direction.clone();
            self.block_high = input.high();
            self.block_low = input.low();
            true
        } else {
            // Extend current block
            if input.high() > self.block_high {
                self.block_high = input.high();
            }
            if input.low() < self.block_low {
                self.block_low = input.low();
            }
            false
        };

        MovementOutput {
            direction: self.current_direction.clone(),
            block_start,
            block_high: self.block_high,
            block_low: self.block_low,
        }
    }
}

impl Reset for Movement {
    fn reset(&mut self) {
        self.current_direction = MovementDirection::None;
        self.block_high = 0.0;
        self.block_low = f64::MAX;
        self.is_new = true;
    }
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Movement(direction={:?})", self.current_direction)
    }
}
