use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let response = client.post("http://localhost:3000/users")
        .header("Content-Type", "application/json")
        .body(
            "{
                \"email\": \"test@test.com\",
                \"password\": \"1234\",
                \"confirm_password\": \"1234\",
                \"is_admin\": true
            }",
        )
        .send()
        .await?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}