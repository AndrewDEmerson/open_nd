use structopt::StructOpt;
#[path = "../backend/avf_backend.rs"]
mod avf_back;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read
    input_file: std::path::PathBuf,
    // The path to the write directory
    output_dir: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    avf_back::avf_to_png(args.input_file, args.output_dir);
}
