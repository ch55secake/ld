use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about="List files and directories in a directory.")]
pub(crate) struct Args {

    /// Name of the directory
    #[clap(default_value = ".")]
    pub(crate) directory: String,

    /// Verbose 
    #[clap(short, long)]
    #[clap(default_value = "false")]
    verbose: bool
}
