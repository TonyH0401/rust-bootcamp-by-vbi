fn main() {
    // -------------------------------------------------
    // this is String.
    // this is mutable.
    let _s1 = String::from("Demon");
    println!("This is String, mutable: {}", _s1);
    // this is reference string, another way to convert String -> &str.
    // this is immutable.
    let _s2 = &_s1[..];
    println!("This is &str, immutable: {}", _s2);
    // the above example is the simple example of owning and borrowing,
    // where _s1 is owning its own data and _s2 is borrowing data from _s1,
    // remember, I said borrowing.
    // the code below is _s1 moved to _s3, where _s1 is no longer valid,
    // _s3 is not borrowing, it's fully moved _s1 to _s3.
    let _s3 = _s1;
    // if you try to print _s1 after the move, you will receive an error.
    // println!("Moved _s1 to _s3, try to print _s1: {}", _s1);
    // data in _s1 is now in a new house called _s3, which you can print out.
    println!("Moved _s1 to _s3, try to print _s3 {}", _s3);
    // because _s1 is no longer exist, if you try to move the data to _s4,
    // you will receive an error.
    // let _s4 = _s1;
    // because _s3 is the new home for the data in _s1,
    // you can move data from _s3 to _s4.
    let _s4 = _s3;
    // if you do this, then _s3 is dead and _s4 is the new home,
    // if you try to print out the data in _s3, you will receive error.
    // println!("Moved _s3 to _s4, try to print _s3 {}", _s3);
    // but trying to print _s4 is ok though.
    println!("Moved _s3 to _s4, try to print _s4 {}", _s4);
    // from the above code, we leant that a varible using ownership,
    // will transfer its data to another, the another is now the main home,
    // while the previous one is dead.
    // in other languages, you can create a bunch of variables,
    // that have the same values, this is redundant and wasteful,
    // rust solve this by removing the redundant from the code itself,
    // once the data is moved and not waiting for compiling.
    println!("-------------------------------------------------");
    
    // -------------------------------------------------
    // scope -> phạm vi
    // _x scope is fn main, it's global.
    let _x = 10;
    {
        // _y scope is _x itself, it's local.
        let _y = "Hello";
        // these 2 code work normally, because _y is inside _x, it's in scope.
        // while _x is inside _x itself, it's still in scope.
        println!("Value of _x in scope: {}!", _x);
        println!("Value of _y in scope: {}!", _y);
    }
    // _x scope is fn main itself, so it is normal.
    println!("Value of _x out of scope: {}!", _x);
    // _y scope is _x, so it is useless when we try to call it in fn main.
    // rust: out of scope -> its value drop dead.
    // println!("Value of _y out of scope: {}!", _y);
    println!("-------------------------------------------------");
    
    // -------------------------------------------------
    // pointer means pointing to the address of a variable.
    // locate variable to access data,
    // but you can also locate address to also access data.
    // pointer does not move your variable like ownership, 
    // it just point to the address of said variable,
    // meaning getting the data while avoiding to move.
    let _x = 100;
    let _y = 10;
    let mut _p = &_x;
    println!("Access data via variable _x: {}!", _x);
    println!("Access data via address _p: {} - The address: {:p}!", _p, _p);
    // this is why we use "mut", to show the flexibility.
    _p = &_y;
    println!("After changing, Access data via address _p: {} - The address: {:p}!", _p, _p);
    // rust is smart enough to auto translate the pointer into value.
    println!("The data is the same whether it's normal or *: {}!", *_p);
    println!("The data is the same whether it's normal or &: {}!", &_p);
    // &str also has an address, you can use as_ptr to view the address,
    // it's just like C.
    let _s3 = String::from("Hello bitch");
    println!("String address: {:p}!", _s3.as_ptr());
    // Stack is static address like String,
    // Heap is dynamic address like primitive variable,
    // although both of the addresses change.
    // primitive is default a Stack cause you know the size,
    // collections like String and Vector are not cause they are dynamic,
    // meaning we do not know the size.
    // we will demonstrate the "getting data while not moving" as followed.
    let _p_s4 = &_s3;
    // after we linked the address, we can get the data of both normally,
    // their addresses are also the same, proving our point.
    println!("After link, Pointer value: {} - Pointer address: {:p}!", _p_s4, _p_s4.as_ptr());
    println!("After link, variable value: {} - Variable address: {:?}!", _s3, _s3.as_ptr());
    println!("-------------------------------------------------");
    
    // -------------------------------------------------
    // length and capacity.
    // length is the actual size, while the capacity is the future size,
    // at first they might be the same, but they will be different later on.
    let mut _s6 = String::from("Ahole");
    println!("Before _s6 length: {} - capacity: {}", _s6.len(), _s6.capacity());
    // more capacity will be granted when the data's length/capacity is exceed.
    _s6.push_str(" bee");
    println!("After _s6 length: {} - capacity: {}", _s6.len(), _s6.capacity());
    println!("-------------------------------------------------");
    
    // -------------------------------------------------
    // remember the owning thing that we defined before, where once we moved house,
    // the old house is dead and the new house is the main house from now on,
    // a way to fix this is to use pointer, where it will pointer to the address,
    // pointer is not a variable or a copy, it just points,
    // there is another way, we will have to use clone to create a copy.
    let _s7 = String::from("Before");
    let _p_s7 = &_s7;
    // we will print like this to show you that the pointer just point,
    // it doesnt create or copy the variable, meaning no move happened.
    println!("Pointer to original variable _s7: {}", _p_s7);
    println!("Original variable _s7: {}", _s7);
    // this is the "move" part, where the old house is dead,
    // and the new house is the main house from now on.
    // let _s8 = _s7;
    // to avoid that, we will have to use clone.
    let _s8 = _s7.clone();
    // we setup like this to show you that the old house still exist,
    // along side with the new house.
    // btw, their addresses are different.
    println!("_s8 the cloned one: {} - address: {:?}", _s8, _s8.as_ptr());
    println!("_s7 the original one: {} - address: {:?}", _s7, _s7.as_ptr());
    // in C you can change the value from the pointer, but rust cant do it,
    // if you try to do it, you cant, cause it is immutable.
    // one variable can have multiple poiters reference it.
    // we already have _p_s7 references _s7, now we have _p_s7_2 as well.
    let _p_s7_2 = &_s7;
    println!("Another pointer to original variable _s7: {}", _p_s7_2);
    println!("-------------------------------------------------");
    
    // -------------------------------------------------
    // mutable reference, the thing the pointer in C can do.
    // there are 2 types of references: immutable and mutable.
    // immutable which can only read and you can have many of them.
    // mutable which can CRUD but you can only have 1 of them at a time.
    // immutable and mutable do not mix, if you found yourself mixing,
    // make sure that you don't have any references to the variable,
    // before trying to access it mutably.
    // mutable references are created by the murable original variable sharing,
    // its mutable ability, as demonstrated below.
    // first, the original variable need to be mutable, add a "mut".
    let mut _s9 = String::from("tired");
    // second, the reference variable must also have a "&mut" when borrowing,
    // this meaning the original mutable variable shared the mutable ability,
    // to the reference variable.
    let _p_s9 = &mut _s9;
    // the thing with the mutable reference is that,
    // once the reference variable changes its data, 
    // the original referenced variable changes as well.
    _p_s9.push_str("string");
    // we need to print in this order, reason explained below.
    println!("_p_s9 value: {} - address: {:p}", _p_s9, _p_s9);
    println!("_s9 value: {} - address: {:?}", _s9, _s9.as_ptr());
    let _s10 = &mut _s9;
    _s10.push_str("string");
    println!("_s10 value: {} - address: {:p}", _s10, _s10);
    let _s11 = &_s9;
    println!("_s11 value: {} - address: {:p}", _s11, _s11);
    // this mutable ref has a different address,
    // because it ref from _s11 not _s9 like the others.
    let _s12 = &_s11;
    println!("_s12 value: {} - address: {:p}", _s12, _s12);
    println!("-------------------------------------------------");
    
    // -------------------------------------------------
    // immutable and mutable reference.
    // firstly, immutable ref are the normal "&", they can only read,
    // that is why there are so many of them all at once,
    // and they can be at any order they like.
    let _immutable_ref_ex = String::from("hello");
    let _p1_immutable_ref_ex = & _immutable_ref_ex;
    let _p2_immutable_ref_ex = & _immutable_ref_ex;
    println!("Immutable reference 2: {}", _p2_immutable_ref_ex);
    println!("Immutable reference 1: {}", _p1_immutable_ref_ex);
    // secondly, mutable ref are the "&mut", they can CRUD,
    // there must be only 1 mutable ref at a time and any time,
    // to create the mutable ref, the orignal variable,
    // the referenced variable must also be a mutable.
    let mut _mutable_ref_ex = String::from("hello");
    let _p_mutable_ref_ex = &mut _mutable_ref_ex;
    _p_mutable_ref_ex.push_str(" world");
    println!("Immutable reference: {}", _p_mutable_ref_ex);
    println!("Original variable, References variable: {}", _mutable_ref_ex);
    // but this code below will result in an error,
    // reason number 1, there are 2 mutable variables,
    // that referencing the same original variable,
    // and that's a no no,
    // reason number 2, _ss1 borrows from ss first,
    // then _ss2 borrows from ss second, but we print _ss1, 
    // in this time _ss2 is borrowing from ss, not _ss1,
    // and _ss1 scope was over.
    // let mut ss = String::from("hello");
    // let _ss1 = &mut ss;
    // let _ss2 = &mut ss;
    // println!("Error code: {}", _ss1);
    // and this code below will result in an error, 
    // it also has the same explanation as above for the error,
    // you can try to switch places but it's still the same error,
    // TL;DR: _ss2 borrows first, _ss1 borrows second,
    // _ss1 is borrowing, not _ss2, _ss1 is in scope, not _ss2,
    // when we try to print _ss1, it's ok, but we also try to print _ss2 below,
    // because it's _ss1 scope, not _ss2, so rust dishes out an error.
    // let mut ss = String::from("hello");
    // let _ss2 = &mut ss;
    // let _ss1 = &mut ss;
    // println!("Error code: {}", _ss1);
    // println!("Error code: {}", _ss2);
    // however, this code below will not create an error,
    // because they are just borrowing, they are not using yet.
    let mut _mutable_ref_ex_temp = String::from("hello");
    let _ss1 = &mut _mutable_ref_ex_temp;
    let _ss2 = &mut _mutable_ref_ex_temp;
    // this code below will also not create an error,
    // we will take the explanation 2 as our references,
    // p1 borrows first, then p2 borrows second,
    // we print p2 which is still borrowing and in scope, so it's ok.
    let mut _mutable_ref_ex_2 = String::from("hello");
    let _p1_mutable_ref_ex_2 = &mut _mutable_ref_ex_2;
    let _p2_mutable_ref_ex_2 = &mut _mutable_ref_ex_2;
    println!("Normal code: {}", _p2_mutable_ref_ex_2);
    // this code below will work normally because it's so simple,
    // _ss2 borrows, print _ss2, _ss2's basically done.
    let mut ss = String::from("hello");
    let _ss2 = &mut ss;
    println!("Simple code: {}", _ss2);
    // _ss1 borrows, print _ss1, _ss1's basically done.
    let _ss1 = &mut ss;
    println!("Simple code: {}", _ss1);
    // if you try to print _ss2, it's not gonna work, 
    // because it's _ss1's borrowing, not _ss2, _ss2 scope is over.
    // println!("Error code: {}", _ss2);
    // original variable that are immutable cannot be mix-matched,
    // you can't even declare borrowing it, let alone trying to print. 
    // let _mix_match_ref_ex = String::from("mix-match");
    // let _p1_mix_match = &_mix_match_ref_ex;
    // let _p2_mix_match = &mut _mix_match_ref_ex;
    // original variable that are mutable can be mix-matched,
    // it follows the basics rule of borrowing, as we have said above,
    // p1 borrows first, then p2 borrows second,
    // we print p2 which is still borrowing and in scope, so it's ok.
    let mut _mix_match_ref_ex = String::from("mix-match");
    let _p1_mix_match = &_mix_match_ref_ex;
    let _p2_mix_match = &mut _mix_match_ref_ex;
    // if we try to print p1, there will be an error,
    // because p2 is borrowing, not p1, and p1's scope is over.
    // println!("{}", _p1_mix_match);
    println!("Normal mix-match code: {}", _p2_mix_match);
    // this code below won't work as I have explained before,
    // and as the explaination above.
    // let mut _mix_match_ref_ex = String::from("mix-match");
    // let _p2_mix_match = &mut _mix_match_ref_ex;
    // let _p1_mix_match = &_mix_match_ref_ex;
    // println!("Error code: {}", _p1_mix_match);
    // println!("Error code: {}", _p2_mix_match);
    // this is a special case, there is a mutable orginal variable,
    // and a mutable ref, that is.
    // this code below works, as we have explained before,
    // p1 is borrowing and in scope, so it's ok, it's normal.
    let mut _mix_match_ref_ex_3 = String::from("matching");
    let _p1_mix_match_3 = &mut _mix_match_ref_ex_3;
    println!("Normal code 2: {}", _p1_mix_match_3);
    // but this code below is a little bit strange,
    // although p1 is borrowing and in scope,
    // but we can still print the mutable original variable,
    // we have a theory, because there is just 1 mutable ref,
    // the p1, and we are printing the mutable original variable,
    // the _mix_match_ref_ex_2, it doesn't matter if p1 is borrowing,
    // or p1's in scope, it's still can work.
    let mut _mix_match_ref_ex_2 = String::from("mixing");
    let _p1_mix_match_2 = &mut _mix_match_ref_ex_2;
    println!("Normal code 1: {}", _mix_match_ref_ex_2);
    // I can't say the same for the code below,
    // the code below will result in an error, I have a theory,
    // p1 is borrowing and in scope, and as the example above, 
    // you can print out the mutable original variable,
    // but you also print p1 RIGHT AFTER it, 
    // this snap the rust compiler, making it realized that,
    // p1 is borrowing and in scope, which make an error call.
    // let mut _mix_match_ref_ex_4 = String::from("matching");
    // let _p1_mix_match_4 = &mut _mix_match_ref_ex_4;
    // println!("Error code: {}", _mix_match_ref_ex_4);
    // println!("Normal code: {}", _p1_mix_match_4);
    // the above theory is pausible because the code below works,
    // currently, p1 is borrowing and in scope, we print p1 first, 
    // it's normal, no error happened, after doing that,
    // then we print the mutable original variable,
    // this doesn't mess with p1's borrowing.
    let mut _mix_match_ref_ex_4 = String::from("matching");
    let _p1_mix_match_4 = &mut _mix_match_ref_ex_4;
    println!("Trying to print p1, normal code: {}", _p1_mix_match_4);
    println!("Trying to print mutable orginal variable, normal code: {}", _mix_match_ref_ex_4);
    // if you have a configuration that have 3 variables like this,
    // it's not gonna work and will produce an error,
    // even if you try to change the places of the 3 prints,
    // reason for this error has been explained above,
    // TL,DR: 2 pointers trying to mutable referencing,
    // the same orignal variable,
    // and both are trying to print them out, which is a no no,
    // btw, printing the orignal variable before is not available,
    // because the 2 pointers are still trying to print at the same time,
    // which are causing the error,
    // btw, printing the original variable after is not available,
    // technically, it works as the example above but,
    // the 2 pointers are still trying to print at the same time,
    // which make the rust compiler snap and register it as an error.
    // let mut _mix_match_ref_ex_5 = String::from("matching-making");
    // let _p1_mix_match_5 = &mut _mix_match_ref_ex_5;
    // let _p2_mix_match_5 = &mut _mix_match_ref_ex_5;
    // println!("Error code 2: {}", _p2_mix_match_5);
    // println!("Error code 1: {}", _p1_mix_match_5);
    // println!("Error code 3: {}", _mix_match_ref_ex_5);
    // a way to fix this is to eliminate 1 variable that is printing.
    // if we eliminate prinitng the original variable,
    // there is still error, even if we swap places, 
    // the reason are explained above.
    // let mut _mix_match_ref_ex_5 = String::from("matching-making");
    // let _p1_mix_match_5 = &mut _mix_match_ref_ex_5;
    // let _p2_mix_match_5 = &mut _mix_match_ref_ex_5;
    // println!("Error code 2: {}", _p2_mix_match_5);
    // println!("Error code 1: {}", _p1_mix_match_5);
    // we need to eliminate the printing of one of the pointer,
    // in this case, the pointer p1, reason explained above,
    // we need to print in this specific order, reason explained above. 
    let mut _mix_match_ref_ex_5 = String::from("matching-making");
    let _p1_mix_match_5 = &mut _mix_match_ref_ex_5;
    let _p2_mix_match_5 = &mut _mix_match_ref_ex_5;
    println!("Error code 2: {}", _p2_mix_match_5);
    println!("Error code 3: {}", _mix_match_ref_ex_5);
    // this code below is a little bit strange, I don't know :(,
    // because it doesn't follow any of the above explanations,
    // you should check the error E0502 for more information.
    // as a rule of thumb. mutable ref is always first,
    // mutable borrowing/access must always completed first,
    // before any immutable references.
    let mut _mutable_original_variable_ex_6 = String::from("value");
    // this is a mutable reference, indicated by the "&mut String".
    let _mutable_ref_s_1: &mut String = &mut _mutable_original_variable_ex_6;
    // this is also a mutable reference, indicated by the "&&mut String",
    // "&&" is more or less the same as "&".
    let _sub_s2: &&mut String = &_mutable_ref_s_1;
    // this is NOT mutable reference, it's a immutable reference,
    // indicated by the "&String" only,
    // that is why there are no error...yet.
    let _sub_s1: &String = &_mutable_original_variable_ex_6;
    // this code will have error because of our rule of thumb,
    // that we have defined above from the explanation of "E0502".
    // let mut _mutable_original_variable_ex_6 = String::from("value");
    // this is a mutable borrow, indicated by the "&mut".
    // let _mutable_ref_s_1: &mut String = &mut _mutable_original_variable_ex_6;
    // but this is an immutable borrow, indicated by "&".
    // let _sub_s1: &String = &_mutable_original_variable_ex_6;
    // but then a mutable borrow again, indicate by "&&mut",
    // but as we have said before mutable borrow must completed first,
    // before ay immutable references.
    // let _sub_s2: &&mut String = &_mutable_ref_s_1;
    println!("-------------------------------------------------");

    // -------------------------------------------------
    let mut _s_fn = String::from("hello");
    println!("_s_fn value before: {} - address: {:p}", _s_fn, _s_fn.as_ptr());
    // as I said, pointer in rust is so much clearer, 
    // the way your fn takes in = the way you put in your parameter,
    // fn change takes in "&mut String" so when you call your fn change,
    // you also put in "&mut String", and you should add "mut" to your String,
    // because you're changing its value.
    change(&mut _s_fn);
    println!("_s_fn value after: {} - address: {:p}", _s_fn, _s_fn.as_ptr());
    // this code below will work, because it's 2 different scopes,
    // one is the scope in the fn main, the other is in fn change,
    // they do not involve each other.
    let mut _s_fn_2 = String::from("try this");
    let _s_fn_var = &_s_fn_2;
    change(&mut _s_fn_2);
    // but all of the above comments change when we try to use print, 
    // there will be an error, we suggest 2 reasons for the error,
    // reason 1, as explained above, TL;DR: _s_fn_var borrows first,
    // then fn change borrows, in this time fn change is borrowing,
    // not _s_fn_var, fn change is in scope, not _s_fn_var,
    // so if you are trying to print out of the scope, there will be error.
    // reason 2, error code E0502, TL;DR: there must be no references,
    // before the mutable reference.
    // println!("Error code: {}", _s_fn_var);
    println!("-------------------------------------------------");
    
    // -------------------------------------------------
    // as we've talked before, once ownership's moved, the old variable is dead,
    // the only way for the old variable to live is to either clone it,
    // or use reference, with reference, it's either immutable or mutable.
    println!("-------------------------------------------------");
    
    println!("Hello, world!");
}

// this is so much better than C, in C there is no indication of a reference,
// there is but it's small, where in rust, you can clearly see it,
// this is a mutable reference of a String.
fn change(some_string: &mut String) {
    some_string.push_str(", this is a function!");
}
