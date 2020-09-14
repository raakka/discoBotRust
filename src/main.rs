use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CheckResult, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, StandardFramework,
        macros::{command, group, help, check, hook},
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    utils::{content_safe, ContentSafeOptions},
};

use serenity ::prelude::*;
use tokio::sync::Mutex;

struct Handler;

//////////////////////////////////////////////////////////////////////////////////////////////////
//This is the best shit ever we have a cool way of setting a responder yay :D
//This is the event handler thingy that we can set other events in
//not really sure abt commands tho seems weird...

#[async_trait]
impl EventHandler for Handler {

	async fn message(&self, ctx: Context, msg: Message) { 
		if msg.content == "My name is Zed" {
			if let Err(whyfail) = msg.channel_id.say(&ctx.http, "fuckoff").await {
				print!("Exploded because: {:?}", whyfail);
			}
		}
	}

	async fn ready(&self, _: Context, ready: Ready) {
		print!("{:?} connected!", ready.user.name);
	}
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}

#[group]
#[commands(r)]
struct General;

#[tokio::main]
async fn main() {
	//setup function
	let token = "NzU0NDgxNTY5MDc0NTExOTkz.X11Xtw.iIn-yE6d8686yuyVR5r1-CBSWnk";
	
	let framework = StandardFramework::new()
		.configure(|c| c
			.with_whitespace(true)
			.prefix("%") 
		.normal_message(normal_message)
		.group(&GENERAL_GROUP);

	let mut client = Client::new(&token)
		//only need one evnt handler
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("client err");
	
	if let Err(whyfail) = client.start().await {
		print!("whoops this happened {:?}", whyfail);
	}

}

#[command]
async fn r(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Err(whyfail) = msg.channel_id.say(&ctx.http, "This should make a spotify recomendation").await {
		println!("Error sending message: {:?}", whyfail);
	}
	
	Ok(())
}
