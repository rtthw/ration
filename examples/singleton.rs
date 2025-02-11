// A FizzBuzz clone that uses the `Block` type to synchronize a singleton string.



use arrayvec::ArrayString;
use ration::Block;



fn main() {
    // Always keep an owned reference to allocated shared memory objects around for longer than
    // all other unowned references. The file won't close correctly if you don't follow this rule.
    // This goes for arrays too.
    let mut owned_block: Block<MySingleton> = Block::alloc("/tmp/RATION_SINGLETON").unwrap();

    // Make `owned_block` a valid instance of `MySingleton`. Without this, your reference
    // instances will be invalid and you'll get the dreaded "undefined behavior".
    *owned_block = MySingleton {
        my_pointerless_string: ArrayString::new(),
    };

    // This program is just a FizzBuzz clone that uses "seconds since program start" instead of
    // counting.
    println!("Starting FizzBuzz 2.0...");
    println!("\tNOTE: Sometimes prints \"BuzzFizz\" instead of \"FizzBuzz\".");

    let program_start_time = std::time::Instant::now();
    {
        std::thread::spawn(|| {
            thread_a();
        });
        // Hopefully, this makes it ordered correctly.
        std::thread::sleep(std::time::Duration::from_millis(1));
        std::thread::spawn(|| {
            thread_b();
        });
    }

    let mut print_count = 0;
    // Only print the first ten Fizzes/Buzzes/FizzBuzzes.
    // Should run for 21 seconds.
    while print_count < 10 {
        if owned_block.my_pointerless_string.len() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(30));
            let seconds_since_start = std::time::Instant::now()
                .duration_since(program_start_time)
                .as_secs();
            println!(
                "{}, {} seconds since program start",
                owned_block.my_pointerless_string,
                seconds_since_start,
            );
            owned_block.my_pointerless_string.clear();
            print_count += 1;
        }
    }
}

// Remember, absolutely no pointers allowed in shared memory objects.
// If you need to have a string in your shared type, use something like `arrayvec::ArrayString`.
struct MySingleton {
    pub my_pointerless_string: ArrayString<8>,
}

fn thread_a() {
    let mut local_block: Block<MySingleton> = Block::open("/tmp/RATION_SINGLETON").unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        local_block.my_pointerless_string.push_str("Fizz");
    }
}

fn thread_b() {
    let mut local_block: Block<MySingleton> = Block::open("/tmp/RATION_SINGLETON").unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        local_block.my_pointerless_string.push_str("Buzz");
    }
}
