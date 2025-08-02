use std::thread;

fn merge(left: &mut [i32], right: &mut [i32], aux: &mut [i32]) {
    let left_size = left.len();
    let right_size = right.len();

    let mut i = 0;
    let mut left_i = 0;
    let mut right_i = 0;

    while left_i < left_size && right_i < right_size {
        if left[left_i] < right[right_i] {
            aux[i] = left[left_i];
            left_i = left_i + 1;
        } else {
            aux[i] = right[right_i];
            right_i = right_i + 1;
        }
        i = i + 1;
    }
    if left_i == left_size {
        aux[i..].copy_from_slice(&right[right_i..]);
    } else if right_i == right_size {
        aux[i..].copy_from_slice(&left[left_i..]);
    }
    left.copy_from_slice(&aux[..left_size]);
    right.copy_from_slice(&aux[left_size..]);
}

fn merge_sort(source: &mut [i32], aux: &mut [i32]) {
    let size = source.len();
    let (left, right) = source.split_at_mut(size / 2);
    let left_size = left.len();
    let right_size = right.len();
    if left_size == 0 {
        return;
    }
    if left_size == 1 && right_size == 1 {
        if right[0] < left[0] {
            let t = left[0];
            left[0] = right[0];
            right[0] = t;
        }
        return;
    }
    let (left_aux, right_aux) = aux.split_at_mut(size / 2);
    thread::scope(|s| {
        let left_thread = s.spawn(|| {
            merge_sort(left, left_aux);
        });
        let right_thread = s.spawn(|| {
            merge_sort(right, right_aux);
        });
        match left_thread.join() {
            Ok(_result) => {}
            Err(_error) => {
                std::process::exit(-1);
            }
        }
        match right_thread.join() {
            Ok(_result) => {}
            Err(_error) => {
                std::process::exit(-1);
            }
        }
    });
    merge(left, right, aux);
    return;
}

fn main() {
    let mut data: Vec<i32> = Vec::from([
        482, 17, 936, 215, 678, 54, 902, 731, 482, 145, 867, 302, 591, 764, 413, 927, 82, 615, 247,
        990, 521, 310, 745, 89, 456, 673, 389, 214, 587, 902, 321, 478, 56, 832, 91, 764, 255, 607,
        348, 999, 432, 178, 645, 713, 382, 951, 268, 590, 724, 120, 843, 413, 299, 675, 888, 142,
        506, 713, 937, 451, 612, 384, 297, 759, 481, 904, 150, 678, 213, 815, 924, 357, 601, 472,
        698, 235, 890, 123, 765, 480, 339, 821, 569, 940, 254, 693, 417, 508, 370, 915, 804, 298,
        531, 662, 741, 99, 874, 210, 588, 477,
    ]);
    let mut aux: Vec<i32> = vec![0; data.len()];
    println!("Starting Sort.");
    merge_sort(&mut data, &mut aux);
    println!("Done.");
    println!("{:?}", data);
}
