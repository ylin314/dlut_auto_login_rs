use crate::des_crypto::str_enc;
use crate::drcom;
use cookie_store::CookieStore;
use scraper::{Html, Selector};
use std::thread;
use std::time::Duration;

/// Extract value from HTML input element by a specific attribute (e.g., "id" or "name")
fn extract_input_value(
    document: &Html,
    attr_name: &str,
    attr_value: &str,
) -> Result<String, String> {
    // Selector for input[attr_name="value"]
    let selector_str = format!("input[{0}='{1}']", attr_name, attr_value);
    let selector = Selector::parse(&selector_str).map_err(|_| "Invalid selector".to_string())?;

    if let Some(element) = document.select(&selector).next() {
        if let Some(value) = element.value().attr("value") {
            return Ok(value.to_string());
        }
    }

    Err(format!(
        "Element with {} = {} not found or has no value!",
        attr_name, attr_value
    ))
}

/// Perform the actual login
fn do_login(username: &str, password: &str, ip: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let initial_url = format!(
        "http://172.20.30.2:8080/Self/sso_login?login_method=1&wlan_user_ip={}&wlan_user_ipv6=&wlan_user_mac=000000000000&wlan_ac_ip=172.20.30.254&wlan_ac_name=&mac_type=1&authex_enable=&type=1",
        ip
    );

    println!("Initial login URL: {}", initial_url);

    let agent = ureq::AgentBuilder::new()
        .cookie_store(CookieStore::default())
        .redirects(10)
        .build();

    // First request to get the login form
    let response = agent
        .get(&initial_url)
        .timeout(Duration::from_secs(10))
        .call()?;

    // Save the final URL before consuming the response body
    let sso_login_url = response.get_url();
    println!("Jumped to sso login page: {}", sso_login_url);
    let sso_login_url_str = sso_login_url.to_string();

    // Read body once (consumes `response`) and extract form values
    let body = response.into_string()?;
    let document = Html::parse_document(&body);

    let lt_value = extract_input_value(&document, "id", "lt")?;
    // println!("lt: {}", lt_value);

    let execution_value = extract_input_value(&document, "name", "execution")?;
    // println!("execution: {}", execution_value);

    let event_id_value = extract_input_value(&document, "name", "_eventId")?;
    // println!("_eventId: {}", event_id_value);

    // Prepare login data
    let rsa_value = str_enc(
        &format!("{}{}{}", username, password, lt_value),
        "1",
        "2",
        "3",
    )?;

    let login_data = [
        ("rsa", rsa_value.as_str()),
        ("ul", &username.len().to_string()),
        ("pl", &password.len().to_string()),
        ("sl", "0"),
        ("lt", &lt_value),
        ("execution", &execution_value),
        ("_eventId", &event_id_value),
    ];

    // println!("Login form: {:?}", login_data);

    // Submit login form
    let login_response = agent
        .post(&sso_login_url_str)
        .timeout(Duration::from_secs(10))
        .send_form(&login_data)?;

    // Check if login was successful by checking for redirection (similar to Python requests' `response.history`)
    let status = login_response.status();
    let location = login_response.header("location");
    let final_url = login_response.get_url();

    let redirected = (300..400).contains(&status)
        || location.is_some()
        || final_url != sso_login_url_str;

    if redirected {
        println!("Redirection detected...");

        // Wait 5 seconds for backend to refresh data
        thread::sleep(Duration::from_secs(5));

        if let Ok(Some(info)) = drcom::get_drcom_info() {
            if info.result == 1 {
                println!("Login successful!");
                return Ok(true);
            } else {
                println!("Login failed, unable to get drcom info after login.");
                return Ok(false);
            }
        } else {
            println!("Login failed, unable to get drcom info after login.");
            return Ok(false);
        }
    } else {
        println!("Login failed, no redirection found. Please check the entered account, password, and IP.");
        Ok(false)
    }
}

/// Handle each login attempt
pub fn login(
    username: &str,
    password: &str,
    ip: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!(
        "Current login information: Username: {}, Password: ******, IP: {}",
        username, ip
    );

    let max_attempts = 3;
    let mut attempt_count = 1;

    while attempt_count < max_attempts {
        match do_login(username, password, ip) {
            Ok(true) => break,
            Ok(false) => return Ok(format!("ip: {}, Login failed!", ip)),
            Err(e) => {
                println!("Login failed: {}, retrying...", e);
                attempt_count += 1;
                thread::sleep(Duration::from_secs(3));
            }
        }
    }

    if attempt_count == max_attempts {
        Ok(format!(
            "ip: {}, Reached the maximum number of login attempts, login failed!",
            ip
        ))
    } else {
        Ok(format!(
            "ip: {}, Please confirm if have successfully connected to the network.",
            ip
        ))
    }
}
