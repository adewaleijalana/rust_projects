use bills_pay_domain;

pub fn repos_test() {
    println!("Inside repos library and calling domain codes..... ");
    bills_pay_domain::domain_test();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
