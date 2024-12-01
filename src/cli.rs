use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Filter to execute
    #[arg(required(true), value_name = "FILTER")]
    pub filter: String,

    /// Path to input Genbank/Embl file. Use `-` to read from stdin
    #[arg(default_value = "-", value_name = "INPUT")]
    pub input: String,
}
