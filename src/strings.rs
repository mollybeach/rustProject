/*
Primitive str = Immutable fixed-length string
String = Growable, heap-allocated data structure. Use when you need to modify or own string data.
string data
*/
pub fn run(){
    let hello = String::from("Hello"); //growable string
    println!("{}", hello);

}