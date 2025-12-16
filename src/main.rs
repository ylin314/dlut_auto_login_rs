mod des_crypto;
mod drcom;
mod login;

use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(name = "DLUT Auto Login")]
#[command(about = "DLUT Campus Network Auto Login Tool", long_about = None)]
struct Args {
    #[arg(short, long, help = "Username")]
    username: Option<String>,

    #[arg(short, long, help = "Password")]
    password: Option<String>,

    #[arg(short, long, help = "IPV4 Address")]
    ip: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let info = match drcom::get_drcom_info().await {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error getting drcom info: {}", e);
            None
        }
    };

    // Check if already online
    if let Some(ref info) = info {
        if info.result == 1 {
            println!("Current status: Online. Never need to login.");
            return;
        }
    }

    let username = args.username.unwrap_or_else(|| {
        print!("Please enter your username: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    });

    let password = args.password.unwrap_or_else(|| rpassword::prompt_password("Please enter your password: ").unwrap());

    let ip = if let Some(ip) = args.ip {
        ip
    } else {
        if let Some(ref info) = info {
            if let Some(ip) = &info.v46ip {
                println!("Detected IP: {}", ip);
                ip.clone()
            } else {
                eprintln!("Failed to get local IP address!");
                return;
            }
        } else {
            eprintln!("Failed to get local IP address!");
            return;
        }
    };

    match login::login(&username, &password, &ip).await {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Login error: {}", e),
    }
}
