# HIS Files

HIS files are audio files used in the nancy drew games. This document will outline the file format
Byte values are by default in little endian format

## Decleration

The first version, used in the first two games, starts with the text: 

> Her Interactive Sound\x1A

The second version starts with the text:

> HIS\0

## Type 1 Header

| offset | length | purpose |
| --- | --- | --- |
| 0x00 | 21 | "Her Interactive Sound\x1A" |
| 0x16 | 2 | audio format (1 = no compression) |
| 0x18 | 4 | sample rate * number of channels | 
| 0x1C | 4 | filesize | 
| 0x20 | 2 | number of channels |
| 0x22 | 2 | bits per sample |
| 0x24 | 4 | "data" |
| 0x28 | 4 | sound data size |
| 0x2C | * | sound data | 

## Type 2 Header

| offset | length | purpose | 
| --- | --- | --- |
| 0x00 | 4 | "HIS\0" |
| 0x04 | 4 | appears to always be 1 |
| 0x08 | 2 | audio format (1 = no compression) |
| 0x0A | 2 | number of channels | 
| 0x0C | 4 | sample rate | 
| 0x10 | 4 | byte rate | 
| 0x14 | 2 | block align |
| 0x16 | 2 | bit per sample |
| 0x18 | 4 | sound data size |
| 0x1C | * | sound data |

## [Standard WAV file format](http://soundfile.sapp.org/doc/WaveFormat/)

| offset | length | name | description | 
| --- | --- | --- | --- |
| 0x00 | 4 | ChunkID | Contains the letters "RIFF" in ASCII form|
| 0x04 | 4 | ChunkSize | 4 + (8 + SubChunk1Size) + (8 + SubChunk2Size) |
| 0x08 | 4 | Format | Contains the letters "WAVE" |
| 0x0C | 4 | SubChunk1ID | Contains the letters "fmt " |
| 0x10 | 4 | SubChunk1Size | 16 for PCM.  This is the size of the rest of the Subchunk |
| 0x14 | 2 | Audio Format | PCM = 1 is no compression
| 0x16 | 2 | NumChannels | Mono = 1, Stereo = 2 ... |
| 0x18 | 4 | sample rate | typically 22050 |
| 0x1C | 4 | Byte rate | sampleRate * NumChannels * BitsPerSample/8 |
| 0x20 | 2 | Block align | numChannels * BitsPerSample/8 | 
| 0x22 | 2 | BitsPerSample | 8 bit, 16 bit, etc |
| 0x24 | 4 | SubChunk2ID | Contains the letters "data" |
| 0x28 | 4 | SubChunk2Size | the number of bytes in the data |
| 0x32 | * | Data | the sound data |
