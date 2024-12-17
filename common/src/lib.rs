use tracing_subscriber::filter::LevelFilter;

pub mod cli;
pub mod grid;
pub mod iterators;
pub mod search;

pub fn advent<P1, P2, O1, O2>(part1: P1, part2: P2) -> anyhow::Result<String>
where
    P1: Fn(Vec<String>) -> anyhow::Result<O1>,
    O1: ToString,
    P2: Fn(Vec<String>) -> anyhow::Result<O2>,
    O2: ToString,
{
    let cli_args = cli::Cli::from_args();

    let level_filter = if cli_args.debug {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(level_filter)
        .with_writer(std::io::stderr)
        .init();

    let input = std::io::stdin().lines().map(Result::unwrap).collect();
    Ok(match cli_args.part {
        1 => part1(input)?.to_string(),
        2 => part2(input)?.to_string(),
        p => anyhow::bail!("Invalid argument: --part {p}"),
    })
}
