use iproc::Array;

fn main() {
    // Open a 32 character array in shared memory.
    let mut my_array: Array<char> = Array::open("/tmp/IPROC_HELLOWORLD").unwrap();
    let mut my_string = String::new();

    while let Some(c) = my_array.pop() {
        my_string.push(c);
    }

    println!("{}", my_string);
}
