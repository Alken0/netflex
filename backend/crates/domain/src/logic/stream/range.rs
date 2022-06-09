use std::cmp::min;

use crate::error::Result;
use crate::error::{require, Error};

const DEFAULT_RANGE: u64 = 1048576;

#[derive(Debug)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Default for Range {
    fn default() -> Self {
        Self { start: 0, end: 0 }
    }
}

impl Range {
    pub fn new(start: u64, end: u64) -> Result<Self> {
        require(start < end, "start has to be smaller than end")?;
        Ok(Self { start, end })
    }

    pub fn new_with_start(start: u64) -> Result<Self> {
        let end = start.checked_add(DEFAULT_RANGE).ok_or_else(|| {
            Error::InvalidArgument(format!(
                "default Range.end = start + {DEFAULT_RANGE} > u64::MAX "
            ))
        })?;

        Ok(Self { start, end })
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }

    pub fn set_max_end(&self, max: u64) -> Self {
        let new_end = min(self.end, max);
        let new_start = min(self.start, new_end);
        Self {
            start: new_start,
            end: new_end,
        }
    }

    pub fn limit(&self) -> u64 {
        self.end() - self.start()
    }
}
