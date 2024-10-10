use std::process::Command;
use std::io::{self, Write};

const NETWORK: &str = "testnet";
const RPC_URL: &str = "https://horizon-testnet.stellar.org";
const NETWORK_PASSPHRASE: &str = "Test SDF Network ; September 2015";
const CONTRACT_ID: &str = "YOUR_CONTRACT_ID_HERE"; // Replace with your actual contract ID
const SOURCE_ACCOUNT: &str = "YOUR_SOURCE_ACCOUNT"; // Replace with your account address

fn main() {
    loop {
        println!("\nCamping Reservation System");
        println!("1. Make a reservation");
        println!("2. View reservations");
        println!("3. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => make_reservation(),
            "2" => view_reservations(),
            "3" => break,
            _ => println!("Invalid option, please try again."),
        }
    }

    println!("Thank you for using the Camping Reservation System!");
}

fn make_reservation() {
    println!("\nMake a Reservation");
    
    let site_number: u32 = get_input("Enter site number: ").parse().unwrap();
    let start_date = get_input("Enter start date (Unix timestamp): ").parse::<u64>().unwrap();
    let end_date = get_input("Enter end date (Unix timestamp): ").parse::<u64>().unwrap();
    let camper = get_input("Enter camper name: ");

    let output = Command::new("stellar")
        .args([
            "contract", "invoke",
            "--id", CONTRACT_ID,
            "--source-account", SOURCE_ACCOUNT,
            "--network", NETWORK,
            "--rpc-url", RPC_URL,
            "--network-passphrase", NETWORK_PASSPHRASE,
            "--",
            "make_reservation",
            "--site_number", &site_number.to_string(),
            "--start_date", &start_date.to_string(),
            "--end_date", &end_date.to_string(),
            "--camper", &camper,
        ])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Reservation made successfully!");
    } else {
        println!("Failed to make reservation: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn view_reservations() {
    let output = Command::new("stellar")
        .args([
            "contract", "invoke",
            "--id", CONTRACT_ID,
            "--source-account", SOURCE_ACCOUNT,
            "--network", NETWORK,
            "--rpc-url", RPC_URL,
            "--network-passphrase", NETWORK_PASSPHRASE,
            "--",
            "get_reservations",
        ])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("\nCurrent Reservations:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("Failed to retrieve reservations: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
