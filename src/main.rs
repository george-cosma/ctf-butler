use settings::UserSettings;
use state::State;

mod commands;
mod settings;
mod state;

type Context<'a> = poise::Context<'a, State, anyhow::Error>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let settings = UserSettings::load(settings::SETTINGS_PATH)?;
    let state = State::new().await?;
    let db_ref = state.db_ref();

    let options: poise::FrameworkOptions<State, anyhow::Error> = poise::FrameworkOptions {
        commands: commands::get_commands(),
        ..Default::default()
    };

    let runner = poise::Framework::builder()
        .token(settings.token())
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(state)
            })
        })
        .options(options)
        .intents(poise::serenity_prelude::GatewayIntents::non_privileged())
        .run();

    let interrupt = tokio::signal::ctrl_c();

    let result: Result<(), anyhow::Error> = tokio::select! {
        ret = runner => {
            ret.map_err(|e| e.into())
        },
        ret = interrupt => {
            ret.map_err(|e| e.into())
        },
    };

    println!("Saving data...");
    db_ref.flush().await;
    println!("Saved data!");
    println!("Goodbye!");

    result
}
