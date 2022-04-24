// This program plays around with different formats of printing

fn main() {
    // No vars
    println!("Hello!\n");

    // int automatically converted to string
    println!("{}\n", 1337);

    // reuse vars
    println!("{0} SYN {1}\n{1} SYN/ACK {0}\n{0} ACK {1}\n", "Alice", "Bob");

    // named vars
    println!("My name is {name}.\n", name="Gimbo");

    // specify var format (example is turn int to binary)
    println!("{} of {:b} people understand binary\n", 1, 2);

    // print text pushed to right without using spaces or \t
    println!("{var:>width$}\n", var="Hi", width=6);

    // print number with padding \t
    println!("{:#010x}", 1337);
}