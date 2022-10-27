fn main() {
    // ---------------- Copying variables that are stored in the stack----------- 
    {
        println!("\n\nSCOPE 1");
        let mut x: i8 = 2;
        println!("x is {x}");
        let y = x;
        println!("y is {y}");
        
        println!("I can still use x: in fact, x is {x}");

        x = 3;
        println!("Now I have bound x to {x}. y is {y}, so the copy I made with let y = x is 'deep'");
    }
    // --------------------------------------------------------------------------

    // ---------------------------Copying variables stored in the heap-----------
    {
        println!("\n\nSCOPE 2");

        let s = String::from("hello ");
        println!("s is {s}");
        let t = s;
        println!("t is {t}");

        // But now s is out of scope. If I try to access s, the code doesn't compile
        // Runinng this code
        // 
        // println!("s is {s}");
        // 
        // Produces the following compile error:
        // 
        //     error[E0382]: borrow of moved value: `s`
        // --> src/main.rs:24:21
        // |
        // 18 |     let s = String::from("hello ");
        // |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
        // 19 |     println!("s is {s}");
        // 20 |     let t = s;
        // |             - value moved here
        // ...
        // 24 |     println!("s is {s}");
        // |                     ^ value borrowed here after move
        // |
    }
    // --------------------------------------------------------------------------

    // ------------------------The same stuff but with functions-----------------
    {
        println!("\n\nSCOPE 3");

        let x: i32 = 2;
        println!("x before calling print_incremented_int is {x}");
        print_incremented_int(x);
        println!("x after calling print_icremented_int is {x}");

        //note that I don't need to declare the variable s mutable
        // even though it is passed as mutable to print_len_with_ownership
        // In other words, s becomes automatically mutable when its ownership
        // passes to print_len_with_ownership
        let s = String::from("hello ");
        println!("s is {s}");
        print_len_with_ownership(s); 


        // But now s is not valid anymore because its ownership was passed to print_len_with_ownership
        // Running the code
        // 
        // println!("s is {s}");
        // 
        // Produces the following compile error
        // 
        // error[E0382]: borrow of moved value: `s`
        // --> src/main.rs:63:25
        // |
        // 58 |         let s = String::from("hello ");
        // |             - move occurs because `s` has type `String`, which does not implement the `Copy` trait
        // 59 |         println!("s is {s}");
        // 60 |         print_len_with_ownership(s);
        // |                                  - value moved here
        // ...
        // 63 |         println!("s is {s}");
        // |                         ^ value borrowed here after move
        // |

        // ADDITIONAL CONSIDERATIONS ABOUT CASTING AN IMMUTABLE VARIABLE 
        // AS MUTABLE WHEN PASSING IT TO A FUNCTION
        // I can pass an immutable variable to a function that has a
        // signature like the following:
        // 
        // fn myfunc(mut myarg: String) //obviously we can have other data types
        // 
        // By doing so, the variable becomes mutable when it is moved to
        // the scope of the function. For example,
        let new_s = String::from("hello ");
        take_and_return_ownership_with_mut(new_s);

        // The take_and_return_ownership_with_mut function prints the length of
        // the argument, appends "world!" to it and then prints the new length.
        // Finally, it returns the modified string.
        // Question: is the new_s variable still valid after passing it to the function?
        // 
        // println!("This is the value of new_s: {new_s}"); //compile error!
        // 
        // Answer: NO!
        // Yet, I can't bind new_s to the return value of the function either.
        // If I had written
        //
        // new_s = take_and_return_ownership_with_mut(new_s); //compile error!
        // 
        // I would have also produced a compile error.
        // Instead, it works with shadowing:

        let new_s = String::from("hello ");
        let new_s = take_and_return_ownership_with_mut(new_s);
        println!("This is the value of new_s: {new_s}");

    }
    // --------------------------------------------------------------------------

    // ---------------Experiments with borrowing and references------------------
    {
        println!("\n\nSCOPE 4");
        let mystr = String::from("Ziopera");

        // In this case I can still use mystr after passing it to print_with_borrowing,
        // because we only pass a reference to the function
        print_len_with_borrowing(&mystr);

        let mut another_str = String::from("ziopera");

        // I can pass the reference to a *mutable* variable to a function that expects a mutable
        // reference and then use the variable after it has been modified by
        // the function
        append_with_borrowing(&mut another_str);
        println!("another_str after calling append_with_borrowing on it: {another_str}");

        // But I cannot pass a reference to an immutable variable to a function that expects
        // a mutable reference:
        // 
        // append_with_borrowing(&mut mystr); //compile error!
    // --------------------------------------------------------------------------
    }
}


fn print_incremented_int(arg: i32) {
    // when this function is called the variable that is passed to it,
    // which resides in the stack, is copied into another variable,
    // that also resides in the stack
    let res = arg + 1;
    println!("print_incremented_int:argument incremented by one is: {res}"); 
}


fn print_len_with_ownership(arg: String) {
    // when this function is called the variable that is passed to it
    // is *moved* into the scope of the function and becomes invalid in the scope
    // from which it has been passed. This means that the triplet
    // (address, len, capacity) in the stack is moved from the scope
    // of the caller to the scope of the callee, while the heap region in which the
    // actual string is stored remains unchanged when the ownership of the
    // variable changes
    let len = arg.len();
    println!("print_len_with_ownership:length of string {}: {}", arg, len);
}


fn take_and_return_ownership_with_mut(mut arg: String) -> String{
    // when this function is called the variable that is passed to it
    // is *moved* into the scope of the function and becomes invalid in the scope
    // from which it has been passed. This means that the triplet
    // (address, len, capacity) in the stack is moved from the scope
    // of the caller to the scope of the callee, while the heap region in which the
    // actual string is stored remains unchanged when the ownership of the
    // variable changes
    let mut len = arg.len();
    println!("take_and_return_ownership_with_mut:length of string {}: {}", arg, len);

    // I'm allowed to modify, if I specify that the argument
    // is passed as mutable
    arg.push_str("world!");
    len = arg.len();
    println!("take_and_return_ownership_with_mut:length of {} after adding 'world!' to it: {}", arg, len);

    arg
}


fn print_len_with_borrowing(s: &String) {

    let len = s.len();
    println!("print_len_with_borrowing:the string {} has length {}", s, len);
}


fn append_with_borrowing(s: &mut String) {
    s.push_str(" fra!");
}