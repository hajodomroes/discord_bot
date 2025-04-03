mod commands;

use poise::FrameworkError;
use serenity::prelude::*;
use std::collections::HashMap;
use std::env;

type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;
pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
}
impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Data").field("votes", &self.votes).finish()
    }
}
async fn on_error(error: FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::Setup { error, .. } => panic!("Error setting up framework: {error:?}"),
        FrameworkError::Command { error, ctx, .. } => println!(
            "Error executing command: {error:?} with context: {:?}",
            ctx.command().name
        ),
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error handling error: {e:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        commands: vec![commands::help::help()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".to_string()),
            additional_prefixes: vec![
                poise::Prefix::Literal("/"),
                poise::Prefix::Literal("-"),
            ],
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        skip_checks_for_owners: true,
        ..Default::default()
    };
    
    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move { 
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .build();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Err creating client");
    client.start().await.expect("Error starting client");
}
