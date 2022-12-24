use std::collections::BTreeMap;

pub fn list(start: i32, end: i32, word_map: &BTreeMap<i32, &str>) -> Vec<String> {
    (start..=end).into_iter().map(|i| {
        word_map.into_iter().rev().find(|w| i % w.0 == 0)
            .map_or(i.to_string(), |w| String::from(*w.1))
    }).collect()
}   