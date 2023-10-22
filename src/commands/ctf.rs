use crate::Context;

#[poise::command(slash_command, subcommands("list", "add", "remove"))]
pub async fn ctf(_ctx: Context<'_>) -> Result<(), anyhow::Error> {
    anyhow::bail!("This command should never be called directly")
}

#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    ctx.say(format!("Test value is: {}", ctx.data().db().await.test))
        .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn add(ctx: Context<'_>, name: String) -> Result<(), anyhow::Error> {
    ctx.data().db_mut().await.test = name.clone();
    ctx.say(format!("Set test value to {}", name)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn remove(_ctx: Context<'_>) -> Result<(), anyhow::Error> {
    anyhow::bail!("Not yet implemented!")
}
