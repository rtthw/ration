use ration::Array;

fn main() {
    // Allocate a 32-character array to shared memory.
    let mut my_array: Array<char> = Array::alloc("/tmp/RATION_HELLOWORLD", 32).unwrap();
    my_array.push_many("Hello, world!".chars());

    // Start the client process, and wait on its output.
    std::process::Command::new("cargo")
        .args(["run", "--example", "helloworld_client"])
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
}
