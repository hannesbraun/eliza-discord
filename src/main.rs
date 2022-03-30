use eliza::Eliza;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use std::time::Duration;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::guild::GuildStatus;

const DOCTOR_SCRIPT: &str = include_str!("doctor.json");

struct ElizaHandler {
    conversations: Mutex<HashMap<u64, Eliza>>,
}

#[async_trait]
impl EventHandler for ElizaHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        // Simulate reaction time and typing for more realism
        tokio::time::sleep(Duration::from_millis(3100)).await;
        msg.channel_id.broadcast_typing(&ctx.http).await;
        tokio::time::sleep(Duration::from_millis(5000)).await;

        // Get Eliza instance for this channel and process the message
        let response = self
            .conversations
            .lock()
            .expect("locking conversations map")
            .entry(*msg.channel_id.as_u64())
            .or_insert(Eliza::from_str(DOCTOR_SCRIPT).expect("Unable to parse doctor script"))
            .respond(&msg.content);

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "{} is connected with the following guilds:",
            ready.user.name
        );
        for guild in ready.guilds {
            match guild {
                GuildStatus::OnlinePartialGuild(opg) => {
                    println!("{}", opg.name)
                }
                GuildStatus::OnlineGuild(og) => {
                    println!("{}", og.name)
                }
                GuildStatus::Offline(o) => {
                    println!("{} (unavailable)", o.id)
                }
                _ => {
                    eprintln!("Unexpected guild status")
                }
            }
        }
        println!();
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("ELIZA_DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(ElizaHandler {
            conversations: Mutex::new(HashMap::new()),
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
