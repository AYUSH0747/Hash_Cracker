//importing some libraries
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;
//Main Function
fn main() {
    let args: Vec<String> = env::args().collect();
    //In the above line we are collecting the arguments and storing in the args variable

    if args.len() != 2 {
        println!("Invalid amount of arguments!");
        println!("Example: cargo run <sha256 hash>");
        exit(1);
    }
    //To crack a hash we need two parameters if parameters are less than two, the above code snippet will show error

    let wanted_hash = &args[1];
    let password_file = "src/passwordlist.txt";
    let mut attempts = 1;

    println!("Attempting to crack: {}!\n", wanted_hash);

    let password_list = File::open(password_file).unwrap();
    let reader = BufReader::new(password_list);
    //The program has to read the same file again and again, so bufReader will improve the speed of the reading the file

    for line in reader.lines() {
        let line = line.unwrap();
        let password = line.trim().to_owned().into_bytes();
        let password_hash = format!("{:x}", Sha256::digest(&password));
        //The above line of code will convert the password from password list into Sha256 Hash.

        println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
        if &password_hash == wanted_hash {
            //If Password corresponding to the hash is found
            println!("Password corresponding to hash found after {} attempts! {} hashes to {}!", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
            exit(0);
        }
        attempts += 1;
    }
    //If no Password corresponding to the Hash is found
    println!("Password corresponding to hash not found! ");
}