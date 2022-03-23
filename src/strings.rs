/*
Primitive str = Immutable fixed-length string
String = Growable, heap-allocated data structure. Use when you need to modify or own string data.
string data
*/
pub fn run(){
    let mut hello = String::from("Hello"); //growable string
    //Get length
    println!("Length: {}", hello.len());
    //Push Char 
    hello.push('W'); //push a char onto the string
    //Push String
    hello.push_str("orld!"); //push a string onto the string
    println!("{}", hello);
    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());
    //Check if empty
    println!("Is Empty: {}", hello.is_empty());
    //Contains a sub string
    println!("Contains 'World': {}", hello.contains("World"));
    //Replace
    println!("Replace: {}", hello.replace("World", "There"));
    //Loop through string by whitespace
    //split by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }
    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
        // Assertion testing
    assert_eq!(2, s.len());
    /* result: nothing */

    //only gives a result if the test fails
    assert_eq!(3, s.len());
    /*  
    result:
    thread 'main' panicked at 'assertion failed: `(left == right)`
        left: `3`,
        right: `2`', src/strings.rs:36:5*/
    // check if 10 is equal to the string capacity
    assert_eq!(10, s.capacity());

    println!("{}", s);

/*
    fn main() {
        let first_name = String::from("Bryan Gula"); // value type copy struct
        //let name = String::from(first_name);
        let name = first_name;
        println!("{}", name);
        println!("{}", first_name);  
        
       
       
        
        
      }*/
      

    


}

/*
LESSON #14


Task: Run your code. You should not see the error anymore.

When you are done, type the following for the next lesson:
    $ fcc 15

 fcc 15

LESSON #15


You want to add your surname (second name) to name.

There are many ways to do this in Rust. If you try to just concatenate " Surname" to &first_name, Rust will error, because you cannot concatenate to a referenced value.

You could remove the &, but then the second println! will cause the program to panic.

In order to concatenate a reference to a str (&str), the first argument needs to be owned. A String can be used as an owned value with the to_owned method:

    let owned_string = my_string.to_owned() + " Surname";

Task: Instead of moving first_name, turn it into an owned value, and concatenate your surname to it - assigning the result to name.

Run your code. If it compiles and prints the two lines, you have completed the lesson correctly. If not, use the output to debug and fix your code.

When you are done, type the following for the next lesson:
    $ fcc 16
*/