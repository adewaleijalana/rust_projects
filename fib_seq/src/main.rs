fn main() {
    println!("Result: {} ", fib(40));
}

fn fib(n: u64) -> u64 {

    let result = if n == 0 || n == 1 {
        n
    }else {
        fib(n - 1) + fib(n - 2)
    };
    result
}
