use rusoto_core::{credential::ProfileProvider, HttpClient, Region};
use rusoto_ses::{
    Body, Content, Destination, ListIdentitiesRequest, Message, SendEmailRequest, Ses, SesClient,
    VerifyEmailIdentityRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // あらかじめ aws configure --profile localstack でプロファイル作っておく
    let mut provider = ProfileProvider::new().unwrap();
    provider.set_profile("localstack");
    let client = SesClient::new_with(
        HttpClient::new().unwrap(),
        provider,
        Region::Custom {
            name: "ap-northeast-1".into(),
            endpoint: "http://localhost:4566".into(),
        },
    );

    println!("### verify email identity");
    match client
        .verify_email_identity(VerifyEmailIdentityRequest {
            email_address: "sendor@example.com".into(),
        })
        .await
    {
        Ok(res) => {
            println!("Ok! {:?}", res)
        }
        Err(err) => {
            println!("Err! {:?}", err)
        }
    }

    println!("\n### list identities");
    match client
        .list_identities(ListIdentitiesRequest {
            ..Default::default()
        })
        .await
    {
        Ok(res) => println!("Ok! {:?}", res),
        Err(err) => println!("Err! {:?}", err),
    }

    println!("\n### send email");
    match client
        .send_email(SendEmailRequest {
            source: "sendor@example.com".into(),
            destination: Destination {
                to_addresses: Some(vec!["receiver@example.com".into()]),
                ..Default::default()
            },
            message: Message {
                subject: Content {
                    charset: Some("UTF-8".into()),
                    data: "メールを送るよ！ From rusoto".into(),
                },
                body: Body {
                    html: None,
                    text: Some(Content {
                        charset: Some("UTF-8".into()),
                        data: "メールの本文だよ！ From rusoto".into(),
                    }),
                },
            },
            ..Default::default()
        })
        .await
    {
        Ok(res) => {
            println!("Ok! {:?}", res);
        }
        Err(err) => {
            println!("Err! {:?}", err);
        }
    }

    Ok(())
}
