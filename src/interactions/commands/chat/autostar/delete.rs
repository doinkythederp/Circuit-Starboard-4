use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::AutoStarChannel, get_guild_id, interactions::context::CommandCtx, unwrap_id,
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "delete", desc = "Delete an autostar channel.")]
pub struct DeleteAutoStarChannel {
    /// The name of the autostar channel to delete.
    #[command(autocomplete = true)]
    name: String,
}

impl DeleteAutoStarChannel {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);
        let ret = AutoStarChannel::delete(&ctx.bot.pool, &self.name, unwrap_id!(guild_id)).await?;
        if ret.is_none() {
            ctx.respond_str("No autostar channel with that name was found.", true)
                .await?;
        } else {
            ctx.bot
                .cache
                .guild_autostar_channel_names
                .remove(&guild_id)
                .await;
            ctx.respond_str(&format!("Deleted autostar channel '{}'.", self.name), false)
                .await?;
        }
        Ok(())
    }
}
