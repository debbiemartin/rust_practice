use std::collections::HashMap; 

fn main() {
    let mut list = Vec::from([1, 3, 3, 4, 5]);
    list.sort();
    let len = list.len();
    println!("Len: {len}");
    
    let median_index = len / 2;
    let median = list[median_index];
    println!("Median: {median}");
    
    let mut number_count = HashMap::new();
    for number in list {
        let count = number_count.entry(number).or_insert(0);
        *count += 1;
    }
    
    let number_and_count_max = number_count.iter().max_by_key(|entry | entry.1).unwrap();
    let max_count = number_and_count_max.0;
    println!("Mode: {max_count}")

}
