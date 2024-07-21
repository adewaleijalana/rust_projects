// pub fn lifetime_test_error() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {r}");
// }

#[allow(dead_code)]
pub fn lifetime_test_2() {
    let x = 5;
    let r = &x;

    println!("r: {r}");
}
