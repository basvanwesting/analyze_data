use stats::{MinMax, OnlineStats};

pub struct NumberStats {
    null_count: usize,
    online_stats: OnlineStats,
    min_max: MinMax<f64>,
}

impl NumberStats {
    pub fn new() -> Self {
        Self {
            null_count: 0,
            online_stats: OnlineStats::new(),
            min_max: MinMax::new(),
        }
    }
    pub fn add(&mut self, num: f64) {
        self.online_stats.add(num);
        self.min_max.add(num);
    }
    pub fn add_null(&mut self) {
        self.null_count += 1;
    }
    pub fn count(&self) -> usize {
        self.min_max.len()
    }
    pub fn null_count(&self) -> usize {
        self.null_count
    }
    pub fn min(&self) -> Option<f64> {
        self.min_max.min().copied()
    }
    pub fn max(&self) -> Option<f64> {
        self.min_max.max().copied()
    }
    pub fn mean(&self) -> f64 {
        self.online_stats.mean()
    }
    pub fn stddev(&self) -> f64 {
        self.online_stats.stddev()
    }
}
