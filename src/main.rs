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

    /// get drcom info and exit
    #[argh(switch)]
    info: bool,

    /// skip status check before login
    #[argh(switch)]
    force: bool,

    /// daemon mode (periodically check status and login if offline)
    #[argh(switch)]
    daemon: bool,

    /// path to pid file
    #[argh(option)]
    pid: Option<String>,
}

fn main() {
    let args: Args = argh::from_env();

    if let Some(pid_path) = &args.pid {
        if let Err(e) = check_pid_file(pid_path) {
            eprintln!("Error: Another instance might be running: {}", e);
            std::process::exit(1);
        }
    }

    if args.info {
        match drcom::get_drcom_info() {
            Ok(Some(info)) => {
                println!("Drcom Info: {:?}", info);
                if info.result == 1 {
                    println!("Status: Online");
                } else {
                    println!("Status: Offline");
                }
            }
            Ok(None) => println!("Failed to get drcom info (parsed as None)."),
            Err(e) => eprintln!("Error getting drcom info: {}", e),
        }
        return;
    }

    let username = match args.username {
        Some(u) => u,
        None => {
            eprintln!("Error: Username is required for login!");
            std::process::exit(1);
        }
    };

    let password = match args.password {
        Some(p) => p,
        None => {
            eprintln!("Error: Password is required for login!");
            std::process::exit(1);
        }
    };

    if args.daemon {
        println!("Daemon mode started. Checking status every 60 seconds...");
        loop {
            if let Err(e) = try_process(&username, &password, args.ip.as_deref(), args.force, true)
            {
                eprintln!("Error: {}", e);
            }
            thread::sleep(Duration::from_secs(60));
        }
    } else {
        // Single attempt
        match try_process(&username, &password, args.ip.as_deref(), args.force, false) {
            Ok(_) => println!("Done."),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn try_process(
    username: &str,
    password: &str,
    arg_ip: Option<&str>,
    force: bool,
    quiet: bool,
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
    if !force {
        if let Some(ref info) = info {
            if info.result == 1 {
                if !quiet {
                    println!("Current status: Online. No login needed.");
                }
                return Ok(());
            }
        }
    } else {
        if !quiet {
            println!("Force login enabled, skipping status check.");
        }
    }

    let ip = if let Some(ip) = arg_ip {
        ip.to_string()
    } else {
        if let Some(ref info) = info {
            if let Some(ip) = &info.v46ip {
                if !quiet {
                    println!("Detected IP: {}", ip);
                }
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

fn check_pid_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::io::Read;
    use std::process::Command;

    if let Ok(mut file) = fs::File::open(path) {
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            if let Ok(pid) = content.trim().parse::<u32>() {
                // Cross-platform process existence check
                let exists = if cfg!(windows) {
                    let output = Command::new("tasklist")
                        .args(&["/FI", &format!("PID eq {}", pid), "/NH"])
                        .output();
                    if let Ok(out) = output {
                        String::from_utf8_lossy(&out.stdout).contains(&pid.to_string())
                    } else {
                        false
                    }
                } else {
                    // Unix (Linux/macOS)
                    Command::new("kill")
                        .args(&["-0", &pid.to_string()])
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false)
                };

                if exists {
                    return Err(format!("Process with PID {} is already running", pid).into());
                }
            }
        }
    }

    fs::write(path, std::process::id().to_string())?;
    Ok(())
}
