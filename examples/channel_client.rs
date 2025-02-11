


use std::time;

use ration::Array;



fn main() {
    let mut client_array: Array<char> = Array::alloc("/tmp/CHANNEL_CLIENT", 64).unwrap();
    let mut server_array: Array<char> = Array::open("/tmp/CHANNEL_SERVER").unwrap();

    let start_time = time::Instant::now();

    let mut char_iter = "thisisatest".chars(); // More than 5 characters long.
    loop {
        // Make sure the client doesn't just run forever.
        if time::Instant::now().duration_since(start_time) >= time::Duration::from_secs(3) {
            println!("CLIENT: Done!");
            break;
        }
        // Check the server
        if let Some(ch) = server_array.pop() {
            println!("CLIENT: Sending message #{ch}...");
            client_array.push(char_iter.next().unwrap());
        }
    }
}
