use crate::traits::{Close, High, Low, Open, Volume};
use crate::{errors::*, Buy, Sell};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Data item is used as an input for indicators.
///
/// # Example
///
/// ```
/// use ta::DataItem;
/// use ta::{Open, High, Low, Close, Volume, Buy, Sell};
///
/// let item = DataItem::builder()
///     .open(20.0)
///     .high(25.0)
///     .low(15.0)
///     .close(21.0)
///     .volume(7500.0)
///     .buy(1000.0)
///     .sell(6500.0)
///     .build()
///     .unwrap();
///
/// assert_eq!(item.open(), 20.0);
/// assert_eq!(item.high(), 25.0);
/// assert_eq!(item.low(), 15.0);
/// assert_eq!(item.close(), 21.0);
/// assert_eq!(item.volume(), 7500.0);
/// assert_eq!(item.buy(), 1000.0);
/// assert_eq!(item.sell(), 6500.0);
/// ```
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataItem {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    buy: f64,
    sell: f64,
}

impl DataItem {
    pub fn builder() -> DataItemBuilder {
        DataItemBuilder::new()
    }
}

impl Open for DataItem {
    fn open(&self) -> f64 {
        self.open
    }
}

impl High for DataItem {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Low for DataItem {
    fn low(&self) -> f64 {
        self.low
    }
}

impl Close for DataItem {
    fn close(&self) -> f64 {
        self.close
    }
}

impl Volume for DataItem {
    fn volume(&self) -> f64 {
        self.volume
    }
}

impl Buy for DataItem {
    fn buy(&self) -> f64 {
        self.buy
    }
}

impl Sell for DataItem {
    fn sell(&self) -> f64 {
        self.sell
    }
}

pub struct DataItemBuilder {
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    volume: Option<f64>,
    buy: Option<f64>,
    sell: Option<f64>,
}

impl DataItemBuilder {
    pub fn new() -> Self {
        Self {
            open: None,
            high: None,
            low: None,
            close: None,
            volume: None,
            buy: None,
            sell: None,
        }
    }

    pub fn open(mut self, val: f64) -> Self {
        self.open = Some(val);
        self
    }

    pub fn high(mut self, val: f64) -> Self {
        self.high = Some(val);
        self
    }

    pub fn low(mut self, val: f64) -> Self {
        self.low = Some(val);
        self
    }

    pub fn close(mut self, val: f64) -> Self {
        self.close = Some(val);
        self
    }

    pub fn volume(mut self, val: f64) -> Self {
        self.volume = Some(val);
        self
    }

    pub fn buy(mut self, val: f64) -> Self {
        self.buy = Some(val);
        self
    }

    pub fn sell(mut self, val: f64) -> Self {
        self.sell = Some(val);
        self
    }

    pub fn build(self) -> Result<DataItem> {
        if let (Some(open), Some(high), Some(low), Some(close), Some(volume)) =
            (self.open, self.high, self.low, self.close, self.volume)
        {
            let buy = self.buy.unwrap_or(0.0);
            let sell = self.sell.unwrap_or(0.0);
            // validate
            if low <= open
                && low <= close
                && low <= high
                && high >= open
                && high >= close
                && buy >= 0.0
                && sell >= 0.0
                && volume >= 0.0
            {
                let item = DataItem {
                    open,
                    high,
                    low,
                    close,
                    volume,
                    buy,
                    sell,
                };
                Ok(item)
            } else {
                Err(TaError::DataItemInvalid)
            }
        } else {
            Err(TaError::DataItemIncomplete)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        fn assert_valid((open, high, low, close, volume): (f64, f64, f64, f64, f64)) {
            let result = DataItem::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .build();
            assert!(result.is_ok());
        }

        fn assert_invalid(record: (f64, f64, f64, f64, f64)) {
            let (open, high, low, close, volume) = record;
            let result = DataItem::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .build();
            assert_eq!(result, Err(TaError::DataItemInvalid));
        }

        let valid_records = vec![
            // open, high, low , close, volume
            (20.0, 25.0, 15.0, 21.0, 7500.0),
            (10.0, 10.0, 10.0, 10.0, 10.0),
            (0.0, 0.0, 0.0, 0.0, 0.0),
        ];
        for record in valid_records {
            assert_valid(record)
        }

        let invalid_records = vec![
            // open, high, low , close, volume
            (-1.0, 25.0, 15.0, 21.0, 7500.0),
            (20.0, -1.0, 15.0, 21.0, 7500.0),
            (20.0, 25.0, 15.0, -1.0, 7500.0),
            (20.0, 25.0, 15.0, 21.0, -1.0),
            (14.9, 25.0, 15.0, 21.0, 7500.0),
            (25.1, 25.0, 15.0, 21.0, 7500.0),
            (20.0, 25.0, 15.0, 14.9, 7500.0),
            (20.0, 25.0, 15.0, 25.1, 7500.0),
            (20.0, 15.0, 25.0, 21.0, 7500.0),
        ];
        for record in invalid_records {
            assert_invalid(record)
        }
    }
}
