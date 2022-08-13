/* See the "Debug Messages" section of the NO$GBA help for more detail. */
/* (the website is outdated, view it in the actual app) */
use voladdress::*;

const CHAR_OUT: VolAddress<u8, (), Safe> = unsafe { VolAddress::new(0x04FFFA1C) };

pub fn print (s: &str) {
    for b in s.bytes() {
        CHAR_OUT.write(b);
    }
}
