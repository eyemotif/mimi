#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Opens the file in readonly mode.
    #[arg(long, short, default_value = "false")]
    pub readonly: bool,
    /// The filepath to edit
    pub file: std::path::PathBuf,
}
