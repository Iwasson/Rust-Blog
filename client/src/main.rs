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

    // let response = client.post("http://localhost:3000/post_blog")
    // .header("Content-Type", "application/x-www-form-urlencoded")
    //     .body(
    //         "{
    //             \"title\": \"Test Blog\",
    //             \"email\": \"test@test.com\",
    //             \"content\": \"#Test h1 \\ ##Test h2\",
    //             \"publish_date\": \"12/3/23\"
    //         }",
    //     )
    //     .send()
    //     .await?;


    // let body = response.text().await?;
    // println!("{}", body);

    Ok(())
}