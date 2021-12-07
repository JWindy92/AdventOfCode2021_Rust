use std::collections::HashMap;
use std::collections::hash_map::Entry;

// fn filter_rows(rows: Vec<Vec<String>>, bit: String, idx: i32) -> Vec<Vec<String>> {
//     return rows.filter(|&row| *row[idx] == bit)
// }

fn get_power_consumption(cols: HashMap<i32, Vec<String>>) -> (String, String) {
    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    
    for key in 0..cols.keys().len() as i32 {
        let col = cols.get(&key).unwrap();
        let ones = col.iter().filter(|&diag| *diag == "1").count();
        let zeros = col.len() - ones;
        if ones > zeros {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }
    (gamma, epsilon)
}

fn get_oxygen_reading(mut rows: Vec<String>) -> String {

    for idx in 0..rows[0].len() {
        let ones = rows.iter().filter(|&row| row.as_bytes()[idx] as char == '1').count();
        let zeros = rows.iter().filter(|&row| row.as_bytes()[idx] as char == '0').count();

        if ones >= zeros {
            rows.retain(|row| row.as_bytes()[idx] as char == '1');
        } else {
            rows.retain(|row| row.as_bytes()[idx] as char == '0');
        }
        if rows.len() == 1 { break }
    }
    rows[0].to_string()
}

fn get_c02_reading(mut rows: Vec<String>) -> String {

    for idx in 0..rows[0].len() {
        let ones = rows.iter().filter(|&row| row.as_bytes()[idx] as char == '1').count();
        let zeros = rows.iter().filter(|&row| row.as_bytes()[idx] as char == '0').count();

        if ones < zeros {
            rows.retain(|row| row.as_bytes()[idx] as char == '1');
        } else {
            rows.retain(|row| row.as_bytes()[idx] as char == '0');
        }
        if rows.len() == 1 { break }
    }
    rows[0].to_string()
}

fn main() {
    
    let file = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut cols: HashMap<i32, Vec<String>> = HashMap::new();
    let mut rows:Vec<String> = vec![];
    for line in file.lines() {
        let split: Vec<String> = line.split("").map(|s| s.to_string()).collect();
        let trimmed = &split[1..split.len()-1];
        for (idx, diag) in trimmed.iter().enumerate() {
            match cols.entry(idx as i32) {
                Entry::Vacant(e) => { e.insert(vec![diag.to_string()]); }
                Entry::Occupied(mut e) => { e.get_mut().push(diag.to_string()); }
            }
        }
        rows.push(String::from(line))
    }

    let (gamma, epsilon) = get_power_consumption(cols.clone());

    let g_dec = isize::from_str_radix(&gamma,2).unwrap() as i32;
    let e_dec = isize::from_str_radix(&epsilon,2).unwrap() as i32;
    println!("gamma: {}", isize::from_str_radix(&gamma,2).unwrap() as i32);
    println!("epsilon: {}", isize::from_str_radix(&epsilon,2).unwrap());
    println!("answer: {}", g_dec * e_dec);
    
    let o2 = get_oxygen_reading(rows.clone());
    let co2 = get_c02_reading(rows.clone());

    let o2_dec = isize::from_str_radix(&o2,2).unwrap() as i32;
    let co2_dec = isize::from_str_radix(&co2,2).unwrap() as i32;
    println!("O2: {}", o2_dec);
    println!("CO2: {}", co2_dec);
    println!("answer: {}", o2_dec * co2_dec);

}