use cardinality_estimator::CardinalityEstimator;
use stats::MinMax;

pub struct StringStats {
    empty_count: usize,
    error_count: usize,
    min_max: MinMax<String>,
    cardinality_estimator: CardinalityEstimator<String>,
}

impl StringStats {
    pub fn new() -> Self {
        Self {
            empty_count: 0,
            error_count: 0,
            min_max: MinMax::new(),
            cardinality_estimator: CardinalityEstimator::new(),
        }
    }
    pub fn add(&mut self, string: String) {
        self.cardinality_estimator.insert(&string);
        self.min_max.add(string);
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
    pub fn min(&self) -> Option<String> {
        self.min_max.min().cloned()
    }
    pub fn max(&self) -> Option<String> {
        self.min_max.max().cloned()
    }
    pub fn cardinality(&self) -> usize {
        self.cardinality_estimator.estimate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stats = StringStats::new();
        assert_eq!(stats.count(), 0);
        assert_eq!(stats.empty_count(), 0);
        assert_eq!(stats.error_count(), 0);
        assert_eq!(stats.min(), None);
        assert_eq!(stats.max(), None);
    }

    #[test]
    fn test_add() {
        let mut stats = StringStats::new();
        stats.add("test".to_string());
        assert_eq!(stats.count(), 1);
        assert_eq!(stats.min(), Some("test".to_string()));
        assert_eq!(stats.max(), Some("test".to_string()));
    }

    #[test]
    fn test_add_empty() {
        let mut stats = StringStats::new();
        stats.add_empty();
        assert_eq!(stats.empty_count(), 1);
    }

    #[test]
    fn test_min_max() {
        let mut stats = StringStats::new();
        stats.add("a".to_string());
        stats.add("b".to_string());
        assert_eq!(stats.min(), Some("a".to_string()));
        assert_eq!(stats.max(), Some("b".to_string()));
    }
}
#[cfg(test)]
mod string_stats_tests {
    use super::*;

    #[test]
    fn test_new() {
        let stats = StringStats::new();
        assert_eq!(stats.count(), 0);
        assert_eq!(stats.empty_count(), 0);
        assert_eq!(stats.error_count(), 0);
        assert_eq!(stats.min(), None);
        assert_eq!(stats.max(), None);
    }

    #[test]
    fn test_add() {
        let mut stats = StringStats::new();
        stats.add("test".to_string());
        assert_eq!(stats.count(), 1);
        assert_eq!(stats.min(), Some("test".to_string()));
        assert_eq!(stats.max(), Some("test".to_string()));
    }

    #[test]
    fn test_add_empty() {
        let mut stats = StringStats::new();
        stats.add_empty();
        assert_eq!(stats.empty_count(), 1);
    }

    #[test]
    fn test_add_error() {
        let mut stats = StringStats::new();
        stats.add_error();
        assert_eq!(stats.error_count(), 1);
    }

    #[test]
    fn test_min_max() {
        let mut stats = StringStats::new();
        stats.add("a".to_string());
        stats.add("b".to_string());
        assert_eq!(stats.min(), Some("a".to_string()));
        assert_eq!(stats.max(), Some("b".to_string()));
    }

    #[test]
    fn test_cardinality() {
        let mut stats = StringStats::new();
        stats.add("a".to_string());
        stats.add("b".to_string());
        stats.add("a".to_string());
        assert_eq!(stats.cardinality(), 2);
    }
}
