use crate::Context;

/// play a song by providing the name
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn play(
    ctx: Context<'_>,
    // #[description = "The song to play"]
    #[rest]
    song: String,
) -> Result<(), crate::Error> {
    ctx.say(format!("Playing {song}")).await?;
    Ok(())
}