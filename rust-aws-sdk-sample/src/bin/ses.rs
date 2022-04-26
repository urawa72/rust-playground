use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ses::{
    model::{Body, Content, Destination, Message},
    Client, Endpoint, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env()
        .region(provider)
        .endpoint_resolver(Endpoint::immutable(
            "http://localhost:4566".parse().expect("valid URI"),
        ))
        .load()
        .await;
    let client = Client::new(&config);

    verify_identity(&client).await?;
    send_email(&client).await?;

    Ok(())
}

async fn verify_identity(client: &Client) -> Result<(), Error> {
    let resp = client.verify_email_identity().email_address("sendor@example.com").send().await?;
    println!("{:?}", resp);
    Ok(())
}

async fn send_email(client: &Client) -> Result<(), Error> {
    let dest = Destination::builder()
        .to_addresses("sample@example.com")
        .build();
    let subject_content = Content::builder()
        .data("Test Subject")
        .charset("UTF-8")
        .build();
    let body_content = Content::builder()
        .data("Body content")
        .charset("UTF-8")
        .build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(Body::builder().text(body_content).build())
        .build();

    let resp = client
        .send_email()
        .source("sendor@example.com")
        .destination(dest)
        .message(msg)
        .send()
        .await?;

    println!("{:?}", resp);
    Ok(())
}
