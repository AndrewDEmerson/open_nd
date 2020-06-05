# AVF files

AVF files are image/video files used in the nancy drew games. This document will outline the file format
Byte values are by default in little endian format

## Decleration

File starts with string:

> AVF WayneSikes\0

along with 6 bytes of unknown data

## Header

The header contains the pixel dimensions of the resultant image/video as well as the number of entries in the file Index

| offset | length | purpose                         |
|--------|--------|---------------------------------|
| 0x15   | 2      | Number of entries in file index |
| 0x17   | 2      | Frame width in pixels           |
| 0x19   | 2      | frame height in pixels          |

followed by 6 bytes of unknown data

## Frame Index

the frame index is a set of data entries, one for each frame in the file (therfore only containing 1 in a still image). 

| offset | length | purpose                                 |
|--------|--------|-----------------------------------------|
| 0x00   | 2      | Zero indexed frame number               |
| 0x02   | 4      | offset into file where the image starts |
| 0x06   | 4      | The length in bytes of the image data   |
| 0x0A   | 9      | data of unknow purpose                  |

## Frame Data

A frame is an array of rgb values in binary format as 0x#RRRRRGG_GGGBBBBB that has been compressed by LZSS and then encrypted by adding the offset of the byte (realitive to the start of the image data) to each byte. The start of the frame as well as its length are defined in the Frame Index for that frame.

### LZSS

The LZSS decompression has the following attributes

* Circular buffer of size 4096 bytes
    - Intital write begins at 0xFEE
* 8 bits of flags come before the relevent data
    - The LSB represents the first piece of data
    - 1 -> literal byte
    - 0 -> data refrence
* data refrence
    - First byte is the low byte of the offset into the buffer
    - Second byte (High Nybble) is the high nybble of the offset into the buffer
    - Second byte (Low Nybble) is the number of bytes to read from the buffer minus 3
