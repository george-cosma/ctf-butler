use crate::Context;

/// Ping the bot!
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    let response = format!("Pong! Uptime: {}", "Unknown");
    ctx.say(response).await?;
    Ok(())
}
