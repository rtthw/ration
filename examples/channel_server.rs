


use ration::Array;



fn main() {
    let mut server_array: Array<char> = Array::alloc("/tmp/CHANNEL_SERVER", 64).unwrap();

    let mut client = std::process::Command::new("cargo")
        .args(["run", "--example", "channel_client"])
        .spawn()
        .unwrap();

    // Give the client some time to start. This should be enough.
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Wait to open the client array until the client has allocated it.
    let mut client_array: Array<char> = Array::open("/tmp/CHANNEL_CLIENT").unwrap();

    let mut msg_count = 0;
    server_array.push(char::from_digit(msg_count, 10).unwrap());
    loop {
        // Only process the first 5 messages.
        if msg_count >= 5 {
            println!("SERVER: Done!");
            let _ = client.wait().unwrap();
            break;
        }
        // Process the next message, if there is any.
        if let Some(client_message) = client_array.pop() {
            println!("SERVER: Received message '{}' from client.", client_message);
            msg_count += 1;
            server_array.push(char::from_digit(msg_count, 10).unwrap());
        }
    }
}
