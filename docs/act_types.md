# List of act types

> This list is for game #4, others may be different!

* 0x0C_01

    Appears to be for automatically going to another scene
    offset| length | purpose
    --- | --- | ---
    0x00 | 4 | the target scene
    0x02 | 32 | unknown
    0x32 | ? | Time in seconds before going to the next scene

* 0x4B_01

    Appears to be the subtitles for a voiceover
    offset| length | purpose
    --- | --- | ---
    0x00 | ? | unknown

* 0x96_01

    
    This appears to call an audio file to be played
    offset | length | purpose
    --- | --- | ---
    0x00 | 32 | name of the audio file without the extension
    0x21 | 2 | Volume percent Left?
    0x23 | 2 | Volume percent Right?
    0x25 | 4? | Unknown purpose 
    0x29 | -> | Unknown

* 0x0A_01

    Unknown use
