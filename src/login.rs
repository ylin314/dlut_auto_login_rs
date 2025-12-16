use scraper::{Html, Selector};
use std::time::Duration;
use crate::des_crypto::str_enc;
use crate::drcom;

/// Extract value from HTML element by id or name
fn extract_value_by_id_or_name(html: &str, attr_type: &str, attr_value: &str) -> Result<String, String> {
    if attr_type != "id" && attr_type != "name" {
        return Err(format!(
            "Attribute type must be 'id' or 'name', but got: {}!",
            attr_type
        ));
    }

    let document = Html::parse_document(html);

    let element = if attr_type == "id" {
        let selector = Selector::parse(&format!("[id='{}']", attr_value))
            .map_err(|_| "Invalid selector".to_string())?;
        document.select(&selector).next()
    } else {
        let selector = Selector::parse(&format!("[name='{}']", attr_value))
            .map_err(|_| "Invalid selector".to_string())?;
        document.select(&selector).next()
    };

    if let Some(elem) = element {
        if let Some(value) = elem.value().attr("value") {
            Ok(value.to_string())
        } else {
            Err(format!("Element with {} = {} has no value attribute!", attr_type, attr_value))
        }
    } else {
        Err(format!(
            "Element with {} = {} not found!",
            attr_type, attr_value
        ))
    }
}

/// Perform the actual login
async fn do_login(username: &str, password: &str, ip: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let initial_url = format!(
        "http://172.20.30.2:8080/Self/sso_login?login_method=1&wlan_user_ip={}&wlan_user_ipv6=&wlan_user_mac=000000000000&wlan_ac_ip=172.20.30.254&wlan_ac_name=&mac_type=1&authex_enable=&type=1",
        ip
    );

    println!("Initial login URL: {}", initial_url);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    // First request to get the login form
    let response = client.get(&initial_url).send().await?;

    // Save the final URL before consuming the response body
    let sso_login_url = response.url().to_string();
    println!("Jumped to sso login page: {}", sso_login_url);

    // Read body once (consumes `response`) and extract form values
    let body = response.text().await?;

    let lt_value = extract_value_by_id_or_name(&body, "id", "lt")?;
    println!("lt: {}", lt_value);

    let execution_value = extract_value_by_id_or_name(&body, "name", "execution")?;
    println!("execution: {}", execution_value);

    let event_id_value = extract_value_by_id_or_name(&body, "name", "_eventId")?;
    println!("_eventId: {}", event_id_value);

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

    println!("Login form: {:?}", login_data);

    // Submit login form
    let login_response = client.post(&sso_login_url).form(&login_data).send().await?;

    // Check if login was successful by checking for redirection
    if !login_response.url().to_string().contains(&sso_login_url) {
        println!("Redirection detected...");

        // Wait 3 seconds for backend to refresh data
        tokio::time::sleep(Duration::from_secs(3)).await;

        if let Ok(Some(info)) = drcom::get_drcom_info().await {
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
pub async fn login(username: &str, password: &str, ip: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!(
        "Current login information: Username: {}, Password: ******, IP: {}",
        username, ip
    );

    let max_attempts = 3;
    let mut attempt_count = 1;

    while attempt_count < max_attempts {
        match do_login(username, password, ip).await {
            Ok(true) => break,
            Ok(false) => return Ok(format!("ip: {}, Login failed!", ip)),
            Err(e) => {
                println!("Login failed: {}, retrying...", e);
                attempt_count += 1;
                tokio::time::sleep(Duration::from_secs(3)).await;
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
