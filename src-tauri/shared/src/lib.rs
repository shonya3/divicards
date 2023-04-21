#![allow(dead_code)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod config;
pub mod csvm;
pub mod error;
pub mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        // let mut map = std::collections::HashMap::<&str, i32>::new();
        // map.insert("The Doctor", 4000);
        // map.insert("The Mayor", 50);
        // let max = map
        //     .iter()
        //     .max_by(|a, b| {
        //         println!("a: {a:?} , b: {b:?}");
        //         a.1.cmp(&b.1)
        //     })
        //     .map(|(k, _v)| k);
        // println!("max: {:?}", max);

        let a = strsim::normalized_damerau_levenshtein("The Hook", "The Hook");
        println!("{a}");
    }
}
