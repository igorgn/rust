use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups = HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        
        groups.entry(chars).or_insert(Vec::new()).push(s);
    }
    groups.into_values().collect()
    
}

fn main() {
    let result = group_anagrams(vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ]);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_result(mut result: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut result {
            group.sort();
        }
        result.sort();
        result
    }

    #[test]
    fn test_basic_anagrams() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut result = group_anagrams(input);
        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
        ];
        assert_eq!(sort_result(result), sort_result(expected));
    }

    #[test]
    fn test_empty_string() {
        let input = vec!["".to_string()];
        let result = group_anagrams(input);
        assert_eq!(result, vec![vec!["".to_string()]]);
    }

    #[test]
    fn test_single_string() {
        let input = vec!["a".to_string()];
        let result = group_anagrams(input);
        assert_eq!(result, vec![vec!["a".to_string()]]);
    }

    #[test]
    fn test_no_anagrams() {
        let input = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        let mut result = group_anagrams(input);
        let expected = vec![
            vec!["abc".to_string()],
            vec!["def".to_string()],
            vec!["ghi".to_string()],
        ];
        assert_eq!(sort_result(result), sort_result(expected));
    }

    #[test]
    fn test_all_anagrams() {
        let input = vec!["abc".to_string(), "bca".to_string(), "cab".to_string()];
        let mut result = group_anagrams(input);
        let expected = vec![vec!["abc".to_string(), "bca".to_string(), "cab".to_string()]];
        assert_eq!(sort_result(result), sort_result(expected));
    }
}