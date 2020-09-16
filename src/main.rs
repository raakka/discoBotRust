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
    }
};

use serenity ::prelude::*;
use tokio::sync::Mutex;

extern crate futures;
extern crate futures_state_stream;
extern crate tokio_core;
extern crate tokio_postgres;

use futures::Future;
use futures_state_stream::StateStream;
use tokio_core::reactor::Core;
//this thing has a stupid overlap with 'Client' so just use postgres::Client for db client
use tokio_postgres::{NoTls, Error};
use std::collections::HashMap;

struct Handler;

//////////////////////////////////////////////////////////////////////////////////////////////////
//POSTGRES STUFF
struct Song{
	link: String,
	votes: i32
}

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
		println!("{:?} connected!", ready.user.name);
	}
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}

#[group]
#[prefixes ("suggest", "rec", "recommend")]
#[description = "Commands for song suggestions: top, rand, add, remove"]
#[commands(top, rand, add, remove, vote)]
struct Sug;

#[tokio::main]
async fn main() {
	let connection = tokio_postgres::connect("host=localhost user=postgres", NoTls).await;
	//setup function
	//dont show this on live idiot
	//"NzU0NDgxNTY5MDc0NTExOTkz.X11Xtw.iIn-yE6d8686yuyVR5r1-CBSWnk"
	let token = "";
	
	let framework = StandardFramework::new()
		.configure(|c| c
			.with_whitespace(true)
			.prefix("%"))
		.normal_message(normal_message)
		.group(&SUG_GROUP);

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

// all these commands need postresql support

#[command]
async fn top(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Err(whyfail) = msg.channel_id.say(&ctx.http, "https://open.spotify.com/track/5nIu0VwPOsjkF61zfevLKh?si=JPbxvSDpRJio9CxZZ-X-ZA").await {
		println!("Error sending message: {:?}", whyfail);
	}
	
	Ok(())
}

#[command]
async fn rand(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Err(whyfail) = msg.channel_id.say(&ctx.http, "https://open.spotify.com/track/5nIu0VwPOsjkF61zfevLKh?si=JPbxvSDpRJio9CxZZ-X-ZA").await {
		println!("Error sending message: {:?}", whyfail);
	}
	
	Ok(())
}

#[command]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Err(whyfail) = msg.channel_id.say(&ctx.http, "Added null to postgresql database!").await {
		println!("Error sending message: {:?}", whyfail);
	}
	
	Ok(())
}

#[command]
async fn remove(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Err(whyfail) = msg.channel_id.say(&ctx.http, "Removed null from postgresql database!").await {
		println!("Error sending message: {:?}", whyfail);
	}
	
	Ok(())
}

#[command]
async fn vote(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if let Err(whyfail) = msg.channel_id.say(&ctx.http, "+1 Vote to This Song").await {
		println!("Error sending message: {:?}", whyfail);
	}
	
	Ok(())
}

