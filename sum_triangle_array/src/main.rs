fn main() {
    // sum_triangle_array(vec![1, 2, 3, 4, 5]);
    // test_1();
    let text = "Almond Coffees";
    println!("{{{} == {}}}", &text[8..], &text[8..=13]);

    println!("{{&text[8..] == &text[8..=13]}}");

}   

fn sum_triangle_array(arr: Vec<i32>){

    if arr.is_empty() || arr.len() == 1 {
        println!("{:?}", arr);
        return;
    }
    let mut temp_arr = vec![];
    for i in 0..arr.len() - 1 {
        temp_arr.push(arr[i] + arr[i + 1]);
    }
    sum_triangle_array(temp_arr);
    println!("{:?}", arr);
}

fn test(mut num: i32) {
    num += 1;
}

fn test_1() {
    let mut st: &mut i32;

    let mut y = 5;
    let mut x = 15;
    st = &mut y;
    st = &mut x;

    println!("{}", st);
}
