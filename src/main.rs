use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use std::time::Duration;

use also::Also;
use chrono::Local;
use eliza::Eliza;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;

use tui::ActiveChannel;
use tui::{display_err, render};

mod tui;

const DOCTOR_SCRIPT: &str = include_str!("doctor.json");

struct ElizaHandler {
    active_channels: Mutex<HashMap<u64, tui::ActiveChannel>>,
    conversations: Mutex<HashMap<u64, Eliza>>,
}

#[async_trait]
impl EventHandler for ElizaHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        // Render channel activity
        let channel_name = msg.channel_id.name(ctx.cache).await.unwrap_or_default();
        drop(
            self.active_channels
                .lock()
                .expect("locking active channels map")
                .also(|ac_map| {
                    let ac =
                        ac_map
                            .entry(*msg.channel_id.as_u64())
                            .or_insert_with(|| ActiveChannel {
                                name: String::new(),
                                last_activity: Local::now(),
                            });
                    ac.name = channel_name;
                    ac.last_activity = Local::now();
                    render(ac_map);
                }),
        );

        // Simulate reaction time and typing for more realism
        tokio::time::sleep(Duration::from_millis(3100)).await;
        if let Err(why) = msg.channel_id.broadcast_typing(&ctx.http).await {
            display_err("Error broadcasting typing", why);
        }
        tokio::time::sleep(Duration::from_millis(5000)).await;

        // Get Eliza instance for this channel and process the message
        let response = self
            .conversations
            .lock()
            .expect("locking conversations map")
            .entry(*msg.channel_id.as_u64())
            .or_insert_with(|| {
                Eliza::from_str(DOCTOR_SCRIPT).expect("Unable to parse doctor script")
            })
            .respond(&msg.content);

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            display_err("Error sending message", why);
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("ELIZA_DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(ElizaHandler {
            active_channels: Mutex::new(HashMap::new()),
            conversations: Mutex::new(HashMap::new()),
        })
        .await
        .expect("Error creating client");

    // Setup terminal
    ctrlc::set_handler(move || {
        // Restore terminal after ctrl-c too
        tui::restore();
        std::process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");
    tui::setup();

    let ret = client.start().await;

    // Restore terminal
    tui::restore();

    if let Err(why) = ret {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
