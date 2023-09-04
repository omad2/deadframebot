mod commands;
mod hooks;
mod utils;

use commands::{general::GENERAL_GROUP, help::HELP, member::MEMBER_GROUP};
use hooks::dispatch_error;
use serenity::model::prelude::{Activity, Ready};
use serenity::{async_trait, framework::StandardFramework, prelude::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Connected as {}#{}",
            ready.user.name, ready.user.discriminator
        );

        ctx.set_activity(Activity::playing("k!help")).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found in .env file");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("k!"))
        .on_dispatch_error(dispatch_error)
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&MEMBER_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Unable to create client");

    if let Err(err) = client.start().await {
        println!("Client error: {err}");
    }
}
