mod print; //mod is a keyword that allows us to create a module
fn main() {
    print::run();
    //Print to console
    println!("Number: {}", 1);
    //Basic Formatting
    println!("{} is from {}", "Brad", "Mass");
    //Positional Arguments
    print!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");
    //Named Arguments
    println!("{name} likes to play {activity}", name="John", activity="Baseball");
    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}
