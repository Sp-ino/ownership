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
    }
    // --------------------------------------------------------------------------

}


fn print_incremented_int(mut arg: i32) {
    // when this function is called the variable that is passed to it,
    // which resides in the stack, is copied into another variable,
    // that also resides in the stack
    arg = arg + 1;
    println!("print_incremented_int:argument incremented by one is: {arg}"); 
}


fn print_len_with_ownership(mut arg: String) {
    // when this function is called the variable that is passed to it
    // is *moved* into the scope of the function and becomes invalid in the scope
    // from which it has been passed. This means that the triplet
    // (address, len, capacity) in the stack is moved from the scope
    // of the caller to the scope of the callee, while the heap region in which the
    // actual string is stored remains unchanged when the ownership of the
    // variable changes
    let mut len = arg.len();
    println!("print_len_with_ownership:length of argument string: {len}");

    // I'm not allowed to modify
    arg.push_str("world!");
    len = arg.len();
    println!("print_len_with_ownership:length of argument string after adding 'world!' to it: {len}");
}