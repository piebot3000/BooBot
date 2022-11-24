use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fs;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

const HELPMSG: &str = "Help:
!booba - Add 1 to the booba count
!boobacount - Show the current count
!boobareset - Reset the counter
!boobasave [count] - Set the counter to a specific value
!help - This msg";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut message = msg.content.split_whitespace();
        let command = if let Some(i) = message.next() { i } else { return };

        match command {
           "!booba" => {
                let count = {
                    let data_read = ctx.data.read().await;
                    data_read.get::<BoobaCount>().expect("Expected Boobacount in typemap").clone()
                };

                count.fetch_add(1, Ordering::SeqCst);

                if let Err(why) = msg.channel_id.say(&ctx.http, "Booba Counted.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!boobacount" => {
                let count = {
                    let data_read = ctx.data.read().await;
                    data_read.get::<BoobaCount>().expect("Expected Boobacount in typemap").clone()
                };

                let count = count.load(Ordering::Relaxed);
                let countstr = count.to_string();
                println!("Countstr = {}", countstr);
                let msg_content;
                if countstr.contains("420") || countstr.contains("69") {
                    msg_content = format!("There have been {} Booba. Haha funny number.", count);
                } else {
                    msg_content = format!("There have been {} Booba.", count);
                }

                if let Err(why) = msg.channel_id.say(&ctx.http, msg_content).await {
                    println!("Error sending message: {:?}", why);
                }

            }
            "!boobareset" => {
                let count = {
                    let data_read = ctx.data.read().await;
                    data_read.get::<BoobaCount>().expect("Expected Boobacount in typemap").clone()
                };

                count.store(0, Ordering::Relaxed);

                if let Err(why) = msg.channel_id.say(&ctx.http, "Count reset.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!boobasave" => {
                let count = {
                    let data_read = ctx.data.read().await;
                    data_read.get::<BoobaCount>().expect("Expected Boobacount in typemap").clone()
                };

                let new_count = if let Some(i) = message.next() { 
                    if let Ok(x) = i.parse::<usize>() {
                        x
                    } else {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "The count needs to be a number.").await {
                            println!("Error sending message: {:?}", why);
                        }
                        return;
                    }
                } else {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Need a count to set to.").await {
                        println!("Error sending message: {:?}", why);
                    }
                    return;
                };

                count.store(new_count, Ordering::Relaxed);

                let msg_content = format!("Count has been set to {}.", new_count);

                if let Err(why) = msg.channel_id.say(&ctx.http, msg_content).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!help" => {

                if let Err(why) = msg.channel_id.say(&ctx.http, HELPMSG).await {
                    println!("Error sending message: {:?}", why);
                }
            }

            _ => {}
        }
    }

    // When the bot is connected, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// A key for the type map that contains our counter.
struct BoobaCount;
impl TypeMapKey for BoobaCount {
    type Value = Arc<AtomicUsize>;
}   

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token read from the file secret.txt.
    let token = fs::read_to_string("secret.txt").expect("failed to read secret");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // initialize the data for the booba counter.
    {
        let mut data = client.data.write().await;

        data.insert::<BoobaCount>(Arc::new(AtomicUsize::new(0)));
    }

    // Finally, start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

