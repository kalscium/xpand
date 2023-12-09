use std::{io, path::Path};

use base64::Engine;
use serenity::{model::prelude::*, async_trait, client::{Context, EventHandler}, Client, builder::{CreateAttachment, CreateMessage}};
use crate::{secrets, crypto};

pub struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Bot is ready");
        crate::segment::upload_segments(2, 1182850912767713310u64, &ctx, "test_tmp/segment").await.unwrap();
        std::process::exit(0);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // msg.channel_id.send_files(&ctx.http, );
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&ctx.http, "Pong!").await;
        } else if msg.content.starts_with("!echo ") {
            let _ = msg.channel_id.say(&ctx.http, format!("> {}", msg.content.strip_prefix("!echo ").unwrap_or("<echo error>"))).await;
        }
    }
}

impl Bot {
    pub async fn run() {
        let mut client = Client::builder(secrets::TOKEN.trim(), GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT)
            .event_handler(Bot)
            .application_id(secrets::APP_ID)
            .await
            .expect("Error creating client");
        client.start().await.unwrap();
    }

    /// Uploads a file to the server
    #[inline]
    pub async fn upload_file(file_path: impl AsRef<Path>, channel_id: u64, ctx: &Context) -> Result<u64, io::Error> {
        let channel_id = ChannelId::from(channel_id);
        let hash = crypto::hash_file(&file_path)?;
        let attachment = CreateAttachment::path(file_path).await.expect("attachment creation failed");
        let message = CreateMessage::default()
            .content(base64::engine::general_purpose::URL_SAFE.encode(hash));

        let message = channel_id.send_files(&ctx.http, [attachment], message)
            .await
            .expect("Error uploading file");
        Ok(message.id.into())
    }
}