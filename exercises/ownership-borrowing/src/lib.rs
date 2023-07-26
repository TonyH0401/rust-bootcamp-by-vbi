// Exercise 1
// Make it compiles
fn exercise1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    // fix 1:
    let y = x;
    let z = y;
    // fix 2:
    // let y = &x;
    // let z = x;
    // in the video he talked about dereference a pointer,
    // this works best in C/C++, where you can perform this method,
    // to get the value within the pointer, but as previous explanation,
    // rust kinda doesn't need this because rust is pretty smart.
}

// Exercise 2
// Make it compile. Don't modify code in exercise2 function!
// fix 1: is to add a return type and then returns the variable.
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);
    println!("{}", s2);
}
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
// fix 2: is to comment out the println line, this fix is acceptable, 
// but the code said not change the main function, so idk :).
// what if we want to add more value to s1/changing s1's value.
// fix 3:
fn exercise2_1() {
    // first, the original needs to be mutable.
    let mut s1 = String::from("hello, world");
    // then you pass in said variable as a mutable reference.
    let s2 = take_ownership_1(&mut s1);
    println!("{}", s2);
}
// the parameter of this function must also be &mut,
// to be the same as when you call the function.
fn take_ownership_1(s: &mut String) -> String {
    println!("{}", s);
    s.push_str("string");
    // to_owned is similar to to_string or clone (explained in Notion),
    // it basically creates the owned version instead of a referenced one.
    s.to_owned()
}
// but can we change the return type from String to &mut String?
// yes we can.
// fix 4:
fn exercise2_2() {
    // first, the original needs to be mutable.
    let mut s1 = String::from("hello, world");
    // then you pass in said variable as a mutable reference.
    let s2 = take_ownership_2(&mut s1);
    println!("{}", s2);
}
// the parameter of this function must also be &mut,
// to be the same as when you call the function.
// this time it's a little bit different, it returns &mut String.
// &mut String is a mutable reference String, it's a String,
// but it's also mutably referenced.
fn take_ownership_2(s: &mut String) -> &mut String {
    println!("{}", s);
    s.push_str("string");
    // when you return, you can't use to_own, to_string or clone,
    // because those functions create String not &mut String.
    s
}

// Exercise 3
// Make it compile
// Dont care about logic
// this exercise is a question from stackoverflow
fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];
    let values_number = values.len();
    let additions: Vec<usize> = vec![0];
    println!("{:?}", values_number);
    // based on the explanation in the video,
    // error happened because someone took the ownership of additions vector,
    // during the first loop, it's ok but then someone took the ownership,
    // then additions is dead on the second loop.
    while additions.len() > 0 {
        let mut addition: f64 = 0.0;
        // you should check out error code E0382, 
        // it has some good explanations.
        // continuing from the explanation above,
        // array doesn't have this kind of error,
        // vector and some other datatypes have this,
        // from what I researched, this is due to the fact that,
        // vector is "not marked for copy",
        // basically, everytime we accessing a vector, we are accessing "it",
        // we are not accessing a cloned version of it, we are accessing "it".
        // when using loops, array doesn't have this problem because, 
        // reason 1, we are moving through the array, reading only the value,
        // or you might say we're getting the immutable value of the array,
        // reason 2, we are accessing a cloned version of the array,
        // not the array itself.
        // when using loops, vector has this problem because,
        // vector is "not marked for copy", when we access the vector,
        // we are accessing "it", it doesn't create a clone for adjustment,
        // hench, we are not moving through the vector when we use loops, 
        // we are iterating/becoming the values within the vector,
        // effectively owning the data within the vector, 
        // this left the original vector dead.
        // to fix this, we need to make the vector become more like an array, 
        // we have to make the vector "marked for copy",
        // by using .clone() to create a copy of the additions vector,
        // seperating them from each other, preserving the original,
        // while making adjustment to the new one.
        for element_index in additions.clone() {
            let addition_aux = values[element_index];
            addition = addition_aux + addition;
        }
        // additionally, we can use .iter() to solve this problem, 
        // using iter yield the same result but it's a little bit different,
        // based on the explantion about the iter, 
        // you can find the explanations about iter, into_iter,
        // in the related posts of class-1 and my-class-2,
        // the method iter borrows the vector it works on, 
        // and provides shared borrows of the elements,
        // that is why iter yield &T, which is a borrow,
        // and to access the value you need to derefernce it using "*".
        for element_index in additions.iter() {
            let addition_aux = values[*element_index];
            addition = addition_aux + addition;
        }
        // another way is to borrow the vector itself,
        // by making a (immutable) reference to the vector,
        // via adding this "&" to the vector itself, creating a reference,
        // and to access the value, you need to dereference it using "*".
        for element_index in &additions {
            let addition_aux = values[*element_index];
            addition = addition_aux + addition;
        }
        // as you can see, we have commented out all three loops,
        // and nothing has happened, 
        // because 1 is cloned, it doesn't bother the original,
        // because 1 is referencing borrowed using iter,
        // because 1 is referencing borrowed using "&".
    }
}

// Exercise 4
// Make it compile
// remember the "dangling" = this is an easy fix, error code E0515.
// fix 1: change the return type to String, avoid returning reference type.
// comment out str_ref and return str_value.
// fn exercise4(value: u32) -> String {
//     let str_value = value.to_string(); // Convert u32 to String
//     return str_value; // Return the reference to the String
// }
// fix 2: make a fixed return.
// "&'static str" this makes sure "the reference return" lives longer,
// it makes sure "the reference return" doesn't die when the function is over,
// that it gonna live till the end of the program.
// str_ref not gonna work because it still references a variable,
// that will be out-of-scope once the function is over, checkout E0515.
// fn exercise4(value: u32) -> &'static str {
//     let str_value = value.to_string(); // Convert u32 to String
//     // this code below means "&'static str" which has the same meaning as,
//     // String, which means it gonna live till the end of the program.
//     // but also make sure the program returns "&'static str" as well. 
//     let res = "hello";
//     return res;
// }
// fix 3: the same as fix 2 but we use the value argument now.
fn exercise4(value: u32) -> &'static str {
    let str_value = value.to_string(); // Convert u32 to String
    match value {
        // these code below mean "&'static str",
        // which has the same meaning as String,
        // which mean they gonna live till the end of the program,
        // as long as the function return "&'static str".
        // "&'static str" is different from "&str",
        // "&str" is a reference to a String,
        // "&'static str" is more like String but not quite.
        0 => "Zero",
        1 => "One",
        _ => "Unknown"
    }
}

// Exercise 5
// Make it compile
// Please check Notion for additional notes and error explanations.
// fix 1: is to make a copy of the value variable, separating them both,
// by adding .clone() to the value variable in the .insert().
// then we need to move the value declaration's scope,
// to be equal to my_map.get(&key)'s scope.
// use std::collections::HashMap;
// fn exercise5() {
//     let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);
//     let key = 3;
//     let value = "3.0".to_string();
//     let res = match my_map.get(&key) {
//         Some(child) => child,
//         None => {
//             my_map.insert(key, value.clone());
//             &value
//         }
//     };
//     println!("{}", res);
// }
// fix 2: is to make a copy of the value variable, separating them both,
// by adding .clone() to the value variable in the .insert().
// then use "my_map.get(&key).unwrap()" to return the right value.
use std::collections::HashMap;
fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);
    let key = 3;
    let res = match my_map.get(&key) {
        Some(child) => child,
        None => {
            let value = "3.0".to_string();
            my_map.insert(key, value.clone());
            my_map.get(&key).unwrap()
        }
    };
    println!("{}", res);
}

// Exercise 6
// Make it compile
// Please check Notion for additional notes and error explanations.
// TL;DR: prev_key becomes "dangling", because prev_key's scope is bigger,
// s's scope is smaller (it's the loop), s got droppped when the loop ends,
// while prev_key is still pointing to s = creating "dangling". 
// fix 1: based on the error explanation, we make a copy of data[0],
// because it's the closest to prev_key before deletion,
// change prev_key from &str to String (to fix mis-match).
// use std::io;
// fn exercise6() {
//     let mut prev_key: String = String::new();
//     for line in io::stdin().lines() {
//         let s = line.unwrap();
//         let data: Vec<&str> = s.split("\t").collect();
//         if prev_key.len() == 0 {
//             prev_key = data[0].to_string();
//         }
//     }
// }
// fix 2: is to make prev_key's scope smaller, make it the same as data[0],
// this will not create "dangling".
use std::io;
fn exercise6() {
    for line in io::stdin().lines() {
        let mut prev_key: &str = "";
        let s = line.unwrap();
        let data: Vec<&str> = s.split("\t").collect();
        if prev_key.len() == 0 {
            prev_key = data[0];
        }
    }
}

// Exercise 7
// Make it compile
// Please check Notion for additional notes and error explanations.
// same error code as Exercise 6 => "dangling".
// fix 1: remove the println.
// fn exercise7() {
//     let mut v: Vec<&str> = Vec::new();
//     {
//         let chars = [b'x', b'y', b'z'];
//         let s: &str = std::str::from_utf8(&chars).unwrap();
//         v.push(&s);
//     }
// }
// fix 2: remove the scope
// fn exercise7() {
//     let mut v: Vec<&str> = Vec::new();
//     let chars = [b'x', b'y', b'z'];
//     let s: &str = std::str::from_utf8(&chars).unwrap();
//     v.push(&s);
//     println!("{:?}", v);
// }
// fix 3: we move the println inside the scope.
fn exercise7() {
    let mut v: Vec<&str> = Vec::new();
    {
        let chars = [b'x', b'y', b'z'];
        let s: &str = std::str::from_utf8(&chars).unwrap();
        v.push(&s);
        println!("{:?}", v);
    }
}

// Exercise 8
// Make it compile
// Please check Notion for additional notes and error explanations.
// same error code as Exercise 6 and Exercise 7 => "dangling".
// fix 1: move accounting into the scope so it's not dangling anymore.
// fn exercise8() {
//     loop {
//         let mut accounting = vec!["Alice", "Ben"];
//         let mut add_input = String::from("");
//         io::stdin()
//             .read_line(&mut add_input)
//             .expect("Failed to read line");

//         let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();
//         if add_vec.len() < 1 {
//             println!("Incorrect input, try again");
//             continue;
//         }
//         let person = add_vec[0];
//         accounting.push(person);
//     }
// }
// fix 2: is to make an independence copy of person via to_string.
// fn exercise8() {
//     let mut accounting: Vec<String> = vec!["Alice".to_string(), "Ben".to_string()];
//     loop {
//         let mut add_input = String::from("");
//         io::stdin()
//             .read_line(&mut add_input)
//             .expect("Failed to read line");

//         let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

//         if add_vec.len() < 1 {
//             println!("Incorrect input, try again");
//             continue;
//         }
//         let person = add_vec[0].to_string();
//         accounting.push(person);
//     }
// }
// fix 3: is to remove the scope, this mirrors fix 1.
fn exercise8() {
    let mut accounting: Vec<&str> = vec!["Alice", "Ben"];
    let mut add_input = String::from("");
    io::stdin()
        .read_line(&mut add_input)
        .expect("Failed to read line");

    let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

    if add_vec.len() < 1 {
        println!("Incorrect input, try again");
    }
    let person = add_vec[0];
    accounting.push(person);
}

