#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    io_ahb_matrix000: IoAhbMatrix000,
    io_ahb_matrix004: IoAhbMatrix004,
    io_ahb_matrix008: IoAhbMatrix008,
    io_ahb_matrix00c: IoAhbMatrix00c,
    io_ahb_matrix010: IoAhbMatrix010,
    io_ahb_matrix014: IoAhbMatrix014,
    io_ahb_matrix018: IoAhbMatrix018,
    io_ahb_matrix01c: IoAhbMatrix01c,
    _reserved8: [u8; 0x10],
    io_ahb_matrix030: IoAhbMatrix030,
    io_ahb_matrix034: IoAhbMatrix034,
    io_ahb_matrix038: IoAhbMatrix038,
    io_ahb_matrix03c: IoAhbMatrix03c,
    io_ahb_matrix040: IoAhbMatrix040,
    io_ahb_matrix044: IoAhbMatrix044,
    io_ahb_matrix048: IoAhbMatrix048,
    io_ahb_matrix04c: IoAhbMatrix04c,
    io_ahb_matrix050: IoAhbMatrix050,
    io_ahb_matrix054: IoAhbMatrix054,
    io_ahb_matrix058: IoAhbMatrix058,
    io_ahb_matrix05c: IoAhbMatrix05c,
    _reserved20: [u8; 0x10],
    io_ahb_matrix070: IoAhbMatrix070,
    io_ahb_matrix074: IoAhbMatrix074,
    io_ahb_matrix078: IoAhbMatrix078,
    io_ahb_matrix07c: IoAhbMatrix07c,
    io_ahb_matrix080: IoAhbMatrix080,
    io_ahb_matrix084: IoAhbMatrix084,
    io_ahb_matrix088: IoAhbMatrix088,
    io_ahb_matrix08c: IoAhbMatrix08c,
    io_ahb_matrix090: IoAhbMatrix090,
    io_ahb_matrix094: IoAhbMatrix094,
    io_ahb_matrix098: IoAhbMatrix098,
    io_ahb_matrix09c: IoAhbMatrix09c,
    _reserved32: [u8; 0x10],
    io_ahb_matrix0b0: IoAhbMatrix0b0,
    io_ahb_matrix0b4: IoAhbMatrix0b4,
    io_ahb_matrix0b8: IoAhbMatrix0b8,
    io_ahb_matrix0bc: IoAhbMatrix0bc,
    io_ahb_matrix0c0: IoAhbMatrix0c0,
    io_ahb_matrix0c4: IoAhbMatrix0c4,
    io_ahb_matrix0c8: IoAhbMatrix0c8,
    io_ahb_matrix0cc: IoAhbMatrix0cc,
    io_ahb_matrix0d0: IoAhbMatrix0d0,
    io_ahb_matrix0d4: IoAhbMatrix0d4,
    io_ahb_matrix0d8: IoAhbMatrix0d8,
    io_ahb_matrix0dc: IoAhbMatrix0dc,
    _reserved44: [u8; 0x10],
    io_ahb_matrix0f0: IoAhbMatrix0f0,
    io_ahb_matrix0f4: IoAhbMatrix0f4,
    io_ahb_matrix0f8: IoAhbMatrix0f8,
    io_ahb_matrix0fc: IoAhbMatrix0fc,
    io_ahb_matrix100: IoAhbMatrix100,
    io_ahb_matrix104: IoAhbMatrix104,
    io_ahb_matrix108: IoAhbMatrix108,
    io_ahb_matrix10c: IoAhbMatrix10c,
    io_ahb_matrix110: IoAhbMatrix110,
    io_ahb_matrix114: IoAhbMatrix114,
    io_ahb_matrix118: IoAhbMatrix118,
    io_ahb_matrix11c: IoAhbMatrix11c,
    _reserved56: [u8; 0x10],
    io_ahb_matrix130: IoAhbMatrix130,
    io_ahb_matrix134: IoAhbMatrix134,
    io_ahb_matrix138: IoAhbMatrix138,
    io_ahb_matrix13c: IoAhbMatrix13c,
    io_ahb_matrix140: IoAhbMatrix140,
    io_ahb_matrix144: IoAhbMatrix144,
    io_ahb_matrix148: IoAhbMatrix148,
    io_ahb_matrix14c: IoAhbMatrix14c,
    io_ahb_matrix150: IoAhbMatrix150,
    io_ahb_matrix154: IoAhbMatrix154,
    io_ahb_matrix158: IoAhbMatrix158,
    io_ahb_matrix15c: IoAhbMatrix15c,
    _reserved68: [u8; 0x10],
    io_ahb_matrix170: IoAhbMatrix170,
    io_ahb_matrix174: IoAhbMatrix174,
    io_ahb_matrix178: IoAhbMatrix178,
    io_ahb_matrix17c: IoAhbMatrix17c,
    io_ahb_matrix180: IoAhbMatrix180,
    io_ahb_matrix184: IoAhbMatrix184,
    io_ahb_matrix188: IoAhbMatrix188,
    io_ahb_matrix18c: IoAhbMatrix18c,
    io_ahb_matrix190: IoAhbMatrix190,
    io_ahb_matrix194: IoAhbMatrix194,
    io_ahb_matrix198: IoAhbMatrix198,
    io_ahb_matrix19c: IoAhbMatrix19c,
    _reserved80: [u8; 0x10],
    io_ahb_matrix1b0: IoAhbMatrix1b0,
    io_ahb_matrix1b4: IoAhbMatrix1b4,
    io_ahb_matrix1b8: IoAhbMatrix1b8,
    io_ahb_matrix1bc: IoAhbMatrix1bc,
    io_ahb_matrix1c0: IoAhbMatrix1c0,
    io_ahb_matrix1c4: IoAhbMatrix1c4,
    io_ahb_matrix1c8: IoAhbMatrix1c8,
    io_ahb_matrix1cc: IoAhbMatrix1cc,
    io_ahb_matrix1d0: IoAhbMatrix1d0,
    io_ahb_matrix1d4: IoAhbMatrix1d4,
    io_ahb_matrix1d8: IoAhbMatrix1d8,
    io_ahb_matrix1dc: IoAhbMatrix1dc,
    _reserved92: [u8; 0x10],
    io_ahb_matrix1f0: IoAhbMatrix1f0,
    io_ahb_matrix1f4: IoAhbMatrix1f4,
    io_ahb_matrix1f8: IoAhbMatrix1f8,
    io_ahb_matrix1fc: IoAhbMatrix1fc,
    io_ahb_matrix200: IoAhbMatrix200,
    io_ahb_matrix204: IoAhbMatrix204,
    io_ahb_matrix208: IoAhbMatrix208,
    io_ahb_matrix20c: IoAhbMatrix20c,
    io_ahb_matrix210: IoAhbMatrix210,
    io_ahb_matrix214: IoAhbMatrix214,
    io_ahb_matrix218: IoAhbMatrix218,
    io_ahb_matrix21c: IoAhbMatrix21c,
    io_ahb_matrix220: IoAhbMatrix220,
    io_ahb_matrix224: IoAhbMatrix224,
    io_ahb_matrix228: IoAhbMatrix228,
    io_ahb_matrix22c: IoAhbMatrix22c,
    io_ahb_matrix230: IoAhbMatrix230,
    io_ahb_matrix234: IoAhbMatrix234,
    io_ahb_matrix238: IoAhbMatrix238,
    io_ahb_matrix23c: IoAhbMatrix23c,
    _reserved112: [u8; 0x20],
    io_ahb_matrix260: IoAhbMatrix260,
    io_ahb_matrix264: IoAhbMatrix264,
    io_ahb_matrix268: IoAhbMatrix268,
    io_ahb_matrix26c: IoAhbMatrix26c,
    io_ahb_matrix270: IoAhbMatrix270,
    io_ahb_matrix274: IoAhbMatrix274,
    io_ahb_matrix278: IoAhbMatrix278,
    io_ahb_matrix27c: IoAhbMatrix27c,
    _reserved120: [u8; 0x20],
    io_ahb_matrix2a0: IoAhbMatrix2a0,
    io_ahb_matrix2a4: IoAhbMatrix2a4,
    io_ahb_matrix2a8: IoAhbMatrix2a8,
    io_ahb_matrix2ac: IoAhbMatrix2ac,
    _reserved124: [u8; 0x4c],
    io_ahb_matrix2fc: IoAhbMatrix2fc,
}
impl RegisterBlock {
    #[doc = "0x00 - AHBM000 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix000(&self) -> &IoAhbMatrix000 {
        &self.io_ahb_matrix000
    }
    #[doc = "0x04 - AHBM004 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix004(&self) -> &IoAhbMatrix004 {
        &self.io_ahb_matrix004
    }
    #[doc = "0x08 - AHBM008 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix008(&self) -> &IoAhbMatrix008 {
        &self.io_ahb_matrix008
    }
    #[doc = "0x0c - AHBM00C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix00c(&self) -> &IoAhbMatrix00c {
        &self.io_ahb_matrix00c
    }
    #[doc = "0x10 - AHBM010 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix010(&self) -> &IoAhbMatrix010 {
        &self.io_ahb_matrix010
    }
    #[doc = "0x14 - AHBM014 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix014(&self) -> &IoAhbMatrix014 {
        &self.io_ahb_matrix014
    }
    #[doc = "0x18 - AHBM018 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix018(&self) -> &IoAhbMatrix018 {
        &self.io_ahb_matrix018
    }
    #[doc = "0x1c - AHBM01C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix01c(&self) -> &IoAhbMatrix01c {
        &self.io_ahb_matrix01c
    }
    #[doc = "0x30 - AHBM030 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix030(&self) -> &IoAhbMatrix030 {
        &self.io_ahb_matrix030
    }
    #[doc = "0x34 - AHBM034 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix034(&self) -> &IoAhbMatrix034 {
        &self.io_ahb_matrix034
    }
    #[doc = "0x38 - AHBM038 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix038(&self) -> &IoAhbMatrix038 {
        &self.io_ahb_matrix038
    }
    #[doc = "0x3c - AHBM03C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix03c(&self) -> &IoAhbMatrix03c {
        &self.io_ahb_matrix03c
    }
    #[doc = "0x40 - AHBM040 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix040(&self) -> &IoAhbMatrix040 {
        &self.io_ahb_matrix040
    }
    #[doc = "0x44 - AHBM044 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix044(&self) -> &IoAhbMatrix044 {
        &self.io_ahb_matrix044
    }
    #[doc = "0x48 - AHBM048 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix048(&self) -> &IoAhbMatrix048 {
        &self.io_ahb_matrix048
    }
    #[doc = "0x4c - AHBM04C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix04c(&self) -> &IoAhbMatrix04c {
        &self.io_ahb_matrix04c
    }
    #[doc = "0x50 - AHBM050 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix050(&self) -> &IoAhbMatrix050 {
        &self.io_ahb_matrix050
    }
    #[doc = "0x54 - AHBM054 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix054(&self) -> &IoAhbMatrix054 {
        &self.io_ahb_matrix054
    }
    #[doc = "0x58 - AHBM058 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix058(&self) -> &IoAhbMatrix058 {
        &self.io_ahb_matrix058
    }
    #[doc = "0x5c - AHBM05C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix05c(&self) -> &IoAhbMatrix05c {
        &self.io_ahb_matrix05c
    }
    #[doc = "0x70 - AHBM070 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix070(&self) -> &IoAhbMatrix070 {
        &self.io_ahb_matrix070
    }
    #[doc = "0x74 - AHBM074 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix074(&self) -> &IoAhbMatrix074 {
        &self.io_ahb_matrix074
    }
    #[doc = "0x78 - AHBM078 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix078(&self) -> &IoAhbMatrix078 {
        &self.io_ahb_matrix078
    }
    #[doc = "0x7c - AHBM07C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix07c(&self) -> &IoAhbMatrix07c {
        &self.io_ahb_matrix07c
    }
    #[doc = "0x80 - AHBM080 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix080(&self) -> &IoAhbMatrix080 {
        &self.io_ahb_matrix080
    }
    #[doc = "0x84 - AHBM084 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix084(&self) -> &IoAhbMatrix084 {
        &self.io_ahb_matrix084
    }
    #[doc = "0x88 - AHBM088 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix088(&self) -> &IoAhbMatrix088 {
        &self.io_ahb_matrix088
    }
    #[doc = "0x8c - AHBM08C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix08c(&self) -> &IoAhbMatrix08c {
        &self.io_ahb_matrix08c
    }
    #[doc = "0x90 - AHBM090 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix090(&self) -> &IoAhbMatrix090 {
        &self.io_ahb_matrix090
    }
    #[doc = "0x94 - AHBM094 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix094(&self) -> &IoAhbMatrix094 {
        &self.io_ahb_matrix094
    }
    #[doc = "0x98 - AHBM098 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix098(&self) -> &IoAhbMatrix098 {
        &self.io_ahb_matrix098
    }
    #[doc = "0x9c - AHBM09C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix09c(&self) -> &IoAhbMatrix09c {
        &self.io_ahb_matrix09c
    }
    #[doc = "0xb0 - AHBM0B0 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0b0(&self) -> &IoAhbMatrix0b0 {
        &self.io_ahb_matrix0b0
    }
    #[doc = "0xb4 - AHBM0B4 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0b4(&self) -> &IoAhbMatrix0b4 {
        &self.io_ahb_matrix0b4
    }
    #[doc = "0xb8 - AHBM0B8 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0b8(&self) -> &IoAhbMatrix0b8 {
        &self.io_ahb_matrix0b8
    }
    #[doc = "0xbc - AHBM0BC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0bc(&self) -> &IoAhbMatrix0bc {
        &self.io_ahb_matrix0bc
    }
    #[doc = "0xc0 - AHBM0C0 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0c0(&self) -> &IoAhbMatrix0c0 {
        &self.io_ahb_matrix0c0
    }
    #[doc = "0xc4 - AHBM0C4 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0c4(&self) -> &IoAhbMatrix0c4 {
        &self.io_ahb_matrix0c4
    }
    #[doc = "0xc8 - AHBM0C8 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0c8(&self) -> &IoAhbMatrix0c8 {
        &self.io_ahb_matrix0c8
    }
    #[doc = "0xcc - AHBM0CC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0cc(&self) -> &IoAhbMatrix0cc {
        &self.io_ahb_matrix0cc
    }
    #[doc = "0xd0 - AHBM0D0 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0d0(&self) -> &IoAhbMatrix0d0 {
        &self.io_ahb_matrix0d0
    }
    #[doc = "0xd4 - AHBM0D4 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0d4(&self) -> &IoAhbMatrix0d4 {
        &self.io_ahb_matrix0d4
    }
    #[doc = "0xd8 - AHBM0D8 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0d8(&self) -> &IoAhbMatrix0d8 {
        &self.io_ahb_matrix0d8
    }
    #[doc = "0xdc - AHBM0DC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0dc(&self) -> &IoAhbMatrix0dc {
        &self.io_ahb_matrix0dc
    }
    #[doc = "0xf0 - AHBM0F0 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0f0(&self) -> &IoAhbMatrix0f0 {
        &self.io_ahb_matrix0f0
    }
    #[doc = "0xf4 - AHBM0F4 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0f4(&self) -> &IoAhbMatrix0f4 {
        &self.io_ahb_matrix0f4
    }
    #[doc = "0xf8 - AHBM0F8 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0f8(&self) -> &IoAhbMatrix0f8 {
        &self.io_ahb_matrix0f8
    }
    #[doc = "0xfc - AHBM0FC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix0fc(&self) -> &IoAhbMatrix0fc {
        &self.io_ahb_matrix0fc
    }
    #[doc = "0x100 - AHBM100 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix100(&self) -> &IoAhbMatrix100 {
        &self.io_ahb_matrix100
    }
    #[doc = "0x104 - AHBM104 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix104(&self) -> &IoAhbMatrix104 {
        &self.io_ahb_matrix104
    }
    #[doc = "0x108 - AHBM108 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix108(&self) -> &IoAhbMatrix108 {
        &self.io_ahb_matrix108
    }
    #[doc = "0x10c - AHBM10C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix10c(&self) -> &IoAhbMatrix10c {
        &self.io_ahb_matrix10c
    }
    #[doc = "0x110 - AHBM110 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix110(&self) -> &IoAhbMatrix110 {
        &self.io_ahb_matrix110
    }
    #[doc = "0x114 - AHBM114 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix114(&self) -> &IoAhbMatrix114 {
        &self.io_ahb_matrix114
    }
    #[doc = "0x118 - AHBM118 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix118(&self) -> &IoAhbMatrix118 {
        &self.io_ahb_matrix118
    }
    #[doc = "0x11c - AHBM11C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix11c(&self) -> &IoAhbMatrix11c {
        &self.io_ahb_matrix11c
    }
    #[doc = "0x130 - AHBM130 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix130(&self) -> &IoAhbMatrix130 {
        &self.io_ahb_matrix130
    }
    #[doc = "0x134 - AHBM134 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix134(&self) -> &IoAhbMatrix134 {
        &self.io_ahb_matrix134
    }
    #[doc = "0x138 - AHBM138 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix138(&self) -> &IoAhbMatrix138 {
        &self.io_ahb_matrix138
    }
    #[doc = "0x13c - AHBM13C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix13c(&self) -> &IoAhbMatrix13c {
        &self.io_ahb_matrix13c
    }
    #[doc = "0x140 - AHBM140 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix140(&self) -> &IoAhbMatrix140 {
        &self.io_ahb_matrix140
    }
    #[doc = "0x144 - AHBM144 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix144(&self) -> &IoAhbMatrix144 {
        &self.io_ahb_matrix144
    }
    #[doc = "0x148 - AHBM148 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix148(&self) -> &IoAhbMatrix148 {
        &self.io_ahb_matrix148
    }
    #[doc = "0x14c - AHBM14C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix14c(&self) -> &IoAhbMatrix14c {
        &self.io_ahb_matrix14c
    }
    #[doc = "0x150 - AHBM150 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix150(&self) -> &IoAhbMatrix150 {
        &self.io_ahb_matrix150
    }
    #[doc = "0x154 - AHBM154 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix154(&self) -> &IoAhbMatrix154 {
        &self.io_ahb_matrix154
    }
    #[doc = "0x158 - AHBM158 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix158(&self) -> &IoAhbMatrix158 {
        &self.io_ahb_matrix158
    }
    #[doc = "0x15c - AHBM15C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix15c(&self) -> &IoAhbMatrix15c {
        &self.io_ahb_matrix15c
    }
    #[doc = "0x170 - AHBM170 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix170(&self) -> &IoAhbMatrix170 {
        &self.io_ahb_matrix170
    }
    #[doc = "0x174 - AHBM174 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix174(&self) -> &IoAhbMatrix174 {
        &self.io_ahb_matrix174
    }
    #[doc = "0x178 - AHBM178 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix178(&self) -> &IoAhbMatrix178 {
        &self.io_ahb_matrix178
    }
    #[doc = "0x17c - AHBM17C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix17c(&self) -> &IoAhbMatrix17c {
        &self.io_ahb_matrix17c
    }
    #[doc = "0x180 - AHBM180 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix180(&self) -> &IoAhbMatrix180 {
        &self.io_ahb_matrix180
    }
    #[doc = "0x184 - AHBM184 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix184(&self) -> &IoAhbMatrix184 {
        &self.io_ahb_matrix184
    }
    #[doc = "0x188 - AHBM188 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix188(&self) -> &IoAhbMatrix188 {
        &self.io_ahb_matrix188
    }
    #[doc = "0x18c - AHBM18C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix18c(&self) -> &IoAhbMatrix18c {
        &self.io_ahb_matrix18c
    }
    #[doc = "0x190 - AHBM190 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix190(&self) -> &IoAhbMatrix190 {
        &self.io_ahb_matrix190
    }
    #[doc = "0x194 - AHBM194 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix194(&self) -> &IoAhbMatrix194 {
        &self.io_ahb_matrix194
    }
    #[doc = "0x198 - AHBM198 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix198(&self) -> &IoAhbMatrix198 {
        &self.io_ahb_matrix198
    }
    #[doc = "0x19c - AHBM19C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix19c(&self) -> &IoAhbMatrix19c {
        &self.io_ahb_matrix19c
    }
    #[doc = "0x1b0 - AHBM1B0 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1b0(&self) -> &IoAhbMatrix1b0 {
        &self.io_ahb_matrix1b0
    }
    #[doc = "0x1b4 - AHBM1B4 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1b4(&self) -> &IoAhbMatrix1b4 {
        &self.io_ahb_matrix1b4
    }
    #[doc = "0x1b8 - AHBM1B8 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1b8(&self) -> &IoAhbMatrix1b8 {
        &self.io_ahb_matrix1b8
    }
    #[doc = "0x1bc - AHBM1BC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1bc(&self) -> &IoAhbMatrix1bc {
        &self.io_ahb_matrix1bc
    }
    #[doc = "0x1c0 - AHBM1C0 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1c0(&self) -> &IoAhbMatrix1c0 {
        &self.io_ahb_matrix1c0
    }
    #[doc = "0x1c4 - AHBM1C4 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1c4(&self) -> &IoAhbMatrix1c4 {
        &self.io_ahb_matrix1c4
    }
    #[doc = "0x1c8 - AHBM1C8 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1c8(&self) -> &IoAhbMatrix1c8 {
        &self.io_ahb_matrix1c8
    }
    #[doc = "0x1cc - AHBM1CC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1cc(&self) -> &IoAhbMatrix1cc {
        &self.io_ahb_matrix1cc
    }
    #[doc = "0x1d0 - AHBM1D0 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1d0(&self) -> &IoAhbMatrix1d0 {
        &self.io_ahb_matrix1d0
    }
    #[doc = "0x1d4 - AHBM1D4 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1d4(&self) -> &IoAhbMatrix1d4 {
        &self.io_ahb_matrix1d4
    }
    #[doc = "0x1d8 - AHBM1D8 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1d8(&self) -> &IoAhbMatrix1d8 {
        &self.io_ahb_matrix1d8
    }
    #[doc = "0x1dc - AHBM1DC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1dc(&self) -> &IoAhbMatrix1dc {
        &self.io_ahb_matrix1dc
    }
    #[doc = "0x1f0 - AHBM1F0 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1f0(&self) -> &IoAhbMatrix1f0 {
        &self.io_ahb_matrix1f0
    }
    #[doc = "0x1f4 - AHBM1F4 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1f4(&self) -> &IoAhbMatrix1f4 {
        &self.io_ahb_matrix1f4
    }
    #[doc = "0x1f8 - AHBM1F8 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1f8(&self) -> &IoAhbMatrix1f8 {
        &self.io_ahb_matrix1f8
    }
    #[doc = "0x1fc - AHBM1FC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix1fc(&self) -> &IoAhbMatrix1fc {
        &self.io_ahb_matrix1fc
    }
    #[doc = "0x200 - AHBM200 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix200(&self) -> &IoAhbMatrix200 {
        &self.io_ahb_matrix200
    }
    #[doc = "0x204 - AHBM204 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix204(&self) -> &IoAhbMatrix204 {
        &self.io_ahb_matrix204
    }
    #[doc = "0x208 - AHBM208 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix208(&self) -> &IoAhbMatrix208 {
        &self.io_ahb_matrix208
    }
    #[doc = "0x20c - AHBM20C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix20c(&self) -> &IoAhbMatrix20c {
        &self.io_ahb_matrix20c
    }
    #[doc = "0x210 - AHBM210 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix210(&self) -> &IoAhbMatrix210 {
        &self.io_ahb_matrix210
    }
    #[doc = "0x214 - AHBM214 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix214(&self) -> &IoAhbMatrix214 {
        &self.io_ahb_matrix214
    }
    #[doc = "0x218 - AHBM218 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix218(&self) -> &IoAhbMatrix218 {
        &self.io_ahb_matrix218
    }
    #[doc = "0x21c - AHBM21C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix21c(&self) -> &IoAhbMatrix21c {
        &self.io_ahb_matrix21c
    }
    #[doc = "0x220 - AHBM220 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix220(&self) -> &IoAhbMatrix220 {
        &self.io_ahb_matrix220
    }
    #[doc = "0x224 - AHBM224 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix224(&self) -> &IoAhbMatrix224 {
        &self.io_ahb_matrix224
    }
    #[doc = "0x228 - AHBM228 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix228(&self) -> &IoAhbMatrix228 {
        &self.io_ahb_matrix228
    }
    #[doc = "0x22c - AHBM22C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix22c(&self) -> &IoAhbMatrix22c {
        &self.io_ahb_matrix22c
    }
    #[doc = "0x230 - AHBM230 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix230(&self) -> &IoAhbMatrix230 {
        &self.io_ahb_matrix230
    }
    #[doc = "0x234 - AHBM234 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix234(&self) -> &IoAhbMatrix234 {
        &self.io_ahb_matrix234
    }
    #[doc = "0x238 - AHBM238 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix238(&self) -> &IoAhbMatrix238 {
        &self.io_ahb_matrix238
    }
    #[doc = "0x23c - AHBM23C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix23c(&self) -> &IoAhbMatrix23c {
        &self.io_ahb_matrix23c
    }
    #[doc = "0x260 - AHBM260 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix260(&self) -> &IoAhbMatrix260 {
        &self.io_ahb_matrix260
    }
    #[doc = "0x264 - AHBM264 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix264(&self) -> &IoAhbMatrix264 {
        &self.io_ahb_matrix264
    }
    #[doc = "0x268 - AHBM268 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix268(&self) -> &IoAhbMatrix268 {
        &self.io_ahb_matrix268
    }
    #[doc = "0x26c - AHBM26C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix26c(&self) -> &IoAhbMatrix26c {
        &self.io_ahb_matrix26c
    }
    #[doc = "0x270 - AHBM270 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix270(&self) -> &IoAhbMatrix270 {
        &self.io_ahb_matrix270
    }
    #[doc = "0x274 - AHBM274 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix274(&self) -> &IoAhbMatrix274 {
        &self.io_ahb_matrix274
    }
    #[doc = "0x278 - AHBM278 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix278(&self) -> &IoAhbMatrix278 {
        &self.io_ahb_matrix278
    }
    #[doc = "0x27c - AHBM27C Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix27c(&self) -> &IoAhbMatrix27c {
        &self.io_ahb_matrix27c
    }
    #[doc = "0x2a0 - AHBM2A0 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix2a0(&self) -> &IoAhbMatrix2a0 {
        &self.io_ahb_matrix2a0
    }
    #[doc = "0x2a4 - AHBM2A4 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix2a4(&self) -> &IoAhbMatrix2a4 {
        &self.io_ahb_matrix2a4
    }
    #[doc = "0x2a8 - AHBM2A8 Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix2a8(&self) -> &IoAhbMatrix2a8 {
        &self.io_ahb_matrix2a8
    }
    #[doc = "0x2ac - AHBM2AC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix2ac(&self) -> &IoAhbMatrix2ac {
        &self.io_ahb_matrix2ac
    }
    #[doc = "0x2fc - AHBM2FC Register"]
    #[inline(always)]
    pub const fn io_ahb_matrix2fc(&self) -> &IoAhbMatrix2fc {
        &self.io_ahb_matrix2fc
    }
}
#[doc = "IO_AHB_MATRIX000 (rw) register accessor: AHBM000 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix000`] module"]
#[doc(alias = "IO_AHB_MATRIX000")]
pub type IoAhbMatrix000 = crate::Reg<io_ahb_matrix000::IoAhbMatrix000Spec>;
#[doc = "AHBM000 Register"]
pub mod io_ahb_matrix000;
#[doc = "IO_AHB_MATRIX004 (rw) register accessor: AHBM004 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix004`] module"]
#[doc(alias = "IO_AHB_MATRIX004")]
pub type IoAhbMatrix004 = crate::Reg<io_ahb_matrix004::IoAhbMatrix004Spec>;
#[doc = "AHBM004 Register"]
pub mod io_ahb_matrix004;
#[doc = "IO_AHB_MATRIX008 (rw) register accessor: AHBM008 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix008`] module"]
#[doc(alias = "IO_AHB_MATRIX008")]
pub type IoAhbMatrix008 = crate::Reg<io_ahb_matrix008::IoAhbMatrix008Spec>;
#[doc = "AHBM008 Register"]
pub mod io_ahb_matrix008;
#[doc = "IO_AHB_MATRIX00C (rw) register accessor: AHBM00C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix00c`] module"]
#[doc(alias = "IO_AHB_MATRIX00C")]
pub type IoAhbMatrix00c = crate::Reg<io_ahb_matrix00c::IoAhbMatrix00cSpec>;
#[doc = "AHBM00C Register"]
pub mod io_ahb_matrix00c;
#[doc = "IO_AHB_MATRIX010 (rw) register accessor: AHBM010 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix010`] module"]
#[doc(alias = "IO_AHB_MATRIX010")]
pub type IoAhbMatrix010 = crate::Reg<io_ahb_matrix010::IoAhbMatrix010Spec>;
#[doc = "AHBM010 Register"]
pub mod io_ahb_matrix010;
#[doc = "IO_AHB_MATRIX014 (rw) register accessor: AHBM014 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix014`] module"]
#[doc(alias = "IO_AHB_MATRIX014")]
pub type IoAhbMatrix014 = crate::Reg<io_ahb_matrix014::IoAhbMatrix014Spec>;
#[doc = "AHBM014 Register"]
pub mod io_ahb_matrix014;
#[doc = "IO_AHB_MATRIX018 (rw) register accessor: AHBM018 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix018`] module"]
#[doc(alias = "IO_AHB_MATRIX018")]
pub type IoAhbMatrix018 = crate::Reg<io_ahb_matrix018::IoAhbMatrix018Spec>;
#[doc = "AHBM018 Register"]
pub mod io_ahb_matrix018;
#[doc = "IO_AHB_MATRIX01C (rw) register accessor: AHBM01C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix01c`] module"]
#[doc(alias = "IO_AHB_MATRIX01C")]
pub type IoAhbMatrix01c = crate::Reg<io_ahb_matrix01c::IoAhbMatrix01cSpec>;
#[doc = "AHBM01C Register"]
pub mod io_ahb_matrix01c;
#[doc = "IO_AHB_MATRIX030 (rw) register accessor: AHBM030 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix030`] module"]
#[doc(alias = "IO_AHB_MATRIX030")]
pub type IoAhbMatrix030 = crate::Reg<io_ahb_matrix030::IoAhbMatrix030Spec>;
#[doc = "AHBM030 Register"]
pub mod io_ahb_matrix030;
#[doc = "IO_AHB_MATRIX034 (rw) register accessor: AHBM034 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix034`] module"]
#[doc(alias = "IO_AHB_MATRIX034")]
pub type IoAhbMatrix034 = crate::Reg<io_ahb_matrix034::IoAhbMatrix034Spec>;
#[doc = "AHBM034 Register"]
pub mod io_ahb_matrix034;
#[doc = "IO_AHB_MATRIX038 (rw) register accessor: AHBM038 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix038`] module"]
#[doc(alias = "IO_AHB_MATRIX038")]
pub type IoAhbMatrix038 = crate::Reg<io_ahb_matrix038::IoAhbMatrix038Spec>;
#[doc = "AHBM038 Register"]
pub mod io_ahb_matrix038;
#[doc = "IO_AHB_MATRIX03C (rw) register accessor: AHBM03C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix03c`] module"]
#[doc(alias = "IO_AHB_MATRIX03C")]
pub type IoAhbMatrix03c = crate::Reg<io_ahb_matrix03c::IoAhbMatrix03cSpec>;
#[doc = "AHBM03C Register"]
pub mod io_ahb_matrix03c;
#[doc = "IO_AHB_MATRIX040 (rw) register accessor: AHBM040 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix040`] module"]
#[doc(alias = "IO_AHB_MATRIX040")]
pub type IoAhbMatrix040 = crate::Reg<io_ahb_matrix040::IoAhbMatrix040Spec>;
#[doc = "AHBM040 Register"]
pub mod io_ahb_matrix040;
#[doc = "IO_AHB_MATRIX044 (rw) register accessor: AHBM044 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix044`] module"]
#[doc(alias = "IO_AHB_MATRIX044")]
pub type IoAhbMatrix044 = crate::Reg<io_ahb_matrix044::IoAhbMatrix044Spec>;
#[doc = "AHBM044 Register"]
pub mod io_ahb_matrix044;
#[doc = "IO_AHB_MATRIX048 (rw) register accessor: AHBM048 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix048`] module"]
#[doc(alias = "IO_AHB_MATRIX048")]
pub type IoAhbMatrix048 = crate::Reg<io_ahb_matrix048::IoAhbMatrix048Spec>;
#[doc = "AHBM048 Register"]
pub mod io_ahb_matrix048;
#[doc = "IO_AHB_MATRIX04C (rw) register accessor: AHBM04C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix04c`] module"]
#[doc(alias = "IO_AHB_MATRIX04C")]
pub type IoAhbMatrix04c = crate::Reg<io_ahb_matrix04c::IoAhbMatrix04cSpec>;
#[doc = "AHBM04C Register"]
pub mod io_ahb_matrix04c;
#[doc = "IO_AHB_MATRIX050 (rw) register accessor: AHBM050 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix050`] module"]
#[doc(alias = "IO_AHB_MATRIX050")]
pub type IoAhbMatrix050 = crate::Reg<io_ahb_matrix050::IoAhbMatrix050Spec>;
#[doc = "AHBM050 Register"]
pub mod io_ahb_matrix050;
#[doc = "IO_AHB_MATRIX054 (rw) register accessor: AHBM054 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix054`] module"]
#[doc(alias = "IO_AHB_MATRIX054")]
pub type IoAhbMatrix054 = crate::Reg<io_ahb_matrix054::IoAhbMatrix054Spec>;
#[doc = "AHBM054 Register"]
pub mod io_ahb_matrix054;
#[doc = "IO_AHB_MATRIX058 (rw) register accessor: AHBM058 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix058`] module"]
#[doc(alias = "IO_AHB_MATRIX058")]
pub type IoAhbMatrix058 = crate::Reg<io_ahb_matrix058::IoAhbMatrix058Spec>;
#[doc = "AHBM058 Register"]
pub mod io_ahb_matrix058;
#[doc = "IO_AHB_MATRIX05C (rw) register accessor: AHBM05C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix05c`] module"]
#[doc(alias = "IO_AHB_MATRIX05C")]
pub type IoAhbMatrix05c = crate::Reg<io_ahb_matrix05c::IoAhbMatrix05cSpec>;
#[doc = "AHBM05C Register"]
pub mod io_ahb_matrix05c;
#[doc = "IO_AHB_MATRIX070 (rw) register accessor: AHBM070 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix070`] module"]
#[doc(alias = "IO_AHB_MATRIX070")]
pub type IoAhbMatrix070 = crate::Reg<io_ahb_matrix070::IoAhbMatrix070Spec>;
#[doc = "AHBM070 Register"]
pub mod io_ahb_matrix070;
#[doc = "IO_AHB_MATRIX074 (rw) register accessor: AHBM074 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix074`] module"]
#[doc(alias = "IO_AHB_MATRIX074")]
pub type IoAhbMatrix074 = crate::Reg<io_ahb_matrix074::IoAhbMatrix074Spec>;
#[doc = "AHBM074 Register"]
pub mod io_ahb_matrix074;
#[doc = "IO_AHB_MATRIX078 (rw) register accessor: AHBM078 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix078`] module"]
#[doc(alias = "IO_AHB_MATRIX078")]
pub type IoAhbMatrix078 = crate::Reg<io_ahb_matrix078::IoAhbMatrix078Spec>;
#[doc = "AHBM078 Register"]
pub mod io_ahb_matrix078;
#[doc = "IO_AHB_MATRIX07C (rw) register accessor: AHBM07C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix07c`] module"]
#[doc(alias = "IO_AHB_MATRIX07C")]
pub type IoAhbMatrix07c = crate::Reg<io_ahb_matrix07c::IoAhbMatrix07cSpec>;
#[doc = "AHBM07C Register"]
pub mod io_ahb_matrix07c;
#[doc = "IO_AHB_MATRIX080 (rw) register accessor: AHBM080 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix080`] module"]
#[doc(alias = "IO_AHB_MATRIX080")]
pub type IoAhbMatrix080 = crate::Reg<io_ahb_matrix080::IoAhbMatrix080Spec>;
#[doc = "AHBM080 Register"]
pub mod io_ahb_matrix080;
#[doc = "IO_AHB_MATRIX084 (rw) register accessor: AHBM084 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix084`] module"]
#[doc(alias = "IO_AHB_MATRIX084")]
pub type IoAhbMatrix084 = crate::Reg<io_ahb_matrix084::IoAhbMatrix084Spec>;
#[doc = "AHBM084 Register"]
pub mod io_ahb_matrix084;
#[doc = "IO_AHB_MATRIX088 (rw) register accessor: AHBM088 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix088`] module"]
#[doc(alias = "IO_AHB_MATRIX088")]
pub type IoAhbMatrix088 = crate::Reg<io_ahb_matrix088::IoAhbMatrix088Spec>;
#[doc = "AHBM088 Register"]
pub mod io_ahb_matrix088;
#[doc = "IO_AHB_MATRIX08C (rw) register accessor: AHBM08C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix08c`] module"]
#[doc(alias = "IO_AHB_MATRIX08C")]
pub type IoAhbMatrix08c = crate::Reg<io_ahb_matrix08c::IoAhbMatrix08cSpec>;
#[doc = "AHBM08C Register"]
pub mod io_ahb_matrix08c;
#[doc = "IO_AHB_MATRIX090 (rw) register accessor: AHBM090 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix090`] module"]
#[doc(alias = "IO_AHB_MATRIX090")]
pub type IoAhbMatrix090 = crate::Reg<io_ahb_matrix090::IoAhbMatrix090Spec>;
#[doc = "AHBM090 Register"]
pub mod io_ahb_matrix090;
#[doc = "IO_AHB_MATRIX094 (rw) register accessor: AHBM094 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix094`] module"]
#[doc(alias = "IO_AHB_MATRIX094")]
pub type IoAhbMatrix094 = crate::Reg<io_ahb_matrix094::IoAhbMatrix094Spec>;
#[doc = "AHBM094 Register"]
pub mod io_ahb_matrix094;
#[doc = "IO_AHB_MATRIX098 (rw) register accessor: AHBM098 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix098`] module"]
#[doc(alias = "IO_AHB_MATRIX098")]
pub type IoAhbMatrix098 = crate::Reg<io_ahb_matrix098::IoAhbMatrix098Spec>;
#[doc = "AHBM098 Register"]
pub mod io_ahb_matrix098;
#[doc = "IO_AHB_MATRIX09C (rw) register accessor: AHBM09C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix09c`] module"]
#[doc(alias = "IO_AHB_MATRIX09C")]
pub type IoAhbMatrix09c = crate::Reg<io_ahb_matrix09c::IoAhbMatrix09cSpec>;
#[doc = "AHBM09C Register"]
pub mod io_ahb_matrix09c;
#[doc = "IO_AHB_MATRIX0B0 (rw) register accessor: AHBM0B0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0b0`] module"]
#[doc(alias = "IO_AHB_MATRIX0B0")]
pub type IoAhbMatrix0b0 = crate::Reg<io_ahb_matrix0b0::IoAhbMatrix0b0Spec>;
#[doc = "AHBM0B0 Register"]
pub mod io_ahb_matrix0b0;
#[doc = "IO_AHB_MATRIX0B4 (rw) register accessor: AHBM0B4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0b4`] module"]
#[doc(alias = "IO_AHB_MATRIX0B4")]
pub type IoAhbMatrix0b4 = crate::Reg<io_ahb_matrix0b4::IoAhbMatrix0b4Spec>;
#[doc = "AHBM0B4 Register"]
pub mod io_ahb_matrix0b4;
#[doc = "IO_AHB_MATRIX0B8 (rw) register accessor: AHBM0B8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0b8`] module"]
#[doc(alias = "IO_AHB_MATRIX0B8")]
pub type IoAhbMatrix0b8 = crate::Reg<io_ahb_matrix0b8::IoAhbMatrix0b8Spec>;
#[doc = "AHBM0B8 Register"]
pub mod io_ahb_matrix0b8;
#[doc = "IO_AHB_MATRIX0BC (rw) register accessor: AHBM0BC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0bc`] module"]
#[doc(alias = "IO_AHB_MATRIX0BC")]
pub type IoAhbMatrix0bc = crate::Reg<io_ahb_matrix0bc::IoAhbMatrix0bcSpec>;
#[doc = "AHBM0BC Register"]
pub mod io_ahb_matrix0bc;
#[doc = "IO_AHB_MATRIX0C0 (rw) register accessor: AHBM0C0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0c0`] module"]
#[doc(alias = "IO_AHB_MATRIX0C0")]
pub type IoAhbMatrix0c0 = crate::Reg<io_ahb_matrix0c0::IoAhbMatrix0c0Spec>;
#[doc = "AHBM0C0 Register"]
pub mod io_ahb_matrix0c0;
#[doc = "IO_AHB_MATRIX0C4 (rw) register accessor: AHBM0C4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0c4`] module"]
#[doc(alias = "IO_AHB_MATRIX0C4")]
pub type IoAhbMatrix0c4 = crate::Reg<io_ahb_matrix0c4::IoAhbMatrix0c4Spec>;
#[doc = "AHBM0C4 Register"]
pub mod io_ahb_matrix0c4;
#[doc = "IO_AHB_MATRIX0C8 (rw) register accessor: AHBM0C8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0c8`] module"]
#[doc(alias = "IO_AHB_MATRIX0C8")]
pub type IoAhbMatrix0c8 = crate::Reg<io_ahb_matrix0c8::IoAhbMatrix0c8Spec>;
#[doc = "AHBM0C8 Register"]
pub mod io_ahb_matrix0c8;
#[doc = "IO_AHB_MATRIX0CC (rw) register accessor: AHBM0CC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0cc`] module"]
#[doc(alias = "IO_AHB_MATRIX0CC")]
pub type IoAhbMatrix0cc = crate::Reg<io_ahb_matrix0cc::IoAhbMatrix0ccSpec>;
#[doc = "AHBM0CC Register"]
pub mod io_ahb_matrix0cc;
#[doc = "IO_AHB_MATRIX0D0 (rw) register accessor: AHBM0D0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0d0`] module"]
#[doc(alias = "IO_AHB_MATRIX0D0")]
pub type IoAhbMatrix0d0 = crate::Reg<io_ahb_matrix0d0::IoAhbMatrix0d0Spec>;
#[doc = "AHBM0D0 Register"]
pub mod io_ahb_matrix0d0;
#[doc = "IO_AHB_MATRIX0D4 (rw) register accessor: AHBM0D4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0d4`] module"]
#[doc(alias = "IO_AHB_MATRIX0D4")]
pub type IoAhbMatrix0d4 = crate::Reg<io_ahb_matrix0d4::IoAhbMatrix0d4Spec>;
#[doc = "AHBM0D4 Register"]
pub mod io_ahb_matrix0d4;
#[doc = "IO_AHB_MATRIX0D8 (rw) register accessor: AHBM0D8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0d8`] module"]
#[doc(alias = "IO_AHB_MATRIX0D8")]
pub type IoAhbMatrix0d8 = crate::Reg<io_ahb_matrix0d8::IoAhbMatrix0d8Spec>;
#[doc = "AHBM0D8 Register"]
pub mod io_ahb_matrix0d8;
#[doc = "IO_AHB_MATRIX0DC (rw) register accessor: AHBM0DC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0dc`] module"]
#[doc(alias = "IO_AHB_MATRIX0DC")]
pub type IoAhbMatrix0dc = crate::Reg<io_ahb_matrix0dc::IoAhbMatrix0dcSpec>;
#[doc = "AHBM0DC Register"]
pub mod io_ahb_matrix0dc;
#[doc = "IO_AHB_MATRIX0F0 (rw) register accessor: AHBM0F0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0f0`] module"]
#[doc(alias = "IO_AHB_MATRIX0F0")]
pub type IoAhbMatrix0f0 = crate::Reg<io_ahb_matrix0f0::IoAhbMatrix0f0Spec>;
#[doc = "AHBM0F0 Register"]
pub mod io_ahb_matrix0f0;
#[doc = "IO_AHB_MATRIX0F4 (rw) register accessor: AHBM0F4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0f4`] module"]
#[doc(alias = "IO_AHB_MATRIX0F4")]
pub type IoAhbMatrix0f4 = crate::Reg<io_ahb_matrix0f4::IoAhbMatrix0f4Spec>;
#[doc = "AHBM0F4 Register"]
pub mod io_ahb_matrix0f4;
#[doc = "IO_AHB_MATRIX0F8 (rw) register accessor: AHBM0F8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0f8`] module"]
#[doc(alias = "IO_AHB_MATRIX0F8")]
pub type IoAhbMatrix0f8 = crate::Reg<io_ahb_matrix0f8::IoAhbMatrix0f8Spec>;
#[doc = "AHBM0F8 Register"]
pub mod io_ahb_matrix0f8;
#[doc = "IO_AHB_MATRIX0FC (rw) register accessor: AHBM0FC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix0fc`] module"]
#[doc(alias = "IO_AHB_MATRIX0FC")]
pub type IoAhbMatrix0fc = crate::Reg<io_ahb_matrix0fc::IoAhbMatrix0fcSpec>;
#[doc = "AHBM0FC Register"]
pub mod io_ahb_matrix0fc;
#[doc = "IO_AHB_MATRIX100 (rw) register accessor: AHBM100 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix100`] module"]
#[doc(alias = "IO_AHB_MATRIX100")]
pub type IoAhbMatrix100 = crate::Reg<io_ahb_matrix100::IoAhbMatrix100Spec>;
#[doc = "AHBM100 Register"]
pub mod io_ahb_matrix100;
#[doc = "IO_AHB_MATRIX104 (rw) register accessor: AHBM104 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix104`] module"]
#[doc(alias = "IO_AHB_MATRIX104")]
pub type IoAhbMatrix104 = crate::Reg<io_ahb_matrix104::IoAhbMatrix104Spec>;
#[doc = "AHBM104 Register"]
pub mod io_ahb_matrix104;
#[doc = "IO_AHB_MATRIX108 (rw) register accessor: AHBM108 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix108`] module"]
#[doc(alias = "IO_AHB_MATRIX108")]
pub type IoAhbMatrix108 = crate::Reg<io_ahb_matrix108::IoAhbMatrix108Spec>;
#[doc = "AHBM108 Register"]
pub mod io_ahb_matrix108;
#[doc = "IO_AHB_MATRIX10C (rw) register accessor: AHBM10C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix10c`] module"]
#[doc(alias = "IO_AHB_MATRIX10C")]
pub type IoAhbMatrix10c = crate::Reg<io_ahb_matrix10c::IoAhbMatrix10cSpec>;
#[doc = "AHBM10C Register"]
pub mod io_ahb_matrix10c;
#[doc = "IO_AHB_MATRIX110 (rw) register accessor: AHBM110 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix110`] module"]
#[doc(alias = "IO_AHB_MATRIX110")]
pub type IoAhbMatrix110 = crate::Reg<io_ahb_matrix110::IoAhbMatrix110Spec>;
#[doc = "AHBM110 Register"]
pub mod io_ahb_matrix110;
#[doc = "IO_AHB_MATRIX114 (rw) register accessor: AHBM114 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix114`] module"]
#[doc(alias = "IO_AHB_MATRIX114")]
pub type IoAhbMatrix114 = crate::Reg<io_ahb_matrix114::IoAhbMatrix114Spec>;
#[doc = "AHBM114 Register"]
pub mod io_ahb_matrix114;
#[doc = "IO_AHB_MATRIX118 (rw) register accessor: AHBM118 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix118`] module"]
#[doc(alias = "IO_AHB_MATRIX118")]
pub type IoAhbMatrix118 = crate::Reg<io_ahb_matrix118::IoAhbMatrix118Spec>;
#[doc = "AHBM118 Register"]
pub mod io_ahb_matrix118;
#[doc = "IO_AHB_MATRIX11C (rw) register accessor: AHBM11C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix11c`] module"]
#[doc(alias = "IO_AHB_MATRIX11C")]
pub type IoAhbMatrix11c = crate::Reg<io_ahb_matrix11c::IoAhbMatrix11cSpec>;
#[doc = "AHBM11C Register"]
pub mod io_ahb_matrix11c;
#[doc = "IO_AHB_MATRIX130 (rw) register accessor: AHBM130 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix130`] module"]
#[doc(alias = "IO_AHB_MATRIX130")]
pub type IoAhbMatrix130 = crate::Reg<io_ahb_matrix130::IoAhbMatrix130Spec>;
#[doc = "AHBM130 Register"]
pub mod io_ahb_matrix130;
#[doc = "IO_AHB_MATRIX134 (rw) register accessor: AHBM134 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix134`] module"]
#[doc(alias = "IO_AHB_MATRIX134")]
pub type IoAhbMatrix134 = crate::Reg<io_ahb_matrix134::IoAhbMatrix134Spec>;
#[doc = "AHBM134 Register"]
pub mod io_ahb_matrix134;
#[doc = "IO_AHB_MATRIX138 (rw) register accessor: AHBM138 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix138`] module"]
#[doc(alias = "IO_AHB_MATRIX138")]
pub type IoAhbMatrix138 = crate::Reg<io_ahb_matrix138::IoAhbMatrix138Spec>;
#[doc = "AHBM138 Register"]
pub mod io_ahb_matrix138;
#[doc = "IO_AHB_MATRIX13C (rw) register accessor: AHBM13C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix13c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix13c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix13c`] module"]
#[doc(alias = "IO_AHB_MATRIX13C")]
pub type IoAhbMatrix13c = crate::Reg<io_ahb_matrix13c::IoAhbMatrix13cSpec>;
#[doc = "AHBM13C Register"]
pub mod io_ahb_matrix13c;
#[doc = "IO_AHB_MATRIX140 (rw) register accessor: AHBM140 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix140`] module"]
#[doc(alias = "IO_AHB_MATRIX140")]
pub type IoAhbMatrix140 = crate::Reg<io_ahb_matrix140::IoAhbMatrix140Spec>;
#[doc = "AHBM140 Register"]
pub mod io_ahb_matrix140;
#[doc = "IO_AHB_MATRIX144 (rw) register accessor: AHBM144 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix144`] module"]
#[doc(alias = "IO_AHB_MATRIX144")]
pub type IoAhbMatrix144 = crate::Reg<io_ahb_matrix144::IoAhbMatrix144Spec>;
#[doc = "AHBM144 Register"]
pub mod io_ahb_matrix144;
#[doc = "IO_AHB_MATRIX148 (rw) register accessor: AHBM148 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix148`] module"]
#[doc(alias = "IO_AHB_MATRIX148")]
pub type IoAhbMatrix148 = crate::Reg<io_ahb_matrix148::IoAhbMatrix148Spec>;
#[doc = "AHBM148 Register"]
pub mod io_ahb_matrix148;
#[doc = "IO_AHB_MATRIX14C (rw) register accessor: AHBM14C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix14c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix14c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix14c`] module"]
#[doc(alias = "IO_AHB_MATRIX14C")]
pub type IoAhbMatrix14c = crate::Reg<io_ahb_matrix14c::IoAhbMatrix14cSpec>;
#[doc = "AHBM14C Register"]
pub mod io_ahb_matrix14c;
#[doc = "IO_AHB_MATRIX150 (rw) register accessor: AHBM150 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix150`] module"]
#[doc(alias = "IO_AHB_MATRIX150")]
pub type IoAhbMatrix150 = crate::Reg<io_ahb_matrix150::IoAhbMatrix150Spec>;
#[doc = "AHBM150 Register"]
pub mod io_ahb_matrix150;
#[doc = "IO_AHB_MATRIX154 (rw) register accessor: AHBM154 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix154`] module"]
#[doc(alias = "IO_AHB_MATRIX154")]
pub type IoAhbMatrix154 = crate::Reg<io_ahb_matrix154::IoAhbMatrix154Spec>;
#[doc = "AHBM154 Register"]
pub mod io_ahb_matrix154;
#[doc = "IO_AHB_MATRIX158 (rw) register accessor: AHBM158 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix158`] module"]
#[doc(alias = "IO_AHB_MATRIX158")]
pub type IoAhbMatrix158 = crate::Reg<io_ahb_matrix158::IoAhbMatrix158Spec>;
#[doc = "AHBM158 Register"]
pub mod io_ahb_matrix158;
#[doc = "IO_AHB_MATRIX15C (rw) register accessor: AHBM15C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix15c`] module"]
#[doc(alias = "IO_AHB_MATRIX15C")]
pub type IoAhbMatrix15c = crate::Reg<io_ahb_matrix15c::IoAhbMatrix15cSpec>;
#[doc = "AHBM15C Register"]
pub mod io_ahb_matrix15c;
#[doc = "IO_AHB_MATRIX170 (rw) register accessor: AHBM170 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix170`] module"]
#[doc(alias = "IO_AHB_MATRIX170")]
pub type IoAhbMatrix170 = crate::Reg<io_ahb_matrix170::IoAhbMatrix170Spec>;
#[doc = "AHBM170 Register"]
pub mod io_ahb_matrix170;
#[doc = "IO_AHB_MATRIX174 (rw) register accessor: AHBM174 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix174`] module"]
#[doc(alias = "IO_AHB_MATRIX174")]
pub type IoAhbMatrix174 = crate::Reg<io_ahb_matrix174::IoAhbMatrix174Spec>;
#[doc = "AHBM174 Register"]
pub mod io_ahb_matrix174;
#[doc = "IO_AHB_MATRIX178 (rw) register accessor: AHBM178 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix178`] module"]
#[doc(alias = "IO_AHB_MATRIX178")]
pub type IoAhbMatrix178 = crate::Reg<io_ahb_matrix178::IoAhbMatrix178Spec>;
#[doc = "AHBM178 Register"]
pub mod io_ahb_matrix178;
#[doc = "IO_AHB_MATRIX17C (rw) register accessor: AHBM17C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix17c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix17c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix17c`] module"]
#[doc(alias = "IO_AHB_MATRIX17C")]
pub type IoAhbMatrix17c = crate::Reg<io_ahb_matrix17c::IoAhbMatrix17cSpec>;
#[doc = "AHBM17C Register"]
pub mod io_ahb_matrix17c;
#[doc = "IO_AHB_MATRIX180 (rw) register accessor: AHBM180 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix180`] module"]
#[doc(alias = "IO_AHB_MATRIX180")]
pub type IoAhbMatrix180 = crate::Reg<io_ahb_matrix180::IoAhbMatrix180Spec>;
#[doc = "AHBM180 Register"]
pub mod io_ahb_matrix180;
#[doc = "IO_AHB_MATRIX184 (rw) register accessor: AHBM184 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix184`] module"]
#[doc(alias = "IO_AHB_MATRIX184")]
pub type IoAhbMatrix184 = crate::Reg<io_ahb_matrix184::IoAhbMatrix184Spec>;
#[doc = "AHBM184 Register"]
pub mod io_ahb_matrix184;
#[doc = "IO_AHB_MATRIX188 (rw) register accessor: AHBM188 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix188`] module"]
#[doc(alias = "IO_AHB_MATRIX188")]
pub type IoAhbMatrix188 = crate::Reg<io_ahb_matrix188::IoAhbMatrix188Spec>;
#[doc = "AHBM188 Register"]
pub mod io_ahb_matrix188;
#[doc = "IO_AHB_MATRIX18C (rw) register accessor: AHBM18C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix18c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix18c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix18c`] module"]
#[doc(alias = "IO_AHB_MATRIX18C")]
pub type IoAhbMatrix18c = crate::Reg<io_ahb_matrix18c::IoAhbMatrix18cSpec>;
#[doc = "AHBM18C Register"]
pub mod io_ahb_matrix18c;
#[doc = "IO_AHB_MATRIX190 (rw) register accessor: AHBM190 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix190`] module"]
#[doc(alias = "IO_AHB_MATRIX190")]
pub type IoAhbMatrix190 = crate::Reg<io_ahb_matrix190::IoAhbMatrix190Spec>;
#[doc = "AHBM190 Register"]
pub mod io_ahb_matrix190;
#[doc = "IO_AHB_MATRIX194 (rw) register accessor: AHBM194 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix194`] module"]
#[doc(alias = "IO_AHB_MATRIX194")]
pub type IoAhbMatrix194 = crate::Reg<io_ahb_matrix194::IoAhbMatrix194Spec>;
#[doc = "AHBM194 Register"]
pub mod io_ahb_matrix194;
#[doc = "IO_AHB_MATRIX198 (rw) register accessor: AHBM198 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix198`] module"]
#[doc(alias = "IO_AHB_MATRIX198")]
pub type IoAhbMatrix198 = crate::Reg<io_ahb_matrix198::IoAhbMatrix198Spec>;
#[doc = "AHBM198 Register"]
pub mod io_ahb_matrix198;
#[doc = "IO_AHB_MATRIX19C (rw) register accessor: AHBM19C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix19c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix19c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix19c`] module"]
#[doc(alias = "IO_AHB_MATRIX19C")]
pub type IoAhbMatrix19c = crate::Reg<io_ahb_matrix19c::IoAhbMatrix19cSpec>;
#[doc = "AHBM19C Register"]
pub mod io_ahb_matrix19c;
#[doc = "IO_AHB_MATRIX1B0 (rw) register accessor: AHBM1B0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1b0`] module"]
#[doc(alias = "IO_AHB_MATRIX1B0")]
pub type IoAhbMatrix1b0 = crate::Reg<io_ahb_matrix1b0::IoAhbMatrix1b0Spec>;
#[doc = "AHBM1B0 Register"]
pub mod io_ahb_matrix1b0;
#[doc = "IO_AHB_MATRIX1B4 (rw) register accessor: AHBM1B4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1b4`] module"]
#[doc(alias = "IO_AHB_MATRIX1B4")]
pub type IoAhbMatrix1b4 = crate::Reg<io_ahb_matrix1b4::IoAhbMatrix1b4Spec>;
#[doc = "AHBM1B4 Register"]
pub mod io_ahb_matrix1b4;
#[doc = "IO_AHB_MATRIX1B8 (rw) register accessor: AHBM1B8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1b8`] module"]
#[doc(alias = "IO_AHB_MATRIX1B8")]
pub type IoAhbMatrix1b8 = crate::Reg<io_ahb_matrix1b8::IoAhbMatrix1b8Spec>;
#[doc = "AHBM1B8 Register"]
pub mod io_ahb_matrix1b8;
#[doc = "IO_AHB_MATRIX1BC (rw) register accessor: AHBM1BC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1bc`] module"]
#[doc(alias = "IO_AHB_MATRIX1BC")]
pub type IoAhbMatrix1bc = crate::Reg<io_ahb_matrix1bc::IoAhbMatrix1bcSpec>;
#[doc = "AHBM1BC Register"]
pub mod io_ahb_matrix1bc;
#[doc = "IO_AHB_MATRIX1C0 (rw) register accessor: AHBM1C0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1c0`] module"]
#[doc(alias = "IO_AHB_MATRIX1C0")]
pub type IoAhbMatrix1c0 = crate::Reg<io_ahb_matrix1c0::IoAhbMatrix1c0Spec>;
#[doc = "AHBM1C0 Register"]
pub mod io_ahb_matrix1c0;
#[doc = "IO_AHB_MATRIX1C4 (rw) register accessor: AHBM1C4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1c4`] module"]
#[doc(alias = "IO_AHB_MATRIX1C4")]
pub type IoAhbMatrix1c4 = crate::Reg<io_ahb_matrix1c4::IoAhbMatrix1c4Spec>;
#[doc = "AHBM1C4 Register"]
pub mod io_ahb_matrix1c4;
#[doc = "IO_AHB_MATRIX1C8 (rw) register accessor: AHBM1C8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1c8`] module"]
#[doc(alias = "IO_AHB_MATRIX1C8")]
pub type IoAhbMatrix1c8 = crate::Reg<io_ahb_matrix1c8::IoAhbMatrix1c8Spec>;
#[doc = "AHBM1C8 Register"]
pub mod io_ahb_matrix1c8;
#[doc = "IO_AHB_MATRIX1CC (rw) register accessor: AHBM1CC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1cc`] module"]
#[doc(alias = "IO_AHB_MATRIX1CC")]
pub type IoAhbMatrix1cc = crate::Reg<io_ahb_matrix1cc::IoAhbMatrix1ccSpec>;
#[doc = "AHBM1CC Register"]
pub mod io_ahb_matrix1cc;
#[doc = "IO_AHB_MATRIX1D0 (rw) register accessor: AHBM1D0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1d0`] module"]
#[doc(alias = "IO_AHB_MATRIX1D0")]
pub type IoAhbMatrix1d0 = crate::Reg<io_ahb_matrix1d0::IoAhbMatrix1d0Spec>;
#[doc = "AHBM1D0 Register"]
pub mod io_ahb_matrix1d0;
#[doc = "IO_AHB_MATRIX1D4 (rw) register accessor: AHBM1D4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1d4`] module"]
#[doc(alias = "IO_AHB_MATRIX1D4")]
pub type IoAhbMatrix1d4 = crate::Reg<io_ahb_matrix1d4::IoAhbMatrix1d4Spec>;
#[doc = "AHBM1D4 Register"]
pub mod io_ahb_matrix1d4;
#[doc = "IO_AHB_MATRIX1D8 (rw) register accessor: AHBM1D8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1d8`] module"]
#[doc(alias = "IO_AHB_MATRIX1D8")]
pub type IoAhbMatrix1d8 = crate::Reg<io_ahb_matrix1d8::IoAhbMatrix1d8Spec>;
#[doc = "AHBM1D8 Register"]
pub mod io_ahb_matrix1d8;
#[doc = "IO_AHB_MATRIX1DC (rw) register accessor: AHBM1DC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1dc`] module"]
#[doc(alias = "IO_AHB_MATRIX1DC")]
pub type IoAhbMatrix1dc = crate::Reg<io_ahb_matrix1dc::IoAhbMatrix1dcSpec>;
#[doc = "AHBM1DC Register"]
pub mod io_ahb_matrix1dc;
#[doc = "IO_AHB_MATRIX1F0 (rw) register accessor: AHBM1F0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1f0`] module"]
#[doc(alias = "IO_AHB_MATRIX1F0")]
pub type IoAhbMatrix1f0 = crate::Reg<io_ahb_matrix1f0::IoAhbMatrix1f0Spec>;
#[doc = "AHBM1F0 Register"]
pub mod io_ahb_matrix1f0;
#[doc = "IO_AHB_MATRIX1F4 (rw) register accessor: AHBM1F4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1f4`] module"]
#[doc(alias = "IO_AHB_MATRIX1F4")]
pub type IoAhbMatrix1f4 = crate::Reg<io_ahb_matrix1f4::IoAhbMatrix1f4Spec>;
#[doc = "AHBM1F4 Register"]
pub mod io_ahb_matrix1f4;
#[doc = "IO_AHB_MATRIX1F8 (rw) register accessor: AHBM1F8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1f8`] module"]
#[doc(alias = "IO_AHB_MATRIX1F8")]
pub type IoAhbMatrix1f8 = crate::Reg<io_ahb_matrix1f8::IoAhbMatrix1f8Spec>;
#[doc = "AHBM1F8 Register"]
pub mod io_ahb_matrix1f8;
#[doc = "IO_AHB_MATRIX1FC (rw) register accessor: AHBM1FC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix1fc`] module"]
#[doc(alias = "IO_AHB_MATRIX1FC")]
pub type IoAhbMatrix1fc = crate::Reg<io_ahb_matrix1fc::IoAhbMatrix1fcSpec>;
#[doc = "AHBM1FC Register"]
pub mod io_ahb_matrix1fc;
#[doc = "IO_AHB_MATRIX200 (rw) register accessor: AHBM200 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix200`] module"]
#[doc(alias = "IO_AHB_MATRIX200")]
pub type IoAhbMatrix200 = crate::Reg<io_ahb_matrix200::IoAhbMatrix200Spec>;
#[doc = "AHBM200 Register"]
pub mod io_ahb_matrix200;
#[doc = "IO_AHB_MATRIX204 (rw) register accessor: AHBM204 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix204`] module"]
#[doc(alias = "IO_AHB_MATRIX204")]
pub type IoAhbMatrix204 = crate::Reg<io_ahb_matrix204::IoAhbMatrix204Spec>;
#[doc = "AHBM204 Register"]
pub mod io_ahb_matrix204;
#[doc = "IO_AHB_MATRIX208 (rw) register accessor: AHBM208 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix208`] module"]
#[doc(alias = "IO_AHB_MATRIX208")]
pub type IoAhbMatrix208 = crate::Reg<io_ahb_matrix208::IoAhbMatrix208Spec>;
#[doc = "AHBM208 Register"]
pub mod io_ahb_matrix208;
#[doc = "IO_AHB_MATRIX20C (rw) register accessor: AHBM20C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix20c`] module"]
#[doc(alias = "IO_AHB_MATRIX20C")]
pub type IoAhbMatrix20c = crate::Reg<io_ahb_matrix20c::IoAhbMatrix20cSpec>;
#[doc = "AHBM20C Register"]
pub mod io_ahb_matrix20c;
#[doc = "IO_AHB_MATRIX210 (rw) register accessor: AHBM210 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix210`] module"]
#[doc(alias = "IO_AHB_MATRIX210")]
pub type IoAhbMatrix210 = crate::Reg<io_ahb_matrix210::IoAhbMatrix210Spec>;
#[doc = "AHBM210 Register"]
pub mod io_ahb_matrix210;
#[doc = "IO_AHB_MATRIX214 (rw) register accessor: AHBM214 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix214`] module"]
#[doc(alias = "IO_AHB_MATRIX214")]
pub type IoAhbMatrix214 = crate::Reg<io_ahb_matrix214::IoAhbMatrix214Spec>;
#[doc = "AHBM214 Register"]
pub mod io_ahb_matrix214;
#[doc = "IO_AHB_MATRIX218 (rw) register accessor: AHBM218 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix218`] module"]
#[doc(alias = "IO_AHB_MATRIX218")]
pub type IoAhbMatrix218 = crate::Reg<io_ahb_matrix218::IoAhbMatrix218Spec>;
#[doc = "AHBM218 Register"]
pub mod io_ahb_matrix218;
#[doc = "IO_AHB_MATRIX21C (rw) register accessor: AHBM21C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix21c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix21c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix21c`] module"]
#[doc(alias = "IO_AHB_MATRIX21C")]
pub type IoAhbMatrix21c = crate::Reg<io_ahb_matrix21c::IoAhbMatrix21cSpec>;
#[doc = "AHBM21C Register"]
pub mod io_ahb_matrix21c;
#[doc = "IO_AHB_MATRIX220 (rw) register accessor: AHBM220 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix220`] module"]
#[doc(alias = "IO_AHB_MATRIX220")]
pub type IoAhbMatrix220 = crate::Reg<io_ahb_matrix220::IoAhbMatrix220Spec>;
#[doc = "AHBM220 Register"]
pub mod io_ahb_matrix220;
#[doc = "IO_AHB_MATRIX224 (rw) register accessor: AHBM224 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix224`] module"]
#[doc(alias = "IO_AHB_MATRIX224")]
pub type IoAhbMatrix224 = crate::Reg<io_ahb_matrix224::IoAhbMatrix224Spec>;
#[doc = "AHBM224 Register"]
pub mod io_ahb_matrix224;
#[doc = "IO_AHB_MATRIX228 (rw) register accessor: AHBM228 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix228`] module"]
#[doc(alias = "IO_AHB_MATRIX228")]
pub type IoAhbMatrix228 = crate::Reg<io_ahb_matrix228::IoAhbMatrix228Spec>;
#[doc = "AHBM228 Register"]
pub mod io_ahb_matrix228;
#[doc = "IO_AHB_MATRIX22C (rw) register accessor: AHBM22C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix22c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix22c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix22c`] module"]
#[doc(alias = "IO_AHB_MATRIX22C")]
pub type IoAhbMatrix22c = crate::Reg<io_ahb_matrix22c::IoAhbMatrix22cSpec>;
#[doc = "AHBM22C Register"]
pub mod io_ahb_matrix22c;
#[doc = "IO_AHB_MATRIX230 (rw) register accessor: AHBM230 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix230`] module"]
#[doc(alias = "IO_AHB_MATRIX230")]
pub type IoAhbMatrix230 = crate::Reg<io_ahb_matrix230::IoAhbMatrix230Spec>;
#[doc = "AHBM230 Register"]
pub mod io_ahb_matrix230;
#[doc = "IO_AHB_MATRIX234 (rw) register accessor: AHBM234 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix234`] module"]
#[doc(alias = "IO_AHB_MATRIX234")]
pub type IoAhbMatrix234 = crate::Reg<io_ahb_matrix234::IoAhbMatrix234Spec>;
#[doc = "AHBM234 Register"]
pub mod io_ahb_matrix234;
#[doc = "IO_AHB_MATRIX238 (rw) register accessor: AHBM238 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix238`] module"]
#[doc(alias = "IO_AHB_MATRIX238")]
pub type IoAhbMatrix238 = crate::Reg<io_ahb_matrix238::IoAhbMatrix238Spec>;
#[doc = "AHBM238 Register"]
pub mod io_ahb_matrix238;
#[doc = "IO_AHB_MATRIX23C (rw) register accessor: AHBM23C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix23c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix23c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix23c`] module"]
#[doc(alias = "IO_AHB_MATRIX23C")]
pub type IoAhbMatrix23c = crate::Reg<io_ahb_matrix23c::IoAhbMatrix23cSpec>;
#[doc = "AHBM23C Register"]
pub mod io_ahb_matrix23c;
#[doc = "IO_AHB_MATRIX260 (rw) register accessor: AHBM260 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix260::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix260::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix260`] module"]
#[doc(alias = "IO_AHB_MATRIX260")]
pub type IoAhbMatrix260 = crate::Reg<io_ahb_matrix260::IoAhbMatrix260Spec>;
#[doc = "AHBM260 Register"]
pub mod io_ahb_matrix260;
#[doc = "IO_AHB_MATRIX264 (rw) register accessor: AHBM264 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix264::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix264::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix264`] module"]
#[doc(alias = "IO_AHB_MATRIX264")]
pub type IoAhbMatrix264 = crate::Reg<io_ahb_matrix264::IoAhbMatrix264Spec>;
#[doc = "AHBM264 Register"]
pub mod io_ahb_matrix264;
#[doc = "IO_AHB_MATRIX268 (rw) register accessor: AHBM268 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix268::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix268::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix268`] module"]
#[doc(alias = "IO_AHB_MATRIX268")]
pub type IoAhbMatrix268 = crate::Reg<io_ahb_matrix268::IoAhbMatrix268Spec>;
#[doc = "AHBM268 Register"]
pub mod io_ahb_matrix268;
#[doc = "IO_AHB_MATRIX26C (rw) register accessor: AHBM26C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix26c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix26c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix26c`] module"]
#[doc(alias = "IO_AHB_MATRIX26C")]
pub type IoAhbMatrix26c = crate::Reg<io_ahb_matrix26c::IoAhbMatrix26cSpec>;
#[doc = "AHBM26C Register"]
pub mod io_ahb_matrix26c;
#[doc = "IO_AHB_MATRIX270 (rw) register accessor: AHBM270 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix270::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix270::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix270`] module"]
#[doc(alias = "IO_AHB_MATRIX270")]
pub type IoAhbMatrix270 = crate::Reg<io_ahb_matrix270::IoAhbMatrix270Spec>;
#[doc = "AHBM270 Register"]
pub mod io_ahb_matrix270;
#[doc = "IO_AHB_MATRIX274 (rw) register accessor: AHBM274 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix274::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix274::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix274`] module"]
#[doc(alias = "IO_AHB_MATRIX274")]
pub type IoAhbMatrix274 = crate::Reg<io_ahb_matrix274::IoAhbMatrix274Spec>;
#[doc = "AHBM274 Register"]
pub mod io_ahb_matrix274;
#[doc = "IO_AHB_MATRIX278 (rw) register accessor: AHBM278 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix278::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix278::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix278`] module"]
#[doc(alias = "IO_AHB_MATRIX278")]
pub type IoAhbMatrix278 = crate::Reg<io_ahb_matrix278::IoAhbMatrix278Spec>;
#[doc = "AHBM278 Register"]
pub mod io_ahb_matrix278;
#[doc = "IO_AHB_MATRIX27C (rw) register accessor: AHBM27C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix27c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix27c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix27c`] module"]
#[doc(alias = "IO_AHB_MATRIX27C")]
pub type IoAhbMatrix27c = crate::Reg<io_ahb_matrix27c::IoAhbMatrix27cSpec>;
#[doc = "AHBM27C Register"]
pub mod io_ahb_matrix27c;
#[doc = "IO_AHB_MATRIX2A0 (rw) register accessor: AHBM2A0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix2a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix2a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix2a0`] module"]
#[doc(alias = "IO_AHB_MATRIX2A0")]
pub type IoAhbMatrix2a0 = crate::Reg<io_ahb_matrix2a0::IoAhbMatrix2a0Spec>;
#[doc = "AHBM2A0 Register"]
pub mod io_ahb_matrix2a0;
#[doc = "IO_AHB_MATRIX2A4 (rw) register accessor: AHBM2A4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix2a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix2a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix2a4`] module"]
#[doc(alias = "IO_AHB_MATRIX2A4")]
pub type IoAhbMatrix2a4 = crate::Reg<io_ahb_matrix2a4::IoAhbMatrix2a4Spec>;
#[doc = "AHBM2A4 Register"]
pub mod io_ahb_matrix2a4;
#[doc = "IO_AHB_MATRIX2A8 (rw) register accessor: AHBM2A8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix2a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix2a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix2a8`] module"]
#[doc(alias = "IO_AHB_MATRIX2A8")]
pub type IoAhbMatrix2a8 = crate::Reg<io_ahb_matrix2a8::IoAhbMatrix2a8Spec>;
#[doc = "AHBM2A8 Register"]
pub mod io_ahb_matrix2a8;
#[doc = "IO_AHB_MATRIX2AC (rw) register accessor: AHBM2AC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix2ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix2ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix2ac`] module"]
#[doc(alias = "IO_AHB_MATRIX2AC")]
pub type IoAhbMatrix2ac = crate::Reg<io_ahb_matrix2ac::IoAhbMatrix2acSpec>;
#[doc = "AHBM2AC Register"]
pub mod io_ahb_matrix2ac;
#[doc = "IO_AHB_MATRIX2FC (rw) register accessor: AHBM2FC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix2fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix2fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ahb_matrix2fc`] module"]
#[doc(alias = "IO_AHB_MATRIX2FC")]
pub type IoAhbMatrix2fc = crate::Reg<io_ahb_matrix2fc::IoAhbMatrix2fcSpec>;
#[doc = "AHBM2FC Register"]
pub mod io_ahb_matrix2fc;
