# Ciftree file format
The ciftree is a file that contains in itself other files that have been compressed. Similar in functinality to a zip file.
The ciftree format appears to vary in structure between different games, and as of now, does not appear to contain a way to identify what version a file is.

## Decleration

File starts with string:

> CIF TREE WayneSikes\0 

## Header

| offset | length | purpose                                                                     |
|--------|--------|-----------------------------------------------------------------------------|
| 0x14   | 8      | Unknown, could be a version number                                          |
| 0x1C   | 2      | Number of entries in the index                                              |
| 0x1E   | 2      | Unknown, could be a version number                                          |
| 0x20   | 2048   | Unknown, mostly appears to be 0xFF padding, but there is other data present |

## File Index

The file index contains information for each file in the tree. the length of content of this index differs between games

### For Games 3->4->?

| Offset | length | Data file | Plain file |
|---|---|---|---|
| 0x00 | 33 | Null padded file name | null padded file name |
| 0x21 | 2 | Entry Number | Entry Number |
| 0x23 | 8 | Zeros | Unknown |
| 0x2B | 2 | Zeros | TGA x-origin of lower left |
| 0x2D | 2 | Zeros | Unknown |
| 0x2F | 2 | Zeros | TGA y-origin of lower left |
| 0x31 | 18 | Zeros | Unknown |
| 0x43 | 2 | Zeros | Width of image |
| 0x45 | 2 | Zeros | Unknown |
| 0x47 | 2 | Zeros | Height of image |
| 0x49 | 1 | Zero | Unknown |
| 0x4A | 1 | Unknown | Unknown |
| 0x4B | 4 | Offset into file | Offset into file |
| 0x4F | 4 | Size of data decompressed | Size of data decompressed |
| 0x53 | 4 | Unknown | Unknown |
| 0x57 | 4 | Sixe of data compressed | Size of data compressed |
| 0x5B | 1 | file type (0x03) | File type (0x02) |
| 0x5C | 2 | Unknown | Unknown |

## File data

Each file is compressed and encrypted the same way that avf frames are, compressed by LZSS and then encrypted by adding the offset of the byte (realitive to the start of the image data) to each byte. 
The proper extensions for each file can be found by reading from CIFLIST.txt; which seems to always be the first entry in the index (but could also be found with the filename listed in the index). PLAIN type files appear to always be . TGA files. After unencryption, decompression, and matching the appropriate file extension, the file can be written to disk.

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
