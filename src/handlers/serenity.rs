pub struct SerenityHandler;
use serenity::{
    prelude::SerenityError,
    async_trait, 
    client::{EventHandler, Context},
    model::{prelude::*, interactions::application_command::ApplicationCommandInteraction},
};
use tracing::{info,error};

#[async_trait]
impl EventHandler for SerenityHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}#{}", ready.user.name, ready.user.discriminator);
        let activity = Activity::listening("/help");
        ctx.set_activity(activity).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {
            if let Err(err) = self.run_command(&ctx, &mut command).await {
                error!("Error: {}", err)
                // self.handle_error(&ctx, &mut command, err).await
            }
        }
    }

}

impl SerenityHandler {
    async fn run_command(
        &self,
        ctx: &Context,
        command: &mut ApplicationCommandInteraction,
    ) -> Result<(), SerenityError> {
        let command_name = command.data.name.as_str();

        let guild_id = command.guild_id.unwrap();
        let guild = ctx.cache.guild(guild_id).await.unwrap();

        // get songbird voice client
        let manager = songbird::get(ctx).await.unwrap();

        // might have been disconnected manually
        if let Some(call) = manager.get(guild.id) {
            let mut handler = call.lock().await;
            if handler.current_connection().is_none() {
                handler.leave().await.unwrap();
            }
        }

        // fetch the user and the bot's user IDs
        let user_id = command.user.id;
        let bot_id = ctx.cache.current_user_id().await;


        match command_name {
            "ping" => command.create_interaction_response(&ctx, |f|{
                f.interaction_response_data(|r|{
                    r.content("reeeeeee")
                })
            }).await,
            
            _ => unreachable!(),
        }
    }
}
