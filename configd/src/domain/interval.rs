use crate::domain::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Interval {
    min: Option<f64>,
    max: Option<f64>,
}

impl Interval {
    pub fn new<N, MIN, MAX>(min: MIN, max: MAX) -> Result<Interval, Error>
    where
        MIN: Into<Option<N>>,
        MAX: Into<Option<N>>,
        N: Into<f64>,
    {
        let min = min.into();
        let max = max.into();

        if min.is_none() && max.is_none() {
            return Err(Error::EmptyInterval);
        }

        Ok(Interval {
            min: min.map(|n| n.into()),
            max: max.map(|n| n.into()),
        })
    }

    pub fn min(&self) -> Option<f64> {
        self.min
    }

    pub fn max(&self) -> Option<f64> {
        self.max
    }

    pub fn validate<N: Into<f64>>(&self, num: N) -> bool {
        let num = num.into();

        if let Some(min) = self.min {
            if num < min {
                return false;
            }
        }

        if let Some(max) = self.max {
            if num > max {
                return false;
            }
        }

        true
    }
}
