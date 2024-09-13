use bills_pay_repos;

pub fn service_test() {
    println!("Inside service library and calling repos codes..... ");
    bills_pay_repos::repos_test();
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
