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
    println!("{}", s);


}