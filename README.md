# Open_ND
An open-source re-implementation of the Nancy Drew game series. Point and Click Mystery-Adventure games.
---

1. [Introduction](#introduction)
2. [Building](#building)
3. [Using](#using)


---

## Introduction <a name="introduction"></a>

**Open_ND** allows for the extraction of assets from the Nancy Drew games published by Her Interactive
the end goal is to become an open source re-implementation of the engine used in the Nancy Drew video game series.
Allowing the games to be played on more platforms than originally designed for.
**Open_ND** requires the original games files in order to be useful

---

## Building <a name="building"></a>

### prerequisites
This program has currently been tested with the fourth game in the series TRT. Success with other games may vary.

1. Intall the [Rust Programming Language](https://www.rust-lang.org/)
2. Download the Repository
3. In the parent directory run `cargo build --release` 
4. The resulting binaries will be located in ./target/release

---

## Using <a name="using"></a>
After building the executables are created:
* avf_decoder
* ciftree_decoder
* his_decoder
* scene_decoder

### extractor
    Auto extracts all of data without using command line arguments.
    To use create a folder called "resources" in the same directory as the extractor program.
    Create a sub-folder(s) with the game abbreviation(s) as its name inside the resources folder.
    place all of the files you want extracted into this folder (or any sub-folder in the games folder)
    run the extractor program; it will create a folder called "extracted_data" with the extracted files.

### avf_decoder
    Converts an .avf file into a series of png frames that can be viewed.
### ciftree_decoder
    Extracts all of the files out of the CIFTREE file.
### his_decoder
    Converts a .his audio file into a standard .wav or ogg file.
### scene_decoder
    Interprets data from a scene file, this is still a heavy WIP. 

# More info
The documentation written by Faye on the various file formats used in the games was very helpfull in the writing of this program.
[Faye's own extraction programs and documentation are available here](https://gitlab.com/ShimmerFairy/oldhertools/-/tree/master)
