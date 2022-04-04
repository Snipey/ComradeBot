use std::{collections::HashSet, env, sync::Arc, error::Error};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{StandardFramework},
    http::Http,
    prelude::*,
};
use songbird::SerenityInit;

use crate::handlers::SerenityHandler;

pub struct Client {
    client: serenity::client::Client,
}

pub struct ShardManagerContainer;


impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl Client {
    pub async fn default() -> Result<Client, Box<dyn Error>> {
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

        Client::new(&token).await
    }
    pub async fn new(token: &str) -> Result<Client, Box<dyn Error>> {
        let app = env::var("DISCORD_APP_ID").expect("Expected a application id in the environment")
            .parse::<u64>().expect("Expected a valid application id");
    
            let http = Http::new_with_token(&token);
            // We will fetch your bot's owners and id
            let (owners, _bot_id) = match http.get_current_application_info().await {
                Ok(info) => {
                    let mut owners = HashSet::new();
                    owners.insert(info.owner.id);
        
                    (owners, info.id)
                },
                Err(why) => panic!("Could not access application info: {:?}", why),
            };
    
        // Create the framework
        let framework =
            StandardFramework::new().configure(|c| c.owners(owners).prefix("~"));
    
            
        let client = serenity::Client::builder(&token)
            .application_id(app)
            .framework(framework)
            .event_handler(SerenityHandler)
            .register_songbird()
            .await
            .expect("Err creating client");
    
        {
            let mut data = client.data.write().await;
            data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        }
        Ok(Client { client})
    }
    
    pub async fn start(&mut self) -> Result<(), serenity::Error> {    
        let shard_manager = self.client.shard_manager.clone();
    
        tokio::spawn(async move {
            // tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
            shard_manager.lock().await.shutdown_all().await;
        });
    
        self.client.start().await
    }


}