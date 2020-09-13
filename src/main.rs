use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{
	channel::Message,
	gateway::Ready
};
use serenity::framework::standard::{
	StandardFramework,
	CommandResult,
	macros::{
		command,
		group
	}
};

//use std::env; 

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "Hello XXIII" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Yo!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

	/*let framework = StandardFramework::new()
		.configure(|c| c
			.with_whitespace(false)
			.prefix("m!"))
		.group(&GENERAL_GROUP);*/
	//wtf commands???

	let token = "need to set this as std::env::var";
	let mut client = Client::new(token)
		.event_handler(Handler)
		//.framework(framework)
		.await
		.expect("Err creating client");

	//we are so good at handling errors yay!	
	if let Err(why) = client.start().await {
		println!("{:?}", why);
	}
}

