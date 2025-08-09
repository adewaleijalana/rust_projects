fn main() {
    sum_triangle_array(vec![1, 2, 3, 4, 5]);
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
