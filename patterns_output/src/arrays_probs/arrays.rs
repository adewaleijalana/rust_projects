pub fn arrays_test() {
    let arr = [0; 5];

    for (i, v) in arr.iter().enumerate() {
        println!("{i} and {v}")
    }
}

pub fn sort_array() {
    const START: usize = 0;
    let mut arr = [1, 87, 67, 4, 9, 43];

    // let mut multi_array: [[i32; 6]; 4] = [[0; 6]; 4];
    // multi_array[0] = [1, 2, 3, 4, 5, 6];

    for i in 1..arr.len() {
        let mut j = i;
        while (j > START && arr[j - 1] > arr[j]) {
            let mut temp = arr[j - 1];
            arr[j - 1] = arr[j];
            arr[j] = temp;
            j = j - 1;
        }
    }

    for (i, v) in arr.iter().enumerate() {
        println!("value is: {v}")
    }
}

pub fn divsion(num1: i32, num2: i32) -> Result<i32, &'static str> {
    if num2 == 0 {
        return Err("You can not divide by zero");
    }

    Ok(num1 / num2)
    
}
