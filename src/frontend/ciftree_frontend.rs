use structopt::StructOpt;
#[path = "../backend/ciftree_backend.rs"]
mod cif_back;

#[derive(StructOpt)]
struct Cli {
    // The path to the file to read
    input_file: std::path::PathBuf,
    // The path to the write directory
    output_dir: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    cif_back::ext_ciftree(args.input_file, args.output_dir);
}