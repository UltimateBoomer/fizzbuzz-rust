use std::collections::BTreeMap;

pub fn list(start: i32, end: i32, word_map: &BTreeMap<i32, &str>) -> Vec<String> {
    (start..=end).into_iter().map(|i| {
        word_map.into_iter().rev().find(|&w| i % w.0 == 0)
            .map_or(i.to_string(), |w| String::from(*w.1))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list1() {
        let r = list(1, 0, &BTreeMap::new());
        assert!(r.is_empty());
    }

    #[test]
    fn test_list2() {
        let r = list(1, 1, &BTreeMap::new());
        assert_eq!(vec!["1"], r);
    }

    #[test]
    fn test_list3() {
        let r = list(1, 1, &BTreeMap::from([(1, "a")]));
        assert_eq!(vec!["a"], r);
    }

    #[test]
    fn test_list4() {
        let r = list(1, 6, &BTreeMap::from([(2, "a"), (3, "b"), (6, "ab")]));
        assert_eq!(vec!["1", "a", "b", "a", "5", "ab"], r);
    }
}