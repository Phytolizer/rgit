use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Local};
use structopt::StructOpt;

mod parsers;

use parsers::parse_config;

#[derive(Debug, PartialEq, StructOpt)]
enum RGit {
    Clone {
        #[structopt(short, long)]
        local: bool,
        #[structopt(long)]
        no_hardlinks: bool,
        #[structopt(short, long)]
        shared: bool,
        #[structopt(long, value_name = "repository")]
        reference: Option<String>,
        #[structopt(long, value_name = "repository")]
        reference_if_able: Option<String>,
        #[structopt(long)]
        dissociate: bool,
        #[structopt(short, long)]
        quiet: bool,
        #[structopt(short, long)]
        verbose: bool,
        #[structopt(long)]
        progress: bool,
        #[structopt(long, value_name = "option")]
        server_option: Option<String>,
        #[structopt(short, long)]
        no_checkout: bool,
        #[structopt(long)]
        bare: bool,
        #[structopt(long)]
        sparse: bool,
        #[structopt(long, value_name = "filter-spec")]
        filter: Option<String>,
        #[structopt(long)]
        mirror: bool,
        #[structopt(short, long, value_name = "name")]
        origin: Option<String>,
        #[structopt(short, long, value_name = "name")]
        branch: Option<String>,
        #[structopt(short, long, value_name = "upload-pack")]
        upload_pack: Option<String>,
        #[structopt(long, value_name = "template_directory")]
        template: Option<String>,
        #[structopt(short, long, parse(try_from_str = parse_config))]
        config: Option<HashMap<String, String>>,
        #[structopt(long)]
        depth: Option<usize>,
        #[structopt(long)]
        shallow_since: Option<DateTime<Local>>,
        #[structopt(long)]
        shallow_exclude: Option<String>,
        #[structopt(long)]
        single_branch: bool,
        #[structopt(long)]
        no_single_branch: bool,
        #[structopt(long)]
        no_tags: bool,
        #[structopt(long, value_name = "pathspec")]
        recurse_submodules: Option<Option<usize>>,
        #[structopt(long)]
        shallow_submodules: bool,
        #[structopt(long)]
        no_shallow_submodules: bool,
        #[structopt(long)]
        remote_submodules: bool,
        #[structopt(long)]
        no_remote_submodules: bool,
        #[structopt(long, value_name = "git dir")]
        separate_git_dir: Option<PathBuf>,
        #[structopt(short, long, value_name = "n")]
        jobs: Option<usize>,
        repository: String,
        directory: Option<String>,
    },
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let opts = RGit::from_args();
    dbg!(opts);
}
