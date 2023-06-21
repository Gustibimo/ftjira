
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response};
use base64;
use serde_json::Value;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Jira API URL and JQL query
    let url = "https://jurnal.atlassian.net/rest/api/latest/search";
    let jql = r#"project = "CLT" AND type = "Fast Track" AND status in("In Progress", "To Do", "NEED FEEDBACK") ORDER BY created DESC"#;

    dotenv().ok();
    // API token and email
    let api_token = std::env::var("JIRA_API_TOKEN").expect("JIRA_API_TOKEN not set");
    let email = "gusti.bimo@mekari.com";

    // Set the request headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    let auth_value = format!("{}:{}", email, api_token);
    let encoded_auth = base64::encode(auth_value);
    headers.insert(AUTHORIZATION, format!("Basic {}", encoded_auth).parse().unwrap());

    // Set the request parameters
    let params = [
        ("jql", jql),
        ("maxResults", "1000"),
    ];

    // Send the API request and retrieve the JSON response
    let client = Client::new();
    let response: Response = client
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await
        .expect("Failed to send request");

    let response_json: Value = response
        .json()
        .await
        .expect("Failed to parse JSON");

    // println!("{}", response_json["issues"][0]["fields"]["summary"]);

    // // Print the issue keys
    let issues = response_json["issues"].as_array().unwrap();
    for issue in issues {
        // println!("------ Issue Key ------");
        println!("{}/{}","https://jurnal.atlassian.net/browse",  issue["key"].as_str().unwrap());
        // println!("{}", issue["fields"]["description"]);
    }

    println!("Total issues: {}", issues.len());
}
