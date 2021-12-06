use std::cmp::Ordering;

fn main() {
    let simple_scan_result = simple_scan(String::from("input.txt"));

    let adv_scan_result = advanced_scan(String::from("input.txt"));

    println!("Count: {}", simple_scan_result);
    println!("Count: {}", adv_scan_result);
}

struct ScanWindow {
    first: i32,
    second: i32,
    third: i32,
}

impl PartialEq for ScanWindow {
    fn eq(&self, other: &Self) -> bool {
        let sum_self = self.first + self.second + self.third;
        let sum_other = other.first + other.second + other.third;
        sum_self == sum_other
    }
}

impl PartialOrd for ScanWindow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let sum_self = self.first + self.second + self.third;
        let sum_other = other.first + other.second + other.third;
        sum_self.partial_cmp(&sum_other)
    }
}

fn file_to_vec(filename: String) -> std::io::Result<Vec<i32>> {
    let file = std::fs::read_to_string(filename).expect("Failed to read input file");

    let mut data = Vec::new();
    for line in file.lines() {
        data.push(line.parse().unwrap())
    }  

    Ok(data)
}

fn perform_scan(data: Vec<i32>) -> std::io::Result<Vec<ScanWindow>> {
    let mut scan: Vec<ScanWindow> = vec![];
    for (idx, val) in data.iter().enumerate() {
        scan.push(
            ScanWindow {
                first: *val,
                second: data[idx+1],
                third: data[idx+2],
            }
        );
        if idx == data.len() - 3 {
            break
        }
    }
    Ok(scan)
}

fn advanced_scan(filename: String) -> i32 {
    let data = file_to_vec(filename).unwrap();
    let scan = perform_scan(data).unwrap();

    let mut depth_inc_cnt = 0;

    for (idx, window) in scan.iter().enumerate() {
        if idx == 0 {
            continue
        }
        if window > &scan[idx-1] {
            depth_inc_cnt = depth_inc_cnt + 1;
        }
    }
    depth_inc_cnt
}

fn simple_scan(filename: String) -> i32 {
    let file = std::fs::read_to_string(filename).expect("Failed to read input file");

    let mut depth_inc_cnt = 0;
    let mut prev_line: i32 = -1;
    let mut curr_line: i32;

    for line in file.lines() {
        curr_line = line.parse().unwrap();
        if prev_line > 0 {
            if prev_line < curr_line {
                depth_inc_cnt = depth_inc_cnt + 1;
            }
        }
        prev_line = line.parse().unwrap();
    }

    depth_inc_cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_windows_are_equal() {
        let some_window = ScanWindow {
            first: 3,
            second: 4,
            third: 5,
        };
        let other_window = ScanWindow {
            first: 4,
            second: 5,
            third: 3,
        };
       assert!(some_window == other_window)
    }

    #[test]
    fn scan_windows_are_not_equal() {
        let some_window = ScanWindow {
            first: 3,
            second: 4,
            third: 5,
        };
        let other_window = ScanWindow {
            first: 4,
            second: 3,
            third: 3,
        };
       assert!(some_window != other_window)
    }

    #[test]
    fn other_scan_greater_than() {
        let some_window = ScanWindow {
            first: 3,
            second: 4,
            third: 5,
        };
        let other_window = ScanWindow {
            first: 4,
            second: 3,
            third: 6,
        };
       assert!(some_window < other_window)
    }
}