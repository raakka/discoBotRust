use serenity::{
	async_trait,
	model::{channel::Message, gateway::Ready},
	prelude::*
};

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

#[tokio::main]
async fn main() {
	//setup function
	let token = "NzU0NDgxNTY5MDc0NTExOTkz.X11Xtw.iIn-yE6d8686yuyVR5r1-CBSWnk";
	
	let mut client = Client::new(&token)
		//only need one evnt handler
		.event_handler(Handler)
		.await
		.expect("client err");
	
	if let Err(whyfail) = client.start().await {
		print!("whoops this happened {:?}", whyfail);
	}

}
