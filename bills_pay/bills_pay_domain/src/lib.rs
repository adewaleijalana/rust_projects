pub fn domain_test() {
    println!("Inside domain library")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        domain_test();
    }
}
