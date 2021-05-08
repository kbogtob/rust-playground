#[allow(dead_code)]
fn fibo(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn fibo_memoized(n: u32, memoized_values: &mut Vec<u64>) -> u64 {
    match memoized_values.get(n as usize) {
        Some(memoized_value) => *memoized_value,
        None => {
            while memoized_values.len() - 1 < n as usize {
                let prev = memoized_values[memoized_values.len() - 2];
                let current = memoized_values[memoized_values.len() - 1];
                memoized_values.push(prev + current)
            }
            *memoized_values.last().unwrap()
        }
    }
}

fn fast_fibo(n: u32) -> u64 {
    // bootstrap with first two values
    fibo_memoized(n, &mut vec![0, 1])
}

fn main() {
    // fibo of 95 is larger than a u64
    for n in 0..94 {
        println!("fibo({}) -> {}", n, fast_fibo(n));
    }
}
