use std::fmt;

impl fmt::Display for Tuple {

}

fn main() {
    let mixed_type_tuple = (1u8, 2u16, 3u32, 4u64,
                            -1i8, -2i16, -3i32, -4i64,
                            0.1f32, 0.2f64,
                            'A', false);
    
    // printing values of tuples
    println!("mixed_type_tuple first value: {}", mixed_type_tuple.1);

    let tuple_of_tuples = ((1i8, 2u32, -16i64), (3i8, -2i16), 1337i32);

    println!("{:?}", tuple_of_tuples);

    println!("one element tuple {:?}", (5u32,));
    println!("unsigned int {:?}", (5u32));

    // Destructure tuple
    let tuple = (1337, true, 'A', 3.14);
    let (a, b, c, d) = tuple;

    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}