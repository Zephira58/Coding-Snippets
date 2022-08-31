use webhook::client::{WebhookClient, WebhookResult};

const IMAGE_URL: &'static str = "https://cdn.discordapp.com/avatars/292971545956188160/eab559efa07f0f3dd13d21ac5f26c4ce.png?size=1024";

#[tokio::main]
async fn main() -> WebhookResult<()> {
    let url = "https://canary.discord.com/api/webhooks/1014531600547074070/IAmOcsyW0QjAAsM6gKdVNrjgOxLoUZAjakURSjWjbuuC61GlJDL5cDxgobjP-kqTDosH";
    let client = WebhookClient::new(&url);
    let webhook_info = client.get_information().await?;
    println!("webhook: {:?}", webhook_info);

    client.send(|message| message
        .content("@everyone Hello World!")
        .username("Xanthus")
        .avatar_url(IMAGE_URL)
        .embed(|embed| embed
            .title("Webhook")
            .description("Hello, World!")
            .footer("", Some(String::from(IMAGE_URL)))
            .image(IMAGE_URL)
            .thumbnail(IMAGE_URL)
            .author("Lmao#0001", Some(String::from(IMAGE_URL)), Some(String::from(IMAGE_URL)))
            .field("name", "Xanthus", false))).await?;

    Ok(())
}