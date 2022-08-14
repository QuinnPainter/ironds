
// https://www.problemkaputt.de/gbatek.htm#dsvideostuff
#[cfg(feature = "arm9")]
pub const MASTER_BRIGHT: usize = 0x0400006C;
#[cfg(feature = "arm9")]
pub const DISPCNT: usize = 0x04000000;
pub const DISPSTAT: usize = 0x04000004;
pub const VCOUNT: usize = 0x04000006;

// https://www.problemkaputt.de/gbatek.htm#dsvideocaptureandmainmemorydisplaymode
#[cfg(feature = "arm9")]
pub const DISPCAPCNT: usize = 0x04000064;
#[cfg(feature = "arm9")]
pub const DISP_MMEM_FIFO: usize = 0x04000068;

// https://www.problemkaputt.de/gbatek.htm#dsdmatransfers
pub const DMA0SAD: usize = 0x040000B0;
pub const DMA1SAD: usize = 0x040000BC;
pub const DMA2SAD: usize = 0x040000C8;
pub const DMA3SAD: usize = 0x040000D4;
pub const DMA0DAD: usize = 0x040000B4;
pub const DMA1DAD: usize = 0x040000C0;
pub const DMA2DAD: usize = 0x040000CC;
pub const DMA3DAD: usize = 0x040000D8;
pub const DMA0CNT_L: usize = 0x040000B8;
pub const DMA1CNT_L: usize = 0x040000C4;
pub const DMA2CNT_L: usize = 0x040000D0;
pub const DMA3CNT_L: usize = 0x040000DC;
pub const DMA0CNT_H: usize = 0x040000BA;
pub const DMA1CNT_H: usize = 0x040000C6;
pub const DMA2CNT_H: usize = 0x040000D2;
pub const DMA3CNT_H: usize = 0x040000DE;
#[cfg(feature = "arm9")]
pub const DMA0FILL: usize = 0x040000E0;
#[cfg(feature = "arm9")]
pub const DMA1FILL: usize = 0x040000E4;
#[cfg(feature = "arm9")]
pub const DMA2FILL: usize = 0x040000E8;
#[cfg(feature = "arm9")]
pub const DMA3FILL: usize = 0x040000EC;

// https://www.problemkaputt.de/gbatek.htm#dstimers
pub const TM0CNT_L: usize = 0x04000100;
pub const TM1CNT_L: usize = 0x04000104;
pub const TM2CNT_L: usize = 0x04000108;
pub const TM3CNT_L: usize = 0x0400010C;
pub const TM0CNT_H: usize = 0x04000102;
pub const TM1CNT_H: usize = 0x04000106;
pub const TM2CNT_H: usize = 0x0400010A;
pub const TM3CNT_H: usize = 0x0400010E;

// https://www.problemkaputt.de/gbatek.htm#dsinterrupts
pub const IME: usize = 0x04000208;
pub const IE: usize = 0x04000210;
pub const IF: usize = 0x04000214;
//pub const IE2: usize = 0x04000218; // DSi7 only
//pub const IF2: usize = 0x0400021C;
// todo: other interrupt addresses (some are DTCM relative)

// https://www.problemkaputt.de/gbatek.htm#dsmaths
#[cfg(feature = "arm9")]
pub const DIVCNT: usize = 0x04000280;
#[cfg(feature = "arm9")]
pub const DIV_NUMER: usize = 0x04000290;
#[cfg(feature = "arm9")]
pub const DIV_DENOM: usize = 0x04000298;
#[cfg(feature = "arm9")]
pub const DIV_RESULT: usize = 0x040002A0;
#[cfg(feature = "arm9")]
pub const DIVREM_RESULT: usize = 0x040002A8;
#[cfg(feature = "arm9")]
pub const SQRTCNT: usize = 0x040002B0;
#[cfg(feature = "arm9")]
pub const SQRT_RESULT: usize = 0x040002B4;
#[cfg(feature = "arm9")]
pub const SQRT_PARAM: usize = 0x040002B8;

// https://www.problemkaputt.de/gbatek.htm#dsinterprocesscommunicationipc
pub const IPCSYNC: usize = 0x04000180;
pub const IPCFIFOCNT: usize = 0x04000184;
pub const IPCFIFOSEND: usize = 0x04000188;
pub const IPCFIFORECV: usize = 0x04100000;

// https://www.problemkaputt.de/gbatek.htm#dskeypad
pub const KEYINPUT: usize = 0x04000130;
pub const KEYCNT: usize = 0x04000132;
#[cfg(feature = "arm7")]
pub const EXTKEYIN: usize = 0x04000136;

// https://www.problemkaputt.de/gbatek.htm#dsserialperipheralinterfacebusspi
#[cfg(feature = "arm7")]
pub const SPICNT: usize = 0x040001C0;
#[cfg(feature = "arm7")]
pub const SPIDATA: usize = 0x040001C2;

// https://www.problemkaputt.de/gbatek.htm#dspowercontrol
#[cfg(feature = "arm9")]
pub const POWCNT1: usize = 0x04000304;
#[cfg(feature = "arm7")]
pub const POWCNT2: usize = 0x04000304;
#[cfg(feature = "arm7")]
pub const WIFIWAITCNT: usize = 0x04000206;
#[cfg(feature = "arm7")]
pub const HALTCNT: usize = 0x04000301;
pub const POSTFLG: usize = 0x04000300;

// https://www.problemkaputt.de/gbatek.htm#dsdebugregistersemulatordevkits
pub const NOCASH_EMUID: usize = 0x04FFFA00;
pub const NOCASH_STROUT_RAW: usize = 0x04FFFA10;
pub const NOCASH_STROUT_PARAM: usize = 0x04FFFA14;
pub const NOCASH_STROUT_PARAM_LF: usize = 0x04FFFA18;
pub const NOCASH_CHAROUT: usize = 0x04FFFA1C;
pub const NOCASH_CLOCKS: usize = 0x04FFFA20;
