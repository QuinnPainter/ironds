# [agbabi](https://github.com/felixjones/agbabi)

GBA optimized library functions for common operations.

Includes implementations for various [aeabi functions](https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst).

## Usage in this project

Only the parts relevant to the DS are used. More may be pulled in in the future.  
Following changes were needed:  
- Converted from divided to unified syntax to support Rust's inline assembly system
- Local labels changed to numbered labels (otherwise we end up with duplicates, since rust doesn't support local labels properly)

(also note labels 0-1 are offlimits due to an LLVM bug, apparently)

(last updated 2022/08/30)
