use std::{fs, thread};

fn merge_sort(left: &mut [i32], right: &mut [i32]) {
    let left_size: usize = left.len();
    let right_size: usize = right.len();
    if left_size == 0 {
        return;
    }
    if left_size == 1 && right_size == 1 {
        if left[0] > right[0] {
            let t: i32 = left[0];
            left[0] = right[0];
            right[0] = t;
        }
        return;
    }
    thread::scope(|s| {
        let (left_split_a, left_split_b) = left.split_at_mut(left_size / 2);
        let (right_split_a, right_split_b) = right.split_at_mut(right_size / 2);
        s.spawn(move || {
            merge_sort(left_split_a, left_split_b);
        });
        s.spawn(move || {
            merge_sort(right_split_a, right_split_b);
        });
    });
    let merge_size = left_size + right_size;
    let mut aux: Vec<i32> = vec![0; merge_size];
    let mut i = 0;
    let mut left_i = 0;
    let mut right_i = 0;
    while i < merge_size {
        if left_i >= left_size {
            aux[i..].copy_from_slice(&right[right_i..]);
            break;
        } else if right_i >= right_size {
            aux[i..].copy_from_slice(&left[left_i..]);
            break;
        }
        if left[left_i] <= right[right_i] {
            aux[i] = left[left_i];
            left_i += 1;
        } else {
            aux[i] = right[right_i];
            right_i += 1;
        }
        i += 1;
    }
    left.copy_from_slice(&aux[..left_size]);
    right.copy_from_slice(&aux[left_size..]);
}

fn main() {
    let mut number_string: String = String::from("");
    match fs::read_to_string("numbers.txt") {
        Ok(result) => {
            number_string = result;
        }
        Err(error) => {
            println!("{}", error);
        }
    }
    let mut numbers: Vec<i32> = number_string
        .split(' ') // split by comma
        .map(|x| x.trim()) // remove spaces
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let size: usize = numbers.len();
    let (left, right) = numbers.split_at_mut(size / 2);
    merge_sort(left, right);
    println!("{:?}", numbers);
}
