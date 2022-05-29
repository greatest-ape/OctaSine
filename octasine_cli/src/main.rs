use std::{
    fs::File,
    io::{stdout, Read},
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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::UnpackPatch { path } => {
            use octasine::sync::serde::{from_bytes, SerdePatch, SerdePatchBank};

            match path.extension().and_then(|s| s.to_str()) {
                Some("fxb") => {
                    let mut file = File::open(path)?;
                    let mut file_buffer = Vec::new();
                    file.read_to_end(&mut file_buffer)?;

                    let patch_bank: SerdePatchBank = from_bytes(&file_buffer)?;

                    serde_json::to_writer_pretty(stdout(), &patch_bank)?;
                }
                Some("fxp") => {
                    let mut file = File::open(path)?;
                    let mut file_buffer = Vec::new();
                    file.read_to_end(&mut file_buffer)?;

                    let patch: SerdePatch = from_bytes(&file_buffer)?;

                    serde_json::to_writer_pretty(stdout(), &patch)?;
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Unrecognized file extension (expected .fxp or .fxb)"
                    ));
                }
            }
        }
    }

    Ok(())
}
