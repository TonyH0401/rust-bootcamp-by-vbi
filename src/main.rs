fn main() {
    //
    let _x: i16 = 130;
    println!("x = {}", _x);
    let _y: f32 = 1000.312321;
    println!("y = {}", _y);
    let _y2 = 10.5f32;
    println!("y2 = {}", _y2);
    let _boohoo: bool = false;
    println!("boolean = {}", _boohoo);

    //
    let mut _x2: i8 = 100;
    println!("x2 = {}", _x2);
    _x2 = 80;
    println!("x2 = {}", _x2);

    //
    let _my_array: [i8; 3] = [10, 20, 30];
    println!("_my_array: {}, {}", _my_array[0], _my_array[2]);
    // this cause error, you should use "cargo check"
    // println!("_my_array: {}", _my_array[5]);
    let _my_array: [f32; 2] = [10.2, 20.2];
    println!("_my_array2: {}, {}", _my_array[0], _my_array[1]);
    // use this to print the whole array
    println!("test this: {:?}", _my_array);

    //
    let _tuple: (i8, i8) = (10, 12);
    println!("tuple = {}, {}", _tuple.0, _tuple.1);
    let _tuple: (i8, f32, &str) = (10, 10.5, "phú");
    println!("tuple = {}, {}, {}", _tuple.0, _tuple.1, _tuple.2);

    // this works, but once we assign value to it, it is immutable
    let _strange: i8;
    _strange = 10;
    println!("strange = {}", _strange);

    let mut _mx: i8 = 10;
    _mx = _mx + 8;
    println!("_mx = {}", _mx);

    const PI: f64 = 3.14;

    let mut _s: String = String::new();
    println!("string is empty : {}", _s.is_empty());
    _s.push('p');
    println!("string is empty : {}", _s.is_empty());

    let _s2: String = String::from("this is the government");
    println!("string value 2: {}", _s2);

    let mut _s3: String = String::from("this demo");
    println!("string value 3: {}", _s3);
    _s3 = String::from("this is not demo");
    println!("string value 3: {}", _s3);
    _s3 = _s3 + " bitch";
    println!("string value 3: {}", _s3);

    // reference str
    // cast
    // string cơ bản cx là con trỏ
    let _s4: &str = "this is reference string";
    println!("reference string value: {}", _s4);
    // _s3 = _s3 + "Dung" --> this does not work?
    _s3 = _s3 + _s4;
    println!("string value 3: {}", _s3);
    println!("display substring of string value 3: {}", &_s3[0..7]);
    println!("display substring of string value 4: {}", &_s4[0..10]);
    // because the substring has an address symbol before it, so it is &str
    let _s5: &str = &_s3[0..10];
    println!("confusion: {}", _s5);
    let _s5: &str = &_s4[0..11];
    println!("confusion: {}", _s5);
    // chúng ta 2 lần let _s5 -> rust tạo _s5 2 lần riêng biệt

    // conversion &str -> String
    let _conversion_string: String = "VBI".to_string();
    println!("len: {}", _conversion_string.len());
    let _s_format: String = format!("{}", "Hello VBI");
    println!("another way to convert $str -> String: {}", _s_format);
    // conversion String -> &str
    let _conversion_str: &String = &_conversion_string;
    let _conversion_str: &str = _conversion_string.as_str();
    //
    let _byte: &[u8] = _conversion_string.as_bytes();
    println!("byte result: {:?}", _byte);

    // if else
    let _x: bool = true;
    if _x {
        println!("true")
    } else {
        println!("false")
    }

    // match: switch case
    match _x {
        true => println!("Hello"),
        false => println!("Bye"),
    }
    let _number: i32 = 10;
    // hđ theo linear, từ trên dưới
    match _number {
        5 => println!("it is 5"),
        10 => println!("it is 10"),
        _ => println!("Invalid"),
    }
    let _tuple: (i32, i32) = (10, 10);
    // check the tuples
    match _tuple {
        // if correct
        (10, 10) | (20, 20) => {
            // check _number variable
            match _number {
                5 => println!("it is 5"),
                10 => println!("it is 10"),
                _ => println!("Invalid"),
            }
        }
        // _ => todo!() // this is a placeholder
        _ => println!("Invalid 2"),
    }

    // Vector
    let _vec = vec![1, 5, 6, 9, 2];
    // i could only call 1 out of 2 for
    // for _value in _vec {
    //     println!("Value = {}", _value)
    // }
    println!("Using iter() ---------------------");
    for _value in _vec.iter() {
        println!("Iter Value: {}", _value);
    }
    println!("Using index ---------------------");
    for _index in 0.._vec.len() {
        println!("Index: {} - Vector Value: {}", _index, _vec[_index]);
    }

    // find max value in vector
    let _max = _vec.iter().max().unwrap();
    println!("Max value: {}", _max);

    //Vector example
    let _my_arr = [10, 11, 15];
    let mut _vec_2 = Vec::new();
    println!("Vector 2: {:?}, Vector 2's len: {}", _vec_2, _vec_2.len());
    for _value in _my_arr {
        _vec_2.push(_value)
    }
    println!("Vector 2: {:?}, Vector 2's len: {}", _vec_2, _vec_2.len());

    // functions
    empty_para_no_return();
    with_para_no_return(String::from("This goes into a function"));
    let _x = with_para_return(String::from("This actually returns something"));
    println!("_x after function: {}", _x);

    //
    let mut _my_arr_2 = [15, 16, -1, 19];
    // let mut _my_vec: Vec<i32> = Vec::new();
    let _sum: i64 = _my_arr_2.iter().sum();
    println!("Sum array: {}", _sum);
    selection_sort_array_increase(&mut _my_arr_2);
    println!("Array before sorted: {:?}", _my_arr_2);
    println!("Selection sorted array: {:?}", _my_arr_2);

    // 
    let mut _string_demo_1 = String::from("Hello");
    let _string_demo_2 = String::from(",world");
    // this works
    // _string_demo_1 =_string_demo_1 + ",random";
    // this does not work, even if we let mut _string_demo_2 
    // _string_demo_1 = _string_demo_1 + _string_demo_2;
    _string_demo_1 = _string_demo_1 + &_string_demo_2;
    println!("String demo: {}", _string_demo_1);

    let _string_1 = String::from("Demon");
    let _string_2 = String::from("nic");
    let _string_3 = _string_1 + &_string_2;


    // 
    println!("\nHello world, Rust VBI!");
}

fn empty_para_no_return() {
    println!("This function has empty parameter and no return");
}

fn with_para_no_return(_s: String) {
    println!("This function has empty parameter and return: {}", _s);
}

fn with_para_return(_s: String) -> String {
    // this is the return and at the end of the function
    _s
}

// this DOES NOT work
// fn empty_para_return(_s: &str) -> String {
//     // this is the return and at the end of the function
//     _s
// }

fn selection_sort_array_increase(_arr: &mut [i64]) {
    let mut _position: usize;
    let mut _swap: i64;
    for _index in 0.._arr.len() - 1 {
        _position = _index;
        for _index2 in _index + 1.._arr.len() {
            if _arr[_position] > _arr[_index2] {
                _position = _index2;
            }
        }
        if _position != _index {
            _swap = _arr[_index];
            _arr[_index] = _arr[_position];
            _arr[_position] = _swap;
        }
    }
}
