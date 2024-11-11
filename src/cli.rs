use clap::{arg, command, ArgAction, ArgGroup, Args, Command, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

// Define value enums for use in arguments
#[derive(ValueEnum, Debug, Clone)]
enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(ValueEnum, Debug, Clone)]
enum OutputFormat {
    Json,
    Yaml,
    Text,
}

// Define the main CLI structure
#[derive(Parser, Debug)]
#[command(
    name = "mycli",
    version = "1.0",
    about = "A comprehensive CLI tool example",
    long_about = "This is a detailed example showing various features of clap for building CLI applications"
)]
struct Cli {
    // Global options available to all subcommands
    #[arg(
        global = true,
        short,
        long,
        value_enum,
        help = "Set the logging level",
        default_value = "info"
    )]
    log_level: LogLevel,

    #[arg(
        global = true,
        short,
        long,
        help = "Enable verbose output",
        action = ArgAction::SetTrue
    )]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

// Define subcommands
#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage files and directories
    Files(FileArgs),

    /// Configure application settings
    Config(ConfigArgs),

    /// Process data with various options
    Process(ProcessArgs),
}

// Arguments for the Files subcommand
#[derive(Args, Debug)]
struct FileArgs {
    /// Source path for file operations
    #[arg(short, long, value_name = "PATH")]
    source: PathBuf,

    /// Destination path for file operations
    #[arg(short, long, value_name = "PATH")]
    destination: Option<PathBuf>,

    /// Recursively process directories
    #[arg(short, long, action = ArgAction::SetTrue)]
    recursive: bool,

    /// File patterns to include (can specify multiple)
    #[arg(short, long, value_name = "PATTERN", num_args = 1..)]
    patterns: Option<Vec<String>>,

    /// Maximum depth for recursive operations
    #[arg(long, value_name = "NUM", default_value = "10")]
    max_depth: u32,
}

// Arguments for the Config subcommand
#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("config_action")
        .required(true)
        .args(["set", "get", "list"]),
))]
struct ConfigArgs {
    /// Set a configuration value
    #[arg(short, long, value_names = ["KEY", "VALUE"], num_args = 2)]
    set: Option<Vec<String>>,

    /// Get a configuration value
    #[arg(short, long, value_name = "KEY")]
    get: Option<String>,

    /// List all configuration values
    #[arg(short, long, action = ArgAction::SetTrue)]
    list: bool,

    /// Configuration file to use
    #[arg(short, long, value_name = "FILE", default_value = "config.yaml")]
    file: PathBuf,
}

// Arguments for the Process subcommand
#[derive(Args, Debug)]
struct ProcessArgs {
    /// Input files to process
    #[arg(required = true, num_args = 1.., value_name = "FILES")]
    input_files: Vec<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    format: OutputFormat,

    /// Number of threads to use
    #[arg(short, long, value_name = "NUM", default_value = "1")]
    threads: u32,

    /// Batch size for processing
    #[arg(short, long, value_name = "SIZE", default_value = "100")]
    batch_size: usize,

    /// Enable dry run mode
    #[arg(long, action = ArgAction::SetTrue)]
    dry_run: bool,

    /// Custom options (key=value pairs)
    #[arg(short, long, value_name = "KEY=VALUE", value_parser = parse_key_val)]
    options: Option<Vec<(String, String)>>,
}

// Helper function to parse key-value pairs
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn main() {
    let cli = Cli::parse();

    // Set up logging based on global options
    println!("Log level set to: {:?}", cli.log_level);
    if cli.verbose {
        println!("Verbose mode enabled");
    }

    // Handle commands
    match cli.command {
        Commands::Files(args) => {
            println!("Running Files command with args:");
            println!("  Source: {:?}", args.source);
            println!("  Destination: {:?}", args.destination);
            println!("  Recursive: {}", args.recursive);
            println!("  Patterns: {:?}", args.patterns);
            println!("  Max depth: {}", args.max_depth);
        }
        Commands::Config(args) => {
            println!("Running Config command with args:");
            if let Some(set_values) = args.set {
                println!("  Setting: {} = {}", set_values[0], set_values[1]);
            }
            if let Some(key) = args.get {
                println!("  Getting value for: {}", key);
            }
            if args.list {
                println!("  Listing all configuration values");
            }
            println!("  Using config file: {:?}", args.file);
        }
        Commands::Process(args) => {
            println!("Running Process command with args:");
            println!("  Input files: {:?}", args.input_files);
            println!("  Output format: {:?}", args.format);
            println!("  Threads: {}", args.threads);
            println!("  Batch size: {}", args.batch_size);
            println!("  Dry run: {}", args.dry_run);
            if let Some(options) = args.options {
                println!("  Custom options:");
                for (key, value) in options {
                    println!("    {}: {}", key, value);
                }
            }
        }
    }
}