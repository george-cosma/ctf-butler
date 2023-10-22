use settings::UserSettings;

mod commands;
mod settings;

type Context<'a> = poise::Context<'a, State, anyhow::Error>;
pub struct State {}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let settings = UserSettings::load(settings::SETTINGS_PATH)?;

    let options: poise::FrameworkOptions<State, anyhow::Error> = poise::FrameworkOptions {
        commands: commands::get_commands(),
        ..Default::default()
    };

    poise::Framework::builder()
        .token(settings.token())
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(State {})
            })
        })
        .options(options)
        .intents(poise::serenity_prelude::GatewayIntents::non_privileged())
        .run()
        .await?;

    Ok(())
}
