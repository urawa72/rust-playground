use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sesv2::model::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::{Client, Endpoint, Error};

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

    send_message(
        &client,
        "sendor@example.com",
        "receiver@example.com",
        "TestSubject",
        "TestBody",
    )
    .await?;

    let resp = client
        .list_contacts()
        .contact_list_name("receiver@example.com")
        .send()
        .await?;
    println!("{:?}", resp);

    Ok(())
}

async fn send_message(
    client: &Client,
    from: &str,
    to: &str,
    subject: &str,
    message: &str,
) -> Result<(), Error> {
    let dest = Destination::builder().to_addresses(to).build();
    let subject_content = Content::builder().data(subject).charset("UTF-8").build();
    let body_content = Content::builder().data(message).charset("UTF-8").build();
    let body = Body::builder().text(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    let resp = client
        .send_email()
        .from_email_address(from)
        .destination(dest)
        .content(email_content)
        .send()
        .await?;

    println!("{:?}", resp);
    Ok(())
}
