use clap::{Parser, Subcommand};

/* Define command line interface */
#[derive(Parser)]
#[command(about = "Statistical test suite for NIST 800-22 revision 1a on the BL602", arg_required_else_help = true, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/* Define subcommands */
#[derive(Subcommand)]
enum Commands {
    /* Capture */
    #[command(about = "Capture data from a connected BL602 device to a file")]
    Capture {
        #[arg(long, help="Path to serial terminal of the device", default_value_t = String::from("/dev/ttyUSB0"))]
        device: String,

        #[arg(long, help = "Baudrate", default_value_t = 2_000_000)]
        baudrate: u32,

        #[arg(long, help="Path to the file where the logged data should be stored", default_value_t = String::from("sts.txt"))]
        path: String,

        #[arg(
            long,
            help = "Number of double words to capture (1dw = 32bits)",
            default_value_t = 31_250
        )]
        dw: u32,
    },
    /* Evaluate */
    #[command(about = "Read captured data from file and evaluate it")]
    Evaluate {
        /* File name */
        #[arg(long, help="Path to the file where the captured bits are stored", default_value_t = String::from("sts.txt"))]
        path: String,

        /* Outfile name */
        #[arg(long, help="Path where to store the result p-values", default_value_t = String::from("results.json"))]
        out_path: String,
    },
}

mod capture;
mod containers;
mod evaluate;

fn main() {
    /* Parse commands */
    let cli = Cli::parse();

    /* Evaluate commands */
    match cli.command {
        /* Start capturing */
        Some(Commands::Capture {
            device,
            baudrate,
            path,
            dw,
        }) => capture::capture(device, baudrate, path, dw),
        /* Start evaluating */
        Some(Commands::Evaluate { path , out_path}) => evaluate::evaluate(path, out_path),
        /* No command case is already handled with arg_required_else_help */
        _ => unreachable!(),
    }
}
