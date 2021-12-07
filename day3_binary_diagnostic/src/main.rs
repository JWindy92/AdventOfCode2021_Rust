use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let file = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut rows: HashMap<i32, Vec<String>> = HashMap::new();
    for line in file.lines() {
        let split: Vec<String> = line.split("").map(|s| s.to_string()).collect();
        let trimmed = &split[1..split.len()-1];
        for (idx, diag) in trimmed.iter().enumerate() {
            match rows.entry(idx as i32) {
                Entry::Vacant(e) => { e.insert(vec![diag.to_string()]); }
                Entry::Occupied(mut e) => { e.get_mut().push(diag.to_string()); }
            }
        }
    }
    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for key in 0..rows.keys().len() as i32 {
        println!("{}", key);
        let row = rows.get(&key).unwrap();
        let ones = row.iter().filter(|&diag| *diag == "1").count();
        let zeros = row.len() - ones;
        if ones > zeros {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
        // gamma = rows.get(key).iter().filter(|&diag| *diag == "1").count();        
    }
    let g_dec = isize::from_str_radix(&gamma,2).unwrap() as i32;
    let e_dec = isize::from_str_radix(&epsilon,2).unwrap() as i32;
    println!("gamma: {}", isize::from_str_radix(&gamma,2).unwrap() as i32);
    println!("epsilon: {}", isize::from_str_radix(&epsilon,2).unwrap());
    println!("answer: {}", g_dec * e_dec);
    
}