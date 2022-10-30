use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error occor when seding mesage {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is conneted", ready.user.name);
    }


#[tokio::main]
fn main() {
    println!("Hello, world!");
}
