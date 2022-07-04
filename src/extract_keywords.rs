use super::Match;

pub fn extract_keywords(matches: &Vec<Match>, top: usize) -> Vec<Match> {
    let mut matches = matches.clone();
    matches.sort_unstable_by(|a, b| {
        let a = normalize_tf_idf(a.tf_idf);
        let b: f64 = normalize_tf_idf(b.tf_idf);
        // 直接控制排序, 节约掉事后reverse的时间
        b.partial_cmp(&a).unwrap()
    });

    let result = matches.into_iter().take(top).collect::<Vec<_>>();

    result
}

fn normalize_tf_idf(tf_idf: Option<f64>) -> f64 {
    match tf_idf {
        Some(tf_idf) => tf_idf,
        None => f64::MIN,
    }
}

#[cfg(test)]
mod tests {
    use super::super::TextRange;
    use super::{extract_keywords, Match};

    #[test]
    fn test_matches_more_than_top() {
        let matches: Vec<Match> = vec![
            Match::new(TextRange::new(0, 1), None),
            Match::new(TextRange::new(1, 2), Some(0f64)),
            Match::new(TextRange::new(2, 3), Some(1f64)),
        ];
        let top = 2;

        let result = extract_keywords(&matches, top);

        assert_eq!(result, vec![matches[2].clone(), matches[1].clone()]);
    }

    #[test]
    fn test_matches_less_than_top() {
        let matches: Vec<Match> = vec![
            Match::new(TextRange::new(0, 1), None),
            Match::new(TextRange::new(1, 2), Some(0f64)),
            Match::new(TextRange::new(2, 3), Some(1f64)),
        ];
        let top = 5;

        let result = extract_keywords(&matches, top);

        assert_eq!(
            result,
            vec![matches[2].clone(), matches[1].clone(), matches[0].clone()]
        )
    }
}
