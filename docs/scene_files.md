# Head of scene file

offset | length | purpose
--- | --- | ---
0x00 | 4 | 'DATA'
0x04 | 4 | Size of file
0x08 | 8 | 'SCENSSUM'
0x10 | 4 | Unknown 
0x14 | 50 | Scene Discription
0x46 | 32 | Main image file to display for the scene
0x67 | 4 | possibally allways 0x02, 0x00, 0x02, 0x00
0x6B | 32 | Main audio file to be play for the scene
0x8C | 4 | possibally allways 0x02, 0x00, 0x02, 0x00
0x90 | 41 | Unknown

# Act records

## After the header there is a series of entries that define mmore info about the scene

offset | length | purpose
--- | --- | ---
0x00 | 4 | 'ACT\0'
0x04 | 4 | Length of Act
0x08 | 47 | Discription
0x38 | 1 | Record Type
0x39 | 1 | Record Type Variation
0x3A | End | Record Data

### It appears that sometimes there are extra zeros appended to the end of an Act, extending beyond the stated length of act + 8
### The interpretation of the Record is dependent upon the Record type; see act_types.md for info
