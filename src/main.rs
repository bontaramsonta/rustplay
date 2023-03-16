// cargo watch -w src/main.rs -w in -cqs 'cargo -q run < in > out'
use rust_play::*;

fn main() {
    let test_cases = get_input::<usize>().unwrap();
    for i in 0..test_cases {
        solve(i);
    }
}

fn solve(_a: usize) {
    println!("CASE: {_a}");
    let n = get_input::<i32>().unwrap();
    let m = get_input::<i32>().unwrap();
    let mut v1 = get_space_separated::<i32>();
    let mut v2 = get_space_separated::<i32>();
    let result = merge(&mut v1, m, &mut v2, n);
    println!("---------- {result:#?}");
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut result: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < m && j < n {
        let val1 = nums1[i as usize];
        let val2 = nums2[j as usize];
        result.push(match val1.cmp(&val2) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {
                i += 1;
                val1
            }
            std::cmp::Ordering::Greater => {
                j += 1;
                val2
            }
        });
    }
    if i < m {
        result.extend_from_slice(&nums1[(i as usize)..]);
    }
    if j < n {
        result.extend_from_slice(&nums2[(j as usize)..]);
    }
    *nums1 = result;
    nums1.iter().for_each(|r| println!("{r}"))
}
