mod bench_process;
#[cfg(feature = "plot")]
mod plot;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run OctaSine GUI (without audio generation)
    #[cfg(any(feature = "glow", feature = "wgpu"))]
    RunGui,
    /// Benchmark OctaSine process functions and check output sample accuracy
    BenchProcess,
    /// Plot envelope and LFO curves (useful during development)
    #[cfg(feature = "plot")]
    Plot,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        #[cfg(any(feature = "glow", feature = "wgpu"))]
        Commands::RunGui => {
            use std::sync::Arc;

            use octasine::{plugin::vst2::editor::Editor, sync::SyncState};
            use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
            use vst::plugin::HostCallback;

            SimpleLogger::init(
                LevelFilter::Info,
                ConfigBuilder::new()
                    .set_time_offset_to_local()
                    .unwrap()
                    .build(),
            )
            .unwrap();

            let sync_state = Arc::new(SyncState::<HostCallback>::new(None));

            Editor::open_blocking(sync_state);

            Ok(())
        }
        Commands::BenchProcess => bench_process::run(),
        #[cfg(feature = "plot")]
        Commands::Plot => plot::run(),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
