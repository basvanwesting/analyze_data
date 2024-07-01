use stats::MinMax;
use std::collections::HashSet;

pub struct StringStats {
    null_count: usize,
    min_max: MinMax<String>,
    frequencies: HashSet<String>,
    cardinality_cap: Option<usize>,
}

impl StringStats {
    pub fn new(cardinality_cap: Option<usize>) -> Self {
        Self {
            null_count: 0,
            min_max: MinMax::new(),
            // frequencies: HashSet::with_capacity(cardinality_cap.unwrap_or(0)),
            frequencies: HashSet::new(),
            cardinality_cap,
        }
    }
    pub fn add(&mut self, string: String) {
        self.min_max.add(string.clone());
        if let Some(cap) = self.cardinality_cap {
            if cap == 0 {
                // skip cardinality
            } else if self.frequencies.len() > cap {
                // skip adding cardinality
            } else {
                self.frequencies.insert(string);
            }
        } else {
            self.frequencies.insert(string);
        }
    }
    pub fn add_null(&mut self) {
        self.null_count += 1;
    }
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.min_max.len()
    }
    #[allow(dead_code)]
    pub fn null_count(&self) -> usize {
        self.null_count
    }
    pub fn min(&self) -> Option<String> {
        self.min_max.min().cloned()
    }
    pub fn max(&self) -> Option<String> {
        self.min_max.max().cloned()
    }
    pub fn cardinality(&self) -> usize {
        self.frequencies.len()
    }
    pub fn is_cardinality_capped(&self) -> bool {
        if let Some(cap) = self.cardinality_cap {
            self.frequencies.len() > cap
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stats = StringStats::new(None);
        assert_eq!(stats.count(), 0);
        assert_eq!(stats.null_count(), 0);
        assert_eq!(stats.min(), None);
        assert_eq!(stats.max(), None);
    }

    #[test]
    fn test_add() {
        let mut stats = StringStats::new(None);
        stats.add("test".to_string());
        assert_eq!(stats.count(), 1);
        assert_eq!(stats.min(), Some("test".to_string()));
        assert_eq!(stats.max(), Some("test".to_string()));
    }

    #[test]
    fn test_add_null() {
        let mut stats = StringStats::new(None);
        stats.add_null();
        assert_eq!(stats.null_count(), 1);
    }

    #[test]
    fn test_min_max() {
        let mut stats = StringStats::new(None);
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
        let stats = StringStats::new(None);
        assert_eq!(stats.count(), 0);
        assert_eq!(stats.null_count(), 0);
        assert_eq!(stats.min(), None);
        assert_eq!(stats.max(), None);
    }

    #[test]
    fn test_add() {
        let mut stats = StringStats::new(None);
        stats.add("test".to_string());
        assert_eq!(stats.count(), 1);
        assert_eq!(stats.min(), Some("test".to_string()));
        assert_eq!(stats.max(), Some("test".to_string()));
    }

    #[test]
    fn test_add_null() {
        let mut stats = StringStats::new(None);
        stats.add_null();
        assert_eq!(stats.null_count(), 1);
    }

    #[test]
    fn test_min_max() {
        let mut stats = StringStats::new(None);
        stats.add("a".to_string());
        stats.add("b".to_string());
        assert_eq!(stats.min(), Some("a".to_string()));
        assert_eq!(stats.max(), Some("b".to_string()));
    }

    #[test]
    fn test_cardinality() {
        let mut stats = StringStats::new(None);
        stats.add("a".to_string());
        stats.add("b".to_string());
        stats.add("a".to_string());
        assert_eq!(stats.cardinality(), 2);
        assert!(!stats.is_cardinality_capped());
    }

    #[test]
    fn test_cardinality_cap() {
        let mut stats = StringStats::new(Some(2));
        stats.add("a".to_string());
        stats.add("b".to_string());
        stats.add("a".to_string());
        assert_eq!(stats.cardinality(), 2);
        assert!(!stats.is_cardinality_capped());

        stats.add("c".to_string());
        assert_eq!(stats.cardinality(), 3);
        assert!(stats.is_cardinality_capped());

        stats.add("d".to_string());
        assert_eq!(stats.cardinality(), 3);
        assert!(stats.is_cardinality_capped());
    }
}
