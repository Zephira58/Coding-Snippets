#![allow(unused_must_use)]
#![allow(clippy::too_many_arguments)]
use self_update::cargo_crate_version;
use std::collections::HashMap;

pub fn get_insult() -> String {
    println!("\nFetching insult from https://insult.mattbas.org/api/insult...");
    //View more details on how this works here (https://www.reddit.com/r/learnrust/comments/wz9flc/how_to_get_request_to_return_as_a_string/)
    let x = reqwest::blocking::get("https://insult.mattbas.org/api/insult")
        .expect("Get failed")
        .text()
        .expect("Couldn't get response body");
    println!("Insult fetched!");
    x
}

pub fn get_affirmation() -> String {
    println!("\nFetching affirmation from https://www.affirmations.dev/...");
    let x = reqwest::blocking::get("https://www.affirmations.dev/")
        .expect("Get failed")
        .text()
        .expect("Couldn't get response body")
        .replace('{', "")
        .replace('}', "")
        .replace(':', "")
        .replace("affirmation", "")
        .replace('"', "");
    println!("Affirmation fetched!");
    x
}

use webhook::client::{WebhookClient, WebhookResult};

#[tokio::main]
pub async fn send_message(msg: &str, webhook: &str, username: &str, avatar_url: &str) {
    println!("\nSending message...");
    let message = msg.to_owned();
    let message = message.as_str();

    let mut request_body = HashMap::new();
    request_body.insert("content", message);
    request_body.insert("username", username);
    request_body.insert("avatar_url", avatar_url);

    reqwest::Client::new()
        .post(webhook)
        .json(&request_body)
        .send()
        .await;
    println!("Message sent!");
}
#[tokio::main]
pub async fn send_embed(
    msg: &str,
    webhook: &str,
    username: &str,
    avatar_url: &str,
    embed_title: &str,
    embed_footer: &str,
    embed_footer_icon: &str,
    embed_image: &str,
    embed_thumbnail: &str,
    embed_field_title: &str,
    embed_field_value: &str,
) -> WebhookResult<()> {
    let client = WebhookClient::new(webhook);
    let webhook_info = client.get_information().await?;
    println!("\nwebhook: {:?}", webhook_info);
    println!("\nSending embed...");
    client
        .send(|message| {
            message
                .content("")
                .username(username)
                .avatar_url(avatar_url)
                .embed(|embed| {
                    embed
                        .title(embed_title)
                        .description(msg)
                        .footer(embed_footer, Some(String::from(embed_footer_icon)))
                        .image(embed_image)
                        .thumbnail(embed_thumbnail)
                        .author(
                            username,
                            Some(String::from(avatar_url)),
                            Some(String::from(avatar_url)),
                        )
                        .field(embed_field_title, embed_field_value, false)
                })
        })
        .await?;
    Ok(())
}

pub fn download_update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("Xanthus58")
        .repo_name("webhook_sender")
        .bin_name("github")
        .show_download_progress(true)
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
