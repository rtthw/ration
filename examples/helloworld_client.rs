use ration::Array;

fn main() {
    // Open a 32 character array in shared memory.
    let mut my_array: Array<char> = Array::open("/tmp/RATION_HELLOWORLD")
        .expect("'helloworld_client' example should only be started by the 'helloworld_server'");

    let mut my_string = String::new();
    while let Some(c) = my_array.pop() {
        my_string.push(c);
    }

    println!("'helloworld_client' got: \"{}\"", my_string);
}
