#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hcicapability000: Hcicapability000,
    hcicapability004: Hcicapability004,
    hcicapability008: Hcicapability008,
    hcicapability00c: Hcicapability00c,
    hcicapability010: Hcicapability010,
    hcicapability014: Hcicapability014,
    _reserved6: [u8; 0x08],
    hcicapability020: Hcicapability020,
    hcicapability024: Hcicapability024,
    hcicapability028: Hcicapability028,
    hcicapability02c: Hcicapability02c,
    hcicapability030: Hcicapability030,
    hcicapability034: Hcicapability034,
    hcicapability038: Hcicapability038,
    hcicapability03c: Hcicapability03c,
    hcicapability040: Hcicapability040,
    _reserved15: [u8; 0x08],
    hcicapability04c: Hcicapability04c,
    _reserved16: [u8; 0x08],
    hcicapability058: Hcicapability058,
    _reserved17: [u8; 0x04],
    hcicapability060: Hcicapability060,
    hcicapability064: Hcicapability064,
    hcicapability068: Hcicapability068,
    _reserved20: [u8; 0x64],
    hcipio000: Hcipio000,
    hcipio004: Hcipio004,
    hcipio008: Hcipio008,
    hcipio00c: Hcipio00c,
    hcipio010: Hcipio010,
    hcipio014: Hcipio014,
    hcipio018: Hcipio018,
    _reserved27: [u8; 0x04],
    hcipio020: Hcipio020,
    hcipio024: Hcipio024,
    hcipio028: Hcipio028,
    hcipio02c: Hcipio02c,
    hcidatsingle000: Hcidatsingle000,
    _reserved32: [u8; 0x03fc],
    hcidct000: Hcidct000,
    hcidct004: Hcidct004,
    hcidct008: Hcidct008,
    hcidct00c: Hcidct00c,
    _reserved36: [u8; 0x02f0],
    hcirhs000: Hcirhs000,
    hcirhs004: Hcirhs004,
    _reserved38: [u8; 0x28],
    hcirhs030: Hcirhs030,
    hcirhs034: Hcirhs034,
    hcirhs038: Hcirhs038,
    _reserved41: [u8; 0x04],
    hcirhs040: Hcirhs040,
    hcirhs044: Hcirhs044,
    hcirhs048: Hcirhs048,
    hcirhs04c: Hcirhs04c,
    hcirhs050: Hcirhs050,
    hcirhs054: Hcirhs054,
    hcirhs058: Hcirhs058,
    hcirhs05c: Hcirhs05c,
    hcirhs060: Hcirhs060,
    hcirhs064: Hcirhs064,
    hcirhs068: Hcirhs068,
    hcirhs06c: Hcirhs06c,
    hcirhs070: Hcirhs070,
    hcirhs074: Hcirhs074,
    hcirhs078: Hcirhs078,
    hcirhs07c: Hcirhs07c,
    _reserved57: [u8; 0x0480],
    i3ccontrol000: I3ccontrol000,
    i3ccontrol004: I3ccontrol004,
    i3ccontrol008: I3ccontrol008,
    i3ccontrol00c: I3ccontrol00c,
    i3ccontrol010: I3ccontrol010,
    i3ccontrol014: I3ccontrol014,
    i3ccontrol018: I3ccontrol018,
    i3ccontrol01c: I3ccontrol01c,
    i3ccontrol020: I3ccontrol020,
    i3ccontrol024: I3ccontrol024,
    i3ccontrol028: I3ccontrol028,
    i3ccontrol02c: I3ccontrol02c,
    i3ccontrol030: I3ccontrol030,
    i3ccontrol034: I3ccontrol034,
    i3ccontrol038: I3ccontrol038,
    i3ccontrol03c: I3ccontrol03c,
    i3ccontrol040: I3ccontrol040,
    i3ccontrol044: I3ccontrol044,
    i3ccontrol048: I3ccontrol048,
    i3ccontrol04c: I3ccontrol04c,
    i3ccontrol050: I3ccontrol050,
    i3ccontrol054: I3ccontrol054,
    i3ccontrol058: I3ccontrol058,
    i3ccontrol05c: I3ccontrol05c,
    i3ccontrol060: I3ccontrol060,
    i3ccontrol064: I3ccontrol064,
    i3ccontrol068: I3ccontrol068,
    i3ccontrol06c: I3ccontrol06c,
    i3ccontrol070: I3ccontrol070,
    i3ccontrol074: I3ccontrol074,
    i3ccontrol078: I3ccontrol078,
    i3ccontrol07c: I3ccontrol07c,
    i3ccontrol080: I3ccontrol080,
    i3ccontrol084: I3ccontrol084,
    i3ccontrol088: I3ccontrol088,
    _reserved92: [u8; 0x04],
    i3ccontrol090: I3ccontrol090,
    i3ccontrol094: I3ccontrol094,
    i3ccontrol098: I3ccontrol098,
    i3ccontrol09c: I3ccontrol09c,
    i3ccontrol0a0: I3ccontrol0a0,
    i3ccontrol0a4: I3ccontrol0a4,
    i3ccontrol0a8: I3ccontrol0a8,
    i3ccontrol0ac: I3ccontrol0ac,
    i3ccontrol0b0: I3ccontrol0b0,
    i3ccontrol0b4: I3ccontrol0b4,
    i3ccontrol0b8: I3ccontrol0b8,
    i3ccontrol0bc: I3ccontrol0bc,
    i3ccontrol0c0: I3ccontrol0c0,
    i3ccontrol0c4: I3ccontrol0c4,
    i3ccontrol0c8: I3ccontrol0c8,
    i3ccontrol0cc: I3ccontrol0cc,
    i3ccontrol0d0: I3ccontrol0d0,
    i3ccontrol0d4: I3ccontrol0d4,
    i3ccontrol0d8: I3ccontrol0d8,
    i3ccontrol0dc: I3ccontrol0dc,
    i3ccontrol0e0: I3ccontrol0e0,
    i3ccontrol0e4: I3ccontrol0e4,
    i3ccontrol0e8: I3ccontrol0e8,
    i3ccontrol0ec: I3ccontrol0ec,
    i3ccontrol0f0: I3ccontrol0f0,
    i3ccontrol0f4: I3ccontrol0f4,
    i3ccontrol0f8: I3ccontrol0f8,
    _reserved119: [u8; 0x04],
    i3cphyctrlreg000: I3cphyctrlreg000,
    i3cphyctrlreg004: I3cphyctrlreg004,
    i3cphyctrlreg008: I3cphyctrlreg008,
    i3cphyctrlreg00c: I3cphyctrlreg00c,
    i3cphyctrlreg010: I3cphyctrlreg010,
    i3cphyctrlreg014: I3cphyctrlreg014,
    i3cphyctrlreg018: I3cphyctrlreg018,
    i3cphyctrlreg01c: I3cphyctrlreg01c,
    i3cphyctrlreg020: I3cphyctrlreg020,
    i3cphyctrlreg024: I3cphyctrlreg024,
    i3cphyctrlreg028: I3cphyctrlreg028,
    i3cphyctrlreg02c: I3cphyctrlreg02c,
    i3cphyctrlreg030: I3cphyctrlreg030,
    i3cphyctrlreg034: I3cphyctrlreg034,
    i3cphyctrlreg038: I3cphyctrlreg038,
    i3cphyctrlreg03c: I3cphyctrlreg03c,
    i3cphyctrlreg040: I3cphyctrlreg040,
    i3cphyctrlreg044: I3cphyctrlreg044,
    i3cphyctrlreg048: I3cphyctrlreg048,
    i3cphyctrlreg04c: I3cphyctrlreg04c,
    i3cphyctrlreg050: I3cphyctrlreg050,
    i3cphyctrlreg054: I3cphyctrlreg054,
    i3cphyctrlreg058: I3cphyctrlreg058,
    i3cphyctrlreg05c: I3cphyctrlreg05c,
    i3cphyctrlreg060: I3cphyctrlreg060,
    i3cphyctrlreg064: I3cphyctrlreg064,
    i3cphyctrlreg068: I3cphyctrlreg068,
    i3cphyctrlreg06c: I3cphyctrlreg06c,
    i3cphyctrlreg070: I3cphyctrlreg070,
    i3cphyctrlreg074: I3cphyctrlreg074,
    i3cphyctrlreg078: I3cphyctrlreg078,
    i3cphyctrlreg07c: I3cphyctrlreg07c,
    i3cphyctrlreg080: I3cphyctrlreg080,
    i3cphyctrlreg084: I3cphyctrlreg084,
    i3cphyctrlreg088: I3cphyctrlreg088,
    i3cphyctrlreg08c: I3cphyctrlreg08c,
    i3cphyctrlreg090: I3cphyctrlreg090,
    i3cphyctrlreg094: I3cphyctrlreg094,
    i3cphyctrlreg098: I3cphyctrlreg098,
    i3cphyctrlreg09c: I3cphyctrlreg09c,
    i3cphyctrlreg0a0: I3cphyctrlreg0a0,
    i3cphyctrlreg0a4: I3cphyctrlreg0a4,
    i3cphyctrlreg0a8: I3cphyctrlreg0a8,
    i3cphyctrlreg0ac: I3cphyctrlreg0ac,
    _reserved163: [u8; 0x04],
    i3cphyctrlreg0b4: I3cphyctrlreg0b4,
    i3cphyctrlreg0b8: I3cphyctrlreg0b8,
    i3cphyctrlreg0bc: I3cphyctrlreg0bc,
    i3cphyctrlreg0c0: I3cphyctrlreg0c0,
    i3cphyctrlreg0c4: I3cphyctrlreg0c4,
    i3cphyctrlreg0c8: I3cphyctrlreg0c8,
    i3cphyctrlreg0cc: I3cphyctrlreg0cc,
    i3cphyctrlreg0d0: I3cphyctrlreg0d0,
    i3cphyctrlreg0d4: I3cphyctrlreg0d4,
    i3cphyctrlreg0d8: I3cphyctrlreg0d8,
    i3cphyctrlreg0dc: I3cphyctrlreg0dc,
    i3cphyctrlreg0e0: I3cphyctrlreg0e0,
    i3cphyctrlreg0e4: I3cphyctrlreg0e4,
    i3cphyctrlreg0e8: I3cphyctrlreg0e8,
    i3cphyctrlreg0ec: I3cphyctrlreg0ec,
    i3cphyctrlreg0f0: I3cphyctrlreg0f0,
    _reserved179: [u8; 0x0c],
    hciextcap000: Hciextcap000,
    hciextcap004: Hciextcap004,
    hciextcap008: Hciextcap008,
    hciextcap00c: Hciextcap00c,
    hciextcap010: Hciextcap010,
    hciextcap014: Hciextcap014,
    hciextcap018: Hciextcap018,
    hciextcap01c: Hciextcap01c,
    hciextcap020: Hciextcap020,
    hciextcap024: Hciextcap024,
    _reserved189: [u8; 0x58],
    hciextcap080: Hciextcap080,
    hciextcap084: Hciextcap084,
    _reserved191: [u8; 0x08],
    hciextcap090: Hciextcap090,
    hciextcap094: Hciextcap094,
    hciextcap098: Hciextcap098,
    hciextcap09c: Hciextcap09c,
}
impl RegisterBlock {
    #[doc = "0x00 - HCI\\_VERSION"]
    #[inline(always)]
    pub const fn hcicapability000(&self) -> &Hcicapability000 {
        &self.hcicapability000
    }
    #[doc = "0x04 - HC\\_CONTROL"]
    #[inline(always)]
    pub const fn hcicapability004(&self) -> &Hcicapability004 {
        &self.hcicapability004
    }
    #[doc = "0x08 - CONTROLLER\\_DEVICE\\_ADDR"]
    #[inline(always)]
    pub const fn hcicapability008(&self) -> &Hcicapability008 {
        &self.hcicapability008
    }
    #[doc = "0x0c - HC\\_CAPABILITIES"]
    #[inline(always)]
    pub const fn hcicapability00c(&self) -> &Hcicapability00c {
        &self.hcicapability00c
    }
    #[doc = "0x10 - RESET\\_CONTROL"]
    #[inline(always)]
    pub const fn hcicapability010(&self) -> &Hcicapability010 {
        &self.hcicapability010
    }
    #[doc = "0x14 - PRESENT\\_STATE"]
    #[inline(always)]
    pub const fn hcicapability014(&self) -> &Hcicapability014 {
        &self.hcicapability014
    }
    #[doc = "0x20 - INTR\\_STATUS"]
    #[inline(always)]
    pub const fn hcicapability020(&self) -> &Hcicapability020 {
        &self.hcicapability020
    }
    #[doc = "0x24 - INTR\\_STATUS\\_ENABLE"]
    #[inline(always)]
    pub const fn hcicapability024(&self) -> &Hcicapability024 {
        &self.hcicapability024
    }
    #[doc = "0x28 - INTR\\_SIGNAL\\_ENABLE"]
    #[inline(always)]
    pub const fn hcicapability028(&self) -> &Hcicapability028 {
        &self.hcicapability028
    }
    #[doc = "0x2c - INTR\\_FORCE"]
    #[inline(always)]
    pub const fn hcicapability02c(&self) -> &Hcicapability02c {
        &self.hcicapability02c
    }
    #[doc = "0x30 - DAT\\_SECTION\\_OFFSET"]
    #[inline(always)]
    pub const fn hcicapability030(&self) -> &Hcicapability030 {
        &self.hcicapability030
    }
    #[doc = "0x34 - DCT\\_SECTION\\_OFFSET"]
    #[inline(always)]
    pub const fn hcicapability034(&self) -> &Hcicapability034 {
        &self.hcicapability034
    }
    #[doc = "0x38 - RING\\_HEADERS\\_SECTION\\_OFFSET"]
    #[inline(always)]
    pub const fn hcicapability038(&self) -> &Hcicapability038 {
        &self.hcicapability038
    }
    #[doc = "0x3c - PIO\\_SECTION\\_OFFSET"]
    #[inline(always)]
    pub const fn hcicapability03c(&self) -> &Hcicapability03c {
        &self.hcicapability03c
    }
    #[doc = "0x40 - EXT\\_CAPS\\_SECTION\\_OFFSET"]
    #[inline(always)]
    pub const fn hcicapability040(&self) -> &Hcicapability040 {
        &self.hcicapability040
    }
    #[doc = "0x4c - INT\\_CTRL\\_CMDS\\_EN"]
    #[inline(always)]
    pub const fn hcicapability04c(&self) -> &Hcicapability04c {
        &self.hcicapability04c
    }
    #[doc = "0x58 - IBI\\_NOTIFY\\_CTRL"]
    #[inline(always)]
    pub const fn hcicapability058(&self) -> &Hcicapability058 {
        &self.hcicapability058
    }
    #[doc = "0x60 - DEV\\_CTX\\_BASE\\_LO"]
    #[inline(always)]
    pub const fn hcicapability060(&self) -> &Hcicapability060 {
        &self.hcicapability060
    }
    #[doc = "0x64 - DEV\\_CTX\\_BASE\\_HI"]
    #[inline(always)]
    pub const fn hcicapability064(&self) -> &Hcicapability064 {
        &self.hcicapability064
    }
    #[doc = "0x68 - DEV\\_CTX\\_SG"]
    #[inline(always)]
    pub const fn hcicapability068(&self) -> &Hcicapability068 {
        &self.hcicapability068
    }
    #[doc = "0xd0 - COMMAND\\_QUEUE\\_PORT"]
    #[inline(always)]
    pub const fn hcipio000(&self) -> &Hcipio000 {
        &self.hcipio000
    }
    #[doc = "0xd4 - RESPONSE\\_QUEUE\\_PORT"]
    #[inline(always)]
    pub const fn hcipio004(&self) -> &Hcipio004 {
        &self.hcipio004
    }
    #[doc = "0xd8 - XFER\\_DATA\\_PORT"]
    #[inline(always)]
    pub const fn hcipio008(&self) -> &Hcipio008 {
        &self.hcipio008
    }
    #[doc = "0xdc - IBI\\_PORT"]
    #[inline(always)]
    pub const fn hcipio00c(&self) -> &Hcipio00c {
        &self.hcipio00c
    }
    #[doc = "0xe0 - QUEUE\\_THLD\\_CTRL"]
    #[inline(always)]
    pub const fn hcipio010(&self) -> &Hcipio010 {
        &self.hcipio010
    }
    #[doc = "0xe4 - DATA\\_BUFFER\\_THLD\\_CTRL"]
    #[inline(always)]
    pub const fn hcipio014(&self) -> &Hcipio014 {
        &self.hcipio014
    }
    #[doc = "0xe8 - QUEUE\\_SIZE"]
    #[inline(always)]
    pub const fn hcipio018(&self) -> &Hcipio018 {
        &self.hcipio018
    }
    #[doc = "0xf0 - PIO\\_INTR\\_STATUS"]
    #[inline(always)]
    pub const fn hcipio020(&self) -> &Hcipio020 {
        &self.hcipio020
    }
    #[doc = "0xf4 - PIO\\_INTR\\_STATUS\\_ENABLE"]
    #[inline(always)]
    pub const fn hcipio024(&self) -> &Hcipio024 {
        &self.hcipio024
    }
    #[doc = "0xf8 - PIO\\_INTR\\_SIGNAL\\_ENABLE"]
    #[inline(always)]
    pub const fn hcipio028(&self) -> &Hcipio028 {
        &self.hcipio028
    }
    #[doc = "0xfc - PIO\\_INTR\\_FORCE"]
    #[inline(always)]
    pub const fn hcipio02c(&self) -> &Hcipio02c {
        &self.hcipio02c
    }
    #[doc = "0x100 - TARGET\\_DAT"]
    #[inline(always)]
    pub const fn hcidatsingle000(&self) -> &Hcidatsingle000 {
        &self.hcidatsingle000
    }
    #[doc = "0x500 - TARGET\\_DCT\\_0"]
    #[inline(always)]
    pub const fn hcidct000(&self) -> &Hcidct000 {
        &self.hcidct000
    }
    #[doc = "0x504 - TARGET\\_DCT\\_1"]
    #[inline(always)]
    pub const fn hcidct004(&self) -> &Hcidct004 {
        &self.hcidct004
    }
    #[doc = "0x508 - TARGET\\_DCT\\_2"]
    #[inline(always)]
    pub const fn hcidct008(&self) -> &Hcidct008 {
        &self.hcidct008
    }
    #[doc = "0x50c - TARGET\\_DCT\\_3"]
    #[inline(always)]
    pub const fn hcidct00c(&self) -> &Hcidct00c {
        &self.hcidct00c
    }
    #[doc = "0x800 - RHS\\_CONTROL"]
    #[inline(always)]
    pub const fn hcirhs000(&self) -> &Hcirhs000 {
        &self.hcirhs000
    }
    #[doc = "0x804 - RH0\\_OFFSET"]
    #[inline(always)]
    pub const fn hcirhs004(&self) -> &Hcirhs004 {
        &self.hcirhs004
    }
    #[doc = "0x830 - CR\\_SETUP"]
    #[inline(always)]
    pub const fn hcirhs030(&self) -> &Hcirhs030 {
        &self.hcirhs030
    }
    #[doc = "0x834 - IBI\\_SETUP"]
    #[inline(always)]
    pub const fn hcirhs034(&self) -> &Hcirhs034 {
        &self.hcirhs034
    }
    #[doc = "0x838 - CHUNK\\_CONTROL"]
    #[inline(always)]
    pub const fn hcirhs038(&self) -> &Hcirhs038 {
        &self.hcirhs038
    }
    #[doc = "0x840 - RH\\_INTR\\_STATUS"]
    #[inline(always)]
    pub const fn hcirhs040(&self) -> &Hcirhs040 {
        &self.hcirhs040
    }
    #[doc = "0x844 - RH\\_INTR\\_STATUS\\_ENABLE"]
    #[inline(always)]
    pub const fn hcirhs044(&self) -> &Hcirhs044 {
        &self.hcirhs044
    }
    #[doc = "0x848 - RH\\_INTR\\_SIGNAL\\_ENABLE"]
    #[inline(always)]
    pub const fn hcirhs048(&self) -> &Hcirhs048 {
        &self.hcirhs048
    }
    #[doc = "0x84c - RH\\_INTR\\_FORCE"]
    #[inline(always)]
    pub const fn hcirhs04c(&self) -> &Hcirhs04c {
        &self.hcirhs04c
    }
    #[doc = "0x850 - RH\\_STATUS"]
    #[inline(always)]
    pub const fn hcirhs050(&self) -> &Hcirhs050 {
        &self.hcirhs050
    }
    #[doc = "0x854 - RH\\_CONTROL"]
    #[inline(always)]
    pub const fn hcirhs054(&self) -> &Hcirhs054 {
        &self.hcirhs054
    }
    #[doc = "0x858 - RH\\_OPERATION1"]
    #[inline(always)]
    pub const fn hcirhs058(&self) -> &Hcirhs058 {
        &self.hcirhs058
    }
    #[doc = "0x85c - RH\\_OPERATION2"]
    #[inline(always)]
    pub const fn hcirhs05c(&self) -> &Hcirhs05c {
        &self.hcirhs05c
    }
    #[doc = "0x860 - RH\\_CMD\\_RING\\_BASE\\_LO"]
    #[inline(always)]
    pub const fn hcirhs060(&self) -> &Hcirhs060 {
        &self.hcirhs060
    }
    #[doc = "0x864 - RH\\_CMD\\_RING\\_BASE\\_HI"]
    #[inline(always)]
    pub const fn hcirhs064(&self) -> &Hcirhs064 {
        &self.hcirhs064
    }
    #[doc = "0x868 - RH\\_RESP\\_RING\\_BASE\\_LO"]
    #[inline(always)]
    pub const fn hcirhs068(&self) -> &Hcirhs068 {
        &self.hcirhs068
    }
    #[doc = "0x86c - RH\\_RESP\\_RING\\_BASE\\_HI"]
    #[inline(always)]
    pub const fn hcirhs06c(&self) -> &Hcirhs06c {
        &self.hcirhs06c
    }
    #[doc = "0x870 - RH\\_IBI\\_STATUS\\_RING\\_BASE\\_LO"]
    #[inline(always)]
    pub const fn hcirhs070(&self) -> &Hcirhs070 {
        &self.hcirhs070
    }
    #[doc = "0x874 - RH\\_IBI\\_STATUS\\_RING\\_BASE\\_HI"]
    #[inline(always)]
    pub const fn hcirhs074(&self) -> &Hcirhs074 {
        &self.hcirhs074
    }
    #[doc = "0x878 - RH\\_IBI\\_DATA\\_RING\\_BASE\\_LO"]
    #[inline(always)]
    pub const fn hcirhs078(&self) -> &Hcirhs078 {
        &self.hcirhs078
    }
    #[doc = "0x87c - RH\\_IBI\\_DATA\\_RING\\_BASE\\_HI"]
    #[inline(always)]
    pub const fn hcirhs07c(&self) -> &Hcirhs07c {
        &self.hcirhs07c
    }
    #[doc = "0xd00 - I3C\\_CONTROL\\_0"]
    #[inline(always)]
    pub const fn i3ccontrol000(&self) -> &I3ccontrol000 {
        &self.i3ccontrol000
    }
    #[doc = "0xd04 - I3C\\_STATUS"]
    #[inline(always)]
    pub const fn i3ccontrol004(&self) -> &I3ccontrol004 {
        &self.i3ccontrol004
    }
    #[doc = "0xd08 - I3C\\_MST\\_MRL"]
    #[inline(always)]
    pub const fn i3ccontrol008(&self) -> &I3ccontrol008 {
        &self.i3ccontrol008
    }
    #[doc = "0xd0c - I3C\\_STATUS\\_C"]
    #[inline(always)]
    pub const fn i3ccontrol00c(&self) -> &I3ccontrol00c {
        &self.i3ccontrol00c
    }
    #[doc = "0xd10 - I3C\\_DAA\\_INDEX\\_0"]
    #[inline(always)]
    pub const fn i3ccontrol010(&self) -> &I3ccontrol010 {
        &self.i3ccontrol010
    }
    #[doc = "0xd14 - I3C\\_DAA\\_INDEX\\_1"]
    #[inline(always)]
    pub const fn i3ccontrol014(&self) -> &I3ccontrol014 {
        &self.i3ccontrol014
    }
    #[doc = "0xd18 - I3C\\_DAA\\_INDEX\\_2"]
    #[inline(always)]
    pub const fn i3ccontrol018(&self) -> &I3ccontrol018 {
        &self.i3ccontrol018
    }
    #[doc = "0xd1c - I3C\\_DAA\\_INDEX\\_3"]
    #[inline(always)]
    pub const fn i3ccontrol01c(&self) -> &I3ccontrol01c {
        &self.i3ccontrol01c
    }
    #[doc = "0xd20 - I3C\\_AUTOCMD\\_0"]
    #[inline(always)]
    pub const fn i3ccontrol020(&self) -> &I3ccontrol020 {
        &self.i3ccontrol020
    }
    #[doc = "0xd24 - I3C\\_AUTOCMD\\_1"]
    #[inline(always)]
    pub const fn i3ccontrol024(&self) -> &I3ccontrol024 {
        &self.i3ccontrol024
    }
    #[doc = "0xd28 - I3C\\_AUTOCMD\\_2"]
    #[inline(always)]
    pub const fn i3ccontrol028(&self) -> &I3ccontrol028 {
        &self.i3ccontrol028
    }
    #[doc = "0xd2c - I3C\\_AUTOCMD\\_3"]
    #[inline(always)]
    pub const fn i3ccontrol02c(&self) -> &I3ccontrol02c {
        &self.i3ccontrol02c
    }
    #[doc = "0xd30 - I3C\\_AUTOCMD\\_4"]
    #[inline(always)]
    pub const fn i3ccontrol030(&self) -> &I3ccontrol030 {
        &self.i3ccontrol030
    }
    #[doc = "0xd34 - I3C\\_AUTOCMD\\_5"]
    #[inline(always)]
    pub const fn i3ccontrol034(&self) -> &I3ccontrol034 {
        &self.i3ccontrol034
    }
    #[doc = "0xd38 - I3C\\_AUTOCMD\\_6"]
    #[inline(always)]
    pub const fn i3ccontrol038(&self) -> &I3ccontrol038 {
        &self.i3ccontrol038
    }
    #[doc = "0xd3c - I3C\\_AUTOCMD\\_7"]
    #[inline(always)]
    pub const fn i3ccontrol03c(&self) -> &I3ccontrol03c {
        &self.i3ccontrol03c
    }
    #[doc = "0xd40 - I3C\\_AUTOCMD\\_SEL\\_040"]
    #[inline(always)]
    pub const fn i3ccontrol040(&self) -> &I3ccontrol040 {
        &self.i3ccontrol040
    }
    #[doc = "0xd44 - I3C\\_AUTOCMD\\_SEL\\_044"]
    #[inline(always)]
    pub const fn i3ccontrol044(&self) -> &I3ccontrol044 {
        &self.i3ccontrol044
    }
    #[doc = "0xd48 - I3C\\_AUTOCMD\\_SEL\\_048"]
    #[inline(always)]
    pub const fn i3ccontrol048(&self) -> &I3ccontrol048 {
        &self.i3ccontrol048
    }
    #[doc = "0xd4c - I3C\\_AUTOCMD\\_SEL\\_04C"]
    #[inline(always)]
    pub const fn i3ccontrol04c(&self) -> &I3ccontrol04c {
        &self.i3ccontrol04c
    }
    #[doc = "0xd50 - I3C\\_AUTOCMD\\_SEL\\_050"]
    #[inline(always)]
    pub const fn i3ccontrol050(&self) -> &I3ccontrol050 {
        &self.i3ccontrol050
    }
    #[doc = "0xd54 - I3C\\_AUTOCMD\\_SEL\\_054"]
    #[inline(always)]
    pub const fn i3ccontrol054(&self) -> &I3ccontrol054 {
        &self.i3ccontrol054
    }
    #[doc = "0xd58 - I3C\\_AUTOCMD\\_SEL\\_058"]
    #[inline(always)]
    pub const fn i3ccontrol058(&self) -> &I3ccontrol058 {
        &self.i3ccontrol058
    }
    #[doc = "0xd5c - I3C\\_AUTOCMD\\_SEL\\_05C"]
    #[inline(always)]
    pub const fn i3ccontrol05c(&self) -> &I3ccontrol05c {
        &self.i3ccontrol05c
    }
    #[doc = "0xd60 - I3C\\_AUTOCMD\\_SEL\\_060"]
    #[inline(always)]
    pub const fn i3ccontrol060(&self) -> &I3ccontrol060 {
        &self.i3ccontrol060
    }
    #[doc = "0xd64 - I3C\\_AUTOCMD\\_SEL\\_064"]
    #[inline(always)]
    pub const fn i3ccontrol064(&self) -> &I3ccontrol064 {
        &self.i3ccontrol064
    }
    #[doc = "0xd68 - I3C\\_AUTOCMD\\_SEL\\_068"]
    #[inline(always)]
    pub const fn i3ccontrol068(&self) -> &I3ccontrol068 {
        &self.i3ccontrol068
    }
    #[doc = "0xd6c - I3C\\_AUTOCMD\\_SEL\\_06C"]
    #[inline(always)]
    pub const fn i3ccontrol06c(&self) -> &I3ccontrol06c {
        &self.i3ccontrol06c
    }
    #[doc = "0xd70 - I3C\\_AUTOCMD\\_SEL\\_070"]
    #[inline(always)]
    pub const fn i3ccontrol070(&self) -> &I3ccontrol070 {
        &self.i3ccontrol070
    }
    #[doc = "0xd74 - I3C\\_AUTOCMD\\_SEL\\_074"]
    #[inline(always)]
    pub const fn i3ccontrol074(&self) -> &I3ccontrol074 {
        &self.i3ccontrol074
    }
    #[doc = "0xd78 - I3C\\_AUTOCMD\\_SEL\\_078"]
    #[inline(always)]
    pub const fn i3ccontrol078(&self) -> &I3ccontrol078 {
        &self.i3ccontrol078
    }
    #[doc = "0xd7c - I3C\\_AUTOCMD\\_SEL\\_07C"]
    #[inline(always)]
    pub const fn i3ccontrol07c(&self) -> &I3ccontrol07c {
        &self.i3ccontrol07c
    }
    #[doc = "0xd80 - I3C\\_WDMA\\_CTL\\_080"]
    #[inline(always)]
    pub const fn i3ccontrol080(&self) -> &I3ccontrol080 {
        &self.i3ccontrol080
    }
    #[doc = "0xd84 - I3C\\_WDMA\\_CTL\\_084"]
    #[inline(always)]
    pub const fn i3ccontrol084(&self) -> &I3ccontrol084 {
        &self.i3ccontrol084
    }
    #[doc = "0xd88 - I3C\\_WDMA\\_CTL\\_088"]
    #[inline(always)]
    pub const fn i3ccontrol088(&self) -> &I3ccontrol088 {
        &self.i3ccontrol088
    }
    #[doc = "0xd90 - I3C\\_RDMA\\_CTL\\_090"]
    #[inline(always)]
    pub const fn i3ccontrol090(&self) -> &I3ccontrol090 {
        &self.i3ccontrol090
    }
    #[doc = "0xd94 - I3C\\_RDMA\\_CTL\\_094"]
    #[inline(always)]
    pub const fn i3ccontrol094(&self) -> &I3ccontrol094 {
        &self.i3ccontrol094
    }
    #[doc = "0xd98 - I3C\\_RDMA\\_CTL\\_098"]
    #[inline(always)]
    pub const fn i3ccontrol098(&self) -> &I3ccontrol098 {
        &self.i3ccontrol098
    }
    #[doc = "0xd9c - I3C\\_RING\\_CTL\\_09C"]
    #[inline(always)]
    pub const fn i3ccontrol09c(&self) -> &I3ccontrol09c {
        &self.i3ccontrol09c
    }
    #[doc = "0xda0 - I3C\\_SLV\\_CTL\\_0A0"]
    #[inline(always)]
    pub const fn i3ccontrol0a0(&self) -> &I3ccontrol0a0 {
        &self.i3ccontrol0a0
    }
    #[doc = "0xda4 - I3C\\_SLV\\_CTL\\_0A4"]
    #[inline(always)]
    pub const fn i3ccontrol0a4(&self) -> &I3ccontrol0a4 {
        &self.i3ccontrol0a4
    }
    #[doc = "0xda8 - I3C\\_SLV\\_CTL\\_0A8"]
    #[inline(always)]
    pub const fn i3ccontrol0a8(&self) -> &I3ccontrol0a8 {
        &self.i3ccontrol0a8
    }
    #[doc = "0xdac - I3C\\_SLV\\_CTL\\_0AC"]
    #[inline(always)]
    pub const fn i3ccontrol0ac(&self) -> &I3ccontrol0ac {
        &self.i3ccontrol0ac
    }
    #[doc = "0xdb0 - I3C\\_SLV\\_CTL\\_0B0"]
    #[inline(always)]
    pub const fn i3ccontrol0b0(&self) -> &I3ccontrol0b0 {
        &self.i3ccontrol0b0
    }
    #[doc = "0xdb4 - I3C\\_SLV\\_CTL\\_0B4"]
    #[inline(always)]
    pub const fn i3ccontrol0b4(&self) -> &I3ccontrol0b4 {
        &self.i3ccontrol0b4
    }
    #[doc = "0xdb8 - I3C\\_SLV\\_CTL\\_0B8"]
    #[inline(always)]
    pub const fn i3ccontrol0b8(&self) -> &I3ccontrol0b8 {
        &self.i3ccontrol0b8
    }
    #[doc = "0xdbc - I3C\\_SLV\\_CTL\\_0BC"]
    #[inline(always)]
    pub const fn i3ccontrol0bc(&self) -> &I3ccontrol0bc {
        &self.i3ccontrol0bc
    }
    #[doc = "0xdc0 - I3C\\_SLV\\_CTL\\_0C0"]
    #[inline(always)]
    pub const fn i3ccontrol0c0(&self) -> &I3ccontrol0c0 {
        &self.i3ccontrol0c0
    }
    #[doc = "0xdc4 - I3C\\_SLV\\_CTL\\_0C4"]
    #[inline(always)]
    pub const fn i3ccontrol0c4(&self) -> &I3ccontrol0c4 {
        &self.i3ccontrol0c4
    }
    #[doc = "0xdc8 - I3C\\_SLV\\_CTL\\_0C8"]
    #[inline(always)]
    pub const fn i3ccontrol0c8(&self) -> &I3ccontrol0c8 {
        &self.i3ccontrol0c8
    }
    #[doc = "0xdcc - I3C\\_SLV\\_CTL\\_0CC"]
    #[inline(always)]
    pub const fn i3ccontrol0cc(&self) -> &I3ccontrol0cc {
        &self.i3ccontrol0cc
    }
    #[doc = "0xdd0 - I3C\\_SLV\\_CTL\\_0D0"]
    #[inline(always)]
    pub const fn i3ccontrol0d0(&self) -> &I3ccontrol0d0 {
        &self.i3ccontrol0d0
    }
    #[doc = "0xdd4 - I3C\\_SLV\\_CTL\\_0D4"]
    #[inline(always)]
    pub const fn i3ccontrol0d4(&self) -> &I3ccontrol0d4 {
        &self.i3ccontrol0d4
    }
    #[doc = "0xdd8 - I3C\\_QUEUE\\_PTR\\_0D8"]
    #[inline(always)]
    pub const fn i3ccontrol0d8(&self) -> &I3ccontrol0d8 {
        &self.i3ccontrol0d8
    }
    #[doc = "0xddc - I3C\\_QUEUE\\_PTR\\_0DC"]
    #[inline(always)]
    pub const fn i3ccontrol0dc(&self) -> &I3ccontrol0dc {
        &self.i3ccontrol0dc
    }
    #[doc = "0xde0 - I3C\\_INTR\\_STATUS"]
    #[inline(always)]
    pub const fn i3ccontrol0e0(&self) -> &I3ccontrol0e0 {
        &self.i3ccontrol0e0
    }
    #[doc = "0xde4 - I3C\\_INTR\\_STATUS\\_ENABLE"]
    #[inline(always)]
    pub const fn i3ccontrol0e4(&self) -> &I3ccontrol0e4 {
        &self.i3ccontrol0e4
    }
    #[doc = "0xde8 - I3C\\_INTR\\_SIGNAL\\_ENABLE"]
    #[inline(always)]
    pub const fn i3ccontrol0e8(&self) -> &I3ccontrol0e8 {
        &self.i3ccontrol0e8
    }
    #[doc = "0xdec - I3C\\_INTR\\_FORCE"]
    #[inline(always)]
    pub const fn i3ccontrol0ec(&self) -> &I3ccontrol0ec {
        &self.i3ccontrol0ec
    }
    #[doc = "0xdf0 - I3C\\_INTR\\_STATUS\\_F0"]
    #[inline(always)]
    pub const fn i3ccontrol0f0(&self) -> &I3ccontrol0f0 {
        &self.i3ccontrol0f0
    }
    #[doc = "0xdf4 - I3C\\_INTR\\_PROCESS"]
    #[inline(always)]
    pub const fn i3ccontrol0f4(&self) -> &I3ccontrol0f4 {
        &self.i3ccontrol0f4
    }
    #[doc = "0xdf8 - I3C\\_IBI\\_TIMEOUT\\_F8"]
    #[inline(always)]
    pub const fn i3ccontrol0f8(&self) -> &I3ccontrol0f8 {
        &self.i3ccontrol0f8
    }
    #[doc = "0xe00 - EXT\\_CAP\\_OFFSET"]
    #[inline(always)]
    pub const fn i3cphyctrlreg000(&self) -> &I3cphyctrlreg000 {
        &self.i3cphyctrlreg000
    }
    #[doc = "0xe04 - SW\\_CTRL"]
    #[inline(always)]
    pub const fn i3cphyctrlreg004(&self) -> &I3cphyctrlreg004 {
        &self.i3cphyctrlreg004
    }
    #[doc = "0xe08 - CR\\_I2C\\_OD\\_FM\\_STA\\_STO\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg008(&self) -> &I3cphyctrlreg008 {
        &self.i3cphyctrlreg008
    }
    #[doc = "0xe0c - CR\\_I2C\\_OD\\_FM\\_SCL\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg00c(&self) -> &I3cphyctrlreg00c {
        &self.i3cphyctrlreg00c
    }
    #[doc = "0xe10 - CR\\_I2C\\_OD\\_FM\\_ACK\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg010(&self) -> &I3cphyctrlreg010 {
        &self.i3cphyctrlreg010
    }
    #[doc = "0xe14 - CR\\_I2C\\_OD\\_FM\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg014(&self) -> &I3cphyctrlreg014 {
        &self.i3cphyctrlreg014
    }
    #[doc = "0xe18 - CR\\_I2C\\_OD\\_FMP\\_STA\\_STO\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg018(&self) -> &I3cphyctrlreg018 {
        &self.i3cphyctrlreg018
    }
    #[doc = "0xe1c - CR\\_I2C\\_OD\\_FMP\\_SCL\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg01c(&self) -> &I3cphyctrlreg01c {
        &self.i3cphyctrlreg01c
    }
    #[doc = "0xe20 - CR\\_I2C\\_OD\\_FMP\\_ACK\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg020(&self) -> &I3cphyctrlreg020 {
        &self.i3cphyctrlreg020
    }
    #[doc = "0xe24 - CR\\_I2C\\_OD\\_FMP\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg024(&self) -> &I3cphyctrlreg024 {
        &self.i3cphyctrlreg024
    }
    #[doc = "0xe28 - CR\\_I3C\\_OD\\_STA\\_STO\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg028(&self) -> &I3cphyctrlreg028 {
        &self.i3cphyctrlreg028
    }
    #[doc = "0xe2c - CR\\_I3C\\_OD\\_SCL\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg02c(&self) -> &I3cphyctrlreg02c {
        &self.i3cphyctrlreg02c
    }
    #[doc = "0xe30 - CR\\_I3C\\_OD\\_ACK\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg030(&self) -> &I3cphyctrlreg030 {
        &self.i3cphyctrlreg030
    }
    #[doc = "0xe34 - CR\\_I3C\\_OD\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg034(&self) -> &I3cphyctrlreg034 {
        &self.i3cphyctrlreg034
    }
    #[doc = "0xe38 - CR\\_I3C\\_SDR0\\_PP\\_SCL\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg038(&self) -> &I3cphyctrlreg038 {
        &self.i3cphyctrlreg038
    }
    #[doc = "0xe3c - CR\\_I3C\\_SDR0\\_PP\\_TBIT\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg03c(&self) -> &I3cphyctrlreg03c {
        &self.i3cphyctrlreg03c
    }
    #[doc = "0xe40 - CR\\_I3C\\_SDR0\\_PP\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg040(&self) -> &I3cphyctrlreg040 {
        &self.i3cphyctrlreg040
    }
    #[doc = "0xe44 - CR\\_I3C\\_SDR1\\_PP\\_SCL\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg044(&self) -> &I3cphyctrlreg044 {
        &self.i3cphyctrlreg044
    }
    #[doc = "0xe48 - CR\\_I3C\\_SDR1\\_PP\\_TBIT\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg048(&self) -> &I3cphyctrlreg048 {
        &self.i3cphyctrlreg048
    }
    #[doc = "0xe4c - CR\\_I3C\\_SDR1\\_PP\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg04c(&self) -> &I3cphyctrlreg04c {
        &self.i3cphyctrlreg04c
    }
    #[doc = "0xe50 - CR\\_I3C\\_SDR2\\_PP\\_SCL\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg050(&self) -> &I3cphyctrlreg050 {
        &self.i3cphyctrlreg050
    }
    #[doc = "0xe54 - CR\\_I3C\\_SDR2\\_PP\\_TBIT\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg054(&self) -> &I3cphyctrlreg054 {
        &self.i3cphyctrlreg054
    }
    #[doc = "0xe58 - CR\\_I3C\\_SDR2\\_PP\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg058(&self) -> &I3cphyctrlreg058 {
        &self.i3cphyctrlreg058
    }
    #[doc = "0xe5c - CR\\_I3C\\_SDR3\\_PP\\_SCL\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg05c(&self) -> &I3cphyctrlreg05c {
        &self.i3cphyctrlreg05c
    }
    #[doc = "0xe60 - CR\\_I3C\\_SDR3\\_PP\\_TBIT\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg060(&self) -> &I3cphyctrlreg060 {
        &self.i3cphyctrlreg060
    }
    #[doc = "0xe64 - CR\\_I3C\\_SDR3\\_PP\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg064(&self) -> &I3cphyctrlreg064 {
        &self.i3cphyctrlreg064
    }
    #[doc = "0xe68 - CR\\_I3C\\_SDR4\\_PP\\_SCL\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg068(&self) -> &I3cphyctrlreg068 {
        &self.i3cphyctrlreg068
    }
    #[doc = "0xe6c - CR\\_I3C\\_SDR4\\_PP\\_TBIT\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg06c(&self) -> &I3cphyctrlreg06c {
        &self.i3cphyctrlreg06c
    }
    #[doc = "0xe70 - CR\\_I3C\\_SDR4\\_PP\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg070(&self) -> &I3cphyctrlreg070 {
        &self.i3cphyctrlreg070
    }
    #[doc = "0xe74 - CR\\_I3C\\_DDR\\_PP\\_SCL\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg074(&self) -> &I3cphyctrlreg074 {
        &self.i3cphyctrlreg074
    }
    #[doc = "0xe78 - CR\\_I3C\\_DDR\\_PP\\_TBIT\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg078(&self) -> &I3cphyctrlreg078 {
        &self.i3cphyctrlreg078
    }
    #[doc = "0xe7c - CR\\_DDR\\_PP\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg07c(&self) -> &I3cphyctrlreg07c {
        &self.i3cphyctrlreg07c
    }
    #[doc = "0xe80 - SR\\_P\\_PREPARE\\_SCL\\_SDA\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg080(&self) -> &I3cphyctrlreg080 {
        &self.i3cphyctrlreg080
    }
    #[doc = "0xe84 - CCR\\_TO\\_NCR\\_OVERLAP\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg084(&self) -> &I3cphyctrlreg084 {
        &self.i3cphyctrlreg084
    }
    #[doc = "0xe88 - CR\\_IBI\\_ADDR\\_ACK\\_PROLONG\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg088(&self) -> &I3cphyctrlreg088 {
        &self.i3cphyctrlreg088
    }
    #[doc = "0xe8c - TG\\_WR\\_ADDR\\_ACK\\_PROLONG"]
    #[inline(always)]
    pub const fn i3cphyctrlreg08c(&self) -> &I3cphyctrlreg08c {
        &self.i3cphyctrlreg08c
    }
    #[doc = "0xe90 - TG\\_SDA\\_TRAN\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg090(&self) -> &I3cphyctrlreg090 {
        &self.i3cphyctrlreg090
    }
    #[doc = "0xe94 - DDR\\_CMD\\_HANDOFF\\_EARLY\\_TM\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg094(&self) -> &I3cphyctrlreg094 {
        &self.i3cphyctrlreg094
    }
    #[doc = "0xe98 - CR\\_SCL\\_SDA\\_PULLUP\\_EN"]
    #[inline(always)]
    pub const fn i3cphyctrlreg098(&self) -> &I3cphyctrlreg098 {
        &self.i3cphyctrlreg098
    }
    #[doc = "0xe9c - SPECIAL\\_PATTERN\\_SET"]
    #[inline(always)]
    pub const fn i3cphyctrlreg09c(&self) -> &I3cphyctrlreg09c {
        &self.i3cphyctrlreg09c
    }
    #[doc = "0xea0 - SPECIAL\\_PATTERN\\_SW\\_OPT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0a0(&self) -> &I3cphyctrlreg0a0 {
        &self.i3cphyctrlreg0a0
    }
    #[doc = "0xea4 - SPECIAL\\_PATTERN\\_SCL\\_TOGGLE\\_SET"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0a4(&self) -> &I3cphyctrlreg0a4 {
        &self.i3cphyctrlreg0a4
    }
    #[doc = "0xea8 - SPECIAL\\_PATTERN\\_SCL\\_TOGGLE\\_PAT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0a8(&self) -> &I3cphyctrlreg0a8 {
        &self.i3cphyctrlreg0a8
    }
    #[doc = "0xeac - SPECIAL\\_PATTERN\\_SCL\\_TIEL\\_SET"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0ac(&self) -> &I3cphyctrlreg0ac {
        &self.i3cphyctrlreg0ac
    }
    #[doc = "0xeb4 - SDA\\_DETECTOR\\_CNT0"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0b4(&self) -> &I3cphyctrlreg0b4 {
        &self.i3cphyctrlreg0b4
    }
    #[doc = "0xeb8 - SDA\\_DETECTOR\\_CNT1"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0b8(&self) -> &I3cphyctrlreg0b8 {
        &self.i3cphyctrlreg0b8
    }
    #[doc = "0xebc - SDA\\_DETECTOR\\_CNT2"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0bc(&self) -> &I3cphyctrlreg0bc {
        &self.i3cphyctrlreg0bc
    }
    #[doc = "0xec0 - SDA\\_STUCK\\_SET1"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0c0(&self) -> &I3cphyctrlreg0c0 {
        &self.i3cphyctrlreg0c0
    }
    #[doc = "0xec4 - SDA\\_STUCK\\_READ"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0c4(&self) -> &I3cphyctrlreg0c4 {
        &self.i3cphyctrlreg0c4
    }
    #[doc = "0xec8 - READ\\_PHY\\_STATE\\_MACHINE"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0c8(&self) -> &I3cphyctrlreg0c8 {
        &self.i3cphyctrlreg0c8
    }
    #[doc = "0xecc - PHY\\_OPTION"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0cc(&self) -> &I3cphyctrlreg0cc {
        &self.i3cphyctrlreg0cc
    }
    #[doc = "0xed0 - CR\\_SCL\\_SDA\\_PULLUP\\_EN\\_ADDITIONAL"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0d0(&self) -> &I3cphyctrlreg0d0 {
        &self.i3cphyctrlreg0d0
    }
    #[doc = "0xed4 - SPIKE\\_FILTER"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0d4(&self) -> &I3cphyctrlreg0d4 {
        &self.i3cphyctrlreg0d4
    }
    #[doc = "0xed8 - SCL\\_SDA\\_TIMMIING\\_CNT\\_ADDITIONAL"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0d8(&self) -> &I3cphyctrlreg0d8 {
        &self.i3cphyctrlreg0d8
    }
    #[doc = "0xedc - BUS\\_FREE\\_TIME\\_CNT"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0dc(&self) -> &I3cphyctrlreg0dc {
        &self.i3cphyctrlreg0dc
    }
    #[doc = "0xee0 - SPECIAL\\_PATTERN\\_SET\\_ADDITIONAL"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0e0(&self) -> &I3cphyctrlreg0e0 {
        &self.i3cphyctrlreg0e0
    }
    #[doc = "0xee4 - BUS\\_CONTENTION\\_CHK0"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0e4(&self) -> &I3cphyctrlreg0e4 {
        &self.i3cphyctrlreg0e4
    }
    #[doc = "0xee8 - BUS\\_CONTENTION\\_CNT0"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0e8(&self) -> &I3cphyctrlreg0e8 {
        &self.i3cphyctrlreg0e8
    }
    #[doc = "0xeec - BUS\\_CONTENTION\\_CNT1"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0ec(&self) -> &I3cphyctrlreg0ec {
        &self.i3cphyctrlreg0ec
    }
    #[doc = "0xef0 - BUS\\_CONTENTION\\_CNT2"]
    #[inline(always)]
    pub const fn i3cphyctrlreg0f0(&self) -> &I3cphyctrlreg0f0 {
        &self.i3cphyctrlreg0f0
    }
    #[doc = "0xf00 - HW\\_ID\\_HEADER"]
    #[inline(always)]
    pub const fn hciextcap000(&self) -> &Hciextcap000 {
        &self.hciextcap000
    }
    #[doc = "0xf04 - HW\\_ID\\_MIPI\\_VENDOR"]
    #[inline(always)]
    pub const fn hciextcap004(&self) -> &Hciextcap004 {
        &self.hciextcap004
    }
    #[doc = "0xf08 - HW\\_ID\\_I3C\\_VER"]
    #[inline(always)]
    pub const fn hciextcap008(&self) -> &Hciextcap008 {
        &self.hciextcap008
    }
    #[doc = "0xf0c - HW\\_ID\\_I3C\\_PRODUCT"]
    #[inline(always)]
    pub const fn hciextcap00c(&self) -> &Hciextcap00c {
        &self.hciextcap00c
    }
    #[doc = "0xf10 - CTL\\_CFG\\_HEADER"]
    #[inline(always)]
    pub const fn hciextcap010(&self) -> &Hciextcap010 {
        &self.hciextcap010
    }
    #[doc = "0xf14 - CTL\\_CFG\\_OPERATION\\_MODE"]
    #[inline(always)]
    pub const fn hciextcap014(&self) -> &Hciextcap014 {
        &self.hciextcap014
    }
    #[doc = "0xf18 - EXTCAP\\_HEADER"]
    #[inline(always)]
    pub const fn hciextcap018(&self) -> &Hciextcap018 {
        &self.hciextcap018
    }
    #[doc = "0xf1c - EXTCAP\\_CTRL"]
    #[inline(always)]
    pub const fn hciextcap01c(&self) -> &Hciextcap01c {
        &self.hciextcap01c
    }
    #[doc = "0xf20 - EXTCAP\\_PHY"]
    #[inline(always)]
    pub const fn hciextcap020(&self) -> &Hciextcap020 {
        &self.hciextcap020
    }
    #[doc = "0xf24 - EXTCAP\\_DMAARB"]
    #[inline(always)]
    pub const fn hciextcap024(&self) -> &Hciextcap024 {
        &self.hciextcap024
    }
    #[doc = "0xf80 - DMA\\_MBUS\\_ARB\\_CTRL\\_0"]
    #[inline(always)]
    pub const fn hciextcap080(&self) -> &Hciextcap080 {
        &self.hciextcap080
    }
    #[doc = "0xf84 - DMA\\_MBUS\\_ARB\\_CLR\\_0"]
    #[inline(always)]
    pub const fn hciextcap084(&self) -> &Hciextcap084 {
        &self.hciextcap084
    }
    #[doc = "0xf90 - DMA\\_MBUS\\_ARB\\_DBG\\_0"]
    #[inline(always)]
    pub const fn hciextcap090(&self) -> &Hciextcap090 {
        &self.hciextcap090
    }
    #[doc = "0xf94 - DMA\\_MBUS\\_ARB\\_DBG\\_1"]
    #[inline(always)]
    pub const fn hciextcap094(&self) -> &Hciextcap094 {
        &self.hciextcap094
    }
    #[doc = "0xf98 - DMA\\_MBUS\\_ARB\\_DBG\\_2"]
    #[inline(always)]
    pub const fn hciextcap098(&self) -> &Hciextcap098 {
        &self.hciextcap098
    }
    #[doc = "0xf9c - DMA\\_MBUS\\_ARB\\_DBG\\_3"]
    #[inline(always)]
    pub const fn hciextcap09c(&self) -> &Hciextcap09c {
        &self.hciextcap09c
    }
}
#[doc = "HCICAPABILITY000 (rw) register accessor: HCI\\_VERSION\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability000`] module"]
#[doc(alias = "HCICAPABILITY000")]
pub type Hcicapability000 = crate::Reg<hcicapability000::Hcicapability000Spec>;
#[doc = "HCI\\_VERSION"]
pub mod hcicapability000;
#[doc = "HCICAPABILITY004 (rw) register accessor: HC\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability004`] module"]
#[doc(alias = "HCICAPABILITY004")]
pub type Hcicapability004 = crate::Reg<hcicapability004::Hcicapability004Spec>;
#[doc = "HC\\_CONTROL"]
pub mod hcicapability004;
#[doc = "HCICAPABILITY008 (rw) register accessor: CONTROLLER\\_DEVICE\\_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability008`] module"]
#[doc(alias = "HCICAPABILITY008")]
pub type Hcicapability008 = crate::Reg<hcicapability008::Hcicapability008Spec>;
#[doc = "CONTROLLER\\_DEVICE\\_ADDR"]
pub mod hcicapability008;
#[doc = "HCICAPABILITY00C (rw) register accessor: HC\\_CAPABILITIES\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability00c`] module"]
#[doc(alias = "HCICAPABILITY00C")]
pub type Hcicapability00c = crate::Reg<hcicapability00c::Hcicapability00cSpec>;
#[doc = "HC\\_CAPABILITIES"]
pub mod hcicapability00c;
#[doc = "HCICAPABILITY010 (rw) register accessor: RESET\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability010`] module"]
#[doc(alias = "HCICAPABILITY010")]
pub type Hcicapability010 = crate::Reg<hcicapability010::Hcicapability010Spec>;
#[doc = "RESET\\_CONTROL"]
pub mod hcicapability010;
#[doc = "HCICAPABILITY014 (rw) register accessor: PRESENT\\_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability014`] module"]
#[doc(alias = "HCICAPABILITY014")]
pub type Hcicapability014 = crate::Reg<hcicapability014::Hcicapability014Spec>;
#[doc = "PRESENT\\_STATE"]
pub mod hcicapability014;
#[doc = "HCICAPABILITY020 (rw) register accessor: INTR\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability020`] module"]
#[doc(alias = "HCICAPABILITY020")]
pub type Hcicapability020 = crate::Reg<hcicapability020::Hcicapability020Spec>;
#[doc = "INTR\\_STATUS"]
pub mod hcicapability020;
#[doc = "HCICAPABILITY024 (rw) register accessor: INTR\\_STATUS\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability024`] module"]
#[doc(alias = "HCICAPABILITY024")]
pub type Hcicapability024 = crate::Reg<hcicapability024::Hcicapability024Spec>;
#[doc = "INTR\\_STATUS\\_ENABLE"]
pub mod hcicapability024;
#[doc = "HCICAPABILITY028 (rw) register accessor: INTR\\_SIGNAL\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability028`] module"]
#[doc(alias = "HCICAPABILITY028")]
pub type Hcicapability028 = crate::Reg<hcicapability028::Hcicapability028Spec>;
#[doc = "INTR\\_SIGNAL\\_ENABLE"]
pub mod hcicapability028;
#[doc = "HCICAPABILITY02C (rw) register accessor: INTR\\_FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability02c`] module"]
#[doc(alias = "HCICAPABILITY02C")]
pub type Hcicapability02c = crate::Reg<hcicapability02c::Hcicapability02cSpec>;
#[doc = "INTR\\_FORCE"]
pub mod hcicapability02c;
#[doc = "HCICAPABILITY030 (rw) register accessor: DAT\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability030`] module"]
#[doc(alias = "HCICAPABILITY030")]
pub type Hcicapability030 = crate::Reg<hcicapability030::Hcicapability030Spec>;
#[doc = "DAT\\_SECTION\\_OFFSET"]
pub mod hcicapability030;
#[doc = "HCICAPABILITY034 (rw) register accessor: DCT\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability034`] module"]
#[doc(alias = "HCICAPABILITY034")]
pub type Hcicapability034 = crate::Reg<hcicapability034::Hcicapability034Spec>;
#[doc = "DCT\\_SECTION\\_OFFSET"]
pub mod hcicapability034;
#[doc = "HCICAPABILITY038 (rw) register accessor: RING\\_HEADERS\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability038`] module"]
#[doc(alias = "HCICAPABILITY038")]
pub type Hcicapability038 = crate::Reg<hcicapability038::Hcicapability038Spec>;
#[doc = "RING\\_HEADERS\\_SECTION\\_OFFSET"]
pub mod hcicapability038;
#[doc = "HCICAPABILITY03C (rw) register accessor: PIO\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability03c`] module"]
#[doc(alias = "HCICAPABILITY03C")]
pub type Hcicapability03c = crate::Reg<hcicapability03c::Hcicapability03cSpec>;
#[doc = "PIO\\_SECTION\\_OFFSET"]
pub mod hcicapability03c;
#[doc = "HCICAPABILITY040 (rw) register accessor: EXT\\_CAPS\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability040`] module"]
#[doc(alias = "HCICAPABILITY040")]
pub type Hcicapability040 = crate::Reg<hcicapability040::Hcicapability040Spec>;
#[doc = "EXT\\_CAPS\\_SECTION\\_OFFSET"]
pub mod hcicapability040;
#[doc = "HCICAPABILITY04C (rw) register accessor: INT\\_CTRL\\_CMDS\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability04c`] module"]
#[doc(alias = "HCICAPABILITY04C")]
pub type Hcicapability04c = crate::Reg<hcicapability04c::Hcicapability04cSpec>;
#[doc = "INT\\_CTRL\\_CMDS\\_EN"]
pub mod hcicapability04c;
#[doc = "HCICAPABILITY058 (rw) register accessor: IBI\\_NOTIFY\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability058`] module"]
#[doc(alias = "HCICAPABILITY058")]
pub type Hcicapability058 = crate::Reg<hcicapability058::Hcicapability058Spec>;
#[doc = "IBI\\_NOTIFY\\_CTRL"]
pub mod hcicapability058;
#[doc = "HCICAPABILITY060 (rw) register accessor: DEV\\_CTX\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability060`] module"]
#[doc(alias = "HCICAPABILITY060")]
pub type Hcicapability060 = crate::Reg<hcicapability060::Hcicapability060Spec>;
#[doc = "DEV\\_CTX\\_BASE\\_LO"]
pub mod hcicapability060;
#[doc = "HCICAPABILITY064 (rw) register accessor: DEV\\_CTX\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability064`] module"]
#[doc(alias = "HCICAPABILITY064")]
pub type Hcicapability064 = crate::Reg<hcicapability064::Hcicapability064Spec>;
#[doc = "DEV\\_CTX\\_BASE\\_HI"]
pub mod hcicapability064;
#[doc = "HCICAPABILITY068 (rw) register accessor: DEV\\_CTX\\_SG\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcicapability068`] module"]
#[doc(alias = "HCICAPABILITY068")]
pub type Hcicapability068 = crate::Reg<hcicapability068::Hcicapability068Spec>;
#[doc = "DEV\\_CTX\\_SG"]
pub mod hcicapability068;
#[doc = "HCIPIO000 (rw) register accessor: COMMAND\\_QUEUE\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio000`] module"]
#[doc(alias = "HCIPIO000")]
pub type Hcipio000 = crate::Reg<hcipio000::Hcipio000Spec>;
#[doc = "COMMAND\\_QUEUE\\_PORT"]
pub mod hcipio000;
#[doc = "HCIPIO004 (rw) register accessor: RESPONSE\\_QUEUE\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio004`] module"]
#[doc(alias = "HCIPIO004")]
pub type Hcipio004 = crate::Reg<hcipio004::Hcipio004Spec>;
#[doc = "RESPONSE\\_QUEUE\\_PORT"]
pub mod hcipio004;
#[doc = "HCIPIO008 (rw) register accessor: XFER\\_DATA\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio008`] module"]
#[doc(alias = "HCIPIO008")]
pub type Hcipio008 = crate::Reg<hcipio008::Hcipio008Spec>;
#[doc = "XFER\\_DATA\\_PORT"]
pub mod hcipio008;
#[doc = "HCIPIO00C (rw) register accessor: IBI\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio00c`] module"]
#[doc(alias = "HCIPIO00C")]
pub type Hcipio00c = crate::Reg<hcipio00c::Hcipio00cSpec>;
#[doc = "IBI\\_PORT"]
pub mod hcipio00c;
#[doc = "HCIPIO010 (rw) register accessor: QUEUE\\_THLD\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio010`] module"]
#[doc(alias = "HCIPIO010")]
pub type Hcipio010 = crate::Reg<hcipio010::Hcipio010Spec>;
#[doc = "QUEUE\\_THLD\\_CTRL"]
pub mod hcipio010;
#[doc = "HCIPIO014 (rw) register accessor: DATA\\_BUFFER\\_THLD\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio014`] module"]
#[doc(alias = "HCIPIO014")]
pub type Hcipio014 = crate::Reg<hcipio014::Hcipio014Spec>;
#[doc = "DATA\\_BUFFER\\_THLD\\_CTRL"]
pub mod hcipio014;
#[doc = "HCIPIO018 (rw) register accessor: QUEUE\\_SIZE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio018`] module"]
#[doc(alias = "HCIPIO018")]
pub type Hcipio018 = crate::Reg<hcipio018::Hcipio018Spec>;
#[doc = "QUEUE\\_SIZE"]
pub mod hcipio018;
#[doc = "HCIPIO020 (rw) register accessor: PIO\\_INTR\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio020`] module"]
#[doc(alias = "HCIPIO020")]
pub type Hcipio020 = crate::Reg<hcipio020::Hcipio020Spec>;
#[doc = "PIO\\_INTR\\_STATUS"]
pub mod hcipio020;
#[doc = "HCIPIO024 (rw) register accessor: PIO\\_INTR\\_STATUS\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio024`] module"]
#[doc(alias = "HCIPIO024")]
pub type Hcipio024 = crate::Reg<hcipio024::Hcipio024Spec>;
#[doc = "PIO\\_INTR\\_STATUS\\_ENABLE"]
pub mod hcipio024;
#[doc = "HCIPIO028 (rw) register accessor: PIO\\_INTR\\_SIGNAL\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio028`] module"]
#[doc(alias = "HCIPIO028")]
pub type Hcipio028 = crate::Reg<hcipio028::Hcipio028Spec>;
#[doc = "PIO\\_INTR\\_SIGNAL\\_ENABLE"]
pub mod hcipio028;
#[doc = "HCIPIO02C (rw) register accessor: PIO\\_INTR\\_FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcipio02c`] module"]
#[doc(alias = "HCIPIO02C")]
pub type Hcipio02c = crate::Reg<hcipio02c::Hcipio02cSpec>;
#[doc = "PIO\\_INTR\\_FORCE"]
pub mod hcipio02c;
#[doc = "HCIDATSINGLE000 (rw) register accessor: TARGET\\_DAT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidatsingle000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidatsingle000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcidatsingle000`] module"]
#[doc(alias = "HCIDATSINGLE000")]
pub type Hcidatsingle000 = crate::Reg<hcidatsingle000::Hcidatsingle000Spec>;
#[doc = "TARGET\\_DAT"]
pub mod hcidatsingle000;
#[doc = "HCIDCT000 (rw) register accessor: TARGET\\_DCT\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidct000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidct000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcidct000`] module"]
#[doc(alias = "HCIDCT000")]
pub type Hcidct000 = crate::Reg<hcidct000::Hcidct000Spec>;
#[doc = "TARGET\\_DCT\\_0"]
pub mod hcidct000;
#[doc = "HCIDCT004 (rw) register accessor: TARGET\\_DCT\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidct004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidct004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcidct004`] module"]
#[doc(alias = "HCIDCT004")]
pub type Hcidct004 = crate::Reg<hcidct004::Hcidct004Spec>;
#[doc = "TARGET\\_DCT\\_1"]
pub mod hcidct004;
#[doc = "HCIDCT008 (rw) register accessor: TARGET\\_DCT\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidct008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidct008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcidct008`] module"]
#[doc(alias = "HCIDCT008")]
pub type Hcidct008 = crate::Reg<hcidct008::Hcidct008Spec>;
#[doc = "TARGET\\_DCT\\_2"]
pub mod hcidct008;
#[doc = "HCIDCT00C (rw) register accessor: TARGET\\_DCT\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidct00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidct00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcidct00c`] module"]
#[doc(alias = "HCIDCT00C")]
pub type Hcidct00c = crate::Reg<hcidct00c::Hcidct00cSpec>;
#[doc = "TARGET\\_DCT\\_3"]
pub mod hcidct00c;
#[doc = "HCIRHS000 (rw) register accessor: RHS\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs000`] module"]
#[doc(alias = "HCIRHS000")]
pub type Hcirhs000 = crate::Reg<hcirhs000::Hcirhs000Spec>;
#[doc = "RHS\\_CONTROL"]
pub mod hcirhs000;
#[doc = "HCIRHS004 (rw) register accessor: RH0\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs004`] module"]
#[doc(alias = "HCIRHS004")]
pub type Hcirhs004 = crate::Reg<hcirhs004::Hcirhs004Spec>;
#[doc = "RH0\\_OFFSET"]
pub mod hcirhs004;
#[doc = "HCIRHS030 (rw) register accessor: CR\\_SETUP\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs030`] module"]
#[doc(alias = "HCIRHS030")]
pub type Hcirhs030 = crate::Reg<hcirhs030::Hcirhs030Spec>;
#[doc = "CR\\_SETUP"]
pub mod hcirhs030;
#[doc = "HCIRHS034 (rw) register accessor: IBI\\_SETUP\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs034`] module"]
#[doc(alias = "HCIRHS034")]
pub type Hcirhs034 = crate::Reg<hcirhs034::Hcirhs034Spec>;
#[doc = "IBI\\_SETUP"]
pub mod hcirhs034;
#[doc = "HCIRHS038 (rw) register accessor: CHUNK\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs038`] module"]
#[doc(alias = "HCIRHS038")]
pub type Hcirhs038 = crate::Reg<hcirhs038::Hcirhs038Spec>;
#[doc = "CHUNK\\_CONTROL"]
pub mod hcirhs038;
#[doc = "HCIRHS040 (rw) register accessor: RH\\_INTR\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs040`] module"]
#[doc(alias = "HCIRHS040")]
pub type Hcirhs040 = crate::Reg<hcirhs040::Hcirhs040Spec>;
#[doc = "RH\\_INTR\\_STATUS"]
pub mod hcirhs040;
#[doc = "HCIRHS044 (rw) register accessor: RH\\_INTR\\_STATUS\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs044`] module"]
#[doc(alias = "HCIRHS044")]
pub type Hcirhs044 = crate::Reg<hcirhs044::Hcirhs044Spec>;
#[doc = "RH\\_INTR\\_STATUS\\_ENABLE"]
pub mod hcirhs044;
#[doc = "HCIRHS048 (rw) register accessor: RH\\_INTR\\_SIGNAL\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs048`] module"]
#[doc(alias = "HCIRHS048")]
pub type Hcirhs048 = crate::Reg<hcirhs048::Hcirhs048Spec>;
#[doc = "RH\\_INTR\\_SIGNAL\\_ENABLE"]
pub mod hcirhs048;
#[doc = "HCIRHS04C (rw) register accessor: RH\\_INTR\\_FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs04c`] module"]
#[doc(alias = "HCIRHS04C")]
pub type Hcirhs04c = crate::Reg<hcirhs04c::Hcirhs04cSpec>;
#[doc = "RH\\_INTR\\_FORCE"]
pub mod hcirhs04c;
#[doc = "HCIRHS050 (rw) register accessor: RH\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs050`] module"]
#[doc(alias = "HCIRHS050")]
pub type Hcirhs050 = crate::Reg<hcirhs050::Hcirhs050Spec>;
#[doc = "RH\\_STATUS"]
pub mod hcirhs050;
#[doc = "HCIRHS054 (rw) register accessor: RH\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs054`] module"]
#[doc(alias = "HCIRHS054")]
pub type Hcirhs054 = crate::Reg<hcirhs054::Hcirhs054Spec>;
#[doc = "RH\\_CONTROL"]
pub mod hcirhs054;
#[doc = "HCIRHS058 (rw) register accessor: RH\\_OPERATION1\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs058`] module"]
#[doc(alias = "HCIRHS058")]
pub type Hcirhs058 = crate::Reg<hcirhs058::Hcirhs058Spec>;
#[doc = "RH\\_OPERATION1"]
pub mod hcirhs058;
#[doc = "HCIRHS05C (rw) register accessor: RH\\_OPERATION2\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs05c`] module"]
#[doc(alias = "HCIRHS05C")]
pub type Hcirhs05c = crate::Reg<hcirhs05c::Hcirhs05cSpec>;
#[doc = "RH\\_OPERATION2"]
pub mod hcirhs05c;
#[doc = "HCIRHS060 (rw) register accessor: RH\\_CMD\\_RING\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs060`] module"]
#[doc(alias = "HCIRHS060")]
pub type Hcirhs060 = crate::Reg<hcirhs060::Hcirhs060Spec>;
#[doc = "RH\\_CMD\\_RING\\_BASE\\_LO"]
pub mod hcirhs060;
#[doc = "HCIRHS064 (rw) register accessor: RH\\_CMD\\_RING\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs064`] module"]
#[doc(alias = "HCIRHS064")]
pub type Hcirhs064 = crate::Reg<hcirhs064::Hcirhs064Spec>;
#[doc = "RH\\_CMD\\_RING\\_BASE\\_HI"]
pub mod hcirhs064;
#[doc = "HCIRHS068 (rw) register accessor: RH\\_RESP\\_RING\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs068`] module"]
#[doc(alias = "HCIRHS068")]
pub type Hcirhs068 = crate::Reg<hcirhs068::Hcirhs068Spec>;
#[doc = "RH\\_RESP\\_RING\\_BASE\\_LO"]
pub mod hcirhs068;
#[doc = "HCIRHS06C (rw) register accessor: RH\\_RESP\\_RING\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs06c`] module"]
#[doc(alias = "HCIRHS06C")]
pub type Hcirhs06c = crate::Reg<hcirhs06c::Hcirhs06cSpec>;
#[doc = "RH\\_RESP\\_RING\\_BASE\\_HI"]
pub mod hcirhs06c;
#[doc = "HCIRHS070 (rw) register accessor: RH\\_IBI\\_STATUS\\_RING\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs070`] module"]
#[doc(alias = "HCIRHS070")]
pub type Hcirhs070 = crate::Reg<hcirhs070::Hcirhs070Spec>;
#[doc = "RH\\_IBI\\_STATUS\\_RING\\_BASE\\_LO"]
pub mod hcirhs070;
#[doc = "HCIRHS074 (rw) register accessor: RH\\_IBI\\_STATUS\\_RING\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs074`] module"]
#[doc(alias = "HCIRHS074")]
pub type Hcirhs074 = crate::Reg<hcirhs074::Hcirhs074Spec>;
#[doc = "RH\\_IBI\\_STATUS\\_RING\\_BASE\\_HI"]
pub mod hcirhs074;
#[doc = "HCIRHS078 (rw) register accessor: RH\\_IBI\\_DATA\\_RING\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs078`] module"]
#[doc(alias = "HCIRHS078")]
pub type Hcirhs078 = crate::Reg<hcirhs078::Hcirhs078Spec>;
#[doc = "RH\\_IBI\\_DATA\\_RING\\_BASE\\_LO"]
pub mod hcirhs078;
#[doc = "HCIRHS07C (rw) register accessor: RH\\_IBI\\_DATA\\_RING\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcirhs07c`] module"]
#[doc(alias = "HCIRHS07C")]
pub type Hcirhs07c = crate::Reg<hcirhs07c::Hcirhs07cSpec>;
#[doc = "RH\\_IBI\\_DATA\\_RING\\_BASE\\_HI"]
pub mod hcirhs07c;
#[doc = "I3CCONTROL000 (rw) register accessor: I3C\\_CONTROL\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol000`] module"]
#[doc(alias = "I3CCONTROL000")]
pub type I3ccontrol000 = crate::Reg<i3ccontrol000::I3ccontrol000Spec>;
#[doc = "I3C\\_CONTROL\\_0"]
pub mod i3ccontrol000;
#[doc = "I3CCONTROL004 (rw) register accessor: I3C\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol004`] module"]
#[doc(alias = "I3CCONTROL004")]
pub type I3ccontrol004 = crate::Reg<i3ccontrol004::I3ccontrol004Spec>;
#[doc = "I3C\\_STATUS"]
pub mod i3ccontrol004;
#[doc = "I3CCONTROL008 (rw) register accessor: I3C\\_MST\\_MRL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol008`] module"]
#[doc(alias = "I3CCONTROL008")]
pub type I3ccontrol008 = crate::Reg<i3ccontrol008::I3ccontrol008Spec>;
#[doc = "I3C\\_MST\\_MRL"]
pub mod i3ccontrol008;
#[doc = "I3CCONTROL00C (rw) register accessor: I3C\\_STATUS\\_C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol00c`] module"]
#[doc(alias = "I3CCONTROL00C")]
pub type I3ccontrol00c = crate::Reg<i3ccontrol00c::I3ccontrol00cSpec>;
#[doc = "I3C\\_STATUS\\_C"]
pub mod i3ccontrol00c;
#[doc = "I3CCONTROL010 (rw) register accessor: I3C\\_DAA\\_INDEX\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol010`] module"]
#[doc(alias = "I3CCONTROL010")]
pub type I3ccontrol010 = crate::Reg<i3ccontrol010::I3ccontrol010Spec>;
#[doc = "I3C\\_DAA\\_INDEX\\_0"]
pub mod i3ccontrol010;
#[doc = "I3CCONTROL014 (rw) register accessor: I3C\\_DAA\\_INDEX\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol014`] module"]
#[doc(alias = "I3CCONTROL014")]
pub type I3ccontrol014 = crate::Reg<i3ccontrol014::I3ccontrol014Spec>;
#[doc = "I3C\\_DAA\\_INDEX\\_1"]
pub mod i3ccontrol014;
#[doc = "I3CCONTROL018 (rw) register accessor: I3C\\_DAA\\_INDEX\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol018`] module"]
#[doc(alias = "I3CCONTROL018")]
pub type I3ccontrol018 = crate::Reg<i3ccontrol018::I3ccontrol018Spec>;
#[doc = "I3C\\_DAA\\_INDEX\\_2"]
pub mod i3ccontrol018;
#[doc = "I3CCONTROL01C (rw) register accessor: I3C\\_DAA\\_INDEX\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol01c`] module"]
#[doc(alias = "I3CCONTROL01C")]
pub type I3ccontrol01c = crate::Reg<i3ccontrol01c::I3ccontrol01cSpec>;
#[doc = "I3C\\_DAA\\_INDEX\\_3"]
pub mod i3ccontrol01c;
#[doc = "I3CCONTROL020 (rw) register accessor: I3C\\_AUTOCMD\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol020`] module"]
#[doc(alias = "I3CCONTROL020")]
pub type I3ccontrol020 = crate::Reg<i3ccontrol020::I3ccontrol020Spec>;
#[doc = "I3C\\_AUTOCMD\\_0"]
pub mod i3ccontrol020;
#[doc = "I3CCONTROL024 (rw) register accessor: I3C\\_AUTOCMD\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol024`] module"]
#[doc(alias = "I3CCONTROL024")]
pub type I3ccontrol024 = crate::Reg<i3ccontrol024::I3ccontrol024Spec>;
#[doc = "I3C\\_AUTOCMD\\_1"]
pub mod i3ccontrol024;
#[doc = "I3CCONTROL028 (rw) register accessor: I3C\\_AUTOCMD\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol028`] module"]
#[doc(alias = "I3CCONTROL028")]
pub type I3ccontrol028 = crate::Reg<i3ccontrol028::I3ccontrol028Spec>;
#[doc = "I3C\\_AUTOCMD\\_2"]
pub mod i3ccontrol028;
#[doc = "I3CCONTROL02C (rw) register accessor: I3C\\_AUTOCMD\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol02c`] module"]
#[doc(alias = "I3CCONTROL02C")]
pub type I3ccontrol02c = crate::Reg<i3ccontrol02c::I3ccontrol02cSpec>;
#[doc = "I3C\\_AUTOCMD\\_3"]
pub mod i3ccontrol02c;
#[doc = "I3CCONTROL030 (rw) register accessor: I3C\\_AUTOCMD\\_4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol030`] module"]
#[doc(alias = "I3CCONTROL030")]
pub type I3ccontrol030 = crate::Reg<i3ccontrol030::I3ccontrol030Spec>;
#[doc = "I3C\\_AUTOCMD\\_4"]
pub mod i3ccontrol030;
#[doc = "I3CCONTROL034 (rw) register accessor: I3C\\_AUTOCMD\\_5\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol034`] module"]
#[doc(alias = "I3CCONTROL034")]
pub type I3ccontrol034 = crate::Reg<i3ccontrol034::I3ccontrol034Spec>;
#[doc = "I3C\\_AUTOCMD\\_5"]
pub mod i3ccontrol034;
#[doc = "I3CCONTROL038 (rw) register accessor: I3C\\_AUTOCMD\\_6\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol038`] module"]
#[doc(alias = "I3CCONTROL038")]
pub type I3ccontrol038 = crate::Reg<i3ccontrol038::I3ccontrol038Spec>;
#[doc = "I3C\\_AUTOCMD\\_6"]
pub mod i3ccontrol038;
#[doc = "I3CCONTROL03C (rw) register accessor: I3C\\_AUTOCMD\\_7\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol03c`] module"]
#[doc(alias = "I3CCONTROL03C")]
pub type I3ccontrol03c = crate::Reg<i3ccontrol03c::I3ccontrol03cSpec>;
#[doc = "I3C\\_AUTOCMD\\_7"]
pub mod i3ccontrol03c;
#[doc = "I3CCONTROL040 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_040\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol040`] module"]
#[doc(alias = "I3CCONTROL040")]
pub type I3ccontrol040 = crate::Reg<i3ccontrol040::I3ccontrol040Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_040"]
pub mod i3ccontrol040;
#[doc = "I3CCONTROL044 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_044\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol044`] module"]
#[doc(alias = "I3CCONTROL044")]
pub type I3ccontrol044 = crate::Reg<i3ccontrol044::I3ccontrol044Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_044"]
pub mod i3ccontrol044;
#[doc = "I3CCONTROL048 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_048\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol048`] module"]
#[doc(alias = "I3CCONTROL048")]
pub type I3ccontrol048 = crate::Reg<i3ccontrol048::I3ccontrol048Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_048"]
pub mod i3ccontrol048;
#[doc = "I3CCONTROL04C (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_04C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol04c`] module"]
#[doc(alias = "I3CCONTROL04C")]
pub type I3ccontrol04c = crate::Reg<i3ccontrol04c::I3ccontrol04cSpec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_04C"]
pub mod i3ccontrol04c;
#[doc = "I3CCONTROL050 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_050\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol050`] module"]
#[doc(alias = "I3CCONTROL050")]
pub type I3ccontrol050 = crate::Reg<i3ccontrol050::I3ccontrol050Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_050"]
pub mod i3ccontrol050;
#[doc = "I3CCONTROL054 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_054\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol054`] module"]
#[doc(alias = "I3CCONTROL054")]
pub type I3ccontrol054 = crate::Reg<i3ccontrol054::I3ccontrol054Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_054"]
pub mod i3ccontrol054;
#[doc = "I3CCONTROL058 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_058\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol058`] module"]
#[doc(alias = "I3CCONTROL058")]
pub type I3ccontrol058 = crate::Reg<i3ccontrol058::I3ccontrol058Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_058"]
pub mod i3ccontrol058;
#[doc = "I3CCONTROL05C (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_05C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol05c`] module"]
#[doc(alias = "I3CCONTROL05C")]
pub type I3ccontrol05c = crate::Reg<i3ccontrol05c::I3ccontrol05cSpec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_05C"]
pub mod i3ccontrol05c;
#[doc = "I3CCONTROL060 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_060\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol060`] module"]
#[doc(alias = "I3CCONTROL060")]
pub type I3ccontrol060 = crate::Reg<i3ccontrol060::I3ccontrol060Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_060"]
pub mod i3ccontrol060;
#[doc = "I3CCONTROL064 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_064\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol064`] module"]
#[doc(alias = "I3CCONTROL064")]
pub type I3ccontrol064 = crate::Reg<i3ccontrol064::I3ccontrol064Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_064"]
pub mod i3ccontrol064;
#[doc = "I3CCONTROL068 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_068\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol068`] module"]
#[doc(alias = "I3CCONTROL068")]
pub type I3ccontrol068 = crate::Reg<i3ccontrol068::I3ccontrol068Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_068"]
pub mod i3ccontrol068;
#[doc = "I3CCONTROL06C (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_06C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol06c`] module"]
#[doc(alias = "I3CCONTROL06C")]
pub type I3ccontrol06c = crate::Reg<i3ccontrol06c::I3ccontrol06cSpec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_06C"]
pub mod i3ccontrol06c;
#[doc = "I3CCONTROL070 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_070\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol070`] module"]
#[doc(alias = "I3CCONTROL070")]
pub type I3ccontrol070 = crate::Reg<i3ccontrol070::I3ccontrol070Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_070"]
pub mod i3ccontrol070;
#[doc = "I3CCONTROL074 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_074\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol074`] module"]
#[doc(alias = "I3CCONTROL074")]
pub type I3ccontrol074 = crate::Reg<i3ccontrol074::I3ccontrol074Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_074"]
pub mod i3ccontrol074;
#[doc = "I3CCONTROL078 (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_078\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol078`] module"]
#[doc(alias = "I3CCONTROL078")]
pub type I3ccontrol078 = crate::Reg<i3ccontrol078::I3ccontrol078Spec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_078"]
pub mod i3ccontrol078;
#[doc = "I3CCONTROL07C (rw) register accessor: I3C\\_AUTOCMD\\_SEL\\_07C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol07c`] module"]
#[doc(alias = "I3CCONTROL07C")]
pub type I3ccontrol07c = crate::Reg<i3ccontrol07c::I3ccontrol07cSpec>;
#[doc = "I3C\\_AUTOCMD\\_SEL\\_07C"]
pub mod i3ccontrol07c;
#[doc = "I3CCONTROL080 (rw) register accessor: I3C\\_WDMA\\_CTL\\_080\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol080`] module"]
#[doc(alias = "I3CCONTROL080")]
pub type I3ccontrol080 = crate::Reg<i3ccontrol080::I3ccontrol080Spec>;
#[doc = "I3C\\_WDMA\\_CTL\\_080"]
pub mod i3ccontrol080;
#[doc = "I3CCONTROL084 (rw) register accessor: I3C\\_WDMA\\_CTL\\_084\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol084`] module"]
#[doc(alias = "I3CCONTROL084")]
pub type I3ccontrol084 = crate::Reg<i3ccontrol084::I3ccontrol084Spec>;
#[doc = "I3C\\_WDMA\\_CTL\\_084"]
pub mod i3ccontrol084;
#[doc = "I3CCONTROL088 (rw) register accessor: I3C\\_WDMA\\_CTL\\_088\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol088`] module"]
#[doc(alias = "I3CCONTROL088")]
pub type I3ccontrol088 = crate::Reg<i3ccontrol088::I3ccontrol088Spec>;
#[doc = "I3C\\_WDMA\\_CTL\\_088"]
pub mod i3ccontrol088;
#[doc = "I3CCONTROL090 (rw) register accessor: I3C\\_RDMA\\_CTL\\_090\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol090`] module"]
#[doc(alias = "I3CCONTROL090")]
pub type I3ccontrol090 = crate::Reg<i3ccontrol090::I3ccontrol090Spec>;
#[doc = "I3C\\_RDMA\\_CTL\\_090"]
pub mod i3ccontrol090;
#[doc = "I3CCONTROL094 (rw) register accessor: I3C\\_RDMA\\_CTL\\_094\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol094`] module"]
#[doc(alias = "I3CCONTROL094")]
pub type I3ccontrol094 = crate::Reg<i3ccontrol094::I3ccontrol094Spec>;
#[doc = "I3C\\_RDMA\\_CTL\\_094"]
pub mod i3ccontrol094;
#[doc = "I3CCONTROL098 (rw) register accessor: I3C\\_RDMA\\_CTL\\_098\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol098`] module"]
#[doc(alias = "I3CCONTROL098")]
pub type I3ccontrol098 = crate::Reg<i3ccontrol098::I3ccontrol098Spec>;
#[doc = "I3C\\_RDMA\\_CTL\\_098"]
pub mod i3ccontrol098;
#[doc = "I3CCONTROL09C (rw) register accessor: I3C\\_RING\\_CTL\\_09C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol09c`] module"]
#[doc(alias = "I3CCONTROL09C")]
pub type I3ccontrol09c = crate::Reg<i3ccontrol09c::I3ccontrol09cSpec>;
#[doc = "I3C\\_RING\\_CTL\\_09C"]
pub mod i3ccontrol09c;
#[doc = "I3CCONTROL0A0 (rw) register accessor: I3C\\_SLV\\_CTL\\_0A0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0a0`] module"]
#[doc(alias = "I3CCONTROL0A0")]
pub type I3ccontrol0a0 = crate::Reg<i3ccontrol0a0::I3ccontrol0a0Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0A0"]
pub mod i3ccontrol0a0;
#[doc = "I3CCONTROL0A4 (rw) register accessor: I3C\\_SLV\\_CTL\\_0A4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0a4`] module"]
#[doc(alias = "I3CCONTROL0A4")]
pub type I3ccontrol0a4 = crate::Reg<i3ccontrol0a4::I3ccontrol0a4Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0A4"]
pub mod i3ccontrol0a4;
#[doc = "I3CCONTROL0A8 (rw) register accessor: I3C\\_SLV\\_CTL\\_0A8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0a8`] module"]
#[doc(alias = "I3CCONTROL0A8")]
pub type I3ccontrol0a8 = crate::Reg<i3ccontrol0a8::I3ccontrol0a8Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0A8"]
pub mod i3ccontrol0a8;
#[doc = "I3CCONTROL0AC (rw) register accessor: I3C\\_SLV\\_CTL\\_0AC\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0ac`] module"]
#[doc(alias = "I3CCONTROL0AC")]
pub type I3ccontrol0ac = crate::Reg<i3ccontrol0ac::I3ccontrol0acSpec>;
#[doc = "I3C\\_SLV\\_CTL\\_0AC"]
pub mod i3ccontrol0ac;
#[doc = "I3CCONTROL0B0 (rw) register accessor: I3C\\_SLV\\_CTL\\_0B0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0b0`] module"]
#[doc(alias = "I3CCONTROL0B0")]
pub type I3ccontrol0b0 = crate::Reg<i3ccontrol0b0::I3ccontrol0b0Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0B0"]
pub mod i3ccontrol0b0;
#[doc = "I3CCONTROL0B4 (rw) register accessor: I3C\\_SLV\\_CTL\\_0B4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0b4`] module"]
#[doc(alias = "I3CCONTROL0B4")]
pub type I3ccontrol0b4 = crate::Reg<i3ccontrol0b4::I3ccontrol0b4Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0B4"]
pub mod i3ccontrol0b4;
#[doc = "I3CCONTROL0B8 (rw) register accessor: I3C\\_SLV\\_CTL\\_0B8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0b8`] module"]
#[doc(alias = "I3CCONTROL0B8")]
pub type I3ccontrol0b8 = crate::Reg<i3ccontrol0b8::I3ccontrol0b8Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0B8"]
pub mod i3ccontrol0b8;
#[doc = "I3CCONTROL0BC (rw) register accessor: I3C\\_SLV\\_CTL\\_0BC\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0bc`] module"]
#[doc(alias = "I3CCONTROL0BC")]
pub type I3ccontrol0bc = crate::Reg<i3ccontrol0bc::I3ccontrol0bcSpec>;
#[doc = "I3C\\_SLV\\_CTL\\_0BC"]
pub mod i3ccontrol0bc;
#[doc = "I3CCONTROL0C0 (rw) register accessor: I3C\\_SLV\\_CTL\\_0C0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0c0`] module"]
#[doc(alias = "I3CCONTROL0C0")]
pub type I3ccontrol0c0 = crate::Reg<i3ccontrol0c0::I3ccontrol0c0Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0C0"]
pub mod i3ccontrol0c0;
#[doc = "I3CCONTROL0C4 (rw) register accessor: I3C\\_SLV\\_CTL\\_0C4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0c4`] module"]
#[doc(alias = "I3CCONTROL0C4")]
pub type I3ccontrol0c4 = crate::Reg<i3ccontrol0c4::I3ccontrol0c4Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0C4"]
pub mod i3ccontrol0c4;
#[doc = "I3CCONTROL0C8 (rw) register accessor: I3C\\_SLV\\_CTL\\_0C8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0c8`] module"]
#[doc(alias = "I3CCONTROL0C8")]
pub type I3ccontrol0c8 = crate::Reg<i3ccontrol0c8::I3ccontrol0c8Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0C8"]
pub mod i3ccontrol0c8;
#[doc = "I3CCONTROL0CC (rw) register accessor: I3C\\_SLV\\_CTL\\_0CC\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0cc`] module"]
#[doc(alias = "I3CCONTROL0CC")]
pub type I3ccontrol0cc = crate::Reg<i3ccontrol0cc::I3ccontrol0ccSpec>;
#[doc = "I3C\\_SLV\\_CTL\\_0CC"]
pub mod i3ccontrol0cc;
#[doc = "I3CCONTROL0D0 (rw) register accessor: I3C\\_SLV\\_CTL\\_0D0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0d0`] module"]
#[doc(alias = "I3CCONTROL0D0")]
pub type I3ccontrol0d0 = crate::Reg<i3ccontrol0d0::I3ccontrol0d0Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0D0"]
pub mod i3ccontrol0d0;
#[doc = "I3CCONTROL0D4 (rw) register accessor: I3C\\_SLV\\_CTL\\_0D4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0d4`] module"]
#[doc(alias = "I3CCONTROL0D4")]
pub type I3ccontrol0d4 = crate::Reg<i3ccontrol0d4::I3ccontrol0d4Spec>;
#[doc = "I3C\\_SLV\\_CTL\\_0D4"]
pub mod i3ccontrol0d4;
#[doc = "I3CCONTROL0D8 (rw) register accessor: I3C\\_QUEUE\\_PTR\\_0D8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0d8`] module"]
#[doc(alias = "I3CCONTROL0D8")]
pub type I3ccontrol0d8 = crate::Reg<i3ccontrol0d8::I3ccontrol0d8Spec>;
#[doc = "I3C\\_QUEUE\\_PTR\\_0D8"]
pub mod i3ccontrol0d8;
#[doc = "I3CCONTROL0DC (rw) register accessor: I3C\\_QUEUE\\_PTR\\_0DC\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0dc`] module"]
#[doc(alias = "I3CCONTROL0DC")]
pub type I3ccontrol0dc = crate::Reg<i3ccontrol0dc::I3ccontrol0dcSpec>;
#[doc = "I3C\\_QUEUE\\_PTR\\_0DC"]
pub mod i3ccontrol0dc;
#[doc = "I3CCONTROL0E0 (rw) register accessor: I3C\\_INTR\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0e0`] module"]
#[doc(alias = "I3CCONTROL0E0")]
pub type I3ccontrol0e0 = crate::Reg<i3ccontrol0e0::I3ccontrol0e0Spec>;
#[doc = "I3C\\_INTR\\_STATUS"]
pub mod i3ccontrol0e0;
#[doc = "I3CCONTROL0E4 (rw) register accessor: I3C\\_INTR\\_STATUS\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0e4`] module"]
#[doc(alias = "I3CCONTROL0E4")]
pub type I3ccontrol0e4 = crate::Reg<i3ccontrol0e4::I3ccontrol0e4Spec>;
#[doc = "I3C\\_INTR\\_STATUS\\_ENABLE"]
pub mod i3ccontrol0e4;
#[doc = "I3CCONTROL0E8 (rw) register accessor: I3C\\_INTR\\_SIGNAL\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0e8`] module"]
#[doc(alias = "I3CCONTROL0E8")]
pub type I3ccontrol0e8 = crate::Reg<i3ccontrol0e8::I3ccontrol0e8Spec>;
#[doc = "I3C\\_INTR\\_SIGNAL\\_ENABLE"]
pub mod i3ccontrol0e8;
#[doc = "I3CCONTROL0EC (rw) register accessor: I3C\\_INTR\\_FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0ec`] module"]
#[doc(alias = "I3CCONTROL0EC")]
pub type I3ccontrol0ec = crate::Reg<i3ccontrol0ec::I3ccontrol0ecSpec>;
#[doc = "I3C\\_INTR\\_FORCE"]
pub mod i3ccontrol0ec;
#[doc = "I3CCONTROL0F0 (rw) register accessor: I3C\\_INTR\\_STATUS\\_F0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0f0`] module"]
#[doc(alias = "I3CCONTROL0F0")]
pub type I3ccontrol0f0 = crate::Reg<i3ccontrol0f0::I3ccontrol0f0Spec>;
#[doc = "I3C\\_INTR\\_STATUS\\_F0"]
pub mod i3ccontrol0f0;
#[doc = "I3CCONTROL0F4 (rw) register accessor: I3C\\_INTR\\_PROCESS\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0f4`] module"]
#[doc(alias = "I3CCONTROL0F4")]
pub type I3ccontrol0f4 = crate::Reg<i3ccontrol0f4::I3ccontrol0f4Spec>;
#[doc = "I3C\\_INTR\\_PROCESS"]
pub mod i3ccontrol0f4;
#[doc = "I3CCONTROL0F8 (rw) register accessor: I3C\\_IBI\\_TIMEOUT\\_F8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3ccontrol0f8`] module"]
#[doc(alias = "I3CCONTROL0F8")]
pub type I3ccontrol0f8 = crate::Reg<i3ccontrol0f8::I3ccontrol0f8Spec>;
#[doc = "I3C\\_IBI\\_TIMEOUT\\_F8"]
pub mod i3ccontrol0f8;
#[doc = "I3CPHYCTRLREG000 (rw) register accessor: EXT\\_CAP\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg000`] module"]
#[doc(alias = "I3CPHYCTRLREG000")]
pub type I3cphyctrlreg000 = crate::Reg<i3cphyctrlreg000::I3cphyctrlreg000Spec>;
#[doc = "EXT\\_CAP\\_OFFSET"]
pub mod i3cphyctrlreg000;
#[doc = "I3CPHYCTRLREG004 (rw) register accessor: SW\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg004`] module"]
#[doc(alias = "I3CPHYCTRLREG004")]
pub type I3cphyctrlreg004 = crate::Reg<i3cphyctrlreg004::I3cphyctrlreg004Spec>;
#[doc = "SW\\_CTRL"]
pub mod i3cphyctrlreg004;
#[doc = "I3CPHYCTRLREG008 (rw) register accessor: CR\\_I2C\\_OD\\_FM\\_STA\\_STO\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg008`] module"]
#[doc(alias = "I3CPHYCTRLREG008")]
pub type I3cphyctrlreg008 = crate::Reg<i3cphyctrlreg008::I3cphyctrlreg008Spec>;
#[doc = "CR\\_I2C\\_OD\\_FM\\_STA\\_STO\\_CNT"]
pub mod i3cphyctrlreg008;
#[doc = "I3CPHYCTRLREG00C (rw) register accessor: CR\\_I2C\\_OD\\_FM\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg00c`] module"]
#[doc(alias = "I3CPHYCTRLREG00C")]
pub type I3cphyctrlreg00c = crate::Reg<i3cphyctrlreg00c::I3cphyctrlreg00cSpec>;
#[doc = "CR\\_I2C\\_OD\\_FM\\_SCL\\_CNT"]
pub mod i3cphyctrlreg00c;
#[doc = "I3CPHYCTRLREG010 (rw) register accessor: CR\\_I2C\\_OD\\_FM\\_ACK\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg010`] module"]
#[doc(alias = "I3CPHYCTRLREG010")]
pub type I3cphyctrlreg010 = crate::Reg<i3cphyctrlreg010::I3cphyctrlreg010Spec>;
#[doc = "CR\\_I2C\\_OD\\_FM\\_ACK\\_CNT"]
pub mod i3cphyctrlreg010;
#[doc = "I3CPHYCTRLREG014 (rw) register accessor: CR\\_I2C\\_OD\\_FM\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg014`] module"]
#[doc(alias = "I3CPHYCTRLREG014")]
pub type I3cphyctrlreg014 = crate::Reg<i3cphyctrlreg014::I3cphyctrlreg014Spec>;
#[doc = "CR\\_I2C\\_OD\\_FM\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg014;
#[doc = "I3CPHYCTRLREG018 (rw) register accessor: CR\\_I2C\\_OD\\_FMP\\_STA\\_STO\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg018`] module"]
#[doc(alias = "I3CPHYCTRLREG018")]
pub type I3cphyctrlreg018 = crate::Reg<i3cphyctrlreg018::I3cphyctrlreg018Spec>;
#[doc = "CR\\_I2C\\_OD\\_FMP\\_STA\\_STO\\_CNT"]
pub mod i3cphyctrlreg018;
#[doc = "I3CPHYCTRLREG01C (rw) register accessor: CR\\_I2C\\_OD\\_FMP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg01c`] module"]
#[doc(alias = "I3CPHYCTRLREG01C")]
pub type I3cphyctrlreg01c = crate::Reg<i3cphyctrlreg01c::I3cphyctrlreg01cSpec>;
#[doc = "CR\\_I2C\\_OD\\_FMP\\_SCL\\_CNT"]
pub mod i3cphyctrlreg01c;
#[doc = "I3CPHYCTRLREG020 (rw) register accessor: CR\\_I2C\\_OD\\_FMP\\_ACK\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg020`] module"]
#[doc(alias = "I3CPHYCTRLREG020")]
pub type I3cphyctrlreg020 = crate::Reg<i3cphyctrlreg020::I3cphyctrlreg020Spec>;
#[doc = "CR\\_I2C\\_OD\\_FMP\\_ACK\\_CNT"]
pub mod i3cphyctrlreg020;
#[doc = "I3CPHYCTRLREG024 (rw) register accessor: CR\\_I2C\\_OD\\_FMP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg024`] module"]
#[doc(alias = "I3CPHYCTRLREG024")]
pub type I3cphyctrlreg024 = crate::Reg<i3cphyctrlreg024::I3cphyctrlreg024Spec>;
#[doc = "CR\\_I2C\\_OD\\_FMP\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg024;
#[doc = "I3CPHYCTRLREG028 (rw) register accessor: CR\\_I3C\\_OD\\_STA\\_STO\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg028`] module"]
#[doc(alias = "I3CPHYCTRLREG028")]
pub type I3cphyctrlreg028 = crate::Reg<i3cphyctrlreg028::I3cphyctrlreg028Spec>;
#[doc = "CR\\_I3C\\_OD\\_STA\\_STO\\_CNT"]
pub mod i3cphyctrlreg028;
#[doc = "I3CPHYCTRLREG02C (rw) register accessor: CR\\_I3C\\_OD\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg02c`] module"]
#[doc(alias = "I3CPHYCTRLREG02C")]
pub type I3cphyctrlreg02c = crate::Reg<i3cphyctrlreg02c::I3cphyctrlreg02cSpec>;
#[doc = "CR\\_I3C\\_OD\\_SCL\\_CNT"]
pub mod i3cphyctrlreg02c;
#[doc = "I3CPHYCTRLREG030 (rw) register accessor: CR\\_I3C\\_OD\\_ACK\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg030`] module"]
#[doc(alias = "I3CPHYCTRLREG030")]
pub type I3cphyctrlreg030 = crate::Reg<i3cphyctrlreg030::I3cphyctrlreg030Spec>;
#[doc = "CR\\_I3C\\_OD\\_ACK\\_CNT"]
pub mod i3cphyctrlreg030;
#[doc = "I3CPHYCTRLREG034 (rw) register accessor: CR\\_I3C\\_OD\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg034`] module"]
#[doc(alias = "I3CPHYCTRLREG034")]
pub type I3cphyctrlreg034 = crate::Reg<i3cphyctrlreg034::I3cphyctrlreg034Spec>;
#[doc = "CR\\_I3C\\_OD\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg034;
#[doc = "I3CPHYCTRLREG038 (rw) register accessor: CR\\_I3C\\_SDR0\\_PP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg038`] module"]
#[doc(alias = "I3CPHYCTRLREG038")]
pub type I3cphyctrlreg038 = crate::Reg<i3cphyctrlreg038::I3cphyctrlreg038Spec>;
#[doc = "CR\\_I3C\\_SDR0\\_PP\\_SCL\\_CNT"]
pub mod i3cphyctrlreg038;
#[doc = "I3CPHYCTRLREG03C (rw) register accessor: CR\\_I3C\\_SDR0\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg03c`] module"]
#[doc(alias = "I3CPHYCTRLREG03C")]
pub type I3cphyctrlreg03c = crate::Reg<i3cphyctrlreg03c::I3cphyctrlreg03cSpec>;
#[doc = "CR\\_I3C\\_SDR0\\_PP\\_TBIT\\_CNT"]
pub mod i3cphyctrlreg03c;
#[doc = "I3CPHYCTRLREG040 (rw) register accessor: CR\\_I3C\\_SDR0\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg040`] module"]
#[doc(alias = "I3CPHYCTRLREG040")]
pub type I3cphyctrlreg040 = crate::Reg<i3cphyctrlreg040::I3cphyctrlreg040Spec>;
#[doc = "CR\\_I3C\\_SDR0\\_PP\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg040;
#[doc = "I3CPHYCTRLREG044 (rw) register accessor: CR\\_I3C\\_SDR1\\_PP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg044`] module"]
#[doc(alias = "I3CPHYCTRLREG044")]
pub type I3cphyctrlreg044 = crate::Reg<i3cphyctrlreg044::I3cphyctrlreg044Spec>;
#[doc = "CR\\_I3C\\_SDR1\\_PP\\_SCL\\_CNT"]
pub mod i3cphyctrlreg044;
#[doc = "I3CPHYCTRLREG048 (rw) register accessor: CR\\_I3C\\_SDR1\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg048`] module"]
#[doc(alias = "I3CPHYCTRLREG048")]
pub type I3cphyctrlreg048 = crate::Reg<i3cphyctrlreg048::I3cphyctrlreg048Spec>;
#[doc = "CR\\_I3C\\_SDR1\\_PP\\_TBIT\\_CNT"]
pub mod i3cphyctrlreg048;
#[doc = "I3CPHYCTRLREG04C (rw) register accessor: CR\\_I3C\\_SDR1\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg04c`] module"]
#[doc(alias = "I3CPHYCTRLREG04C")]
pub type I3cphyctrlreg04c = crate::Reg<i3cphyctrlreg04c::I3cphyctrlreg04cSpec>;
#[doc = "CR\\_I3C\\_SDR1\\_PP\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg04c;
#[doc = "I3CPHYCTRLREG050 (rw) register accessor: CR\\_I3C\\_SDR2\\_PP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg050`] module"]
#[doc(alias = "I3CPHYCTRLREG050")]
pub type I3cphyctrlreg050 = crate::Reg<i3cphyctrlreg050::I3cphyctrlreg050Spec>;
#[doc = "CR\\_I3C\\_SDR2\\_PP\\_SCL\\_CNT"]
pub mod i3cphyctrlreg050;
#[doc = "I3CPHYCTRLREG054 (rw) register accessor: CR\\_I3C\\_SDR2\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg054`] module"]
#[doc(alias = "I3CPHYCTRLREG054")]
pub type I3cphyctrlreg054 = crate::Reg<i3cphyctrlreg054::I3cphyctrlreg054Spec>;
#[doc = "CR\\_I3C\\_SDR2\\_PP\\_TBIT\\_CNT"]
pub mod i3cphyctrlreg054;
#[doc = "I3CPHYCTRLREG058 (rw) register accessor: CR\\_I3C\\_SDR2\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg058`] module"]
#[doc(alias = "I3CPHYCTRLREG058")]
pub type I3cphyctrlreg058 = crate::Reg<i3cphyctrlreg058::I3cphyctrlreg058Spec>;
#[doc = "CR\\_I3C\\_SDR2\\_PP\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg058;
#[doc = "I3CPHYCTRLREG05C (rw) register accessor: CR\\_I3C\\_SDR3\\_PP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg05c`] module"]
#[doc(alias = "I3CPHYCTRLREG05C")]
pub type I3cphyctrlreg05c = crate::Reg<i3cphyctrlreg05c::I3cphyctrlreg05cSpec>;
#[doc = "CR\\_I3C\\_SDR3\\_PP\\_SCL\\_CNT"]
pub mod i3cphyctrlreg05c;
#[doc = "I3CPHYCTRLREG060 (rw) register accessor: CR\\_I3C\\_SDR3\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg060`] module"]
#[doc(alias = "I3CPHYCTRLREG060")]
pub type I3cphyctrlreg060 = crate::Reg<i3cphyctrlreg060::I3cphyctrlreg060Spec>;
#[doc = "CR\\_I3C\\_SDR3\\_PP\\_TBIT\\_CNT"]
pub mod i3cphyctrlreg060;
#[doc = "I3CPHYCTRLREG064 (rw) register accessor: CR\\_I3C\\_SDR3\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg064`] module"]
#[doc(alias = "I3CPHYCTRLREG064")]
pub type I3cphyctrlreg064 = crate::Reg<i3cphyctrlreg064::I3cphyctrlreg064Spec>;
#[doc = "CR\\_I3C\\_SDR3\\_PP\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg064;
#[doc = "I3CPHYCTRLREG068 (rw) register accessor: CR\\_I3C\\_SDR4\\_PP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg068`] module"]
#[doc(alias = "I3CPHYCTRLREG068")]
pub type I3cphyctrlreg068 = crate::Reg<i3cphyctrlreg068::I3cphyctrlreg068Spec>;
#[doc = "CR\\_I3C\\_SDR4\\_PP\\_SCL\\_CNT"]
pub mod i3cphyctrlreg068;
#[doc = "I3CPHYCTRLREG06C (rw) register accessor: CR\\_I3C\\_SDR4\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg06c`] module"]
#[doc(alias = "I3CPHYCTRLREG06C")]
pub type I3cphyctrlreg06c = crate::Reg<i3cphyctrlreg06c::I3cphyctrlreg06cSpec>;
#[doc = "CR\\_I3C\\_SDR4\\_PP\\_TBIT\\_CNT"]
pub mod i3cphyctrlreg06c;
#[doc = "I3CPHYCTRLREG070 (rw) register accessor: CR\\_I3C\\_SDR4\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg070`] module"]
#[doc(alias = "I3CPHYCTRLREG070")]
pub type I3cphyctrlreg070 = crate::Reg<i3cphyctrlreg070::I3cphyctrlreg070Spec>;
#[doc = "CR\\_I3C\\_SDR4\\_PP\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg070;
#[doc = "I3CPHYCTRLREG074 (rw) register accessor: CR\\_I3C\\_DDR\\_PP\\_SCL\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg074`] module"]
#[doc(alias = "I3CPHYCTRLREG074")]
pub type I3cphyctrlreg074 = crate::Reg<i3cphyctrlreg074::I3cphyctrlreg074Spec>;
#[doc = "CR\\_I3C\\_DDR\\_PP\\_SCL\\_CNT"]
pub mod i3cphyctrlreg074;
#[doc = "I3CPHYCTRLREG078 (rw) register accessor: CR\\_I3C\\_DDR\\_PP\\_TBIT\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg078`] module"]
#[doc(alias = "I3CPHYCTRLREG078")]
pub type I3cphyctrlreg078 = crate::Reg<i3cphyctrlreg078::I3cphyctrlreg078Spec>;
#[doc = "CR\\_I3C\\_DDR\\_PP\\_TBIT\\_CNT"]
pub mod i3cphyctrlreg078;
#[doc = "I3CPHYCTRLREG07C (rw) register accessor: CR\\_DDR\\_PP\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg07c`] module"]
#[doc(alias = "I3CPHYCTRLREG07C")]
pub type I3cphyctrlreg07c = crate::Reg<i3cphyctrlreg07c::I3cphyctrlreg07cSpec>;
#[doc = "CR\\_DDR\\_PP\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg07c;
#[doc = "I3CPHYCTRLREG080 (rw) register accessor: SR\\_P\\_PREPARE\\_SCL\\_SDA\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg080`] module"]
#[doc(alias = "I3CPHYCTRLREG080")]
pub type I3cphyctrlreg080 = crate::Reg<i3cphyctrlreg080::I3cphyctrlreg080Spec>;
#[doc = "SR\\_P\\_PREPARE\\_SCL\\_SDA\\_CNT"]
pub mod i3cphyctrlreg080;
#[doc = "I3CPHYCTRLREG084 (rw) register accessor: CCR\\_TO\\_NCR\\_OVERLAP\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg084`] module"]
#[doc(alias = "I3CPHYCTRLREG084")]
pub type I3cphyctrlreg084 = crate::Reg<i3cphyctrlreg084::I3cphyctrlreg084Spec>;
#[doc = "CCR\\_TO\\_NCR\\_OVERLAP\\_CNT"]
pub mod i3cphyctrlreg084;
#[doc = "I3CPHYCTRLREG088 (rw) register accessor: CR\\_IBI\\_ADDR\\_ACK\\_PROLONG\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg088`] module"]
#[doc(alias = "I3CPHYCTRLREG088")]
pub type I3cphyctrlreg088 = crate::Reg<i3cphyctrlreg088::I3cphyctrlreg088Spec>;
#[doc = "CR\\_IBI\\_ADDR\\_ACK\\_PROLONG\\_CNT"]
pub mod i3cphyctrlreg088;
#[doc = "I3CPHYCTRLREG08C (rw) register accessor: TG\\_WR\\_ADDR\\_ACK\\_PROLONG\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg08c`] module"]
#[doc(alias = "I3CPHYCTRLREG08C")]
pub type I3cphyctrlreg08c = crate::Reg<i3cphyctrlreg08c::I3cphyctrlreg08cSpec>;
#[doc = "TG\\_WR\\_ADDR\\_ACK\\_PROLONG"]
pub mod i3cphyctrlreg08c;
#[doc = "I3CPHYCTRLREG090 (rw) register accessor: TG\\_SDA\\_TRAN\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg090`] module"]
#[doc(alias = "I3CPHYCTRLREG090")]
pub type I3cphyctrlreg090 = crate::Reg<i3cphyctrlreg090::I3cphyctrlreg090Spec>;
#[doc = "TG\\_SDA\\_TRAN\\_CNT"]
pub mod i3cphyctrlreg090;
#[doc = "I3CPHYCTRLREG094 (rw) register accessor: DDR\\_CMD\\_HANDOFF\\_EARLY\\_TM\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg094`] module"]
#[doc(alias = "I3CPHYCTRLREG094")]
pub type I3cphyctrlreg094 = crate::Reg<i3cphyctrlreg094::I3cphyctrlreg094Spec>;
#[doc = "DDR\\_CMD\\_HANDOFF\\_EARLY\\_TM\\_CNT"]
pub mod i3cphyctrlreg094;
#[doc = "I3CPHYCTRLREG098 (rw) register accessor: CR\\_SCL\\_SDA\\_PULLUP\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg098`] module"]
#[doc(alias = "I3CPHYCTRLREG098")]
pub type I3cphyctrlreg098 = crate::Reg<i3cphyctrlreg098::I3cphyctrlreg098Spec>;
#[doc = "CR\\_SCL\\_SDA\\_PULLUP\\_EN"]
pub mod i3cphyctrlreg098;
#[doc = "I3CPHYCTRLREG09C (rw) register accessor: SPECIAL\\_PATTERN\\_SET\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg09c`] module"]
#[doc(alias = "I3CPHYCTRLREG09C")]
pub type I3cphyctrlreg09c = crate::Reg<i3cphyctrlreg09c::I3cphyctrlreg09cSpec>;
#[doc = "SPECIAL\\_PATTERN\\_SET"]
pub mod i3cphyctrlreg09c;
#[doc = "I3CPHYCTRLREG0A0 (rw) register accessor: SPECIAL\\_PATTERN\\_SW\\_OPT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0a0`] module"]
#[doc(alias = "I3CPHYCTRLREG0A0")]
pub type I3cphyctrlreg0a0 = crate::Reg<i3cphyctrlreg0a0::I3cphyctrlreg0a0Spec>;
#[doc = "SPECIAL\\_PATTERN\\_SW\\_OPT"]
pub mod i3cphyctrlreg0a0;
#[doc = "I3CPHYCTRLREG0A4 (rw) register accessor: SPECIAL\\_PATTERN\\_SCL\\_TOGGLE\\_SET\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0a4`] module"]
#[doc(alias = "I3CPHYCTRLREG0A4")]
pub type I3cphyctrlreg0a4 = crate::Reg<i3cphyctrlreg0a4::I3cphyctrlreg0a4Spec>;
#[doc = "SPECIAL\\_PATTERN\\_SCL\\_TOGGLE\\_SET"]
pub mod i3cphyctrlreg0a4;
#[doc = "I3CPHYCTRLREG0A8 (rw) register accessor: SPECIAL\\_PATTERN\\_SCL\\_TOGGLE\\_PAT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0a8`] module"]
#[doc(alias = "I3CPHYCTRLREG0A8")]
pub type I3cphyctrlreg0a8 = crate::Reg<i3cphyctrlreg0a8::I3cphyctrlreg0a8Spec>;
#[doc = "SPECIAL\\_PATTERN\\_SCL\\_TOGGLE\\_PAT"]
pub mod i3cphyctrlreg0a8;
#[doc = "I3CPHYCTRLREG0AC (rw) register accessor: SPECIAL\\_PATTERN\\_SCL\\_TIEL\\_SET\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0ac`] module"]
#[doc(alias = "I3CPHYCTRLREG0AC")]
pub type I3cphyctrlreg0ac = crate::Reg<i3cphyctrlreg0ac::I3cphyctrlreg0acSpec>;
#[doc = "SPECIAL\\_PATTERN\\_SCL\\_TIEL\\_SET"]
pub mod i3cphyctrlreg0ac;
#[doc = "I3CPHYCTRLREG0B4 (rw) register accessor: SDA\\_DETECTOR\\_CNT0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0b4`] module"]
#[doc(alias = "I3CPHYCTRLREG0B4")]
pub type I3cphyctrlreg0b4 = crate::Reg<i3cphyctrlreg0b4::I3cphyctrlreg0b4Spec>;
#[doc = "SDA\\_DETECTOR\\_CNT0"]
pub mod i3cphyctrlreg0b4;
#[doc = "I3CPHYCTRLREG0B8 (rw) register accessor: SDA\\_DETECTOR\\_CNT1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0b8`] module"]
#[doc(alias = "I3CPHYCTRLREG0B8")]
pub type I3cphyctrlreg0b8 = crate::Reg<i3cphyctrlreg0b8::I3cphyctrlreg0b8Spec>;
#[doc = "SDA\\_DETECTOR\\_CNT1"]
pub mod i3cphyctrlreg0b8;
#[doc = "I3CPHYCTRLREG0BC (rw) register accessor: SDA\\_DETECTOR\\_CNT2\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0bc`] module"]
#[doc(alias = "I3CPHYCTRLREG0BC")]
pub type I3cphyctrlreg0bc = crate::Reg<i3cphyctrlreg0bc::I3cphyctrlreg0bcSpec>;
#[doc = "SDA\\_DETECTOR\\_CNT2"]
pub mod i3cphyctrlreg0bc;
#[doc = "I3CPHYCTRLREG0C0 (rw) register accessor: SDA\\_STUCK\\_SET1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0c0`] module"]
#[doc(alias = "I3CPHYCTRLREG0C0")]
pub type I3cphyctrlreg0c0 = crate::Reg<i3cphyctrlreg0c0::I3cphyctrlreg0c0Spec>;
#[doc = "SDA\\_STUCK\\_SET1"]
pub mod i3cphyctrlreg0c0;
#[doc = "I3CPHYCTRLREG0C4 (rw) register accessor: SDA\\_STUCK\\_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0c4`] module"]
#[doc(alias = "I3CPHYCTRLREG0C4")]
pub type I3cphyctrlreg0c4 = crate::Reg<i3cphyctrlreg0c4::I3cphyctrlreg0c4Spec>;
#[doc = "SDA\\_STUCK\\_READ"]
pub mod i3cphyctrlreg0c4;
#[doc = "I3CPHYCTRLREG0C8 (rw) register accessor: READ\\_PHY\\_STATE\\_MACHINE\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0c8`] module"]
#[doc(alias = "I3CPHYCTRLREG0C8")]
pub type I3cphyctrlreg0c8 = crate::Reg<i3cphyctrlreg0c8::I3cphyctrlreg0c8Spec>;
#[doc = "READ\\_PHY\\_STATE\\_MACHINE"]
pub mod i3cphyctrlreg0c8;
#[doc = "I3CPHYCTRLREG0CC (rw) register accessor: PHY\\_OPTION\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0cc`] module"]
#[doc(alias = "I3CPHYCTRLREG0CC")]
pub type I3cphyctrlreg0cc = crate::Reg<i3cphyctrlreg0cc::I3cphyctrlreg0ccSpec>;
#[doc = "PHY\\_OPTION"]
pub mod i3cphyctrlreg0cc;
#[doc = "I3CPHYCTRLREG0D0 (rw) register accessor: CR\\_SCL\\_SDA\\_PULLUP\\_EN\\_ADDITIONAL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0d0`] module"]
#[doc(alias = "I3CPHYCTRLREG0D0")]
pub type I3cphyctrlreg0d0 = crate::Reg<i3cphyctrlreg0d0::I3cphyctrlreg0d0Spec>;
#[doc = "CR\\_SCL\\_SDA\\_PULLUP\\_EN\\_ADDITIONAL"]
pub mod i3cphyctrlreg0d0;
#[doc = "I3CPHYCTRLREG0D4 (rw) register accessor: SPIKE\\_FILTER\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0d4`] module"]
#[doc(alias = "I3CPHYCTRLREG0D4")]
pub type I3cphyctrlreg0d4 = crate::Reg<i3cphyctrlreg0d4::I3cphyctrlreg0d4Spec>;
#[doc = "SPIKE\\_FILTER"]
pub mod i3cphyctrlreg0d4;
#[doc = "I3CPHYCTRLREG0D8 (rw) register accessor: SCL\\_SDA\\_TIMMIING\\_CNT\\_ADDITIONAL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0d8`] module"]
#[doc(alias = "I3CPHYCTRLREG0D8")]
pub type I3cphyctrlreg0d8 = crate::Reg<i3cphyctrlreg0d8::I3cphyctrlreg0d8Spec>;
#[doc = "SCL\\_SDA\\_TIMMIING\\_CNT\\_ADDITIONAL"]
pub mod i3cphyctrlreg0d8;
#[doc = "I3CPHYCTRLREG0DC (rw) register accessor: BUS\\_FREE\\_TIME\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0dc`] module"]
#[doc(alias = "I3CPHYCTRLREG0DC")]
pub type I3cphyctrlreg0dc = crate::Reg<i3cphyctrlreg0dc::I3cphyctrlreg0dcSpec>;
#[doc = "BUS\\_FREE\\_TIME\\_CNT"]
pub mod i3cphyctrlreg0dc;
#[doc = "I3CPHYCTRLREG0E0 (rw) register accessor: SPECIAL\\_PATTERN\\_SET\\_ADDITIONAL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0e0`] module"]
#[doc(alias = "I3CPHYCTRLREG0E0")]
pub type I3cphyctrlreg0e0 = crate::Reg<i3cphyctrlreg0e0::I3cphyctrlreg0e0Spec>;
#[doc = "SPECIAL\\_PATTERN\\_SET\\_ADDITIONAL"]
pub mod i3cphyctrlreg0e0;
#[doc = "I3CPHYCTRLREG0E4 (rw) register accessor: BUS\\_CONTENTION\\_CHK0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0e4`] module"]
#[doc(alias = "I3CPHYCTRLREG0E4")]
pub type I3cphyctrlreg0e4 = crate::Reg<i3cphyctrlreg0e4::I3cphyctrlreg0e4Spec>;
#[doc = "BUS\\_CONTENTION\\_CHK0"]
pub mod i3cphyctrlreg0e4;
#[doc = "I3CPHYCTRLREG0E8 (rw) register accessor: BUS\\_CONTENTION\\_CNT0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0e8`] module"]
#[doc(alias = "I3CPHYCTRLREG0E8")]
pub type I3cphyctrlreg0e8 = crate::Reg<i3cphyctrlreg0e8::I3cphyctrlreg0e8Spec>;
#[doc = "BUS\\_CONTENTION\\_CNT0"]
pub mod i3cphyctrlreg0e8;
#[doc = "I3CPHYCTRLREG0EC (rw) register accessor: BUS\\_CONTENTION\\_CNT1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0ec`] module"]
#[doc(alias = "I3CPHYCTRLREG0EC")]
pub type I3cphyctrlreg0ec = crate::Reg<i3cphyctrlreg0ec::I3cphyctrlreg0ecSpec>;
#[doc = "BUS\\_CONTENTION\\_CNT1"]
pub mod i3cphyctrlreg0ec;
#[doc = "I3CPHYCTRLREG0F0 (rw) register accessor: BUS\\_CONTENTION\\_CNT2\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cphyctrlreg0f0`] module"]
#[doc(alias = "I3CPHYCTRLREG0F0")]
pub type I3cphyctrlreg0f0 = crate::Reg<i3cphyctrlreg0f0::I3cphyctrlreg0f0Spec>;
#[doc = "BUS\\_CONTENTION\\_CNT2"]
pub mod i3cphyctrlreg0f0;
#[doc = "HCIEXTCAP000 (rw) register accessor: HW\\_ID\\_HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap000`] module"]
#[doc(alias = "HCIEXTCAP000")]
pub type Hciextcap000 = crate::Reg<hciextcap000::Hciextcap000Spec>;
#[doc = "HW\\_ID\\_HEADER"]
pub mod hciextcap000;
#[doc = "HCIEXTCAP004 (rw) register accessor: HW\\_ID\\_MIPI\\_VENDOR\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap004`] module"]
#[doc(alias = "HCIEXTCAP004")]
pub type Hciextcap004 = crate::Reg<hciextcap004::Hciextcap004Spec>;
#[doc = "HW\\_ID\\_MIPI\\_VENDOR"]
pub mod hciextcap004;
#[doc = "HCIEXTCAP008 (rw) register accessor: HW\\_ID\\_I3C\\_VER\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap008`] module"]
#[doc(alias = "HCIEXTCAP008")]
pub type Hciextcap008 = crate::Reg<hciextcap008::Hciextcap008Spec>;
#[doc = "HW\\_ID\\_I3C\\_VER"]
pub mod hciextcap008;
#[doc = "HCIEXTCAP00C (rw) register accessor: HW\\_ID\\_I3C\\_PRODUCT\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap00c`] module"]
#[doc(alias = "HCIEXTCAP00C")]
pub type Hciextcap00c = crate::Reg<hciextcap00c::Hciextcap00cSpec>;
#[doc = "HW\\_ID\\_I3C\\_PRODUCT"]
pub mod hciextcap00c;
#[doc = "HCIEXTCAP010 (rw) register accessor: CTL\\_CFG\\_HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap010`] module"]
#[doc(alias = "HCIEXTCAP010")]
pub type Hciextcap010 = crate::Reg<hciextcap010::Hciextcap010Spec>;
#[doc = "CTL\\_CFG\\_HEADER"]
pub mod hciextcap010;
#[doc = "HCIEXTCAP014 (rw) register accessor: CTL\\_CFG\\_OPERATION\\_MODE\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap014`] module"]
#[doc(alias = "HCIEXTCAP014")]
pub type Hciextcap014 = crate::Reg<hciextcap014::Hciextcap014Spec>;
#[doc = "CTL\\_CFG\\_OPERATION\\_MODE"]
pub mod hciextcap014;
#[doc = "HCIEXTCAP018 (rw) register accessor: EXTCAP\\_HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap018`] module"]
#[doc(alias = "HCIEXTCAP018")]
pub type Hciextcap018 = crate::Reg<hciextcap018::Hciextcap018Spec>;
#[doc = "EXTCAP\\_HEADER"]
pub mod hciextcap018;
#[doc = "HCIEXTCAP01C (rw) register accessor: EXTCAP\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap01c`] module"]
#[doc(alias = "HCIEXTCAP01C")]
pub type Hciextcap01c = crate::Reg<hciextcap01c::Hciextcap01cSpec>;
#[doc = "EXTCAP\\_CTRL"]
pub mod hciextcap01c;
#[doc = "HCIEXTCAP020 (rw) register accessor: EXTCAP\\_PHY\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap020`] module"]
#[doc(alias = "HCIEXTCAP020")]
pub type Hciextcap020 = crate::Reg<hciextcap020::Hciextcap020Spec>;
#[doc = "EXTCAP\\_PHY"]
pub mod hciextcap020;
#[doc = "HCIEXTCAP024 (rw) register accessor: EXTCAP\\_DMAARB\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap024`] module"]
#[doc(alias = "HCIEXTCAP024")]
pub type Hciextcap024 = crate::Reg<hciextcap024::Hciextcap024Spec>;
#[doc = "EXTCAP\\_DMAARB"]
pub mod hciextcap024;
#[doc = "HCIEXTCAP080 (rw) register accessor: DMA\\_MBUS\\_ARB\\_CTRL\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap080`] module"]
#[doc(alias = "HCIEXTCAP080")]
pub type Hciextcap080 = crate::Reg<hciextcap080::Hciextcap080Spec>;
#[doc = "DMA\\_MBUS\\_ARB\\_CTRL\\_0"]
pub mod hciextcap080;
#[doc = "HCIEXTCAP084 (rw) register accessor: DMA\\_MBUS\\_ARB\\_CLR\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap084`] module"]
#[doc(alias = "HCIEXTCAP084")]
pub type Hciextcap084 = crate::Reg<hciextcap084::Hciextcap084Spec>;
#[doc = "DMA\\_MBUS\\_ARB\\_CLR\\_0"]
pub mod hciextcap084;
#[doc = "HCIEXTCAP090 (rw) register accessor: DMA\\_MBUS\\_ARB\\_DBG\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap090`] module"]
#[doc(alias = "HCIEXTCAP090")]
pub type Hciextcap090 = crate::Reg<hciextcap090::Hciextcap090Spec>;
#[doc = "DMA\\_MBUS\\_ARB\\_DBG\\_0"]
pub mod hciextcap090;
#[doc = "HCIEXTCAP094 (rw) register accessor: DMA\\_MBUS\\_ARB\\_DBG\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap094`] module"]
#[doc(alias = "HCIEXTCAP094")]
pub type Hciextcap094 = crate::Reg<hciextcap094::Hciextcap094Spec>;
#[doc = "DMA\\_MBUS\\_ARB\\_DBG\\_1"]
pub mod hciextcap094;
#[doc = "HCIEXTCAP098 (rw) register accessor: DMA\\_MBUS\\_ARB\\_DBG\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap098`] module"]
#[doc(alias = "HCIEXTCAP098")]
pub type Hciextcap098 = crate::Reg<hciextcap098::Hciextcap098Spec>;
#[doc = "DMA\\_MBUS\\_ARB\\_DBG\\_2"]
pub mod hciextcap098;
#[doc = "HCIEXTCAP09C (rw) register accessor: DMA\\_MBUS\\_ARB\\_DBG\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciextcap09c`] module"]
#[doc(alias = "HCIEXTCAP09C")]
pub type Hciextcap09c = crate::Reg<hciextcap09c::Hciextcap09cSpec>;
#[doc = "DMA\\_MBUS\\_ARB\\_DBG\\_3"]
pub mod hciextcap09c;
