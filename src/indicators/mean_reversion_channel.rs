use std::f64::consts::PI;

use crate::{Close, High, Low, Next, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const INNER_MULTIPLIER: f64 = PI;
const SMALL_MULTIPLIER: f64 = 1.77 * PI;
const OUTER_MULTIPLIER: f64 = 2.415 * PI;
const BIG_MULTIPLIER: f64 = 3.04 * PI;

/// Mean Reversion Channel based on two-pole Super Smoothers for typical price
/// and true range. Band factors follow KLinecharts MRC and are multiplied by PI.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct MeanReversionChannel {
    period: usize,
    c1: f64,
    c2: f64,
    c3: f64,
    price_input_prev: Option<f64>,
    price_output_prev: Option<f64>,
    price_output_prev2: Option<f64>,
    range_input_prev: Option<f64>,
    range_output_prev: Option<f64>,
    range_output_prev2: Option<f64>,
    previous_close: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeanReversionChannelOutput {
    pub middle: f64,
    pub up_small: f64,
    pub down_small: f64,
    pub up_inner: f64,
    pub down_inner: f64,
    pub up_outer: f64,
    pub down_outer: f64,
    pub up_big: f64,
    pub down_big: f64,
}

impl MeanReversionChannel {
    pub fn new(period: usize) -> Self {
        let angle = std::f64::consts::SQRT_2 * PI / period.max(1) as f64;
        let a = (-angle).exp();
        let c2 = 2.0 * a * angle.cos();
        let c3 = -a * a;
        Self {
            period: period.max(1),
            c1: 1.0 - c2 - c3,
            c2,
            c3,
            price_input_prev: None,
            price_output_prev: None,
            price_output_prev2: None,
            range_input_prev: None,
            range_output_prev: None,
            range_output_prev2: None,
            previous_close: None,
        }
    }

    pub fn period(&self) -> usize {
        self.period
    }

    fn smooth(
        input: f64,
        input_previous: Option<f64>,
        output_previous: Option<f64>,
        output_previous2: Option<f64>,
        c1: f64,
        c2: f64,
        c3: f64,
    ) -> f64 {
        let output_previous = output_previous.or(input_previous).unwrap_or(0.0);
        let output_previous2 = output_previous2.unwrap_or(0.0);
        c1 * input + c2 * output_previous + c3 * output_previous2
    }
}

impl Default for MeanReversionChannel {
    fn default() -> Self {
        Self::new(200)
    }
}

impl<T: High + Low + Close> Next<&T> for MeanReversionChannel {
    type Output = MeanReversionChannelOutput;

    fn next(&mut self, bar: &T) -> Self::Output {
        let close = bar.close();
        let typical = (bar.high() + bar.low() + close) / 3.0;
        let true_range = self
            .previous_close
            .map_or(bar.high() - bar.low(), |previous| {
                (bar.high() - bar.low())
                    .max((bar.high() - previous).abs())
                    .max((bar.low() - previous).abs())
            });
        self.previous_close = Some(close);

        let middle = Self::smooth(
            typical,
            self.price_input_prev,
            self.price_output_prev,
            self.price_output_prev2,
            self.c1,
            self.c2,
            self.c3,
        );
        let range = Self::smooth(
            true_range,
            self.range_input_prev,
            self.range_output_prev,
            self.range_output_prev2,
            self.c1,
            self.c2,
            self.c3,
        );
        self.price_input_prev = Some(typical);
        self.price_output_prev2 = self.price_output_prev;
        self.price_output_prev = Some(middle);
        self.range_input_prev = Some(true_range);
        self.range_output_prev2 = self.range_output_prev;
        self.range_output_prev = Some(range);

        MeanReversionChannelOutput {
            middle,
            up_inner: middle + range * INNER_MULTIPLIER,
            down_inner: middle - range * INNER_MULTIPLIER,
            up_small: middle + range * SMALL_MULTIPLIER,
            down_small: middle - range * SMALL_MULTIPLIER,
            up_outer: middle + range * OUTER_MULTIPLIER,
            down_outer: middle - range * OUTER_MULTIPLIER,
            up_big: middle + range * BIG_MULTIPLIER,
            down_big: middle - range * BIG_MULTIPLIER,
        }
    }
}

impl Reset for MeanReversionChannel {
    fn reset(&mut self) {
        *self = Self::new(self.period);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataItem;

    fn bar(high: f64, low: f64, close: f64) -> DataItem {
        DataItem::builder()
            .open(close)
            .high(high)
            .low(low)
            .close(close)
            .volume(1.0)
            .build()
            .unwrap()
    }

    #[test]
    fn channel_bands_are_ordered_around_middle() {
        let mut mrc = MeanReversionChannel::new(20);
        let output = mrc.next(&bar(12.0, 8.0, 10.0));
        assert!(output.down_outer < output.down_inner);
        assert!(output.down_inner < output.middle);
        assert!(output.middle < output.up_inner);
        assert!(output.up_inner < output.up_outer);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialized_state_continues_identically() {
        let mut original = MeanReversionChannel::new(20);
        for index in 0..5 {
            original.next(&bar(10.0 + index as f64, 8.0, 9.0 + index as f64));
        }
        let encoded = bincode::serialize(&original).unwrap();
        let mut restored: MeanReversionChannel = bincode::deserialize(&encoded).unwrap();
        let next = bar(18.0, 9.0, 14.0);
        assert_eq!(original.next(&next), restored.next(&next));
    }
}
