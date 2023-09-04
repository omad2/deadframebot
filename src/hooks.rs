use serenity::{
    client::Context,
    framework::standard::{macros::hook, DispatchError},
    model::channel::Message,
};

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::LackingRole = error {
        let author_id = msg.author.id;

        msg.channel_id
            .say(
                &ctx,
                format!(
                    "<@{}> You need to be a `Kasaki Member` to use this command.",
                    &author_id
                ),
            )
            .await
            .unwrap();
    }
}
