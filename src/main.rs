use serenity::{
    Client,
    all::{
        Command, Context, CreateCommand, CreateInteractionResponse,
        CreateInteractionResponseMessage, EventHandler, GatewayIntents, Interaction, Ready,
    },
    async_trait,
};
use zr_app::config_builder;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            dbg!(&command);
            let content = match command.data.name.as_str() {
                "ping" => "Pong !".to_string(),
                _ => "Commande non reconnue.".to_string(),
            };

            if let Err(why) = command
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(content),
                    ),
                )
                .await
            {
                println!("Erreur lors de la réponse à la slash command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} est connecté !", ready.user.name);
        ctx.dnd();

        // Créer les slash commands globalement
        let commands = Command::set_global_commands(
            &ctx.http,
            vec![CreateCommand::new("ping").description("Répond avec Pong!")],
        )
        .await;

        match commands {
            Ok(commands) => {
                println!("{} slash commands créées !", commands.len());
            }
            Err(why) => {
                println!("Erreur lors de la création des slash commands: {why}");
            }
        }
    }
}

config_builder! {
    Settings {
        bot: Bot {
            token: String,
        }
    }
}

#[tokio::main]
#[zr_app::app(conf = Settings, app_folder = "./settings")]
async fn main() {
    let token = config.bot.token;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Erreur lors de la création du client");

    if let Err(why) = client.start().await {
        println!("Erreur du client: {why:?}");
    }
}
