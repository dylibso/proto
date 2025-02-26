use clap::Args;
use proto_core::{load_tool, Id, UnresolvedVersionSpec};
use starbase::system;
use std::process;
use tracing::debug;

#[derive(Args, Clone, Debug)]
pub struct ListRemoteArgs {
    #[arg(required = true, help = "ID of tool")]
    id: Id,
}

#[system]
pub async fn list_remote(args: ArgsRef<ListRemoteArgs>) {
    let mut tool = load_tool(&args.id).await?;
    tool.disable_caching();

    debug!("Loading versions");

    let resolver = tool
        .load_version_resolver(&UnresolvedVersionSpec::default())
        .await?;
    let mut versions = resolver.versions;

    if versions.is_empty() {
        eprintln!("No versions available");
        process::exit(1);
    }

    versions.sort();

    println!(
        "{}",
        versions
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
