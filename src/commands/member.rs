use crate::utils::{check_filetype, execute_ffmpeg_cmd, parse_filetype};
use serenity::framework::standard::macros::group;
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::http::AttachmentType;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::borrow::Cow;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

#[group("Member Commands")]
#[commands(deadframe, convert)]
#[only_in(guild)]
#[allowed_roles("Kasaki | Member")]
pub struct Member;

#[command]
#[description("Removes duplicated frames from a video clip.")]
#[aliases(df)]
async fn deadframe(ctx: &Context, msg: &Message) -> CommandResult {
    let author_id = msg.author.id;

    if msg.attachments.is_empty() {
        msg.channel_id
            .say(
                &ctx,
                format!("<@{}> You must attach a video clip to use the deadframe command.\nFile Types supported: MKV, AVI, MP4, MOV.", &author_id)
            )
            .await
            .unwrap();

        return Ok(());
    }

    let attachment: &Attachment = &msg.attachments[0];
    let content_type: &str = attachment.content_type.as_ref().unwrap();
    let file_type = parse_filetype(content_type);

    if !content_type.starts_with("video") && file_type.is_err() {
        msg.channel_id
            .say(
                &ctx,
                format!("<@{}> You must attach a video clip to use the deadframe command.\nFile Types supported: MKV, AVI, MP4, MOV.", &author_id)
            )
            .await
            .unwrap();

        return Ok(());
    } else if attachment.size > 8000000 {
        // This check is caused by the discord API where
        // files that are uploaded must be less than 8MB.

        msg.channel_id
            .say(
                &ctx,
                format!(
                    "<@{}> The video clip must be less or equal to 8MB.",
                    &author_id
                ),
            )
            .await
            .unwrap();

        return Ok(());
    }

    let file_type = file_type.unwrap();
    let content = match attachment.download().await {
        Ok(content) => content,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx,
                    format!("<@{}> An error has occured, please try again.", &author_id),
                )
                .await
                .unwrap();

            return Ok(());
        }
    };

    let timestamp = chrono::Utc::now().timestamp();
    let original_file_name = format!("org_{}_{}.{}", author_id, timestamp, file_type);

    let mut file = match File::create(&original_file_name).await {
        Ok(file) => file,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx,
                    format!("<@{}> An error has occured, please try again.", &author_id),
                )
                .await
                .unwrap();

            return Ok(());
        }
    };

    if file.write_all(&content).await.is_err() {
        msg.channel_id
            .say(
                &ctx,
                format!("<@{}> An error has occured, please try again.", &author_id),
            )
            .await
            .unwrap();

        return Ok(());
    }

    let final_file_name = format!("res_{}_{}.{}", author_id, timestamp, file_type);
    let default_args = vec![
        "-i",
        &original_file_name,
        "-vf",
        "mpdecimate,setpts=N/FRAME_RATE/TB",
        "-an",
        &final_file_name,
    ];

    execute_ffmpeg_cmd(default_args);
    fs::remove_file(original_file_name).await.unwrap();

    let file_contents = fs::read(&final_file_name).await.unwrap();
    fs::remove_file(final_file_name).await.unwrap();

    msg.channel_id.send_message(&ctx, |m| {
            m.content(format!("<@{}>", &author_id));
            m.add_file(AttachmentType::Bytes {
                data: Cow::Owned(file_contents),
                filename: format!("result.{}", file_type),
            })
        })
        .await
        .unwrap();

    Ok(())
}

#[command]
#[description("Converts a video clip to another file type.")]
#[usage("<mkv, avi, mp4, mov>")]
#[example("mov")]
#[min_args(1)]
async fn convert(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let author_id = msg.author.id;
    let convert_file_type = args.single::<String>();

    if convert_file_type.is_err() {
        msg.channel_id
            .say(
                &ctx,
                format!("<@{}> You must provide a file type to use the convert command.\nFile Types supported: MKV, AVI, MP4, MOV.", &author_id)
            )
            .await
            .unwrap();

        return Ok(());
    }

    let convert_file_type = convert_file_type.unwrap().to_lowercase();

    if check_filetype(convert_file_type.as_str()).is_err() {
        msg.channel_id
            .say(
                &ctx,
                format!("<@{}> You must provide a correct file type to use the convert command.\nFile Types supported: MKV, AVI, MP4, MOV.", &author_id)
            )
            .await
            .unwrap();

        return Ok(());
    }

    if msg.attachments.is_empty() {
        msg.channel_id
            .say(
                &ctx,
                format!("<@{}> You must provide a video clip to use the convert command.\nFile Types supported: MKV, AVI, MP4, MOV.", &author_id)
            )
            .await
            .unwrap();

        return Ok(());
    }

    let attachment: &Attachment = &msg.attachments[0];
    let content_type: &str = attachment.content_type.as_ref().unwrap();
    let file_type = parse_filetype(content_type);

    if !content_type.starts_with("video") && file_type.is_err() {
        msg.channel_id
            .say(
                &ctx,
                format!("<@{}> You must provide a file type to use the convert command.\nFile Types supported: MKV, AVI, MP4, MOV.", &author_id)
            )
            .await
            .unwrap();

        return Ok(());
    } else if attachment.size > 8000000 {
        // This check is caused by the discord API where
        // files that are uploaded must be less than 8MB.

        msg.channel_id
            .say(
                &ctx,
                format!(
                    "<@{}> The video clip must be less or equal to 8MB.",
                    &author_id
                ),
            )
            .await
            .unwrap();

        return Ok(());
    }

    let file_type = file_type.unwrap();

    if file_type == convert_file_type {
        msg.channel_id
            .say(
                &ctx,
                format!("<@{}> You can't convert to the same file type.", &author_id),
            )
            .await
            .unwrap();

        return Ok(());
    }

    let content = match attachment.download().await {
        Ok(content) => content,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx,
                    format!("<@{}> An error has occured, please try again.", &author_id),
                )
                .await
                .unwrap();

            return Ok(());
        }
    };

    let timestamp = chrono::Utc::now().timestamp();
    let original_file_name = format!("org_{}_{}.{}", author_id, timestamp, file_type);

    let mut file = match File::create(&original_file_name).await {
        Ok(file) => file,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx,
                    format!("<@{}> An error has occured, please try again.", &author_id),
                )
                .await
                .unwrap();

            return Ok(());
        }
    };

    if file.write_all(&content).await.is_err() {
        msg.channel_id
            .say(
                &ctx,
                format!("<@{}> An error has occured, please try again.", &author_id),
            )
            .await
            .unwrap();

        return Ok(());
    }

    let final_file_name = format!("res_{}_{}.{}", author_id, timestamp, convert_file_type);
    let default_args = vec!["-i", &original_file_name, "-an", &final_file_name];

    execute_ffmpeg_cmd(default_args);
    fs::remove_file(original_file_name).await.unwrap();

    let file_contents = fs::read(&final_file_name).await.unwrap();
    fs::remove_file(final_file_name).await.unwrap();

    msg.channel_id.send_message(&ctx, |m| {
            m.content(format!("<@{}>", &author_id));
            m.add_file(AttachmentType::Bytes {
                data: Cow::Owned(file_contents),
                filename: format!("result.{}", convert_file_type),
            })
        })
        .await
        .unwrap();

    Ok(())
}
