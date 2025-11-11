use std::collections::HashSet;

/// Functions for generating and uniquifying names.
use rand::Rng;
// Common names from around the world.
const NAMES: [&str; 12] = [
    "Bob", "Alice", "Cali", "Arjun", "Bianca", "Kalyna", "Chen", "Zhu", "Cielo", "Eva", "Franco",
    "Lopa",
];

/// Return n distinct names, where n is up to the length of NAMES.
pub fn get_names(n: usize) -> Result<Vec<String>, &'static str> {
    if n > NAMES.len() {
        return Err("Request a smaller number of names");
    }

    let mut rng = rand::rng();
    // create n random indices.
    let mut indices: Vec<u8> = Vec::new();
    while indices.len() < n {
        let i = rng.random_range(0..n) as u8;
        if !indices.contains(&i) {
            indices.push(i);
        }
    }
    Ok(indices
        .iter()
        .map(|i| NAMES[*i as usize].to_string())
        .collect())
}

/// Modify the incoming list to make them distinct.
#[allow(clippy::ptr_arg)]
pub fn uniquify(names: &Vec<String>) -> Vec<String> {
    let mut names = names.clone();
    let mut names_set: HashSet<String> = HashSet::from_iter(names.iter().cloned());
    if names.len() == names_set.len() {
        return names;
    }
    // Add random digits to the end of names until the list contains only distinct values.
    let mut rng = rand::rng();
    for i in 0..names.len() - 1 {
        if names[(i + 1)..].contains(&names[i]) {
            let d = rng.random_range(0..10).to_string();
            names[i] = names[i].clone() + &d;
        }
    }
    names_set = HashSet::from_iter(names.iter().cloned());
    if names.len() == names_set.len() {
        names
    } else {
        uniquify(&names)
    }
}

/// Modify name to make it distinct with respect to names.
#[allow(clippy::ptr_arg)]
pub fn uniquify_name(name: &String, names: &Vec<String>) -> String {
    if !names.contains(name) {
        return name.to_owned();
    }
    // Add a random digit to the end of name.
    let mut rng = rand::rng();
    let d = rng.random_range(0..10).to_string();
    let name_plus = name.clone() + &d;
    if !names.contains(&name_plus) {
        name_plus
    } else {
        uniquify_name(&name_plus, names)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_names() {
        let six_names_result = get_names(6);
        assert!(
            six_names_result.is_ok(),
            "Should be able to retrieve six names."
        );
        let six_names = six_names_result.unwrap();
        assert!(
            six_names.len() == 6,
            "Expected six_names.len() == 6, was {}",
            six_names.len()
        );
        let max_names_result = get_names(NAMES.len());
        assert!(
            max_names_result.is_ok(),
            "Should be able to retrieve the max number of names ({}).",
            NAMES.len()
        );
        let max_names = max_names_result.unwrap();
        assert!(
            max_names.len() == NAMES.len(),
            "Expected max_names.len() == {}, was {}",
            NAMES.len(),
            max_names.len()
        );
        max_names.iter().for_each(|name| {
            assert!(
                NAMES.contains(&&name[..]),
                "Expected NAMES to contain {}",
                name
            )
        });
        let too_many_names_result = get_names(NAMES.len() + 1);
        assert!(
            too_many_names_result.is_err(),
            "Should not be able to retrieve more than max number of names ({}).",
            NAMES.len()
        );
        let no_names_result = get_names(0);
        assert!(
            no_names_result.is_ok(),
            "Should be able to retrieve zero names."
        );
        let no_names = no_names_result.unwrap();
        assert!(
            no_names.is_empty(),
            "Expected no_names to be empty, length was {}",
            no_names.len()
        );
    }

    #[test]
    fn test_uniquify_names() {
        let dups = vec!["a".to_string(), "a".to_string()];
        let result = uniquify(&dups);
        assert!(
            dups.len() == result.len(),
            "Expected result to have same length ({}), was {}",
            dups.len(),
            result.len()
        );
        let uniqs: HashSet<String> = HashSet::from_iter(result.iter().cloned());
        assert!(
            dups.len() == uniqs.len(),
            "Expected unique names to have same length ({}) as result, was {}",
            result.len(),
            uniqs.len()
        );
        let uniqs = vec!["a".to_string(), "b".to_string()];
        let result = uniquify(&uniqs);
        (0..2).for_each(|i| {
            assert!(
                uniqs[i] == result[i],
                "Expected function to be stable, found {} in place of {}",
                result[i],
                uniqs[i]
            )
        });
        let many_dups = vec![
            "Bob".to_string(),
            "Bob".to_string(),
            "Bob".to_string(),
            "Bob".to_string(),
            "Bob".to_string(),
            "Bob".to_string(),
        ];
        let result = uniquify(&many_dups);
        assert!(
            many_dups.len() == result.len(),
            "Expected result to have same length as original ({}), was {}",
            many_dups.len(),
            result.len()
        );
        let uniqs: HashSet<String> = HashSet::from_iter(result.iter().cloned());
        assert!(
            result.len() == uniqs.len(),
            "Expected unique names to have same length ({}) as result, was {}",
            result.len(),
            uniqs.len()
        );
    }

    #[test]
    fn test_uniquify_name() {
        let name = "a".to_string();
        let names = vec!["a".to_string()];
        let result = uniquify_name(&name, &names);
        println!("result: {}", result);
        assert!(
            name != result,
            "Expected name to have changed from {}",
            name
        );
        assert!(
            !names.contains(&result),
            "Expected result ({}) to be distinct from values in {:?}",
            result,
            names
        );
        let alphabet_minus_n = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'o', 'p', 'q', 'r',
            's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ]
        .iter()
        .map(|c| c.to_string())
        .collect();
        let n = "n".to_string();
        let result = uniquify_name(&n, &alphabet_minus_n);
        println!("result: {}", result);
        assert!(
            n == result,
            "Expected n to be unchanged ({}), was {}",
            n,
            result
        );
        let result = uniquify_name(&n, &vec![]);
        assert!(n == result, "Expected n to be unchanged, was {}", result);
    }
}
