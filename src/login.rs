use crate::des_crypto::str_enc;
use crate::drcom::get_drcom_info;
use scraper::{Html, Selector};
use std::thread;
use std::time::Duration;

/// Extract value from HTML element by id or name attribute
fn extract_value_by_id_or_name(
    document: &Html,
    attribute_type: &str,
    attribute_value: &str,
) -> Result<String, String> {
    let selector_str = match attribute_type {
        "id" => format!("[id='{}']", attribute_value),
        "name" => format!("[name='{}']", attribute_value),
        _ => {
            return Err(format!(
                "Attribute type must be 'id' or 'name', but got: {}!",
                attribute_type
            ))
        }
    };

    let selector =
        Selector::parse(&selector_str).map_err(|_| "Failed to parse selector".to_string())?;

    if let Some(element) = document.select(&selector).next() {
        if let Some(value) = element.value().attr("value") {
            return Ok(value.to_string());
        }
    }

    Err(format!(
        "Element with {} = {} not found!",
        attribute_type, attribute_value
    ))
}

/// Actual login function
fn do_login(username: &str, password: &str, ip: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Initial URL that redirects to SSO login page
    let initial_url = format!(
        "http://172.20.30.2:8080/Self/sso_login?login_method=1&wlan_user_ip={}&wlan_user_ipv6=&wlan_user_mac=000000000000&wlan_ac_ip=172.20.30.254&wlan_ac_name=&mac_type=1&authex_enable=&type=1",
        ip
    );

    println!("Initial login URL: {}", initial_url);

    // Create an agent with cookie support
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .redirects(10)
        .build();

    // First, access the initial URL
    let response = agent.get(&initial_url).call()?;

    // Get the final URL after redirects
    let final_url = response.get_url().to_string();

    // Read the response body
    let body = response.into_string()?;

    // Parse the HTML
    let document = Html::parse_document(&body);

    // Extract lt value
    let lt_value = extract_value_by_id_or_name(&document, "id", "lt")?;
    println!("lt: {}", lt_value);

    // Extract execution value
    let execution_value = extract_value_by_id_or_name(&document, "name", "execution")?;
    println!("execution: {}", execution_value);

    // Extract _eventId value
    let event_id_value = extract_value_by_id_or_name(&document, "name", "_eventId")?;
    println!("_eventId: {}", event_id_value);

    // Check if we were redirected to SSO login page
    if final_url != initial_url {
        println!("跳转到sso登录页面: {}", final_url);

        // Prepare login data
        let rsa = str_enc(
            &format!("{}{}{}", username, password, lt_value),
            "1",
            "2",
            "3",
        );

        println!("Login form:");
        println!(
            "  rsa: {}, ul: {}, pl: {}, sl: 0, lt: {}, execution: {}, _eventId: {}",
            rsa,
            username.len(),
            password.len(),
            lt_value,
            execution_value,
            event_id_value
        );

        // Submit login form
        let login_response = agent.post(&final_url).send_form(&[
            ("rsa", &rsa),
            ("ul", &username.len().to_string()),
            ("pl", &password.len().to_string()),
            ("sl", "0"),
            ("lt", &lt_value),
            ("execution", &execution_value),
            ("_eventId", &event_id_value),
        ])?;

        // Check if login was successful by checking the final URL
        let post_login_url = login_response.get_url().to_string();

        // If we were redirected away from the SSO login page, it might be successful
        if post_login_url != final_url {
            println!("Redirection...");

            // Wait 3 seconds for the backend to update the login status
            thread::sleep(Duration::from_secs(3));

            // Check drcom info to confirm login success
            match get_drcom_info() {
                Ok(Some(info)) if info.result == 1 => {
                    println!("Login successful!");
                    return Ok(true);
                }
                _ => {
                    println!("Login failed, unable to get drcom info after login.");
                    return Ok(false);
                }
            }
        } else {
            println!(
                "Login failed, no redirection found. Please check the entered account, password, and IP."
            );
            return Ok(false);
        }
    } else {
        return Err("No redirection, direct access!".into());
    }
}

/// Handle login process with retry mechanism
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
        println!("Attempting to log in for the {}th time...", attempt_count);

        match do_login(username, password, ip) {
            Ok(true) => {
                return Ok(format!(
                    "ip: {}, Please confirm if have successfully connected to the network.",
                    ip
                ));
            }
            Ok(false) => {
                return Ok(format!("ip: {}, Login failed!", ip));
            }
            Err(e) => {
                println!("Login failed: {}, retrying...", e);
                attempt_count += 1;
                thread::sleep(Duration::from_secs(3));
            }
        }
    }

    Ok(format!(
        "ip: {}, Reached the maximum number of login attempts, login failed!",
        ip
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_value() {
        let html = r#"
        <html>
            <body>
                <input type="hidden" id="lt" value="LT-123456" />
                <input type="hidden" name="execution" value="e1s1" />
                <input type="hidden" name="_eventId" value="submit" />
            </body>
        </html>
        "#;
        let document = Html::parse_document(html);

        assert_eq!(
            extract_value_by_id_or_name(&document, "id", "lt").unwrap(),
            "LT-123456"
        );
        assert_eq!(
            extract_value_by_id_or_name(&document, "name", "execution").unwrap(),
            "e1s1"
        );
        assert_eq!(
            extract_value_by_id_or_name(&document, "name", "_eventId").unwrap(),
            "submit"
        );
    }
}
