use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hey there! 👋

Neebo here at your service.

❓ Need technical help?
➡️ Post in the <#858281434699923467> channel and someone will be able to assist you.

❓ Looking for the Code of Conduct?
➡️ Here it is: <https://bynarr.com/resources/code-of-conduct>

❓ Something wrong?
➡️ You can flag an admin with @admin

I hope that helps and resolves your issue.

— Neebo 😊
";

const HELP_COMMAND: &str = "?help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(e) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
        std::process::exit(1);
    }
}
