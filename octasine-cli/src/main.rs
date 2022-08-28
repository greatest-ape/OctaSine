#[cfg(feature = "bench")]
mod bench_process;
#[cfg(feature = "plot")]
mod plot;

use std::{
    fs::File,
    io::{stdout, Read, Write},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use octasine::sync::serde::{SerdePatch, SerdePatchBank};
use serde::Deserialize;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Unpack JSON from a patch (.fxp) or patch bank file (.fxp)
    UnpackPatch { path: PathBuf },
    /// Pack JSON into patch (.fxp) or patch bank file (.fxb)
    PackPatch { path: PathBuf },
    /// Run OctaSine GUI (without audio generation)
    #[cfg(any(feature = "glow", feature = "wgpu"))]
    RunGui,
    /// Benchmark OctaSine process functions and check output sample accuracy
    #[cfg(feature = "bench")]
    BenchProcess,
    /// Plot envelope and LFO curves (useful during development)
    #[cfg(feature = "plot")]
    Plot,
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
                _ => Err(anyhow::anyhow!(
                    "Unrecognized file extension (expected .fxp or .fxb)"
                )),
            }
        }
        Commands::PackPatch { path } => {
            let mut file = File::open(path)?;
            let mut bytes = Vec::new();

            file.read_to_end(&mut bytes)?;

            #[derive(Deserialize)]
            #[serde(untagged)]
            enum PatchOrBank {
                Patch(SerdePatch),
                Bank(SerdePatchBank),
            }

            stdout()
                .lock()
                .write_all(&match serde_json::from_slice(&bytes)? {
                    PatchOrBank::Patch(patch) => patch.to_fxp_bytes()?,
                    PatchOrBank::Bank(bank) => bank.to_fxb_bytes()?,
                })?;

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
        Commands::BenchProcess => bench_process::run(),
        #[cfg(feature = "plot")]
        Commands::Plot => plot::run(),
    }
}
