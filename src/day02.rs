use std::fs;

fn is_gradual(levels: &[i32], skip_index: Option<usize>) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut prev_idx = 0;
    while prev_idx < levels.len() - 1 {
        // Skip the level we're testing removal of
        if Some(prev_idx) == skip_index {
            prev_idx += 1;
            continue;
        }
        let next_idx = prev_idx + 1;
        if Some(next_idx) == skip_index {
            // If we're skipping the next index, compare with the one after
            if next_idx >= levels.len() - 1 {
                break;
            }
            let diff = levels[next_idx + 1] - levels[prev_idx];
            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }
            prev_idx = next_idx + 1;
            continue;
        }
        
        let diff = levels[next_idx] - levels[prev_idx];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        prev_idx = next_idx;
    }
    true
}

fn is_consistent_direction(levels: &[i32], skip_index: Option<usize>) -> Option<bool> {
    if levels.len() < 2 {
        return Some(true);
    }

    let mut increasing: Option<bool> = None;
    let mut prev_idx = 0;
    
    while prev_idx < levels.len() - 1 {
        if Some(prev_idx) == skip_index {
            prev_idx += 1;
            continue;
        }
        let next_idx = prev_idx + 1;
        if Some(next_idx) == skip_index {
            if next_idx >= levels.len() - 1 {
                break;
            }
            let diff = levels[next_idx + 1] - levels[prev_idx];
            if increasing.is_none() {
                increasing = Some(diff > 0);
            } else if (diff > 0) != increasing.unwrap() {
                return None;
            }
            prev_idx = next_idx + 1;
            continue;
        }

        let diff = levels[next_idx] - levels[prev_idx];
        if diff == 0 {
            return None;
        }
        if increasing.is_none() {
            increasing = Some(diff > 0);
        } else if (diff > 0) != increasing.unwrap() {
            return None;
        }
        prev_idx = next_idx;
    }
    increasing
}

fn is_safe_report(levels: &[i32], skip_index: Option<usize>) -> bool {
    is_consistent_direction(levels, skip_index).is_some() && is_gradual(levels, skip_index)
}

pub fn main() {
    // let data = fs::read_to_string("02/input-test-granular.txt").expect("Unable to read file");
    // let data = fs::read_to_string("02/input-test.txt").expect("Unable to read file");
    let data = fs::read_to_string("02/input.txt").expect("Unable to read file");

    let mut reports = Vec::new();
    let lines = data.split('\n').filter(|line| !line.is_empty());
    for line in lines {
        let report: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        reports.push(report);
    }

    let mut safe_count = 0;
    for report in &reports {
        if is_safe_report(report, None) {
            safe_count += 1;
            continue;
        }
        
        // Try removing each level one at a time
        for skip_idx in 0..report.len() {
            if is_safe_report(report, Some(skip_idx)) {
                safe_count += 1;
                break;
            }
        }
    }

    println!("Safe_Count = {}", safe_count);
}