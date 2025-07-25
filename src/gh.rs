use octocrab::Octocrab;
use octocrab::models::issues::Comment;
use tracing::info;

use crate::Ctx;
use crate::prelude::*;

pub async fn get_latest_master_commit(ctx: &Ctx, octocrab: &Octocrab) -> Result<String> {
    info!("Fetching latest master commit");
    let commits = octocrab
        .repos(&ctx.gh_owner, &ctx.gh_repo)
        .list_commits()
        .sha("master")
        .per_page(1)
        .send()
        .await?;

    let commit = commits
        .items
        .first()
        .ok_or(Error::FailedToFetchMasterCommit)?;
    Ok(commit.sha.clone())
}
pub async fn get_pr_comment(
    ctx: &Ctx,
    octocrab: &Octocrab,
    pr_number: u64,
    signature: &str,
) -> Result<Option<Comment>> {
    info!("Fetching latest PR comment for #{pr_number}");
    let pr_comments = octocrab
        .issues(&ctx.gh_owner, &ctx.gh_repo)
        .list_comments(pr_number)
        .send()
        .await?;

    Ok(pr_comments.items.into_iter().find(|comment| {
        if let Some(body) = &comment.body {
            body.contains(signature)
        } else {
            false
        }
    }))
}
pub fn add_comment_signature(comment: String, signature: &str) -> String {
    format!("<!-- {} -->\n{}", signature, comment)
}
