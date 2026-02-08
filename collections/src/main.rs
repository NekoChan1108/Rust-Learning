//!
//! the answer of chapter 8
//!

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

///
/// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
/// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
/// # Examples
/// ```
/// let list = vec![1, 2, 3, 4, 5, 5, 5];
/// println!("{:?}", median_and_mode(&list));
/// ```
///
fn median_and_mode(list: &Vec<i32>) -> (Option<i32>, Option<i32>) {
    if list.is_empty() {
        (None, None)
    } else {
        let mut median: Option<i32> = None;
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        if list.len() % 2 == 1 {
            median = Some(list[list.len() / 2]);
        } else {
            median = Some(list[list.len() / 2 - 1] + list[list.len() / 2] / 2);
        }
        for i in list {
            let count = count_map.entry(*i).or_insert(0);
            *count += 1;
        }
        let mode = count_map
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k);

        (median, mode)
    }
}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn main() {
    println!("{:?}", median_and_mode(&vec![1, 2, 3, 4, 5, 5, 5]));
}
