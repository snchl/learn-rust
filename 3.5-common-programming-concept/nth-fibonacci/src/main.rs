fn main() {
    fibonacci(11);
}

fn fibonacci(count: i64) {
    let mut n_minus_one = 0;
    let mut n_minus_two = 1;

    for i in 0..count {
        let n = n_minus_one + n_minus_two;
        println!("{}", n);

        n_minus_two = n_minus_one;
        n_minus_one = n;
    }
}