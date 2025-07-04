use stats::{MinMax, OnlineStats};

pub struct NumberStats {
    empty_count: usize,
    error_count: usize,
    online_stats: OnlineStats,
    min_max: MinMax<f64>,
    sum: f64,
}

impl NumberStats {
    pub fn new() -> Self {
        Self {
            empty_count: 0,
            error_count: 0,
            online_stats: OnlineStats::new(),
            min_max: MinMax::new(),
            sum: 0.0,
        }
    }
    pub fn add(&mut self, num: f64) {
        self.online_stats.add(num);
        self.min_max.add(num);
        self.sum += num;
    }
    pub fn add_empty(&mut self) {
        self.empty_count += 1;
    }
    pub fn add_error(&mut self) {
        self.error_count += 1;
    }
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.min_max.len()
    }
    #[allow(dead_code)]
    pub fn empty_count(&self) -> usize {
        self.empty_count
    }
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.error_count
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
    pub fn sum(&self) -> f64 {
        self.sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stats = NumberStats::new();
        assert_eq!(stats.count(), 0);
        assert_eq!(stats.empty_count(), 0);
        assert_eq!(stats.error_count(), 0);
        assert_eq!(stats.min(), None);
        assert_eq!(stats.max(), None);
        assert_eq!(stats.sum(), 0.0);
        assert_eq!(stats.mean(), 0.0);
        assert_eq!(stats.stddev(), 0.0);
    }

    #[test]
    fn test_add() {
        let mut stats = NumberStats::new();
        stats.add(1.0);
        stats.add(2.0);
        assert_eq!(stats.count(), 2);
        assert_eq!(stats.min(), Some(1.0));
        assert_eq!(stats.max(), Some(2.0));
        assert_eq!(stats.sum(), 3.0);
        assert_eq!(stats.mean(), 1.5);
        assert!((stats.stddev() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_empty() {
        let mut stats = NumberStats::new();
        stats.add_empty();
        assert_eq!(stats.empty_count(), 1);
    }

    #[test]
    fn test_add_error() {
        let mut stats = NumberStats::new();
        stats.add_error();
        assert_eq!(stats.error_count(), 1);
    }
}
