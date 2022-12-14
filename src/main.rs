use std::env;
use std::fs::File;

use dotenvy::dotenv;

use serenity::model::gateway::GatewayIntents;
use serenity::Client;

mod events;
use events::Events;

mod commands;
mod database;
mod feed;
mod logger;
mod structs;

#[macro_use(info, error)]
extern crate tracing;

#[tokio::main]
async fn main() {
    dotenv().ok();

    logger::set_logger().unwrap();

    let token = env::var("BOT_TOKEN").expect("Expected BOT_TOKEN environment variable.");
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_WEBHOOKS;

    let mut client = Client::builder(token, intents)
        .event_handler(Events)
        .await
        .expect("Could not build client.");

    {
        use structs::shard_manager::ShardManagerContainer;

        let mut client_data = client.data.write().await;
        client_data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let database_file_path =
        env::var("DATABASE_FILE_PATH").unwrap_or_else(|_| "./data/database.json".to_string());
    if File::open(&database_file_path).is_err() {
        if let Err(why) = database::new(database_file_path) {
            error!("Error occurred when creating database file. {why}");
        }
    }

    if let Err(why) = client.start().await {
        error!("Error occurred when starting the client. {why}");
    }
}
