use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct ProgramArguments {
    /// URL of the wiki to be viewed
    #[clap(short, long)]
    pub url: Option<String>,

    /// Language to view the wiki in
    #[clap(short, long)]
    pub lang: Option<String>,
}
