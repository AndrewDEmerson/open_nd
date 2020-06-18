#[path = "avf.rs"]
mod avf;
#[path = "ciftree.rs"]
mod cif;
#[path = "his.rs"]
mod his;

fn main() {
    let dir = std::path::Path::new("./resources");
    read_dir(dir);
}

fn read_dir(dir: &std::path::Path) {
    let path_image = std::path::Path::new("./extracted_data/images");
    let path_audio = std::path::Path::new("./extracted_data/audio");
    let path_tree = std::path::Path::new("./extracted_data/tree");
    std::fs::create_dir_all(path_image).unwrap();
    std::fs::create_dir_all(path_audio).unwrap();
    std::fs::create_dir_all(path_tree).unwrap();
    println!("{:?}", dir);
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                if entry.path().extension().unwrap().to_str().unwrap() == "avf" {
                    avf::avf_to_png(entry.path(), std::path::PathBuf::from(path_image));
                }
                if entry.path().extension().unwrap().to_str().unwrap() == "his" {
                    his::his_to_wav(entry.path(), std::path::PathBuf::from(path_audio), 8);
                }
                if entry.path().file_name().unwrap() == "CIFTREE.DAT" {
                    cif::ext_ciftree(entry.path(), std::path::PathBuf::from(path_tree), 7, true);
                }
            } else if entry.path().is_dir() {
                read_dir(&entry.path());
            }
        }
    }
}
