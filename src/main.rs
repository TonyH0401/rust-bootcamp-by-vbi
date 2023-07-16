fn main() {
    // -------------------------------------------------
    // rust basic variables and basic println.
    let _x: i16 = 130;
    println!("Basic variables and println x = {}", _x);
    let _y: f32 = 1000.312321;
    println!("Basic variables and println y = {}", _y);
    let _y2 = 10.5f32;
    println!("Basic variables and println y2 = {}", _y2);
    let _boohoo: bool = false;
    println!("Basic variables and println boolean = {}", _boohoo);
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // every variables in rust is immutable by default,
    // if you want to use the variable like in other language,
    // you must use keyword 'mut' before said variable.
    let mut _x2: i8 = 100;
    println!("Mutable variables before x2 = {}", _x2);
    _x2 = 80;
    println!("Mutable variables after x2 = {}", _x2);
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // this is how you define and initialize an array, you must be specific.
    // array also is immutable btw.
    let _my_array: [i8; 3] = [10, 20, 30];
    // this is how you access data in the array, pretty similar to other languages.
    println!("Access data in array: {}, {}", _my_array[0], _my_array[2]);
    // access data out-of-bound of an array will result in error,
    // this is why you should use "cargo check" before "cargo run".
    // println!("Access data out-of-bound of array: {}", _my_array[5]);
    // we said that array is immutable, so why can we still use it like this,
    // notice the "let" keyword, we are not re-using it, we are re-initialize it.
    let _my_array: [f32; 2] = [10.2, 20.2];
    println!(
        "Access data in array (re-initialize): {}, {}",
        _my_array[0], _my_array[1]
    );
    // usually you can use a loop to print out all of the data in the array,
    // but you can use this to print the whole array,
    // the "{:?}" will iterate through data from the begining to the end.
    println!("Print out the whole array: {:?}", _my_array);
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // tuple has the same principals as array, still not mutable.
    let _tuple: (i8, i8) = (10, 12);
    // access data in a tuple is more easy though.
    println!("Access data in tuple = {}, {}", _tuple.0, _tuple.1);
    // tuple is different from array as in,
    // with array you can only have 1 datatype within an array,
    // with tuple you can have many datatype within a tuple.
    let _tuple: (i8, f32, &str) = (10, 10.5, "phú");
    println!(
        "Access data in tuple, multiple datatype = {}, {}, {}",
        _tuple.0, _tuple.1, _tuple.2
    );
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // you can initialize a variable without assigning it a value,
    // this will work but once we assign a value to the variable,
    // that variable is immutable.
    let _strange: i8;
    _strange = 10;
    println!("Assigned a value to the variable = {}", _strange);
    // the above example is different from the below, the below is mutable,
    // you assign a value at the initalize phase,
    // you later can change that value into something different.
    let mut _mx: i8 = 10;
    _mx = _mx + 8;
    println!("Mutable variable _mx = {}", _mx);
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // although normal variables in rust are immutable,
    // so they are kinda basically a const, but there are const in rust,
    // they are defined as followed and have UPPERCASE name.
    const PI: f64 = 3.14;
    println!("The value of constant PI: {}", PI);
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // we will learn about String first, String is immutable, as always.
    // this create an empty String.
    let mut _s: String = String::new();
    println!("Check if String is empty: {}", _s.is_empty());
    // we can push only character into the String,
    // we can do this because we declare String as mutable,
    // notice the "mut" keyword.
    _s.push('p');
    println!(
        "Check if String is empty: {} - Its value: {}.",
        _s.is_empty(),
        _s
    );
    // but it also supports pushing string, you just gonna be specific.
    _s.push_str("peepeepoopoo");
    println!(
        "Check if String is empty: {} - Its value: {}.",
        _s.is_empty(),
        _s
    );
    // initialize the String with value.
    // this string is immutable.
    let _s2: String = String::from("this is the government");
    println!(
        "This String has value: {} - The length is: {}",
        _s2,
        _s2.len()
    );
    // checkout this mutable String, we gonna change its value.
    let mut _s3: String = String::from("this is a demo");
    println!("Mutable String value 1: {}", _s3);
    _s3 = String::from("this is not a demo");
    println!("Mutable String value 2: {}", _s3);
    // this code below is funny and will be explained later,
    // in short, str = str + &str (this is acceptable),
    // str = str + str (this is not acceptable),
    // ", bitch" - this in itself is a &str.
    _s3 = _s3 + ", bitch";
    println!("Mutable String value 3 with hint of &str: {}", _s3);
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // reference str or &str or &String.
    // cast, i dont know what this is, too little information.
    // String is like a Warpper Class and &str is like a pointer string in C.
    // you can create an empty &str like this,
    // they are kinda like declaring normal variables.
    let _s = "";
    println!("Check if &str is empty: {}", _s.is_empty());
    let _s4: &str = "this is a normal reference string!";
    println!("Reference string (&str) value: {}", _s4);
    // as we have explained before, str = str + &str is acceptable,
    // the code below works because _s4 is a &str not a String.
    _s3 = _s3 + _s4;
    println!("String concat value using _s4 (&str): {}", _s3);
    // because the substring has "&" symbol before it, so it is a &str.
    println!("Display substring of _s3: {}", &_s3[0..7]);
    println!("Display substring of _s4: {}", &_s4[0..10]);
    // you can create a new string from the substring.
    // chúng ta 2 lần let _s5 -> rust tạo _s5 2 lần riêng biệt (another way of understanding it).
    let _s5: &str = &_s3[0..10];
    println!("New string of substring _s3: {}", _s5);
    let _s5: &str = &_s4[0..11];
    println!("New string of substring _s4: {}", _s5);
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // gonna let this guy be a "mut".
    let mut _string_1 = String::new();
    // then we push a String in.
    _string_1.push_str("hello");
    println!("Version 1 with just the mut String: {}!", _string_1);
    // now let declare some &str.
    let _str_1 = ", welcome";
    let _str_2 = ", fazbear pizza";
    // checking the first concat &str, this gonna work fine.
    _string_1 = _string_1 + _str_1;
    println!("Version 2 after concat: {}!", _string_1);
    // concat with mutiple &str.
    _string_1 = _string_1 + _str_1 + _str_2;
    println!("Version 3 after concat with 2 &str: {}!", _string_1);
    // concat with mutiple &str + a wild &str.
    _string_1 = _string_1 + _str_1 + _str_2 + ", enjoy your stay";
    println!(
        "Version 4 after concat with 2 &str and a wild &str: {}!",
        _string_1
    );
    // str = str + &str + &str is acceptable.
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // conversion &str -> String.
    // "VBI" is a &str.
    let _conversion_string: String = "VBI".to_string();
    println!(
        "Length after &str -> String conversion: {}",
        _conversion_string.len()
    );
    // another way to convert is to use the format fn.
    let _s_format: String = format!("{}", "Hello VBI");
    println!("Another way to convert $str -> String: {}", _s_format);
    let _byte: &[u8] = _conversion_string.as_bytes();
    println!("String's characters as byte value: {:?}", _byte);
    // conversion String -> &str.
    // too easy, too easy.
    // &str is the same as &String.
    let _conversion_str: &String = &_conversion_string;
    let _conversion_str: &str = _conversion_string.as_str();
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // if else, the normal way.
    // you can add "else if" in as well.
    let _x: bool = true;
    if _x {
        println!("if-else: true");
    } else {
        println!("if-else: false");
    }
    // match: switch case.
    match _x {
        true => println!("Using match fn: true"),
        false => println!("Using match fn: false"),
    }
    // another example.
    // match fn works in linear, from top to bottom.
    let _number: i32 = 10;
    match _number {
        5 => println!("It is 5"),
        10 => println!("It is 10"),
        _ => println!("Invalid"),
    }
    // using match + tuple.
    let _tuple: (i32, i32) = (10, 10);
    // check the tuple.
    match _tuple {
        // if the tuple is correct.
        (10, 10) | (20, 20) => {
            // check _number variable.
            match _number {
                5 => println!("It is 5 after the tuple"),
                10 => println!("It is 10 after the tuple"),
                _ => println!("Invalid after the tuple"),
            }
        }
        // if the tuple is not correct.
        // we ca use "_ => todo!()" as a placeholder.
        _ => println!("Invalid 2"),
    }
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // Vector.
    // vector is the same as array, still immutable,
    // it also access data like an array as well.
    let _vec = vec![1, 5, 6, 9, 2];
    // i could only use 1 out of 2 "for" given.
    // probably involve with some accessing data,
    // where 1 data can only be accessed once, at a time.
    // println!(">>>>>Using normal for<<<<<");
    // for _value in _vec {
    //     println!("Value = {}", _value)
    // }
    println!(">>>>>Using iter()<<<<<");
    for _value in _vec.iter() {
        println!("Vector values using iter: {}", _value);
    }
    // there is nothing too special about this method,
    // you just iter from 0 to before a number and then access the data,
    // given 0..10 it is actually 0 to 9.
    println!(">>>>>Using the index<<<<<");
    for _index in 0.._vec.len() {
        println!("Index: {} - Vector Value: {}", _index, _vec[_index]);
    }
    // given 0..=10 it is actually 0 to 10, for real.
    println!(">>>>>Using the index 2<<<<<");
    for _i in 0..=3 {
        println!("Index: {} - Vector value: {}", _i, _vec[_i]);
    }
    // iter supplies us with so many built in methods,
    // this is one example, we can find max value in vector using iter.
    let _max_value = _vec.iter().max().unwrap();
    println!("Max value in a vector using iter: {}", _max_value);
    // using iter, we can find the sum of an array.
    let _my_arr = [10, 15, 12, -100, -3];
    let _sum: i32 = _my_arr.iter().sum();
    println!("Sum array using built-in iter method: {}", _sum);
    // try to turn an array into a vector.
    // gonna create a "mut" vector because we gonna push data in.
    let _my_arr = [-12, 10, 11, 15, -1];
    let mut _vec_2 = Vec::new();
    println!(
        "Vector len before: {} - Vector value before: {:?}",
        _vec_2.len(),
        _vec_2
    );
    // this for loop will push data in the vector,
    for _value in _my_arr {
        _vec_2.push(_value)
    }
    println!(
        "Vector len after: {} - Vector value after: {:?}",
        _vec_2.len(),
        _vec_2
    );
    // we will try to sort out a vector using the sort method,
    // remember the vector also need to be "mut".
    _vec_2.sort();
    println!(
        "Vector len after sort: {} - Vector value after sort: {:?}",
        _vec_2.len(),
        _vec_2
    );
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // functions.
    // here are the basics, you can ctrl+click to view the functions.
    empty_para_no_return();
    with_para_no_return(String::from("This goes into a function"));
    let _x = with_para_return(String::from("This actually returns something"));
    println!("_x after function: {}", _x);
    // we gonna try to create a sorting method for the array,
    // the sorting is selection sort, kinda basic.
    let mut _my_arr_2 = [15, 16, -1, 19];
    println!("Array before sorted: {:?}", _my_arr_2);
    // calling the function itself,
    // we are expecting this function to change the array itself,
    // so we add a "mut" to the array.
    // when given the array in the function as a parameter,
    // you add a "&", this is an address/pointer, it is just like C.
    // you might not see but at the function,
    // you can also see a "&" as an input parameter,
    // this signaling that it is an address, just like in C.
    selection_sort_array_increase(&mut _my_arr_2);
    println!("Array after selection sort: {:?}", _my_arr_2);
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // we will concat a string into itself, for this we will use "mut".
    let mut _string_demo_1 = String::from("Hello");
    let _string_demo_2 = String::from(", world");
    _string_demo_1 = _string_demo_1 + &_string_demo_2;
    println!("Concat a string into itself: {}", _string_demo_1);
    // as we have discussed before, this works,
    // _string_demo_1 =_string_demo_1 + ",random";
    // while this doesnt, even if we let mut _string_demo_2,
    // _string_demo_1 = _string_demo_1 + _string_demo_2;
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // we will concat a string but not into itself,
    // but export it to a different variable,
    // for this, you need to turn 1 String into &str.
    let _string_1 = String::from("Demon");
    let _string_2 = String::from("nic");
    let _string_3 = _string_1 + &_string_2;
    println!("Concat a string to a different variable: {}", _string_3);
    println!("-------------------------------------------------");

    // -------------------------------------------------
    // closure, it is just like lamda in C#, it's a variable function.
    // the || is the parameter input.
    // this takes in no parameter and return nothing.
    let x = || {
        println!("Hello empty parameter and not return closure!");
    };
    // this is how you call it.
    x();
    // this takes in a parameter but return nothing.
    let x2 = |para| {
        println!(
            "Hello parameter and not return closure, the para: {}!",
            para
        );
    };
    x2(100);
    // this takes in a parameter and return something (this case a String).
    // as we have discussed before "CosmosLee" is a &str not a String.
    let x3 = |para: &str| -> String {
        return para.to_string();
    };
    let _result = x3("CosmosLee");
    println!("Closure that have a para and return something: {}", _result);
    println!("-------------------------------------------------");
    
    // -------------------------------------------------
    // for, iter, into_iter, iter_mut.
    // using a normal for loop.
    let _vec = vec![10, 11, 12, 15, 16];
    for value in _vec {
        println!("Value 1, normal for: {}", value);
    }
    // you can use iter as a for loop, but what's the different,
    // iter supplies you with some built in function that you can use,
    let _vec2 = vec![100, -11, 1];
    // for example, this one let you get the index and the value.
    _vec2.iter().enumerate().for_each(|x| {
        println!(
            "Using iter short verion 1 - Index: {} - Value: {}",
            x.0, x.1
        )
    });
    // if you dont like the |x|, the for_each function also support tuple,
    // the for_each function takes in the |x| as tuple,
    // this means |x| is a tuple itself, a singular tuple.
    _vec2
        .iter()
        .enumerate()
        .for_each(|(x, y)| println!("Using iter short verion 2 - Index: {} - Value: {}", x, y));
    // this is reason we learnt closure first,
    // we will code the above but using closure, the longer version.
    _vec2.iter().enumerate().for_each(|x| {
        println!(
            "Using iter long version 1 - Index: {} - Value: {}",
            x.0, x.1
        );
    });
    _vec2.iter().enumerate().for_each(|(x, y)| {
        println!("Using iter long version 2 - Index: {} - Value: {}", x, y);
    });
    let _vec3 = vec![-100, -10, 100, 10];
    // iter().map() is like the map function in JS.
    // without the use of .collect(), it will return a generics datatype.
    // with the use of .collect(), you can return the datatype that you prefer,
    // we choose vector as our return datatype.
    let _res: Vec<i32> = _vec3.iter().map(|x| {
        x * 3
    }).collect();
    println!("Using iter map + collect: {:?}", _res);
    // the below code has the same meaning as the above code,
    // but this below code is messier.
    let mut _vec4 = vec![];
    for value in _vec3 {
        _vec4.push(value*5);
    }
    println!("Using a normal for loop: {:?}", _vec4);
    // into_iter is explained in the function itself, click to see.
    // assert_eq is used to check on stuff, if it's correct, nothing happened.
    assert_eq!(count_char_occurrences("Mississippi", 'i'), 4);
    // if it is not correct, rust compiler will give out an error,
    // like the code below.
    assert_eq!(count_char_occurrences("Rust is fun", 'u'), 1);
    println!("-------------------------------------------------");
}

fn count_char_occurrences(_string: &str, ch: char) -> usize {
    // .chars() takes in the string and convert it into iterable chars,
    // basically, it iterates through a string.
    // .into_iter() takes that thing from .chars() and yield either,
    // datatype T, &T or &mut T (according to stackoverflow).
    // .filter() use a closure to filter data, 
    // remember the yield from into_inter, 
    // you can see the yield's result in the closure parameter, 
    // the into_iter yield &T cause closure parameter is &char,
    // and then you just need to compare c: &char and &ch (we add & to the ch).
    // .count() basically counts the amount of time that character repeated.
    let _res = _string.chars().into_iter().filter(|c: &char| {
        c == &ch
    }).count();
    _res
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
