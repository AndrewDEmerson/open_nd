use structopt::StructOpt;
#[path = "../backend/his_backend.rs"]
mod his_back;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read
    input_file: std::path::PathBuf,
    // The path to the write directory
    output_dir: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    his_back::his_to_wav(args.input_file, args.output_dir, 8 as u8);
}
