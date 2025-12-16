use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DrcomInfo {
    pub result: i32,
    #[serde(default)]
    pub v46ip: Option<String>,
}

/// Get drcom info (including IP and login status)
pub async fn get_drcom_info() -> Result<Option<DrcomInfo>, Box<dyn std::error::Error>> {
    let url = "http://172.20.30.1/drcom/chkstatus?callback=";
    
    let client = reqwest::Client::new();
    let response = client.get(url).timeout(std::time::Duration::from_secs(5)).send().await?;
    
    let content = response.text().await?;
    let content = content.trim();
    
    // Remove outer parentheses if present
    let content = if content.starts_with('(') && content.ends_with(')') {
        &content[1..content.len() - 1]
    } else {
        content
    };

    match serde_json::from_str::<DrcomInfo>(content) {
        Ok(data) => Ok(Some(data)),
        Err(e) => {
            eprintln!("Failed to parse drcom info: {}", e);
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_drcom_info() {
        match get_drcom_info().await {
            Ok(info) => println!("Info: {:?}", info),
            Err(e) => println!("Error: {}", e),
        }
    }
}
