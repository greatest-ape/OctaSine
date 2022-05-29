#[cfg(feature = "bench")]
mod bench_process;

use std::{
    fs::File,
    io::{stdout, Read, Write},
    path::PathBuf,
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Unpack a patch (.fxp) or patch bank (.fxp) file to JSON
    UnpackPatch { path: PathBuf },
    /// Pack JSON to patch (.fxp) or patch bank (.fxb) file
    PackPatch { path: PathBuf },
    /// Run OctaSine GUI (without audio generation)
    #[cfg(any(feature = "glow", feature = "wgpu"))]
    RunGui,
    /// Benchmark OctaSine process functions and check output sample accuracy
    #[cfg(feature = "bench")]
    BenchProcess,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::UnpackPatch { path } => {
            use octasine::sync::serde::from_bytes;

            match path.extension().and_then(|s| s.to_str()) {
                Some("fxp") | Some("fxb") => {
                    let mut file = File::open(path)?;
                    let mut file_buffer = Vec::new();
                    file.read_to_end(&mut file_buffer)?;

                    let patch_bank: serde_json::Value = from_bytes(&file_buffer)?;

                    serde_json::to_writer_pretty(stdout().lock(), &patch_bank)?;

                    Ok(())
                }
                _ => {
                    Err(anyhow::anyhow!(
                        "Unrecognized file extension (expected .fxp or .fxb)"
                    ))
                }
            }
        }
        Commands::PackPatch { path } => {
            use octasine::sync::serde::to_bytes;

            let file = File::open(path)?;
            let patch_bank: serde_json::Value = serde_json::from_reader(&file)?;
            let bytes = to_bytes(&patch_bank)?;

            stdout().lock().write_all(&bytes)?;

            Ok(())
        }
        #[cfg(any(feature = "glow", feature = "wgpu"))]
        Commands::RunGui => {
            use std::sync::Arc;

            use octasine::{gui::Gui, sync::SyncState};
            use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

            SimpleLogger::init(
                LevelFilter::Info,
                ConfigBuilder::new()
                    .set_time_offset_to_local()
                    .unwrap()
                    .build(),
            )
            .unwrap();

            let sync_state = Arc::new(SyncState::new(None));

            Gui::open_blocking(sync_state);

            Ok(())
        }
        #[cfg(feature = "bench")]
        Commands::BenchProcess => {
            bench_process::run()
        }
    }
}
