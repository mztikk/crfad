use clap::Parser;
use rand::Rng;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: PathBuf,
    /// Depth of the tree to generate
    #[clap(default_value_t = 5, long, short)]
    depth: usize,
    /// Number of nodes to generate
    #[clap(default_value_t = 5, long, short)]
    num_files: usize,
}

fn generate_random_files(amount: usize, path: &Path) -> anyhow::Result<()> {
    let mut n = 0;
    while n < amount {
        let mut path = path.to_path_buf().clone();
        let filename = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();
        path.push(filename);
        std::fs::File::create(path)?;
        n += 1;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if !cli.path.exists() {
        std::fs::create_dir_all(&cli.path)?;
    }

    let mut depth = 0;
    let mut path = cli.path.clone();

    while depth < cli.depth {
        let dir_name = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();
        path.push(dir_name);
        std::fs::create_dir_all(path.clone())?;
        generate_random_files(cli.num_files, &path)?;
        depth += 1;
    }

    Ok(())
}
