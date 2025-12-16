mod des_crypto;
mod drcom;
mod login;

use argh::FromArgs;
use std::thread;
use std::time::Duration;

#[derive(FromArgs, Debug)]
/// DLUT Campus Network Auto Login Tool
struct Args {
    /// username
    #[argh(option, short = 'u')]
    username: Option<String>,

    /// password
    #[argh(option, short = 'p')]
    password: Option<String>,

    /// ipv4 address
    #[argh(option, short = 'i')]
    ip: Option<String>,
}

fn main() {
    let args: Args = argh::from_env();

    let username = match args.username {
        Some(u) => u,
        None => {
            eprintln!("Error: Username is required! (Interactive mode disabled for embedded use)");
            std::process::exit(1);
        }
    };

    let password = match args.password {
        Some(p) => p,
        None => {
            eprintln!("Error: Password is required! (Interactive mode disabled for embedded use)");
            std::process::exit(1);
        }
    };

    // Retry loop for network readiness
    let mut retry_count = 0;
    loop {
        match try_process(&username, &password, args.ip.as_deref()) {
            Ok(_) => {
                println!("Done.");
                break;
            }
            Err(e) => {
                retry_count += 1;
                eprintln!(
                    "Attempt {} failed: {}. Retrying in 5 seconds...",
                    retry_count, e
                );
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}

fn try_process(
    username: &str,
    password: &str,
    arg_ip: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let info = match drcom::get_drcom_info() {
        Ok(info) => info,
        Err(e) => {
            // If we can't even reach the drcom page, it might be a network issue.
            // We propagate the error to trigger a retry.
            return Err(format!("Error getting drcom info: {}", e).into());
        }
    };

    // Check if already online
    if let Some(ref info) = info {
        if info.result == 1 {
            println!("Current status: Online. No login needed.");
            return Ok(());
        }
    }

    let ip = if let Some(ip) = arg_ip {
        ip.to_string()
    } else {
        if let Some(ref info) = info {
            if let Some(ip) = &info.v46ip {
                println!("Detected IP: {}", ip);
                ip.clone()
            } else {
                return Err("Failed to get local IP address from drcom info!".into());
            }
        } else {
            return Err("Failed to get local IP address!".into());
        }
    };

    match login::login(username, password, &ip) {
        Ok(result) => {
            println!("{}", result);
            Ok(())
        }
        Err(e) => Err(format!("Login error: {}", e).into()),
    }
}
