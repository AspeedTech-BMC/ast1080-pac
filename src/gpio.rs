#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio000: Gpio000,
    gpio004: Gpio004,
    gpio008: Gpio008,
    gpio00c: Gpio00c,
    gpio010: Gpio010,
    gpio014: Gpio014,
    gpio018: Gpio018,
    gpio01c: Gpio01c,
    gpio020: Gpio020,
    _reserved9: [u8; 0x18],
    gpio03c: Gpio03c,
    _reserved10: [u8; 0xc0],
    gpio100: Gpio100,
    gpio104: Gpio104,
    gpio108: Gpio108,
    gpio10c: Gpio10c,
    gpio110: Gpio110,
    gpio114: Gpio114,
    gpio118: Gpio118,
    _reserved17: [u8; 0x64],
    gpio180: Gpio180,
    gpio184: Gpio184,
    gpio188: Gpio188,
    gpio18c: Gpio18c,
    gpio190: Gpio190,
    gpio194: Gpio194,
    gpio198: Gpio198,
    gpio19c: Gpio19c,
    gpio1a0: Gpio1a0,
    gpio1a4: Gpio1a4,
    gpio1a8: Gpio1a8,
    gpio1ac: Gpio1ac,
    gpio1b0: Gpio1b0,
    gpio1b4: Gpio1b4,
    gpio1b8: Gpio1b8,
    gpio1bc: Gpio1bc,
    gpio1c0: Gpio1c0,
    gpio1c4: Gpio1c4,
    gpio1c8: Gpio1c8,
    gpio1cc: Gpio1cc,
    gpio1d0: Gpio1d0,
    gpio1d4: Gpio1d4,
    gpio1d8: Gpio1d8,
    gpio1dc: Gpio1dc,
    gpio1e0: Gpio1e0,
    gpio1e4: Gpio1e4,
    gpio1e8: Gpio1e8,
    gpio1ec: Gpio1ec,
    gpio1f0: Gpio1f0,
    gpio1f4: Gpio1f4,
    gpio1f8: Gpio1f8,
    gpio1fc: Gpio1fc,
    gpio200: Gpio200,
    gpio204: Gpio204,
    gpio208: Gpio208,
    gpio20c: Gpio20c,
    gpio210: Gpio210,
    gpio214: Gpio214,
    gpio218: Gpio218,
    gpio21c: Gpio21c,
    gpio220: Gpio220,
    gpio224: Gpio224,
    gpio228: Gpio228,
    gpio22c: Gpio22c,
    gpio230: Gpio230,
    gpio234: Gpio234,
    gpio238: Gpio238,
    gpio23c: Gpio23c,
    gpio240: Gpio240,
    gpio244: Gpio244,
    gpio248: Gpio248,
    gpio24c: Gpio24c,
    gpio250: Gpio250,
    gpio254: Gpio254,
    gpio258: Gpio258,
    gpio25c: Gpio25c,
    gpio260: Gpio260,
    gpio264: Gpio264,
    gpio268: Gpio268,
    gpio26c: Gpio26c,
    gpio270: Gpio270,
    gpio274: Gpio274,
    gpio278: Gpio278,
    gpio27c: Gpio27c,
    gpio280: Gpio280,
    gpio284: Gpio284,
    gpio288: Gpio288,
    gpio28c: Gpio28c,
    gpio290: Gpio290,
    gpio294: Gpio294,
    gpio298: Gpio298,
    gpio29c: Gpio29c,
    gpio2a0: Gpio2a0,
    gpio2a4: Gpio2a4,
    gpio2a8: Gpio2a8,
    gpio2ac: Gpio2ac,
    gpio2b0: Gpio2b0,
    gpio2b4: Gpio2b4,
    gpio2b8: Gpio2b8,
    gpio2bc: Gpio2bc,
    gpio2c0: Gpio2c0,
    gpio2c4: Gpio2c4,
    gpio2c8: Gpio2c8,
    gpio2cc: Gpio2cc,
    gpio2d0: Gpio2d0,
    gpio2d4: Gpio2d4,
    gpio2d8: Gpio2d8,
    gpio2dc: Gpio2dc,
    gpio2e0: Gpio2e0,
    gpio2e4: Gpio2e4,
    gpio2e8: Gpio2e8,
    gpio2ec: Gpio2ec,
    gpio2f0: Gpio2f0,
    gpio2f4: Gpio2f4,
    gpio2f8: Gpio2f8,
    gpio2fc: Gpio2fc,
    gpio300: Gpio300,
    gpio304: Gpio304,
    gpio308: Gpio308,
    gpio30c: Gpio30c,
    gpio310: Gpio310,
    gpio314: Gpio314,
    gpio318: Gpio318,
    gpio31c: Gpio31c,
    gpio320: Gpio320,
    gpio324: Gpio324,
    gpio328: Gpio328,
    gpio32c: Gpio32c,
    gpio330: Gpio330,
    gpio334: Gpio334,
    gpio338: Gpio338,
    gpio33c: Gpio33c,
    gpio340: Gpio340,
    gpio344: Gpio344,
    gpio348: Gpio348,
    gpio34c: Gpio34c,
    gpio350: Gpio350,
    gpio354: Gpio354,
    gpio358: Gpio358,
    gpio35c: Gpio35c,
    gpio360: Gpio360,
    gpio364: Gpio364,
    gpio368: Gpio368,
    gpio36c: Gpio36c,
    gpio370: Gpio370,
    gpio374: Gpio374,
    gpio378: Gpio378,
    gpio37c: Gpio37c,
    gpio380: Gpio380,
    gpio384: Gpio384,
    gpio388: Gpio388,
    gpio38c: Gpio38c,
    gpio390: Gpio390,
    gpio394: Gpio394,
    gpio398: Gpio398,
    gpio39c: Gpio39c,
    gpio3a0: Gpio3a0,
    gpio3a4: Gpio3a4,
    gpio3a8: Gpio3a8,
    gpio3ac: Gpio3ac,
    gpio3b0: Gpio3b0,
    gpio3b4: Gpio3b4,
    gpio3b8: Gpio3b8,
    gpio3bc: Gpio3bc,
    gpio3c0: Gpio3c0,
    gpio3c4: Gpio3c4,
    gpio3c8: Gpio3c8,
    gpio3cc: Gpio3cc,
    gpio3d0: Gpio3d0,
    gpio3d4: Gpio3d4,
    gpio3d8: Gpio3d8,
    gpio3dc: Gpio3dc,
    gpio3e0: Gpio3e0,
    gpio3e4: Gpio3e4,
    gpio3e8: Gpio3e8,
    gpio3ec: Gpio3ec,
    gpio3f0: Gpio3f0,
    gpio3f4: Gpio3f4,
    gpio3f8: Gpio3f8,
    gpio3fc: Gpio3fc,
    gpio400: Gpio400,
    gpio404: Gpio404,
    gpio408: Gpio408,
    gpio40c: Gpio40c,
    gpio410: Gpio410,
    gpio414: Gpio414,
    gpio418: Gpio418,
    gpio41c: Gpio41c,
    gpio420: Gpio420,
    gpio424: Gpio424,
    gpio428: Gpio428,
    gpio42c: Gpio42c,
    gpio430: Gpio430,
    gpio434: Gpio434,
    gpio438: Gpio438,
    gpio43c: Gpio43c,
    gpio440: Gpio440,
    gpio444: Gpio444,
    gpio448: Gpio448,
    gpio44c: Gpio44c,
    gpio450: Gpio450,
    gpio454: Gpio454,
    gpio458: Gpio458,
    gpio45c: Gpio45c,
    gpio460: Gpio460,
    gpio464: Gpio464,
    gpio468: Gpio468,
    gpio46c: Gpio46c,
    gpio470: Gpio470,
    gpio474: Gpio474,
    gpio478: Gpio478,
    gpio47c: Gpio47c,
    gpio480: Gpio480,
    gpio484: Gpio484,
    _reserved211: [u8; 0x0378],
    gpio800: Gpio800,
    gpio804: Gpio804,
    gpio808: Gpio808,
    _reserved214: [u8; 0x04],
    gpio810: Gpio810,
    gpio814: Gpio814,
    gpio818: Gpio818,
    gpio81c: Gpio81c,
    gpio820: Gpio820,
    gpio824: Gpio824,
    gpio828: Gpio828,
    gpio82c: Gpio82c,
    gpio830: Gpio830,
    gpio834: Gpio834,
    gpio838: Gpio838,
    gpio83c: Gpio83c,
    gpio840: Gpio840,
    gpio844: Gpio844,
    gpio848: Gpio848,
    gpio84c: Gpio84c,
    gpio850: Gpio850,
    gpio854: Gpio854,
    gpio858: Gpio858,
    gpio85c: Gpio85c,
    gpio860: Gpio860,
    gpio864: Gpio864,
    gpio868: Gpio868,
    gpio86c: Gpio86c,
    gpio870: Gpio870,
    gpio874: Gpio874,
    gpio878: Gpio878,
    gpio87c: Gpio87c,
    gpio880: Gpio880,
    gpio884: Gpio884,
    gpio888: Gpio888,
    gpio88c: Gpio88c,
    gpio890: Gpio890,
    gpio894: Gpio894,
    gpio898: Gpio898,
    gpio89c: Gpio89c,
    gpio8a0: Gpio8a0,
    gpio8a4: Gpio8a4,
    gpio8a8: Gpio8a8,
    gpio8ac: Gpio8ac,
    gpio8b0: Gpio8b0,
    gpio8b4: Gpio8b4,
    gpio8b8: Gpio8b8,
    gpio8bc: Gpio8bc,
    gpio8c0: Gpio8c0,
    gpio8c4: Gpio8c4,
    gpio8c8: Gpio8c8,
    gpio8cc: Gpio8cc,
    gpio8d0: Gpio8d0,
    _reserved263: [u8; 0x3c],
    gpio910: Gpio910,
    gpio914: Gpio914,
    gpio918: Gpio918,
    gpio91c: Gpio91c,
    gpio920: Gpio920,
    gpio924: Gpio924,
    gpio928: Gpio928,
    gpio92c: Gpio92c,
    gpio930: Gpio930,
    gpio934: Gpio934,
    gpio938: Gpio938,
    gpio93c: Gpio93c,
    gpio940: Gpio940,
    gpio944: Gpio944,
    gpio948: Gpio948,
    gpio94c: Gpio94c,
    gpio950: Gpio950,
    gpio954: Gpio954,
    gpio958: Gpio958,
    gpio95c: Gpio95c,
    gpio960: Gpio960,
    gpio964: Gpio964,
    gpio968: Gpio968,
    gpio96c: Gpio96c,
    gpio970: Gpio970,
    gpio974: Gpio974,
    gpio978: Gpio978,
    gpio97c: Gpio97c,
    gpio980: Gpio980,
    gpio984: Gpio984,
    gpio988: Gpio988,
    gpio98c: Gpio98c,
    gpio990: Gpio990,
    gpio994: Gpio994,
    gpio998: Gpio998,
    gpio99c: Gpio99c,
    gpio9a0: Gpio9a0,
    gpio9a4: Gpio9a4,
    gpio9a8: Gpio9a8,
    gpio9ac: Gpio9ac,
    gpio9b0: Gpio9b0,
    gpio9b4: Gpio9b4,
    gpio9b8: Gpio9b8,
    gpio9bc: Gpio9bc,
    gpio9c0: Gpio9c0,
    gpio9c4: Gpio9c4,
    gpio9c8: Gpio9c8,
    gpio9cc: Gpio9cc,
    gpio9d0: Gpio9d0,
    _reserved312: [u8; 0x40],
    gpioa14: Gpioa14,
    gpioa18: Gpioa18,
    gpioa1c: Gpioa1c,
    gpioa20: Gpioa20,
    gpioa24: Gpioa24,
    gpioa28: Gpioa28,
    gpioa2c: Gpioa2c,
    gpioa30: Gpioa30,
    gpioa34: Gpioa34,
    gpioa38: Gpioa38,
    gpioa3c: Gpioa3c,
    gpioa40: Gpioa40,
    gpioa44: Gpioa44,
    gpioa48: Gpioa48,
    gpioa4c: Gpioa4c,
    gpioa50: Gpioa50,
    gpioa54: Gpioa54,
    gpioa58: Gpioa58,
    gpioa5c: Gpioa5c,
    gpioa60: Gpioa60,
    gpioa64: Gpioa64,
    gpioa68: Gpioa68,
    gpioa6c: Gpioa6c,
    gpioa70: Gpioa70,
    gpioa74: Gpioa74,
    gpioa78: Gpioa78,
    gpioa7c: Gpioa7c,
    gpioa80: Gpioa80,
    gpioa84: Gpioa84,
    gpioa88: Gpioa88,
    gpioa8c: Gpioa8c,
    gpioa90: Gpioa90,
    gpioa94: Gpioa94,
    gpioa98: Gpioa98,
    gpioa9c: Gpioa9c,
    gpioaa0: Gpioaa0,
    gpioaa4: Gpioaa4,
    gpioaa8: Gpioaa8,
    gpioaac: Gpioaac,
    gpioab0: Gpioab0,
    gpioab4: Gpioab4,
    gpioab8: Gpioab8,
    gpioabc: Gpioabc,
    gpioac0: Gpioac0,
    gpioac4: Gpioac4,
    gpioac8: Gpioac8,
    gpioacc: Gpioacc,
    gpioad0: Gpioad0,
    _reserved360: [u8; 0x3c],
    gpiob10: Gpiob10,
    gpiob14: Gpiob14,
    gpiob18: Gpiob18,
    gpiob1c: Gpiob1c,
    gpiob20: Gpiob20,
    gpiob24: Gpiob24,
    gpiob28: Gpiob28,
    _reserved367: [u8; 0xe4],
    gpioc10: Gpioc10,
    gpioc14: Gpioc14,
    gpioc18: Gpioc18,
    gpioc1c: Gpioc1c,
    gpioc20: Gpioc20,
    gpioc24: Gpioc24,
    gpioc28: Gpioc28,
    _reserved374: [u8; 0xe4],
    gpiod10: Gpiod10,
    gpiod14: Gpiod14,
    gpiod18: Gpiod18,
    gpiod1c: Gpiod1c,
    gpiod20: Gpiod20,
    gpiod24: Gpiod24,
    gpiod28: Gpiod28,
    _reserved381: [u8; 0xe4],
    gpioe10: Gpioe10,
    gpioe14: Gpioe14,
    gpioe18: Gpioe18,
    gpioe1c: Gpioe1c,
    gpioe20: Gpioe20,
    gpioe24: Gpioe24,
    gpioe28: Gpioe28,
}
impl RegisterBlock {
    #[doc = "0x00 - Debounce Timer Setting Register \\#1"]
    #[inline(always)]
    pub const fn gpio000(&self) -> &Gpio000 {
        &self.gpio000
    }
    #[doc = "0x04 - Debounce Timer Setting Register \\#2"]
    #[inline(always)]
    pub const fn gpio004(&self) -> &Gpio004 {
        &self.gpio004
    }
    #[doc = "0x08 - Debounce Timer Setting Register \\#3"]
    #[inline(always)]
    pub const fn gpio008(&self) -> &Gpio008 {
        &self.gpio008
    }
    #[doc = "0x0c - GPIO Blink Source Clock Division"]
    #[inline(always)]
    pub const fn gpio00c(&self) -> &Gpio00c {
        &self.gpio00c
    }
    #[doc = "0x10 - GPIO Blink Control"]
    #[inline(always)]
    pub const fn gpio010(&self) -> &Gpio010 {
        &self.gpio010
    }
    #[doc = "0x14 - GPIO Blink Counter \\#1 Configuration"]
    #[inline(always)]
    pub const fn gpio014(&self) -> &Gpio014 {
        &self.gpio014
    }
    #[doc = "0x18 - GPIO Blink Counter \\#2 Configuration"]
    #[inline(always)]
    pub const fn gpio018(&self) -> &Gpio018 {
        &self.gpio018
    }
    #[doc = "0x1c - GPIO Blink Counter \\#3 Configuration"]
    #[inline(always)]
    pub const fn gpio01c(&self) -> &Gpio01c {
        &self.gpio01c
    }
    #[doc = "0x20 - 80h Enable Register"]
    #[inline(always)]
    pub const fn gpio020(&self) -> &Gpio020 {
        &self.gpio020
    }
    #[doc = "0x3c - Write Protection Register"]
    #[inline(always)]
    pub const fn gpio03c(&self) -> &Gpio03c {
        &self.gpio03c
    }
    #[doc = "0x100 - Interrupt Status Register \\#1"]
    #[inline(always)]
    pub const fn gpio100(&self) -> &Gpio100 {
        &self.gpio100
    }
    #[doc = "0x104 - Interrupt Status Register \\#2"]
    #[inline(always)]
    pub const fn gpio104(&self) -> &Gpio104 {
        &self.gpio104
    }
    #[doc = "0x108 - Interrupt Status Register \\#3"]
    #[inline(always)]
    pub const fn gpio108(&self) -> &Gpio108 {
        &self.gpio108
    }
    #[doc = "0x10c - Interrupt Status Register \\#4"]
    #[inline(always)]
    pub const fn gpio10c(&self) -> &Gpio10c {
        &self.gpio10c
    }
    #[doc = "0x110 - Interrupt Status Register \\#5"]
    #[inline(always)]
    pub const fn gpio110(&self) -> &Gpio110 {
        &self.gpio110
    }
    #[doc = "0x114 - Interrupt Status Register \\#6"]
    #[inline(always)]
    pub const fn gpio114(&self) -> &Gpio114 {
        &self.gpio114
    }
    #[doc = "0x118 - Interrupt Status Register \\#7"]
    #[inline(always)]
    pub const fn gpio118(&self) -> &Gpio118 {
        &self.gpio118
    }
    #[doc = "0x180 - GPIO000 Control Register"]
    #[inline(always)]
    pub const fn gpio180(&self) -> &Gpio180 {
        &self.gpio180
    }
    #[doc = "0x184 - GPIO001 Control Register"]
    #[inline(always)]
    pub const fn gpio184(&self) -> &Gpio184 {
        &self.gpio184
    }
    #[doc = "0x188 - GPIO002 Control Register"]
    #[inline(always)]
    pub const fn gpio188(&self) -> &Gpio188 {
        &self.gpio188
    }
    #[doc = "0x18c - GPIO003 Control Register"]
    #[inline(always)]
    pub const fn gpio18c(&self) -> &Gpio18c {
        &self.gpio18c
    }
    #[doc = "0x190 - GPIO004 Control Register"]
    #[inline(always)]
    pub const fn gpio190(&self) -> &Gpio190 {
        &self.gpio190
    }
    #[doc = "0x194 - GPIO005 Control Register"]
    #[inline(always)]
    pub const fn gpio194(&self) -> &Gpio194 {
        &self.gpio194
    }
    #[doc = "0x198 - GPIO006 Control Register"]
    #[inline(always)]
    pub const fn gpio198(&self) -> &Gpio198 {
        &self.gpio198
    }
    #[doc = "0x19c - GPIO007 Control Register"]
    #[inline(always)]
    pub const fn gpio19c(&self) -> &Gpio19c {
        &self.gpio19c
    }
    #[doc = "0x1a0 - GPIO008 Control Register"]
    #[inline(always)]
    pub const fn gpio1a0(&self) -> &Gpio1a0 {
        &self.gpio1a0
    }
    #[doc = "0x1a4 - GPIO009 Control Register"]
    #[inline(always)]
    pub const fn gpio1a4(&self) -> &Gpio1a4 {
        &self.gpio1a4
    }
    #[doc = "0x1a8 - GPIO010 Control Register"]
    #[inline(always)]
    pub const fn gpio1a8(&self) -> &Gpio1a8 {
        &self.gpio1a8
    }
    #[doc = "0x1ac - GPIO011 Control Register"]
    #[inline(always)]
    pub const fn gpio1ac(&self) -> &Gpio1ac {
        &self.gpio1ac
    }
    #[doc = "0x1b0 - GPIO012 Control Register"]
    #[inline(always)]
    pub const fn gpio1b0(&self) -> &Gpio1b0 {
        &self.gpio1b0
    }
    #[doc = "0x1b4 - GPIO013 Control Register"]
    #[inline(always)]
    pub const fn gpio1b4(&self) -> &Gpio1b4 {
        &self.gpio1b4
    }
    #[doc = "0x1b8 - GPIO014 Control Register"]
    #[inline(always)]
    pub const fn gpio1b8(&self) -> &Gpio1b8 {
        &self.gpio1b8
    }
    #[doc = "0x1bc - GPIO015 Control Register"]
    #[inline(always)]
    pub const fn gpio1bc(&self) -> &Gpio1bc {
        &self.gpio1bc
    }
    #[doc = "0x1c0 - GPIO016 Control Register"]
    #[inline(always)]
    pub const fn gpio1c0(&self) -> &Gpio1c0 {
        &self.gpio1c0
    }
    #[doc = "0x1c4 - GPIO017 Control Register"]
    #[inline(always)]
    pub const fn gpio1c4(&self) -> &Gpio1c4 {
        &self.gpio1c4
    }
    #[doc = "0x1c8 - GPIO018 Control Register"]
    #[inline(always)]
    pub const fn gpio1c8(&self) -> &Gpio1c8 {
        &self.gpio1c8
    }
    #[doc = "0x1cc - GPIO019 Control Register"]
    #[inline(always)]
    pub const fn gpio1cc(&self) -> &Gpio1cc {
        &self.gpio1cc
    }
    #[doc = "0x1d0 - GPIO020 Control Register"]
    #[inline(always)]
    pub const fn gpio1d0(&self) -> &Gpio1d0 {
        &self.gpio1d0
    }
    #[doc = "0x1d4 - GPIO021 Control Register"]
    #[inline(always)]
    pub const fn gpio1d4(&self) -> &Gpio1d4 {
        &self.gpio1d4
    }
    #[doc = "0x1d8 - GPIO022 Control Register"]
    #[inline(always)]
    pub const fn gpio1d8(&self) -> &Gpio1d8 {
        &self.gpio1d8
    }
    #[doc = "0x1dc - GPIO023 Control Register"]
    #[inline(always)]
    pub const fn gpio1dc(&self) -> &Gpio1dc {
        &self.gpio1dc
    }
    #[doc = "0x1e0 - GPIO024 Control Register"]
    #[inline(always)]
    pub const fn gpio1e0(&self) -> &Gpio1e0 {
        &self.gpio1e0
    }
    #[doc = "0x1e4 - GPIO025 Control Register"]
    #[inline(always)]
    pub const fn gpio1e4(&self) -> &Gpio1e4 {
        &self.gpio1e4
    }
    #[doc = "0x1e8 - GPIO026 Control Register"]
    #[inline(always)]
    pub const fn gpio1e8(&self) -> &Gpio1e8 {
        &self.gpio1e8
    }
    #[doc = "0x1ec - GPIO027 Control Register"]
    #[inline(always)]
    pub const fn gpio1ec(&self) -> &Gpio1ec {
        &self.gpio1ec
    }
    #[doc = "0x1f0 - GPIO028 Control Register"]
    #[inline(always)]
    pub const fn gpio1f0(&self) -> &Gpio1f0 {
        &self.gpio1f0
    }
    #[doc = "0x1f4 - GPIO029 Control Register"]
    #[inline(always)]
    pub const fn gpio1f4(&self) -> &Gpio1f4 {
        &self.gpio1f4
    }
    #[doc = "0x1f8 - GPIO030 Control Register"]
    #[inline(always)]
    pub const fn gpio1f8(&self) -> &Gpio1f8 {
        &self.gpio1f8
    }
    #[doc = "0x1fc - GPIO031 Control Register"]
    #[inline(always)]
    pub const fn gpio1fc(&self) -> &Gpio1fc {
        &self.gpio1fc
    }
    #[doc = "0x200 - GPIO032 Control Register"]
    #[inline(always)]
    pub const fn gpio200(&self) -> &Gpio200 {
        &self.gpio200
    }
    #[doc = "0x204 - GPIO033 Control Register"]
    #[inline(always)]
    pub const fn gpio204(&self) -> &Gpio204 {
        &self.gpio204
    }
    #[doc = "0x208 - GPIO034 Control Register"]
    #[inline(always)]
    pub const fn gpio208(&self) -> &Gpio208 {
        &self.gpio208
    }
    #[doc = "0x20c - GPIO035 Control Register"]
    #[inline(always)]
    pub const fn gpio20c(&self) -> &Gpio20c {
        &self.gpio20c
    }
    #[doc = "0x210 - GPIO036 Control Register"]
    #[inline(always)]
    pub const fn gpio210(&self) -> &Gpio210 {
        &self.gpio210
    }
    #[doc = "0x214 - GPIO037 Control Register"]
    #[inline(always)]
    pub const fn gpio214(&self) -> &Gpio214 {
        &self.gpio214
    }
    #[doc = "0x218 - GPIO038 Control Register"]
    #[inline(always)]
    pub const fn gpio218(&self) -> &Gpio218 {
        &self.gpio218
    }
    #[doc = "0x21c - GPIO039 Control Register"]
    #[inline(always)]
    pub const fn gpio21c(&self) -> &Gpio21c {
        &self.gpio21c
    }
    #[doc = "0x220 - GPIO040 Control Register"]
    #[inline(always)]
    pub const fn gpio220(&self) -> &Gpio220 {
        &self.gpio220
    }
    #[doc = "0x224 - GPIO041 Control Register"]
    #[inline(always)]
    pub const fn gpio224(&self) -> &Gpio224 {
        &self.gpio224
    }
    #[doc = "0x228 - GPIO042 Control Register"]
    #[inline(always)]
    pub const fn gpio228(&self) -> &Gpio228 {
        &self.gpio228
    }
    #[doc = "0x22c - GPIO043 Control Register"]
    #[inline(always)]
    pub const fn gpio22c(&self) -> &Gpio22c {
        &self.gpio22c
    }
    #[doc = "0x230 - GPIO044 Control Register"]
    #[inline(always)]
    pub const fn gpio230(&self) -> &Gpio230 {
        &self.gpio230
    }
    #[doc = "0x234 - GPIO045 Control Register"]
    #[inline(always)]
    pub const fn gpio234(&self) -> &Gpio234 {
        &self.gpio234
    }
    #[doc = "0x238 - GPIO046 Control Register"]
    #[inline(always)]
    pub const fn gpio238(&self) -> &Gpio238 {
        &self.gpio238
    }
    #[doc = "0x23c - GPIO047 Control Register"]
    #[inline(always)]
    pub const fn gpio23c(&self) -> &Gpio23c {
        &self.gpio23c
    }
    #[doc = "0x240 - GPIO048 Control Register"]
    #[inline(always)]
    pub const fn gpio240(&self) -> &Gpio240 {
        &self.gpio240
    }
    #[doc = "0x244 - GPIO049 Control Register"]
    #[inline(always)]
    pub const fn gpio244(&self) -> &Gpio244 {
        &self.gpio244
    }
    #[doc = "0x248 - GPIO050 Control Register"]
    #[inline(always)]
    pub const fn gpio248(&self) -> &Gpio248 {
        &self.gpio248
    }
    #[doc = "0x24c - GPIO051 Control Register"]
    #[inline(always)]
    pub const fn gpio24c(&self) -> &Gpio24c {
        &self.gpio24c
    }
    #[doc = "0x250 - GPIO052 Control Register"]
    #[inline(always)]
    pub const fn gpio250(&self) -> &Gpio250 {
        &self.gpio250
    }
    #[doc = "0x254 - GPIO053 Control Register"]
    #[inline(always)]
    pub const fn gpio254(&self) -> &Gpio254 {
        &self.gpio254
    }
    #[doc = "0x258 - GPIO054 Control Register"]
    #[inline(always)]
    pub const fn gpio258(&self) -> &Gpio258 {
        &self.gpio258
    }
    #[doc = "0x25c - GPIO055 Control Register"]
    #[inline(always)]
    pub const fn gpio25c(&self) -> &Gpio25c {
        &self.gpio25c
    }
    #[doc = "0x260 - GPIO056 Control Register"]
    #[inline(always)]
    pub const fn gpio260(&self) -> &Gpio260 {
        &self.gpio260
    }
    #[doc = "0x264 - GPIO057 Control Register"]
    #[inline(always)]
    pub const fn gpio264(&self) -> &Gpio264 {
        &self.gpio264
    }
    #[doc = "0x268 - GPIO058 Control Register"]
    #[inline(always)]
    pub const fn gpio268(&self) -> &Gpio268 {
        &self.gpio268
    }
    #[doc = "0x26c - GPIO059 Control Register"]
    #[inline(always)]
    pub const fn gpio26c(&self) -> &Gpio26c {
        &self.gpio26c
    }
    #[doc = "0x270 - GPIO060 Control Register"]
    #[inline(always)]
    pub const fn gpio270(&self) -> &Gpio270 {
        &self.gpio270
    }
    #[doc = "0x274 - GPIO061 Control Register"]
    #[inline(always)]
    pub const fn gpio274(&self) -> &Gpio274 {
        &self.gpio274
    }
    #[doc = "0x278 - GPIO062 Control Register"]
    #[inline(always)]
    pub const fn gpio278(&self) -> &Gpio278 {
        &self.gpio278
    }
    #[doc = "0x27c - GPIO063 Control Register"]
    #[inline(always)]
    pub const fn gpio27c(&self) -> &Gpio27c {
        &self.gpio27c
    }
    #[doc = "0x280 - GPIO064 Control Register"]
    #[inline(always)]
    pub const fn gpio280(&self) -> &Gpio280 {
        &self.gpio280
    }
    #[doc = "0x284 - GPIO065 Control Register"]
    #[inline(always)]
    pub const fn gpio284(&self) -> &Gpio284 {
        &self.gpio284
    }
    #[doc = "0x288 - GPIO066 Control Register"]
    #[inline(always)]
    pub const fn gpio288(&self) -> &Gpio288 {
        &self.gpio288
    }
    #[doc = "0x28c - GPIO067 Control Register"]
    #[inline(always)]
    pub const fn gpio28c(&self) -> &Gpio28c {
        &self.gpio28c
    }
    #[doc = "0x290 - GPIO068 Control Register"]
    #[inline(always)]
    pub const fn gpio290(&self) -> &Gpio290 {
        &self.gpio290
    }
    #[doc = "0x294 - GPIO069 Control Register"]
    #[inline(always)]
    pub const fn gpio294(&self) -> &Gpio294 {
        &self.gpio294
    }
    #[doc = "0x298 - GPIO070 Control Register"]
    #[inline(always)]
    pub const fn gpio298(&self) -> &Gpio298 {
        &self.gpio298
    }
    #[doc = "0x29c - GPIO071 Control Register"]
    #[inline(always)]
    pub const fn gpio29c(&self) -> &Gpio29c {
        &self.gpio29c
    }
    #[doc = "0x2a0 - GPIO072 Control Register"]
    #[inline(always)]
    pub const fn gpio2a0(&self) -> &Gpio2a0 {
        &self.gpio2a0
    }
    #[doc = "0x2a4 - GPIO073 Control Register"]
    #[inline(always)]
    pub const fn gpio2a4(&self) -> &Gpio2a4 {
        &self.gpio2a4
    }
    #[doc = "0x2a8 - GPIO074 Control Register"]
    #[inline(always)]
    pub const fn gpio2a8(&self) -> &Gpio2a8 {
        &self.gpio2a8
    }
    #[doc = "0x2ac - GPIO075 Control Register"]
    #[inline(always)]
    pub const fn gpio2ac(&self) -> &Gpio2ac {
        &self.gpio2ac
    }
    #[doc = "0x2b0 - GPIO076 Control Register"]
    #[inline(always)]
    pub const fn gpio2b0(&self) -> &Gpio2b0 {
        &self.gpio2b0
    }
    #[doc = "0x2b4 - GPIO077 Control Register"]
    #[inline(always)]
    pub const fn gpio2b4(&self) -> &Gpio2b4 {
        &self.gpio2b4
    }
    #[doc = "0x2b8 - GPIO078 Control Register"]
    #[inline(always)]
    pub const fn gpio2b8(&self) -> &Gpio2b8 {
        &self.gpio2b8
    }
    #[doc = "0x2bc - GPIO079 Control Register"]
    #[inline(always)]
    pub const fn gpio2bc(&self) -> &Gpio2bc {
        &self.gpio2bc
    }
    #[doc = "0x2c0 - GPIO080 Control Register"]
    #[inline(always)]
    pub const fn gpio2c0(&self) -> &Gpio2c0 {
        &self.gpio2c0
    }
    #[doc = "0x2c4 - GPIO081 Control Register"]
    #[inline(always)]
    pub const fn gpio2c4(&self) -> &Gpio2c4 {
        &self.gpio2c4
    }
    #[doc = "0x2c8 - GPIO082 Control Register"]
    #[inline(always)]
    pub const fn gpio2c8(&self) -> &Gpio2c8 {
        &self.gpio2c8
    }
    #[doc = "0x2cc - GPIO083 Control Register"]
    #[inline(always)]
    pub const fn gpio2cc(&self) -> &Gpio2cc {
        &self.gpio2cc
    }
    #[doc = "0x2d0 - GPIO084 Control Register"]
    #[inline(always)]
    pub const fn gpio2d0(&self) -> &Gpio2d0 {
        &self.gpio2d0
    }
    #[doc = "0x2d4 - GPIO085 Control Register"]
    #[inline(always)]
    pub const fn gpio2d4(&self) -> &Gpio2d4 {
        &self.gpio2d4
    }
    #[doc = "0x2d8 - GPIO086 Control Register"]
    #[inline(always)]
    pub const fn gpio2d8(&self) -> &Gpio2d8 {
        &self.gpio2d8
    }
    #[doc = "0x2dc - GPIO087 Control Register"]
    #[inline(always)]
    pub const fn gpio2dc(&self) -> &Gpio2dc {
        &self.gpio2dc
    }
    #[doc = "0x2e0 - GPIO088 Control Register"]
    #[inline(always)]
    pub const fn gpio2e0(&self) -> &Gpio2e0 {
        &self.gpio2e0
    }
    #[doc = "0x2e4 - GPIO089 Control Register"]
    #[inline(always)]
    pub const fn gpio2e4(&self) -> &Gpio2e4 {
        &self.gpio2e4
    }
    #[doc = "0x2e8 - GPIO090 Control Register"]
    #[inline(always)]
    pub const fn gpio2e8(&self) -> &Gpio2e8 {
        &self.gpio2e8
    }
    #[doc = "0x2ec - GPIO091 Control Register"]
    #[inline(always)]
    pub const fn gpio2ec(&self) -> &Gpio2ec {
        &self.gpio2ec
    }
    #[doc = "0x2f0 - GPIO092 Control Register"]
    #[inline(always)]
    pub const fn gpio2f0(&self) -> &Gpio2f0 {
        &self.gpio2f0
    }
    #[doc = "0x2f4 - GPIO093 Control Register"]
    #[inline(always)]
    pub const fn gpio2f4(&self) -> &Gpio2f4 {
        &self.gpio2f4
    }
    #[doc = "0x2f8 - GPIO094 Control Register"]
    #[inline(always)]
    pub const fn gpio2f8(&self) -> &Gpio2f8 {
        &self.gpio2f8
    }
    #[doc = "0x2fc - GPIO095 Control Register"]
    #[inline(always)]
    pub const fn gpio2fc(&self) -> &Gpio2fc {
        &self.gpio2fc
    }
    #[doc = "0x300 - GPIO096 Control Register"]
    #[inline(always)]
    pub const fn gpio300(&self) -> &Gpio300 {
        &self.gpio300
    }
    #[doc = "0x304 - GPIO097 Control Register"]
    #[inline(always)]
    pub const fn gpio304(&self) -> &Gpio304 {
        &self.gpio304
    }
    #[doc = "0x308 - GPIO098 Control Register"]
    #[inline(always)]
    pub const fn gpio308(&self) -> &Gpio308 {
        &self.gpio308
    }
    #[doc = "0x30c - GPIO099 Control Register"]
    #[inline(always)]
    pub const fn gpio30c(&self) -> &Gpio30c {
        &self.gpio30c
    }
    #[doc = "0x310 - GPIO100 Control Register"]
    #[inline(always)]
    pub const fn gpio310(&self) -> &Gpio310 {
        &self.gpio310
    }
    #[doc = "0x314 - GPIO101 Control Register"]
    #[inline(always)]
    pub const fn gpio314(&self) -> &Gpio314 {
        &self.gpio314
    }
    #[doc = "0x318 - GPIO102 Control Register"]
    #[inline(always)]
    pub const fn gpio318(&self) -> &Gpio318 {
        &self.gpio318
    }
    #[doc = "0x31c - GPIO103 Control Register"]
    #[inline(always)]
    pub const fn gpio31c(&self) -> &Gpio31c {
        &self.gpio31c
    }
    #[doc = "0x320 - GPIO104 Control Register"]
    #[inline(always)]
    pub const fn gpio320(&self) -> &Gpio320 {
        &self.gpio320
    }
    #[doc = "0x324 - GPIO105 Control Register"]
    #[inline(always)]
    pub const fn gpio324(&self) -> &Gpio324 {
        &self.gpio324
    }
    #[doc = "0x328 - GPIO106 Control Register"]
    #[inline(always)]
    pub const fn gpio328(&self) -> &Gpio328 {
        &self.gpio328
    }
    #[doc = "0x32c - GPIO107 Control Register"]
    #[inline(always)]
    pub const fn gpio32c(&self) -> &Gpio32c {
        &self.gpio32c
    }
    #[doc = "0x330 - GPIO108 Control Register"]
    #[inline(always)]
    pub const fn gpio330(&self) -> &Gpio330 {
        &self.gpio330
    }
    #[doc = "0x334 - GPIO109 Control Register"]
    #[inline(always)]
    pub const fn gpio334(&self) -> &Gpio334 {
        &self.gpio334
    }
    #[doc = "0x338 - GPIO110 Control Register"]
    #[inline(always)]
    pub const fn gpio338(&self) -> &Gpio338 {
        &self.gpio338
    }
    #[doc = "0x33c - GPIO111 Control Register"]
    #[inline(always)]
    pub const fn gpio33c(&self) -> &Gpio33c {
        &self.gpio33c
    }
    #[doc = "0x340 - GPIO112 Control Register"]
    #[inline(always)]
    pub const fn gpio340(&self) -> &Gpio340 {
        &self.gpio340
    }
    #[doc = "0x344 - GPIO113 Control Register"]
    #[inline(always)]
    pub const fn gpio344(&self) -> &Gpio344 {
        &self.gpio344
    }
    #[doc = "0x348 - GPIO114 Control Register"]
    #[inline(always)]
    pub const fn gpio348(&self) -> &Gpio348 {
        &self.gpio348
    }
    #[doc = "0x34c - GPIO115 Control Register"]
    #[inline(always)]
    pub const fn gpio34c(&self) -> &Gpio34c {
        &self.gpio34c
    }
    #[doc = "0x350 - GPIO116 Control Register"]
    #[inline(always)]
    pub const fn gpio350(&self) -> &Gpio350 {
        &self.gpio350
    }
    #[doc = "0x354 - GPIO117 Control Register"]
    #[inline(always)]
    pub const fn gpio354(&self) -> &Gpio354 {
        &self.gpio354
    }
    #[doc = "0x358 - GPIO118 Control Register"]
    #[inline(always)]
    pub const fn gpio358(&self) -> &Gpio358 {
        &self.gpio358
    }
    #[doc = "0x35c - GPIO119 Control Register"]
    #[inline(always)]
    pub const fn gpio35c(&self) -> &Gpio35c {
        &self.gpio35c
    }
    #[doc = "0x360 - GPIO120 Control Register"]
    #[inline(always)]
    pub const fn gpio360(&self) -> &Gpio360 {
        &self.gpio360
    }
    #[doc = "0x364 - GPIO121 Control Register"]
    #[inline(always)]
    pub const fn gpio364(&self) -> &Gpio364 {
        &self.gpio364
    }
    #[doc = "0x368 - GPIO122 Control Register"]
    #[inline(always)]
    pub const fn gpio368(&self) -> &Gpio368 {
        &self.gpio368
    }
    #[doc = "0x36c - GPIO123 Control Register"]
    #[inline(always)]
    pub const fn gpio36c(&self) -> &Gpio36c {
        &self.gpio36c
    }
    #[doc = "0x370 - GPIO124 Control Register"]
    #[inline(always)]
    pub const fn gpio370(&self) -> &Gpio370 {
        &self.gpio370
    }
    #[doc = "0x374 - GPIO125 Control Register"]
    #[inline(always)]
    pub const fn gpio374(&self) -> &Gpio374 {
        &self.gpio374
    }
    #[doc = "0x378 - GPIO126 Control Register"]
    #[inline(always)]
    pub const fn gpio378(&self) -> &Gpio378 {
        &self.gpio378
    }
    #[doc = "0x37c - GPIO127 Control Register"]
    #[inline(always)]
    pub const fn gpio37c(&self) -> &Gpio37c {
        &self.gpio37c
    }
    #[doc = "0x380 - GPIO128 Control Register"]
    #[inline(always)]
    pub const fn gpio380(&self) -> &Gpio380 {
        &self.gpio380
    }
    #[doc = "0x384 - GPIO129 Control Register"]
    #[inline(always)]
    pub const fn gpio384(&self) -> &Gpio384 {
        &self.gpio384
    }
    #[doc = "0x388 - GPIO130 Control Register"]
    #[inline(always)]
    pub const fn gpio388(&self) -> &Gpio388 {
        &self.gpio388
    }
    #[doc = "0x38c - GPIO131 Control Register"]
    #[inline(always)]
    pub const fn gpio38c(&self) -> &Gpio38c {
        &self.gpio38c
    }
    #[doc = "0x390 - GPIO132 Control Register"]
    #[inline(always)]
    pub const fn gpio390(&self) -> &Gpio390 {
        &self.gpio390
    }
    #[doc = "0x394 - GPIO133 Control Register"]
    #[inline(always)]
    pub const fn gpio394(&self) -> &Gpio394 {
        &self.gpio394
    }
    #[doc = "0x398 - GPIO134 Control Register"]
    #[inline(always)]
    pub const fn gpio398(&self) -> &Gpio398 {
        &self.gpio398
    }
    #[doc = "0x39c - GPIO135 Control Register"]
    #[inline(always)]
    pub const fn gpio39c(&self) -> &Gpio39c {
        &self.gpio39c
    }
    #[doc = "0x3a0 - GPIO136 Control Register"]
    #[inline(always)]
    pub const fn gpio3a0(&self) -> &Gpio3a0 {
        &self.gpio3a0
    }
    #[doc = "0x3a4 - GPIO137 Control Register"]
    #[inline(always)]
    pub const fn gpio3a4(&self) -> &Gpio3a4 {
        &self.gpio3a4
    }
    #[doc = "0x3a8 - GPIO138 Control Register"]
    #[inline(always)]
    pub const fn gpio3a8(&self) -> &Gpio3a8 {
        &self.gpio3a8
    }
    #[doc = "0x3ac - GPIO139 Control Register"]
    #[inline(always)]
    pub const fn gpio3ac(&self) -> &Gpio3ac {
        &self.gpio3ac
    }
    #[doc = "0x3b0 - GPIO140 Control Register"]
    #[inline(always)]
    pub const fn gpio3b0(&self) -> &Gpio3b0 {
        &self.gpio3b0
    }
    #[doc = "0x3b4 - GPIO141 Control Register"]
    #[inline(always)]
    pub const fn gpio3b4(&self) -> &Gpio3b4 {
        &self.gpio3b4
    }
    #[doc = "0x3b8 - GPIO142 Control Register"]
    #[inline(always)]
    pub const fn gpio3b8(&self) -> &Gpio3b8 {
        &self.gpio3b8
    }
    #[doc = "0x3bc - GPIO143 Control Register"]
    #[inline(always)]
    pub const fn gpio3bc(&self) -> &Gpio3bc {
        &self.gpio3bc
    }
    #[doc = "0x3c0 - GPIO144 Control Register"]
    #[inline(always)]
    pub const fn gpio3c0(&self) -> &Gpio3c0 {
        &self.gpio3c0
    }
    #[doc = "0x3c4 - GPIO145 Control Register"]
    #[inline(always)]
    pub const fn gpio3c4(&self) -> &Gpio3c4 {
        &self.gpio3c4
    }
    #[doc = "0x3c8 - GPIO146 Control Register"]
    #[inline(always)]
    pub const fn gpio3c8(&self) -> &Gpio3c8 {
        &self.gpio3c8
    }
    #[doc = "0x3cc - GPIO147 Control Register"]
    #[inline(always)]
    pub const fn gpio3cc(&self) -> &Gpio3cc {
        &self.gpio3cc
    }
    #[doc = "0x3d0 - GPIO148 Control Register"]
    #[inline(always)]
    pub const fn gpio3d0(&self) -> &Gpio3d0 {
        &self.gpio3d0
    }
    #[doc = "0x3d4 - GPIO149 Control Register"]
    #[inline(always)]
    pub const fn gpio3d4(&self) -> &Gpio3d4 {
        &self.gpio3d4
    }
    #[doc = "0x3d8 - GPIO150 Control Register"]
    #[inline(always)]
    pub const fn gpio3d8(&self) -> &Gpio3d8 {
        &self.gpio3d8
    }
    #[doc = "0x3dc - GPIO151 Control Register"]
    #[inline(always)]
    pub const fn gpio3dc(&self) -> &Gpio3dc {
        &self.gpio3dc
    }
    #[doc = "0x3e0 - GPIO152 Control Register"]
    #[inline(always)]
    pub const fn gpio3e0(&self) -> &Gpio3e0 {
        &self.gpio3e0
    }
    #[doc = "0x3e4 - GPIO153 Control Register"]
    #[inline(always)]
    pub const fn gpio3e4(&self) -> &Gpio3e4 {
        &self.gpio3e4
    }
    #[doc = "0x3e8 - GPIO154 Control Register"]
    #[inline(always)]
    pub const fn gpio3e8(&self) -> &Gpio3e8 {
        &self.gpio3e8
    }
    #[doc = "0x3ec - GPIO155 Control Register"]
    #[inline(always)]
    pub const fn gpio3ec(&self) -> &Gpio3ec {
        &self.gpio3ec
    }
    #[doc = "0x3f0 - GPIO156 Control Register"]
    #[inline(always)]
    pub const fn gpio3f0(&self) -> &Gpio3f0 {
        &self.gpio3f0
    }
    #[doc = "0x3f4 - GPIO157 Control Register"]
    #[inline(always)]
    pub const fn gpio3f4(&self) -> &Gpio3f4 {
        &self.gpio3f4
    }
    #[doc = "0x3f8 - GPIO158 Control Register"]
    #[inline(always)]
    pub const fn gpio3f8(&self) -> &Gpio3f8 {
        &self.gpio3f8
    }
    #[doc = "0x3fc - GPIO159 Control Register"]
    #[inline(always)]
    pub const fn gpio3fc(&self) -> &Gpio3fc {
        &self.gpio3fc
    }
    #[doc = "0x400 - GPIO160 Control Register"]
    #[inline(always)]
    pub const fn gpio400(&self) -> &Gpio400 {
        &self.gpio400
    }
    #[doc = "0x404 - GPIO161 Control Register"]
    #[inline(always)]
    pub const fn gpio404(&self) -> &Gpio404 {
        &self.gpio404
    }
    #[doc = "0x408 - GPIO162 Control Register"]
    #[inline(always)]
    pub const fn gpio408(&self) -> &Gpio408 {
        &self.gpio408
    }
    #[doc = "0x40c - GPIO163 Control Register"]
    #[inline(always)]
    pub const fn gpio40c(&self) -> &Gpio40c {
        &self.gpio40c
    }
    #[doc = "0x410 - GPIO164 Control Register"]
    #[inline(always)]
    pub const fn gpio410(&self) -> &Gpio410 {
        &self.gpio410
    }
    #[doc = "0x414 - GPIO165 Control Register"]
    #[inline(always)]
    pub const fn gpio414(&self) -> &Gpio414 {
        &self.gpio414
    }
    #[doc = "0x418 - GPIO166 Control Register"]
    #[inline(always)]
    pub const fn gpio418(&self) -> &Gpio418 {
        &self.gpio418
    }
    #[doc = "0x41c - GPIO167 Control Register"]
    #[inline(always)]
    pub const fn gpio41c(&self) -> &Gpio41c {
        &self.gpio41c
    }
    #[doc = "0x420 - GPIO168 Control Register"]
    #[inline(always)]
    pub const fn gpio420(&self) -> &Gpio420 {
        &self.gpio420
    }
    #[doc = "0x424 - GPIO169 Control Register"]
    #[inline(always)]
    pub const fn gpio424(&self) -> &Gpio424 {
        &self.gpio424
    }
    #[doc = "0x428 - GPIO170 Control Register"]
    #[inline(always)]
    pub const fn gpio428(&self) -> &Gpio428 {
        &self.gpio428
    }
    #[doc = "0x42c - GPIO171 Control Register"]
    #[inline(always)]
    pub const fn gpio42c(&self) -> &Gpio42c {
        &self.gpio42c
    }
    #[doc = "0x430 - GPIO172 Control Register"]
    #[inline(always)]
    pub const fn gpio430(&self) -> &Gpio430 {
        &self.gpio430
    }
    #[doc = "0x434 - GPIO173 Control Register"]
    #[inline(always)]
    pub const fn gpio434(&self) -> &Gpio434 {
        &self.gpio434
    }
    #[doc = "0x438 - GPIO174 Control Register"]
    #[inline(always)]
    pub const fn gpio438(&self) -> &Gpio438 {
        &self.gpio438
    }
    #[doc = "0x43c - GPIO175 Control Register"]
    #[inline(always)]
    pub const fn gpio43c(&self) -> &Gpio43c {
        &self.gpio43c
    }
    #[doc = "0x440 - GPIO176 Control Register"]
    #[inline(always)]
    pub const fn gpio440(&self) -> &Gpio440 {
        &self.gpio440
    }
    #[doc = "0x444 - GPIO177 Control Register"]
    #[inline(always)]
    pub const fn gpio444(&self) -> &Gpio444 {
        &self.gpio444
    }
    #[doc = "0x448 - GPIO178 Control Register"]
    #[inline(always)]
    pub const fn gpio448(&self) -> &Gpio448 {
        &self.gpio448
    }
    #[doc = "0x44c - GPIO179 Control Register"]
    #[inline(always)]
    pub const fn gpio44c(&self) -> &Gpio44c {
        &self.gpio44c
    }
    #[doc = "0x450 - GPIO180 Control Register"]
    #[inline(always)]
    pub const fn gpio450(&self) -> &Gpio450 {
        &self.gpio450
    }
    #[doc = "0x454 - GPIO181 Control Register"]
    #[inline(always)]
    pub const fn gpio454(&self) -> &Gpio454 {
        &self.gpio454
    }
    #[doc = "0x458 - GPIO182 Control Register"]
    #[inline(always)]
    pub const fn gpio458(&self) -> &Gpio458 {
        &self.gpio458
    }
    #[doc = "0x45c - GPIO183 Control Register"]
    #[inline(always)]
    pub const fn gpio45c(&self) -> &Gpio45c {
        &self.gpio45c
    }
    #[doc = "0x460 - GPIO184 Control Register"]
    #[inline(always)]
    pub const fn gpio460(&self) -> &Gpio460 {
        &self.gpio460
    }
    #[doc = "0x464 - GPIO185 Control Register"]
    #[inline(always)]
    pub const fn gpio464(&self) -> &Gpio464 {
        &self.gpio464
    }
    #[doc = "0x468 - GPIO186 Control Register"]
    #[inline(always)]
    pub const fn gpio468(&self) -> &Gpio468 {
        &self.gpio468
    }
    #[doc = "0x46c - GPIO187 Control Register"]
    #[inline(always)]
    pub const fn gpio46c(&self) -> &Gpio46c {
        &self.gpio46c
    }
    #[doc = "0x470 - GPIO188 Control Register"]
    #[inline(always)]
    pub const fn gpio470(&self) -> &Gpio470 {
        &self.gpio470
    }
    #[doc = "0x474 - GPIO189 Control Register"]
    #[inline(always)]
    pub const fn gpio474(&self) -> &Gpio474 {
        &self.gpio474
    }
    #[doc = "0x478 - GPIO190 Control Register"]
    #[inline(always)]
    pub const fn gpio478(&self) -> &Gpio478 {
        &self.gpio478
    }
    #[doc = "0x47c - GPIO191 Control Register"]
    #[inline(always)]
    pub const fn gpio47c(&self) -> &Gpio47c {
        &self.gpio47c
    }
    #[doc = "0x480 - GPIO192 Control Register"]
    #[inline(always)]
    pub const fn gpio480(&self) -> &Gpio480 {
        &self.gpio480
    }
    #[doc = "0x484 - GPIO193 Control Register"]
    #[inline(always)]
    pub const fn gpio484(&self) -> &Gpio484 {
        &self.gpio484
    }
    #[doc = "0x800 - Master Control Register \\#1"]
    #[inline(always)]
    pub const fn gpio800(&self) -> &Gpio800 {
        &self.gpio800
    }
    #[doc = "0x804 - Master Control Register \\#2"]
    #[inline(always)]
    pub const fn gpio804(&self) -> &Gpio804 {
        &self.gpio804
    }
    #[doc = "0x808 - GPIO Global Privilege Control Register"]
    #[inline(always)]
    pub const fn gpio808(&self) -> &Gpio808 {
        &self.gpio808
    }
    #[doc = "0x810 - GPIO Write Privilege Control Register \\#0"]
    #[inline(always)]
    pub const fn gpio810(&self) -> &Gpio810 {
        &self.gpio810
    }
    #[doc = "0x814 - GPIO Write Privilege Control Register \\#1"]
    #[inline(always)]
    pub const fn gpio814(&self) -> &Gpio814 {
        &self.gpio814
    }
    #[doc = "0x818 - GPIO Write Privilege Control Register \\#2"]
    #[inline(always)]
    pub const fn gpio818(&self) -> &Gpio818 {
        &self.gpio818
    }
    #[doc = "0x81c - GPIO Write Privilege Control Register \\#3"]
    #[inline(always)]
    pub const fn gpio81c(&self) -> &Gpio81c {
        &self.gpio81c
    }
    #[doc = "0x820 - GPIO Write Privilege Control Register \\#4"]
    #[inline(always)]
    pub const fn gpio820(&self) -> &Gpio820 {
        &self.gpio820
    }
    #[doc = "0x824 - GPIO Write Privilege Control Register \\#5"]
    #[inline(always)]
    pub const fn gpio824(&self) -> &Gpio824 {
        &self.gpio824
    }
    #[doc = "0x828 - GPIO Write Privilege Control Register \\#6"]
    #[inline(always)]
    pub const fn gpio828(&self) -> &Gpio828 {
        &self.gpio828
    }
    #[doc = "0x82c - GPIO Write Privilege Control Register \\#7"]
    #[inline(always)]
    pub const fn gpio82c(&self) -> &Gpio82c {
        &self.gpio82c
    }
    #[doc = "0x830 - GPIO Write Privilege Control Register \\#8"]
    #[inline(always)]
    pub const fn gpio830(&self) -> &Gpio830 {
        &self.gpio830
    }
    #[doc = "0x834 - GPIO Write Privilege Control Register \\#9"]
    #[inline(always)]
    pub const fn gpio834(&self) -> &Gpio834 {
        &self.gpio834
    }
    #[doc = "0x838 - GPIO Write Privilege Control Register \\#10"]
    #[inline(always)]
    pub const fn gpio838(&self) -> &Gpio838 {
        &self.gpio838
    }
    #[doc = "0x83c - GPIO Write Privilege Control Register \\#11"]
    #[inline(always)]
    pub const fn gpio83c(&self) -> &Gpio83c {
        &self.gpio83c
    }
    #[doc = "0x840 - GPIO Write Privilege Control Register \\#12"]
    #[inline(always)]
    pub const fn gpio840(&self) -> &Gpio840 {
        &self.gpio840
    }
    #[doc = "0x844 - GPIO Write Privilege Control Register \\#13"]
    #[inline(always)]
    pub const fn gpio844(&self) -> &Gpio844 {
        &self.gpio844
    }
    #[doc = "0x848 - GPIO Write Privilege Control Register \\#14"]
    #[inline(always)]
    pub const fn gpio848(&self) -> &Gpio848 {
        &self.gpio848
    }
    #[doc = "0x84c - GPIO Write Privilege Control Register \\#15"]
    #[inline(always)]
    pub const fn gpio84c(&self) -> &Gpio84c {
        &self.gpio84c
    }
    #[doc = "0x850 - GPIO Write Privilege Control Register \\#16"]
    #[inline(always)]
    pub const fn gpio850(&self) -> &Gpio850 {
        &self.gpio850
    }
    #[doc = "0x854 - GPIO Write Privilege Control Register \\#17"]
    #[inline(always)]
    pub const fn gpio854(&self) -> &Gpio854 {
        &self.gpio854
    }
    #[doc = "0x858 - GPIO Write Privilege Control Register \\#18"]
    #[inline(always)]
    pub const fn gpio858(&self) -> &Gpio858 {
        &self.gpio858
    }
    #[doc = "0x85c - GPIO Write Privilege Control Register \\#19"]
    #[inline(always)]
    pub const fn gpio85c(&self) -> &Gpio85c {
        &self.gpio85c
    }
    #[doc = "0x860 - GPIO Write Privilege Control Register \\#20"]
    #[inline(always)]
    pub const fn gpio860(&self) -> &Gpio860 {
        &self.gpio860
    }
    #[doc = "0x864 - GPIO Write Privilege Control Register \\#21"]
    #[inline(always)]
    pub const fn gpio864(&self) -> &Gpio864 {
        &self.gpio864
    }
    #[doc = "0x868 - GPIO Write Privilege Control Register \\#22"]
    #[inline(always)]
    pub const fn gpio868(&self) -> &Gpio868 {
        &self.gpio868
    }
    #[doc = "0x86c - GPIO Write Privilege Control Register \\#23"]
    #[inline(always)]
    pub const fn gpio86c(&self) -> &Gpio86c {
        &self.gpio86c
    }
    #[doc = "0x870 - GPIO Write Privilege Control Register \\#24"]
    #[inline(always)]
    pub const fn gpio870(&self) -> &Gpio870 {
        &self.gpio870
    }
    #[doc = "0x874 - GPIO Write Privilege Control Register \\#25"]
    #[inline(always)]
    pub const fn gpio874(&self) -> &Gpio874 {
        &self.gpio874
    }
    #[doc = "0x878 - GPIO Write Privilege Control Register \\#26"]
    #[inline(always)]
    pub const fn gpio878(&self) -> &Gpio878 {
        &self.gpio878
    }
    #[doc = "0x87c - GPIO Write Privilege Control Register \\#27"]
    #[inline(always)]
    pub const fn gpio87c(&self) -> &Gpio87c {
        &self.gpio87c
    }
    #[doc = "0x880 - GPIO Write Privilege Control Register \\#28"]
    #[inline(always)]
    pub const fn gpio880(&self) -> &Gpio880 {
        &self.gpio880
    }
    #[doc = "0x884 - GPIO Write Privilege Control Register \\#29"]
    #[inline(always)]
    pub const fn gpio884(&self) -> &Gpio884 {
        &self.gpio884
    }
    #[doc = "0x888 - GPIO Write Privilege Control Register \\#30"]
    #[inline(always)]
    pub const fn gpio888(&self) -> &Gpio888 {
        &self.gpio888
    }
    #[doc = "0x88c - GPIO Write Privilege Control Register \\#31"]
    #[inline(always)]
    pub const fn gpio88c(&self) -> &Gpio88c {
        &self.gpio88c
    }
    #[doc = "0x890 - GPIO Write Privilege Control Register \\#32"]
    #[inline(always)]
    pub const fn gpio890(&self) -> &Gpio890 {
        &self.gpio890
    }
    #[doc = "0x894 - GPIO Write Privilege Control Register \\#33"]
    #[inline(always)]
    pub const fn gpio894(&self) -> &Gpio894 {
        &self.gpio894
    }
    #[doc = "0x898 - GPIO Write Privilege Control Register \\#34"]
    #[inline(always)]
    pub const fn gpio898(&self) -> &Gpio898 {
        &self.gpio898
    }
    #[doc = "0x89c - GPIO Write Privilege Control Register \\#35"]
    #[inline(always)]
    pub const fn gpio89c(&self) -> &Gpio89c {
        &self.gpio89c
    }
    #[doc = "0x8a0 - GPIO Write Privilege Control Register \\#36"]
    #[inline(always)]
    pub const fn gpio8a0(&self) -> &Gpio8a0 {
        &self.gpio8a0
    }
    #[doc = "0x8a4 - GPIO Write Privilege Control Register \\#37"]
    #[inline(always)]
    pub const fn gpio8a4(&self) -> &Gpio8a4 {
        &self.gpio8a4
    }
    #[doc = "0x8a8 - GPIO Write Privilege Control Register \\#38"]
    #[inline(always)]
    pub const fn gpio8a8(&self) -> &Gpio8a8 {
        &self.gpio8a8
    }
    #[doc = "0x8ac - GPIO Write Privilege Control Register \\#39"]
    #[inline(always)]
    pub const fn gpio8ac(&self) -> &Gpio8ac {
        &self.gpio8ac
    }
    #[doc = "0x8b0 - GPIO Write Privilege Control Register \\#40"]
    #[inline(always)]
    pub const fn gpio8b0(&self) -> &Gpio8b0 {
        &self.gpio8b0
    }
    #[doc = "0x8b4 - GPIO Write Privilege Control Register \\#41"]
    #[inline(always)]
    pub const fn gpio8b4(&self) -> &Gpio8b4 {
        &self.gpio8b4
    }
    #[doc = "0x8b8 - GPIO Write Privilege Control Register \\#42"]
    #[inline(always)]
    pub const fn gpio8b8(&self) -> &Gpio8b8 {
        &self.gpio8b8
    }
    #[doc = "0x8bc - GPIO Write Privilege Control Register \\#43"]
    #[inline(always)]
    pub const fn gpio8bc(&self) -> &Gpio8bc {
        &self.gpio8bc
    }
    #[doc = "0x8c0 - GPIO Write Privilege Control Register \\#44"]
    #[inline(always)]
    pub const fn gpio8c0(&self) -> &Gpio8c0 {
        &self.gpio8c0
    }
    #[doc = "0x8c4 - GPIO Write Privilege Control Register \\#45"]
    #[inline(always)]
    pub const fn gpio8c4(&self) -> &Gpio8c4 {
        &self.gpio8c4
    }
    #[doc = "0x8c8 - GPIO Write Privilege Control Register \\#46"]
    #[inline(always)]
    pub const fn gpio8c8(&self) -> &Gpio8c8 {
        &self.gpio8c8
    }
    #[doc = "0x8cc - GPIO Write Privilege Control Register \\#47"]
    #[inline(always)]
    pub const fn gpio8cc(&self) -> &Gpio8cc {
        &self.gpio8cc
    }
    #[doc = "0x8d0 - GPIO Write Privilege Control Register \\#48"]
    #[inline(always)]
    pub const fn gpio8d0(&self) -> &Gpio8d0 {
        &self.gpio8d0
    }
    #[doc = "0x910 - GPIO Read Privilege Control Register \\#0"]
    #[inline(always)]
    pub const fn gpio910(&self) -> &Gpio910 {
        &self.gpio910
    }
    #[doc = "0x914 - GPIO Read Privilege Control Register \\#1"]
    #[inline(always)]
    pub const fn gpio914(&self) -> &Gpio914 {
        &self.gpio914
    }
    #[doc = "0x918 - GPIO Read Privilege Control Register \\#2"]
    #[inline(always)]
    pub const fn gpio918(&self) -> &Gpio918 {
        &self.gpio918
    }
    #[doc = "0x91c - GPIO Read Privilege Control Register \\#3"]
    #[inline(always)]
    pub const fn gpio91c(&self) -> &Gpio91c {
        &self.gpio91c
    }
    #[doc = "0x920 - GPIO Read Privilege Control Register \\#4"]
    #[inline(always)]
    pub const fn gpio920(&self) -> &Gpio920 {
        &self.gpio920
    }
    #[doc = "0x924 - GPIO Read Privilege Control Register \\#5"]
    #[inline(always)]
    pub const fn gpio924(&self) -> &Gpio924 {
        &self.gpio924
    }
    #[doc = "0x928 - GPIO Read Privilege Control Register \\#6"]
    #[inline(always)]
    pub const fn gpio928(&self) -> &Gpio928 {
        &self.gpio928
    }
    #[doc = "0x92c - GPIO Read Privilege Control Register \\#7"]
    #[inline(always)]
    pub const fn gpio92c(&self) -> &Gpio92c {
        &self.gpio92c
    }
    #[doc = "0x930 - GPIO Read Privilege Control Register \\#8"]
    #[inline(always)]
    pub const fn gpio930(&self) -> &Gpio930 {
        &self.gpio930
    }
    #[doc = "0x934 - GPIO Read Privilege Control Register \\#9"]
    #[inline(always)]
    pub const fn gpio934(&self) -> &Gpio934 {
        &self.gpio934
    }
    #[doc = "0x938 - GPIO Read Privilege Control Register \\#10"]
    #[inline(always)]
    pub const fn gpio938(&self) -> &Gpio938 {
        &self.gpio938
    }
    #[doc = "0x93c - GPIO Read Privilege Control Register \\#11"]
    #[inline(always)]
    pub const fn gpio93c(&self) -> &Gpio93c {
        &self.gpio93c
    }
    #[doc = "0x940 - GPIO Read Privilege Control Register \\#12"]
    #[inline(always)]
    pub const fn gpio940(&self) -> &Gpio940 {
        &self.gpio940
    }
    #[doc = "0x944 - GPIO Read Privilege Control Register \\#13"]
    #[inline(always)]
    pub const fn gpio944(&self) -> &Gpio944 {
        &self.gpio944
    }
    #[doc = "0x948 - GPIO Read Privilege Control Register \\#14"]
    #[inline(always)]
    pub const fn gpio948(&self) -> &Gpio948 {
        &self.gpio948
    }
    #[doc = "0x94c - GPIO Read Privilege Control Register \\#15"]
    #[inline(always)]
    pub const fn gpio94c(&self) -> &Gpio94c {
        &self.gpio94c
    }
    #[doc = "0x950 - GPIO Read Privilege Control Register \\#16"]
    #[inline(always)]
    pub const fn gpio950(&self) -> &Gpio950 {
        &self.gpio950
    }
    #[doc = "0x954 - GPIO Read Privilege Control Register \\#17"]
    #[inline(always)]
    pub const fn gpio954(&self) -> &Gpio954 {
        &self.gpio954
    }
    #[doc = "0x958 - GPIO Read Privilege Control Register \\#18"]
    #[inline(always)]
    pub const fn gpio958(&self) -> &Gpio958 {
        &self.gpio958
    }
    #[doc = "0x95c - GPIO Read Privilege Control Register \\#19"]
    #[inline(always)]
    pub const fn gpio95c(&self) -> &Gpio95c {
        &self.gpio95c
    }
    #[doc = "0x960 - GPIO Read Privilege Control Register \\#20"]
    #[inline(always)]
    pub const fn gpio960(&self) -> &Gpio960 {
        &self.gpio960
    }
    #[doc = "0x964 - GPIO Read Privilege Control Register \\#21"]
    #[inline(always)]
    pub const fn gpio964(&self) -> &Gpio964 {
        &self.gpio964
    }
    #[doc = "0x968 - GPIO Read Privilege Control Register \\#22"]
    #[inline(always)]
    pub const fn gpio968(&self) -> &Gpio968 {
        &self.gpio968
    }
    #[doc = "0x96c - GPIO Read Privilege Control Register \\#23"]
    #[inline(always)]
    pub const fn gpio96c(&self) -> &Gpio96c {
        &self.gpio96c
    }
    #[doc = "0x970 - GPIO Read Privilege Control Register \\#24"]
    #[inline(always)]
    pub const fn gpio970(&self) -> &Gpio970 {
        &self.gpio970
    }
    #[doc = "0x974 - GPIO Read Privilege Control Register \\#25"]
    #[inline(always)]
    pub const fn gpio974(&self) -> &Gpio974 {
        &self.gpio974
    }
    #[doc = "0x978 - GPIO Read Privilege Control Register \\#26"]
    #[inline(always)]
    pub const fn gpio978(&self) -> &Gpio978 {
        &self.gpio978
    }
    #[doc = "0x97c - GPIO Read Privilege Control Register \\#27"]
    #[inline(always)]
    pub const fn gpio97c(&self) -> &Gpio97c {
        &self.gpio97c
    }
    #[doc = "0x980 - GPIO Read Privilege Control Register \\#28"]
    #[inline(always)]
    pub const fn gpio980(&self) -> &Gpio980 {
        &self.gpio980
    }
    #[doc = "0x984 - GPIO Read Privilege Control Register \\#29"]
    #[inline(always)]
    pub const fn gpio984(&self) -> &Gpio984 {
        &self.gpio984
    }
    #[doc = "0x988 - GPIO Read Privilege Control Register \\#30"]
    #[inline(always)]
    pub const fn gpio988(&self) -> &Gpio988 {
        &self.gpio988
    }
    #[doc = "0x98c - GPIO Read Privilege Control Register \\#31"]
    #[inline(always)]
    pub const fn gpio98c(&self) -> &Gpio98c {
        &self.gpio98c
    }
    #[doc = "0x990 - GPIO Read Privilege Control Register \\#32"]
    #[inline(always)]
    pub const fn gpio990(&self) -> &Gpio990 {
        &self.gpio990
    }
    #[doc = "0x994 - GPIO Read Privilege Control Register \\#33"]
    #[inline(always)]
    pub const fn gpio994(&self) -> &Gpio994 {
        &self.gpio994
    }
    #[doc = "0x998 - GPIO Read Privilege Control Register \\#34"]
    #[inline(always)]
    pub const fn gpio998(&self) -> &Gpio998 {
        &self.gpio998
    }
    #[doc = "0x99c - GPIO Read Privilege Control Register \\#35"]
    #[inline(always)]
    pub const fn gpio99c(&self) -> &Gpio99c {
        &self.gpio99c
    }
    #[doc = "0x9a0 - GPIO Read Privilege Control Register \\#36"]
    #[inline(always)]
    pub const fn gpio9a0(&self) -> &Gpio9a0 {
        &self.gpio9a0
    }
    #[doc = "0x9a4 - GPIO Read Privilege Control Register \\#37"]
    #[inline(always)]
    pub const fn gpio9a4(&self) -> &Gpio9a4 {
        &self.gpio9a4
    }
    #[doc = "0x9a8 - GPIO Read Privilege Control Register \\#38"]
    #[inline(always)]
    pub const fn gpio9a8(&self) -> &Gpio9a8 {
        &self.gpio9a8
    }
    #[doc = "0x9ac - GPIO Read Privilege Control Register \\#39"]
    #[inline(always)]
    pub const fn gpio9ac(&self) -> &Gpio9ac {
        &self.gpio9ac
    }
    #[doc = "0x9b0 - GPIO Read Privilege Control Register \\#40"]
    #[inline(always)]
    pub const fn gpio9b0(&self) -> &Gpio9b0 {
        &self.gpio9b0
    }
    #[doc = "0x9b4 - GPIO Read Privilege Control Register \\#41"]
    #[inline(always)]
    pub const fn gpio9b4(&self) -> &Gpio9b4 {
        &self.gpio9b4
    }
    #[doc = "0x9b8 - GPIO Read Privilege Control Register \\#42"]
    #[inline(always)]
    pub const fn gpio9b8(&self) -> &Gpio9b8 {
        &self.gpio9b8
    }
    #[doc = "0x9bc - GPIO Read Privilege Control Register \\#43"]
    #[inline(always)]
    pub const fn gpio9bc(&self) -> &Gpio9bc {
        &self.gpio9bc
    }
    #[doc = "0x9c0 - GPIO Read Privilege Control Register \\#44"]
    #[inline(always)]
    pub const fn gpio9c0(&self) -> &Gpio9c0 {
        &self.gpio9c0
    }
    #[doc = "0x9c4 - GPIO Read Privilege Control Register \\#45"]
    #[inline(always)]
    pub const fn gpio9c4(&self) -> &Gpio9c4 {
        &self.gpio9c4
    }
    #[doc = "0x9c8 - GPIO Read Privilege Control Register \\#46"]
    #[inline(always)]
    pub const fn gpio9c8(&self) -> &Gpio9c8 {
        &self.gpio9c8
    }
    #[doc = "0x9cc - GPIO Read Privilege Control Register \\#47"]
    #[inline(always)]
    pub const fn gpio9cc(&self) -> &Gpio9cc {
        &self.gpio9cc
    }
    #[doc = "0x9d0 - GPIO Read Privilege Control Register \\#48"]
    #[inline(always)]
    pub const fn gpio9d0(&self) -> &Gpio9d0 {
        &self.gpio9d0
    }
    #[doc = "0xa14 - GPIO Interrupt Target Control Register \\#1"]
    #[inline(always)]
    pub const fn gpioa14(&self) -> &Gpioa14 {
        &self.gpioa14
    }
    #[doc = "0xa18 - GPIO Interrupt Target Control Register \\#2"]
    #[inline(always)]
    pub const fn gpioa18(&self) -> &Gpioa18 {
        &self.gpioa18
    }
    #[doc = "0xa1c - GPIO Interrupt Target Control Register \\#3"]
    #[inline(always)]
    pub const fn gpioa1c(&self) -> &Gpioa1c {
        &self.gpioa1c
    }
    #[doc = "0xa20 - GPIO Interrupt Target Control Register \\#4"]
    #[inline(always)]
    pub const fn gpioa20(&self) -> &Gpioa20 {
        &self.gpioa20
    }
    #[doc = "0xa24 - GPIO Interrupt Target Control Register \\#5"]
    #[inline(always)]
    pub const fn gpioa24(&self) -> &Gpioa24 {
        &self.gpioa24
    }
    #[doc = "0xa28 - GPIO Interrupt Target Control Register \\#6"]
    #[inline(always)]
    pub const fn gpioa28(&self) -> &Gpioa28 {
        &self.gpioa28
    }
    #[doc = "0xa2c - GPIO Interrupt Target Control Register \\#7"]
    #[inline(always)]
    pub const fn gpioa2c(&self) -> &Gpioa2c {
        &self.gpioa2c
    }
    #[doc = "0xa30 - GPIO Interrupt Target Control Register \\#8"]
    #[inline(always)]
    pub const fn gpioa30(&self) -> &Gpioa30 {
        &self.gpioa30
    }
    #[doc = "0xa34 - GPIO Interrupt Target Control Register \\#9"]
    #[inline(always)]
    pub const fn gpioa34(&self) -> &Gpioa34 {
        &self.gpioa34
    }
    #[doc = "0xa38 - GPIO Interrupt Target Control Register \\#10"]
    #[inline(always)]
    pub const fn gpioa38(&self) -> &Gpioa38 {
        &self.gpioa38
    }
    #[doc = "0xa3c - GPIO Interrupt Target Control Register \\#11"]
    #[inline(always)]
    pub const fn gpioa3c(&self) -> &Gpioa3c {
        &self.gpioa3c
    }
    #[doc = "0xa40 - GPIO Interrupt Target Control Register \\#12"]
    #[inline(always)]
    pub const fn gpioa40(&self) -> &Gpioa40 {
        &self.gpioa40
    }
    #[doc = "0xa44 - GPIO Interrupt Target Control Register \\#13"]
    #[inline(always)]
    pub const fn gpioa44(&self) -> &Gpioa44 {
        &self.gpioa44
    }
    #[doc = "0xa48 - GPIO Interrupt Target Control Register \\#14"]
    #[inline(always)]
    pub const fn gpioa48(&self) -> &Gpioa48 {
        &self.gpioa48
    }
    #[doc = "0xa4c - GPIO Interrupt Target Control Register \\#15"]
    #[inline(always)]
    pub const fn gpioa4c(&self) -> &Gpioa4c {
        &self.gpioa4c
    }
    #[doc = "0xa50 - GPIO Interrupt Target Control Register \\#16"]
    #[inline(always)]
    pub const fn gpioa50(&self) -> &Gpioa50 {
        &self.gpioa50
    }
    #[doc = "0xa54 - GPIO Interrupt Target Control Register \\#17"]
    #[inline(always)]
    pub const fn gpioa54(&self) -> &Gpioa54 {
        &self.gpioa54
    }
    #[doc = "0xa58 - GPIO Interrupt Target Control Register \\#18"]
    #[inline(always)]
    pub const fn gpioa58(&self) -> &Gpioa58 {
        &self.gpioa58
    }
    #[doc = "0xa5c - GPIO Interrupt Target Control Register \\#19"]
    #[inline(always)]
    pub const fn gpioa5c(&self) -> &Gpioa5c {
        &self.gpioa5c
    }
    #[doc = "0xa60 - GPIO Interrupt Target Control Register \\#20"]
    #[inline(always)]
    pub const fn gpioa60(&self) -> &Gpioa60 {
        &self.gpioa60
    }
    #[doc = "0xa64 - GPIO Interrupt Target Control Register \\#21"]
    #[inline(always)]
    pub const fn gpioa64(&self) -> &Gpioa64 {
        &self.gpioa64
    }
    #[doc = "0xa68 - GPIO Interrupt Target Control Register \\#22"]
    #[inline(always)]
    pub const fn gpioa68(&self) -> &Gpioa68 {
        &self.gpioa68
    }
    #[doc = "0xa6c - GPIO Interrupt Target Control Register \\#23"]
    #[inline(always)]
    pub const fn gpioa6c(&self) -> &Gpioa6c {
        &self.gpioa6c
    }
    #[doc = "0xa70 - GPIO Interrupt Target Control Register \\#24"]
    #[inline(always)]
    pub const fn gpioa70(&self) -> &Gpioa70 {
        &self.gpioa70
    }
    #[doc = "0xa74 - GPIO Interrupt Target Control Register \\#25"]
    #[inline(always)]
    pub const fn gpioa74(&self) -> &Gpioa74 {
        &self.gpioa74
    }
    #[doc = "0xa78 - GPIO Interrupt Target Control Register \\#26"]
    #[inline(always)]
    pub const fn gpioa78(&self) -> &Gpioa78 {
        &self.gpioa78
    }
    #[doc = "0xa7c - GPIO Interrupt Target Control Register \\#27"]
    #[inline(always)]
    pub const fn gpioa7c(&self) -> &Gpioa7c {
        &self.gpioa7c
    }
    #[doc = "0xa80 - GPIO Interrupt Target Control Register \\#28"]
    #[inline(always)]
    pub const fn gpioa80(&self) -> &Gpioa80 {
        &self.gpioa80
    }
    #[doc = "0xa84 - GPIO Interrupt Target Control Register \\#29"]
    #[inline(always)]
    pub const fn gpioa84(&self) -> &Gpioa84 {
        &self.gpioa84
    }
    #[doc = "0xa88 - GPIO Interrupt Target Control Register \\#30"]
    #[inline(always)]
    pub const fn gpioa88(&self) -> &Gpioa88 {
        &self.gpioa88
    }
    #[doc = "0xa8c - GPIO Interrupt Target Control Register \\#31"]
    #[inline(always)]
    pub const fn gpioa8c(&self) -> &Gpioa8c {
        &self.gpioa8c
    }
    #[doc = "0xa90 - GPIO Interrupt Target Control Register \\#32"]
    #[inline(always)]
    pub const fn gpioa90(&self) -> &Gpioa90 {
        &self.gpioa90
    }
    #[doc = "0xa94 - GPIO Interrupt Target Control Register \\#33"]
    #[inline(always)]
    pub const fn gpioa94(&self) -> &Gpioa94 {
        &self.gpioa94
    }
    #[doc = "0xa98 - GPIO Interrupt Target Control Register \\#34"]
    #[inline(always)]
    pub const fn gpioa98(&self) -> &Gpioa98 {
        &self.gpioa98
    }
    #[doc = "0xa9c - GPIO Interrupt Target Control Register \\#35"]
    #[inline(always)]
    pub const fn gpioa9c(&self) -> &Gpioa9c {
        &self.gpioa9c
    }
    #[doc = "0xaa0 - GPIO Interrupt Target Control Register \\#36"]
    #[inline(always)]
    pub const fn gpioaa0(&self) -> &Gpioaa0 {
        &self.gpioaa0
    }
    #[doc = "0xaa4 - GPIO Interrupt Target Control Register \\#37"]
    #[inline(always)]
    pub const fn gpioaa4(&self) -> &Gpioaa4 {
        &self.gpioaa4
    }
    #[doc = "0xaa8 - GPIO Interrupt Target Control Register \\#38"]
    #[inline(always)]
    pub const fn gpioaa8(&self) -> &Gpioaa8 {
        &self.gpioaa8
    }
    #[doc = "0xaac - GPIO Interrupt Target Control Register \\#39"]
    #[inline(always)]
    pub const fn gpioaac(&self) -> &Gpioaac {
        &self.gpioaac
    }
    #[doc = "0xab0 - GPIO Interrupt Target Control Register \\#40"]
    #[inline(always)]
    pub const fn gpioab0(&self) -> &Gpioab0 {
        &self.gpioab0
    }
    #[doc = "0xab4 - GPIO Interrupt Target Control Register \\#41"]
    #[inline(always)]
    pub const fn gpioab4(&self) -> &Gpioab4 {
        &self.gpioab4
    }
    #[doc = "0xab8 - GPIO Interrupt Target Control Register \\#42"]
    #[inline(always)]
    pub const fn gpioab8(&self) -> &Gpioab8 {
        &self.gpioab8
    }
    #[doc = "0xabc - GPIO Interrupt Target Control Register \\#43"]
    #[inline(always)]
    pub const fn gpioabc(&self) -> &Gpioabc {
        &self.gpioabc
    }
    #[doc = "0xac0 - GPIO Interrupt Target Control Register \\#44"]
    #[inline(always)]
    pub const fn gpioac0(&self) -> &Gpioac0 {
        &self.gpioac0
    }
    #[doc = "0xac4 - GPIO Interrupt Target Control Register \\#45"]
    #[inline(always)]
    pub const fn gpioac4(&self) -> &Gpioac4 {
        &self.gpioac4
    }
    #[doc = "0xac8 - GPIO Interrupt Target Control Register \\#46"]
    #[inline(always)]
    pub const fn gpioac8(&self) -> &Gpioac8 {
        &self.gpioac8
    }
    #[doc = "0xacc - GPIO Interrupt Target Control Register \\#47"]
    #[inline(always)]
    pub const fn gpioacc(&self) -> &Gpioacc {
        &self.gpioacc
    }
    #[doc = "0xad0 - GPIO Interrupt Target Control Register \\#48"]
    #[inline(always)]
    pub const fn gpioad0(&self) -> &Gpioad0 {
        &self.gpioad0
    }
    #[doc = "0xb10 - Write Privilege Reset Tolerance Register \\#1"]
    #[inline(always)]
    pub const fn gpiob10(&self) -> &Gpiob10 {
        &self.gpiob10
    }
    #[doc = "0xb14 - Write Privilege Reset Tolerance Register \\#2"]
    #[inline(always)]
    pub const fn gpiob14(&self) -> &Gpiob14 {
        &self.gpiob14
    }
    #[doc = "0xb18 - Write Privilege Reset Tolerance Register \\#3"]
    #[inline(always)]
    pub const fn gpiob18(&self) -> &Gpiob18 {
        &self.gpiob18
    }
    #[doc = "0xb1c - Write Privilege Reset Tolerance Register \\#4"]
    #[inline(always)]
    pub const fn gpiob1c(&self) -> &Gpiob1c {
        &self.gpiob1c
    }
    #[doc = "0xb20 - Write Privilege Reset Tolerance Register \\#5"]
    #[inline(always)]
    pub const fn gpiob20(&self) -> &Gpiob20 {
        &self.gpiob20
    }
    #[doc = "0xb24 - Write Privilege Reset Tolerance Register \\#6"]
    #[inline(always)]
    pub const fn gpiob24(&self) -> &Gpiob24 {
        &self.gpiob24
    }
    #[doc = "0xb28 - Write Privilege Reset Tolerance Register \\#7"]
    #[inline(always)]
    pub const fn gpiob28(&self) -> &Gpiob28 {
        &self.gpiob28
    }
    #[doc = "0xc10 - Read Privilege Reset Tolerance Register \\#1"]
    #[inline(always)]
    pub const fn gpioc10(&self) -> &Gpioc10 {
        &self.gpioc10
    }
    #[doc = "0xc14 - Read Privilege Reset Tolerance Register \\#2"]
    #[inline(always)]
    pub const fn gpioc14(&self) -> &Gpioc14 {
        &self.gpioc14
    }
    #[doc = "0xc18 - Read Privilege Reset Tolerance Register \\#3"]
    #[inline(always)]
    pub const fn gpioc18(&self) -> &Gpioc18 {
        &self.gpioc18
    }
    #[doc = "0xc1c - Read Privilege Reset Tolerance Register \\#4"]
    #[inline(always)]
    pub const fn gpioc1c(&self) -> &Gpioc1c {
        &self.gpioc1c
    }
    #[doc = "0xc20 - Read Privilege Reset Tolerance Register \\#5"]
    #[inline(always)]
    pub const fn gpioc20(&self) -> &Gpioc20 {
        &self.gpioc20
    }
    #[doc = "0xc24 - Read Privilege Reset Tolerance Register \\#6"]
    #[inline(always)]
    pub const fn gpioc24(&self) -> &Gpioc24 {
        &self.gpioc24
    }
    #[doc = "0xc28 - Read Privilege Reset Tolerance Register \\#7"]
    #[inline(always)]
    pub const fn gpioc28(&self) -> &Gpioc28 {
        &self.gpioc28
    }
    #[doc = "0xd10 - Write Privilege Write Protection Register \\#1"]
    #[inline(always)]
    pub const fn gpiod10(&self) -> &Gpiod10 {
        &self.gpiod10
    }
    #[doc = "0xd14 - Write Privilege Write Protection Register \\#2"]
    #[inline(always)]
    pub const fn gpiod14(&self) -> &Gpiod14 {
        &self.gpiod14
    }
    #[doc = "0xd18 - Write Privilege Write Protection Register \\#3"]
    #[inline(always)]
    pub const fn gpiod18(&self) -> &Gpiod18 {
        &self.gpiod18
    }
    #[doc = "0xd1c - Write Privilege Write Protection Register \\#4"]
    #[inline(always)]
    pub const fn gpiod1c(&self) -> &Gpiod1c {
        &self.gpiod1c
    }
    #[doc = "0xd20 - Write Privilege Write Protection Register \\#5"]
    #[inline(always)]
    pub const fn gpiod20(&self) -> &Gpiod20 {
        &self.gpiod20
    }
    #[doc = "0xd24 - Write Privilege Write Protection Register \\#6"]
    #[inline(always)]
    pub const fn gpiod24(&self) -> &Gpiod24 {
        &self.gpiod24
    }
    #[doc = "0xd28 - Write Privilege Write Protection Register \\#7"]
    #[inline(always)]
    pub const fn gpiod28(&self) -> &Gpiod28 {
        &self.gpiod28
    }
    #[doc = "0xe10 - Read Privilege Write Protection Register \\#1"]
    #[inline(always)]
    pub const fn gpioe10(&self) -> &Gpioe10 {
        &self.gpioe10
    }
    #[doc = "0xe14 - Read Privilege Write Protection Register \\#2"]
    #[inline(always)]
    pub const fn gpioe14(&self) -> &Gpioe14 {
        &self.gpioe14
    }
    #[doc = "0xe18 - Read Privilege Write Protection Register \\#3"]
    #[inline(always)]
    pub const fn gpioe18(&self) -> &Gpioe18 {
        &self.gpioe18
    }
    #[doc = "0xe1c - Read Privilege Write Protection Register \\#4"]
    #[inline(always)]
    pub const fn gpioe1c(&self) -> &Gpioe1c {
        &self.gpioe1c
    }
    #[doc = "0xe20 - Read Privilege Write Protection Register \\#5"]
    #[inline(always)]
    pub const fn gpioe20(&self) -> &Gpioe20 {
        &self.gpioe20
    }
    #[doc = "0xe24 - Read Privilege Write Protection Register \\#6"]
    #[inline(always)]
    pub const fn gpioe24(&self) -> &Gpioe24 {
        &self.gpioe24
    }
    #[doc = "0xe28 - Read Privilege Write Protection Register \\#7"]
    #[inline(always)]
    pub const fn gpioe28(&self) -> &Gpioe28 {
        &self.gpioe28
    }
}
#[doc = "GPIO000 (rw) register accessor: Debounce Timer Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio000`] module"]
#[doc(alias = "GPIO000")]
pub type Gpio000 = crate::Reg<gpio000::Gpio000Spec>;
#[doc = "Debounce Timer Setting Register \\#1"]
pub mod gpio000;
#[doc = "GPIO004 (rw) register accessor: Debounce Timer Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio004`] module"]
#[doc(alias = "GPIO004")]
pub type Gpio004 = crate::Reg<gpio004::Gpio004Spec>;
#[doc = "Debounce Timer Setting Register \\#2"]
pub mod gpio004;
#[doc = "GPIO008 (rw) register accessor: Debounce Timer Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio008`] module"]
#[doc(alias = "GPIO008")]
pub type Gpio008 = crate::Reg<gpio008::Gpio008Spec>;
#[doc = "Debounce Timer Setting Register \\#3"]
pub mod gpio008;
#[doc = "GPIO00C (rw) register accessor: GPIO Blink Source Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio00c`] module"]
#[doc(alias = "GPIO00C")]
pub type Gpio00c = crate::Reg<gpio00c::Gpio00cSpec>;
#[doc = "GPIO Blink Source Clock Division"]
pub mod gpio00c;
#[doc = "GPIO010 (rw) register accessor: GPIO Blink Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio010`] module"]
#[doc(alias = "GPIO010")]
pub type Gpio010 = crate::Reg<gpio010::Gpio010Spec>;
#[doc = "GPIO Blink Control"]
pub mod gpio010;
#[doc = "GPIO014 (rw) register accessor: GPIO Blink Counter \\#1 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio014`] module"]
#[doc(alias = "GPIO014")]
pub type Gpio014 = crate::Reg<gpio014::Gpio014Spec>;
#[doc = "GPIO Blink Counter \\#1 Configuration"]
pub mod gpio014;
#[doc = "GPIO018 (rw) register accessor: GPIO Blink Counter \\#2 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio018`] module"]
#[doc(alias = "GPIO018")]
pub type Gpio018 = crate::Reg<gpio018::Gpio018Spec>;
#[doc = "GPIO Blink Counter \\#2 Configuration"]
pub mod gpio018;
#[doc = "GPIO01C (rw) register accessor: GPIO Blink Counter \\#3 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio01c`] module"]
#[doc(alias = "GPIO01C")]
pub type Gpio01c = crate::Reg<gpio01c::Gpio01cSpec>;
#[doc = "GPIO Blink Counter \\#3 Configuration"]
pub mod gpio01c;
#[doc = "GPIO020 (rw) register accessor: 80h Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio020`] module"]
#[doc(alias = "GPIO020")]
pub type Gpio020 = crate::Reg<gpio020::Gpio020Spec>;
#[doc = "80h Enable Register"]
pub mod gpio020;
#[doc = "GPIO03C (rw) register accessor: Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio03c`] module"]
#[doc(alias = "GPIO03C")]
pub type Gpio03c = crate::Reg<gpio03c::Gpio03cSpec>;
#[doc = "Write Protection Register"]
pub mod gpio03c;
#[doc = "GPIO100 (rw) register accessor: Interrupt Status Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio100`] module"]
#[doc(alias = "GPIO100")]
pub type Gpio100 = crate::Reg<gpio100::Gpio100Spec>;
#[doc = "Interrupt Status Register \\#1"]
pub mod gpio100;
#[doc = "GPIO104 (rw) register accessor: Interrupt Status Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio104`] module"]
#[doc(alias = "GPIO104")]
pub type Gpio104 = crate::Reg<gpio104::Gpio104Spec>;
#[doc = "Interrupt Status Register \\#2"]
pub mod gpio104;
#[doc = "GPIO108 (rw) register accessor: Interrupt Status Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio108`] module"]
#[doc(alias = "GPIO108")]
pub type Gpio108 = crate::Reg<gpio108::Gpio108Spec>;
#[doc = "Interrupt Status Register \\#3"]
pub mod gpio108;
#[doc = "GPIO10C (rw) register accessor: Interrupt Status Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio10c`] module"]
#[doc(alias = "GPIO10C")]
pub type Gpio10c = crate::Reg<gpio10c::Gpio10cSpec>;
#[doc = "Interrupt Status Register \\#4"]
pub mod gpio10c;
#[doc = "GPIO110 (rw) register accessor: Interrupt Status Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio110`] module"]
#[doc(alias = "GPIO110")]
pub type Gpio110 = crate::Reg<gpio110::Gpio110Spec>;
#[doc = "Interrupt Status Register \\#5"]
pub mod gpio110;
#[doc = "GPIO114 (rw) register accessor: Interrupt Status Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio114`] module"]
#[doc(alias = "GPIO114")]
pub type Gpio114 = crate::Reg<gpio114::Gpio114Spec>;
#[doc = "Interrupt Status Register \\#6"]
pub mod gpio114;
#[doc = "GPIO118 (rw) register accessor: Interrupt Status Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio118`] module"]
#[doc(alias = "GPIO118")]
pub type Gpio118 = crate::Reg<gpio118::Gpio118Spec>;
#[doc = "Interrupt Status Register \\#7"]
pub mod gpio118;
#[doc = "GPIO180 (rw) register accessor: GPIO000 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio180`] module"]
#[doc(alias = "GPIO180")]
pub type Gpio180 = crate::Reg<gpio180::Gpio180Spec>;
#[doc = "GPIO000 Control Register"]
pub mod gpio180;
#[doc = "GPIO184 (rw) register accessor: GPIO001 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio184`] module"]
#[doc(alias = "GPIO184")]
pub type Gpio184 = crate::Reg<gpio184::Gpio184Spec>;
#[doc = "GPIO001 Control Register"]
pub mod gpio184;
#[doc = "GPIO188 (rw) register accessor: GPIO002 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio188`] module"]
#[doc(alias = "GPIO188")]
pub type Gpio188 = crate::Reg<gpio188::Gpio188Spec>;
#[doc = "GPIO002 Control Register"]
pub mod gpio188;
#[doc = "GPIO18C (rw) register accessor: GPIO003 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio18c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio18c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio18c`] module"]
#[doc(alias = "GPIO18C")]
pub type Gpio18c = crate::Reg<gpio18c::Gpio18cSpec>;
#[doc = "GPIO003 Control Register"]
pub mod gpio18c;
#[doc = "GPIO190 (rw) register accessor: GPIO004 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio190`] module"]
#[doc(alias = "GPIO190")]
pub type Gpio190 = crate::Reg<gpio190::Gpio190Spec>;
#[doc = "GPIO004 Control Register"]
pub mod gpio190;
#[doc = "GPIO194 (rw) register accessor: GPIO005 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio194`] module"]
#[doc(alias = "GPIO194")]
pub type Gpio194 = crate::Reg<gpio194::Gpio194Spec>;
#[doc = "GPIO005 Control Register"]
pub mod gpio194;
#[doc = "GPIO198 (rw) register accessor: GPIO006 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio198`] module"]
#[doc(alias = "GPIO198")]
pub type Gpio198 = crate::Reg<gpio198::Gpio198Spec>;
#[doc = "GPIO006 Control Register"]
pub mod gpio198;
#[doc = "GPIO19C (rw) register accessor: GPIO007 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio19c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio19c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio19c`] module"]
#[doc(alias = "GPIO19C")]
pub type Gpio19c = crate::Reg<gpio19c::Gpio19cSpec>;
#[doc = "GPIO007 Control Register"]
pub mod gpio19c;
#[doc = "GPIO1A0 (rw) register accessor: GPIO008 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1a0`] module"]
#[doc(alias = "GPIO1A0")]
pub type Gpio1a0 = crate::Reg<gpio1a0::Gpio1a0Spec>;
#[doc = "GPIO008 Control Register"]
pub mod gpio1a0;
#[doc = "GPIO1A4 (rw) register accessor: GPIO009 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1a4`] module"]
#[doc(alias = "GPIO1A4")]
pub type Gpio1a4 = crate::Reg<gpio1a4::Gpio1a4Spec>;
#[doc = "GPIO009 Control Register"]
pub mod gpio1a4;
#[doc = "GPIO1A8 (rw) register accessor: GPIO010 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1a8`] module"]
#[doc(alias = "GPIO1A8")]
pub type Gpio1a8 = crate::Reg<gpio1a8::Gpio1a8Spec>;
#[doc = "GPIO010 Control Register"]
pub mod gpio1a8;
#[doc = "GPIO1AC (rw) register accessor: GPIO011 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1ac`] module"]
#[doc(alias = "GPIO1AC")]
pub type Gpio1ac = crate::Reg<gpio1ac::Gpio1acSpec>;
#[doc = "GPIO011 Control Register"]
pub mod gpio1ac;
#[doc = "GPIO1B0 (rw) register accessor: GPIO012 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1b0`] module"]
#[doc(alias = "GPIO1B0")]
pub type Gpio1b0 = crate::Reg<gpio1b0::Gpio1b0Spec>;
#[doc = "GPIO012 Control Register"]
pub mod gpio1b0;
#[doc = "GPIO1B4 (rw) register accessor: GPIO013 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1b4`] module"]
#[doc(alias = "GPIO1B4")]
pub type Gpio1b4 = crate::Reg<gpio1b4::Gpio1b4Spec>;
#[doc = "GPIO013 Control Register"]
pub mod gpio1b4;
#[doc = "GPIO1B8 (rw) register accessor: GPIO014 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1b8`] module"]
#[doc(alias = "GPIO1B8")]
pub type Gpio1b8 = crate::Reg<gpio1b8::Gpio1b8Spec>;
#[doc = "GPIO014 Control Register"]
pub mod gpio1b8;
#[doc = "GPIO1BC (rw) register accessor: GPIO015 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1bc`] module"]
#[doc(alias = "GPIO1BC")]
pub type Gpio1bc = crate::Reg<gpio1bc::Gpio1bcSpec>;
#[doc = "GPIO015 Control Register"]
pub mod gpio1bc;
#[doc = "GPIO1C0 (rw) register accessor: GPIO016 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1c0`] module"]
#[doc(alias = "GPIO1C0")]
pub type Gpio1c0 = crate::Reg<gpio1c0::Gpio1c0Spec>;
#[doc = "GPIO016 Control Register"]
pub mod gpio1c0;
#[doc = "GPIO1C4 (rw) register accessor: GPIO017 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1c4`] module"]
#[doc(alias = "GPIO1C4")]
pub type Gpio1c4 = crate::Reg<gpio1c4::Gpio1c4Spec>;
#[doc = "GPIO017 Control Register"]
pub mod gpio1c4;
#[doc = "GPIO1C8 (rw) register accessor: GPIO018 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1c8`] module"]
#[doc(alias = "GPIO1C8")]
pub type Gpio1c8 = crate::Reg<gpio1c8::Gpio1c8Spec>;
#[doc = "GPIO018 Control Register"]
pub mod gpio1c8;
#[doc = "GPIO1CC (rw) register accessor: GPIO019 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1cc`] module"]
#[doc(alias = "GPIO1CC")]
pub type Gpio1cc = crate::Reg<gpio1cc::Gpio1ccSpec>;
#[doc = "GPIO019 Control Register"]
pub mod gpio1cc;
#[doc = "GPIO1D0 (rw) register accessor: GPIO020 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1d0`] module"]
#[doc(alias = "GPIO1D0")]
pub type Gpio1d0 = crate::Reg<gpio1d0::Gpio1d0Spec>;
#[doc = "GPIO020 Control Register"]
pub mod gpio1d0;
#[doc = "GPIO1D4 (rw) register accessor: GPIO021 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1d4`] module"]
#[doc(alias = "GPIO1D4")]
pub type Gpio1d4 = crate::Reg<gpio1d4::Gpio1d4Spec>;
#[doc = "GPIO021 Control Register"]
pub mod gpio1d4;
#[doc = "GPIO1D8 (rw) register accessor: GPIO022 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1d8`] module"]
#[doc(alias = "GPIO1D8")]
pub type Gpio1d8 = crate::Reg<gpio1d8::Gpio1d8Spec>;
#[doc = "GPIO022 Control Register"]
pub mod gpio1d8;
#[doc = "GPIO1DC (rw) register accessor: GPIO023 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1dc`] module"]
#[doc(alias = "GPIO1DC")]
pub type Gpio1dc = crate::Reg<gpio1dc::Gpio1dcSpec>;
#[doc = "GPIO023 Control Register"]
pub mod gpio1dc;
#[doc = "GPIO1E0 (rw) register accessor: GPIO024 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1e0`] module"]
#[doc(alias = "GPIO1E0")]
pub type Gpio1e0 = crate::Reg<gpio1e0::Gpio1e0Spec>;
#[doc = "GPIO024 Control Register"]
pub mod gpio1e0;
#[doc = "GPIO1E4 (rw) register accessor: GPIO025 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1e4`] module"]
#[doc(alias = "GPIO1E4")]
pub type Gpio1e4 = crate::Reg<gpio1e4::Gpio1e4Spec>;
#[doc = "GPIO025 Control Register"]
pub mod gpio1e4;
#[doc = "GPIO1E8 (rw) register accessor: GPIO026 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1e8`] module"]
#[doc(alias = "GPIO1E8")]
pub type Gpio1e8 = crate::Reg<gpio1e8::Gpio1e8Spec>;
#[doc = "GPIO026 Control Register"]
pub mod gpio1e8;
#[doc = "GPIO1EC (rw) register accessor: GPIO027 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1ec`] module"]
#[doc(alias = "GPIO1EC")]
pub type Gpio1ec = crate::Reg<gpio1ec::Gpio1ecSpec>;
#[doc = "GPIO027 Control Register"]
pub mod gpio1ec;
#[doc = "GPIO1F0 (rw) register accessor: GPIO028 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1f0`] module"]
#[doc(alias = "GPIO1F0")]
pub type Gpio1f0 = crate::Reg<gpio1f0::Gpio1f0Spec>;
#[doc = "GPIO028 Control Register"]
pub mod gpio1f0;
#[doc = "GPIO1F4 (rw) register accessor: GPIO029 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1f4`] module"]
#[doc(alias = "GPIO1F4")]
pub type Gpio1f4 = crate::Reg<gpio1f4::Gpio1f4Spec>;
#[doc = "GPIO029 Control Register"]
pub mod gpio1f4;
#[doc = "GPIO1F8 (rw) register accessor: GPIO030 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1f8`] module"]
#[doc(alias = "GPIO1F8")]
pub type Gpio1f8 = crate::Reg<gpio1f8::Gpio1f8Spec>;
#[doc = "GPIO030 Control Register"]
pub mod gpio1f8;
#[doc = "GPIO1FC (rw) register accessor: GPIO031 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1fc`] module"]
#[doc(alias = "GPIO1FC")]
pub type Gpio1fc = crate::Reg<gpio1fc::Gpio1fcSpec>;
#[doc = "GPIO031 Control Register"]
pub mod gpio1fc;
#[doc = "GPIO200 (rw) register accessor: GPIO032 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio200`] module"]
#[doc(alias = "GPIO200")]
pub type Gpio200 = crate::Reg<gpio200::Gpio200Spec>;
#[doc = "GPIO032 Control Register"]
pub mod gpio200;
#[doc = "GPIO204 (rw) register accessor: GPIO033 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio204`] module"]
#[doc(alias = "GPIO204")]
pub type Gpio204 = crate::Reg<gpio204::Gpio204Spec>;
#[doc = "GPIO033 Control Register"]
pub mod gpio204;
#[doc = "GPIO208 (rw) register accessor: GPIO034 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio208`] module"]
#[doc(alias = "GPIO208")]
pub type Gpio208 = crate::Reg<gpio208::Gpio208Spec>;
#[doc = "GPIO034 Control Register"]
pub mod gpio208;
#[doc = "GPIO20C (rw) register accessor: GPIO035 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio20c`] module"]
#[doc(alias = "GPIO20C")]
pub type Gpio20c = crate::Reg<gpio20c::Gpio20cSpec>;
#[doc = "GPIO035 Control Register"]
pub mod gpio20c;
#[doc = "GPIO210 (rw) register accessor: GPIO036 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio210`] module"]
#[doc(alias = "GPIO210")]
pub type Gpio210 = crate::Reg<gpio210::Gpio210Spec>;
#[doc = "GPIO036 Control Register"]
pub mod gpio210;
#[doc = "GPIO214 (rw) register accessor: GPIO037 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio214`] module"]
#[doc(alias = "GPIO214")]
pub type Gpio214 = crate::Reg<gpio214::Gpio214Spec>;
#[doc = "GPIO037 Control Register"]
pub mod gpio214;
#[doc = "GPIO218 (rw) register accessor: GPIO038 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio218`] module"]
#[doc(alias = "GPIO218")]
pub type Gpio218 = crate::Reg<gpio218::Gpio218Spec>;
#[doc = "GPIO038 Control Register"]
pub mod gpio218;
#[doc = "GPIO21C (rw) register accessor: GPIO039 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio21c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio21c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio21c`] module"]
#[doc(alias = "GPIO21C")]
pub type Gpio21c = crate::Reg<gpio21c::Gpio21cSpec>;
#[doc = "GPIO039 Control Register"]
pub mod gpio21c;
#[doc = "GPIO220 (rw) register accessor: GPIO040 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio220`] module"]
#[doc(alias = "GPIO220")]
pub type Gpio220 = crate::Reg<gpio220::Gpio220Spec>;
#[doc = "GPIO040 Control Register"]
pub mod gpio220;
#[doc = "GPIO224 (rw) register accessor: GPIO041 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio224`] module"]
#[doc(alias = "GPIO224")]
pub type Gpio224 = crate::Reg<gpio224::Gpio224Spec>;
#[doc = "GPIO041 Control Register"]
pub mod gpio224;
#[doc = "GPIO228 (rw) register accessor: GPIO042 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio228`] module"]
#[doc(alias = "GPIO228")]
pub type Gpio228 = crate::Reg<gpio228::Gpio228Spec>;
#[doc = "GPIO042 Control Register"]
pub mod gpio228;
#[doc = "GPIO22C (rw) register accessor: GPIO043 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio22c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio22c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio22c`] module"]
#[doc(alias = "GPIO22C")]
pub type Gpio22c = crate::Reg<gpio22c::Gpio22cSpec>;
#[doc = "GPIO043 Control Register"]
pub mod gpio22c;
#[doc = "GPIO230 (rw) register accessor: GPIO044 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio230`] module"]
#[doc(alias = "GPIO230")]
pub type Gpio230 = crate::Reg<gpio230::Gpio230Spec>;
#[doc = "GPIO044 Control Register"]
pub mod gpio230;
#[doc = "GPIO234 (rw) register accessor: GPIO045 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio234`] module"]
#[doc(alias = "GPIO234")]
pub type Gpio234 = crate::Reg<gpio234::Gpio234Spec>;
#[doc = "GPIO045 Control Register"]
pub mod gpio234;
#[doc = "GPIO238 (rw) register accessor: GPIO046 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio238`] module"]
#[doc(alias = "GPIO238")]
pub type Gpio238 = crate::Reg<gpio238::Gpio238Spec>;
#[doc = "GPIO046 Control Register"]
pub mod gpio238;
#[doc = "GPIO23C (rw) register accessor: GPIO047 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio23c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio23c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio23c`] module"]
#[doc(alias = "GPIO23C")]
pub type Gpio23c = crate::Reg<gpio23c::Gpio23cSpec>;
#[doc = "GPIO047 Control Register"]
pub mod gpio23c;
#[doc = "GPIO240 (rw) register accessor: GPIO048 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio240`] module"]
#[doc(alias = "GPIO240")]
pub type Gpio240 = crate::Reg<gpio240::Gpio240Spec>;
#[doc = "GPIO048 Control Register"]
pub mod gpio240;
#[doc = "GPIO244 (rw) register accessor: GPIO049 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio244`] module"]
#[doc(alias = "GPIO244")]
pub type Gpio244 = crate::Reg<gpio244::Gpio244Spec>;
#[doc = "GPIO049 Control Register"]
pub mod gpio244;
#[doc = "GPIO248 (rw) register accessor: GPIO050 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio248`] module"]
#[doc(alias = "GPIO248")]
pub type Gpio248 = crate::Reg<gpio248::Gpio248Spec>;
#[doc = "GPIO050 Control Register"]
pub mod gpio248;
#[doc = "GPIO24C (rw) register accessor: GPIO051 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio24c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio24c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio24c`] module"]
#[doc(alias = "GPIO24C")]
pub type Gpio24c = crate::Reg<gpio24c::Gpio24cSpec>;
#[doc = "GPIO051 Control Register"]
pub mod gpio24c;
#[doc = "GPIO250 (rw) register accessor: GPIO052 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio250`] module"]
#[doc(alias = "GPIO250")]
pub type Gpio250 = crate::Reg<gpio250::Gpio250Spec>;
#[doc = "GPIO052 Control Register"]
pub mod gpio250;
#[doc = "GPIO254 (rw) register accessor: GPIO053 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio254`] module"]
#[doc(alias = "GPIO254")]
pub type Gpio254 = crate::Reg<gpio254::Gpio254Spec>;
#[doc = "GPIO053 Control Register"]
pub mod gpio254;
#[doc = "GPIO258 (rw) register accessor: GPIO054 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio258::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio258::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio258`] module"]
#[doc(alias = "GPIO258")]
pub type Gpio258 = crate::Reg<gpio258::Gpio258Spec>;
#[doc = "GPIO054 Control Register"]
pub mod gpio258;
#[doc = "GPIO25C (rw) register accessor: GPIO055 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio25c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio25c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio25c`] module"]
#[doc(alias = "GPIO25C")]
pub type Gpio25c = crate::Reg<gpio25c::Gpio25cSpec>;
#[doc = "GPIO055 Control Register"]
pub mod gpio25c;
#[doc = "GPIO260 (rw) register accessor: GPIO056 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio260::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio260::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio260`] module"]
#[doc(alias = "GPIO260")]
pub type Gpio260 = crate::Reg<gpio260::Gpio260Spec>;
#[doc = "GPIO056 Control Register"]
pub mod gpio260;
#[doc = "GPIO264 (rw) register accessor: GPIO057 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio264::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio264::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio264`] module"]
#[doc(alias = "GPIO264")]
pub type Gpio264 = crate::Reg<gpio264::Gpio264Spec>;
#[doc = "GPIO057 Control Register"]
pub mod gpio264;
#[doc = "GPIO268 (rw) register accessor: GPIO058 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio268::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio268::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio268`] module"]
#[doc(alias = "GPIO268")]
pub type Gpio268 = crate::Reg<gpio268::Gpio268Spec>;
#[doc = "GPIO058 Control Register"]
pub mod gpio268;
#[doc = "GPIO26C (rw) register accessor: GPIO059 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio26c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio26c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio26c`] module"]
#[doc(alias = "GPIO26C")]
pub type Gpio26c = crate::Reg<gpio26c::Gpio26cSpec>;
#[doc = "GPIO059 Control Register"]
pub mod gpio26c;
#[doc = "GPIO270 (rw) register accessor: GPIO060 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio270::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio270::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio270`] module"]
#[doc(alias = "GPIO270")]
pub type Gpio270 = crate::Reg<gpio270::Gpio270Spec>;
#[doc = "GPIO060 Control Register"]
pub mod gpio270;
#[doc = "GPIO274 (rw) register accessor: GPIO061 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio274::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio274::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio274`] module"]
#[doc(alias = "GPIO274")]
pub type Gpio274 = crate::Reg<gpio274::Gpio274Spec>;
#[doc = "GPIO061 Control Register"]
pub mod gpio274;
#[doc = "GPIO278 (rw) register accessor: GPIO062 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio278::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio278::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio278`] module"]
#[doc(alias = "GPIO278")]
pub type Gpio278 = crate::Reg<gpio278::Gpio278Spec>;
#[doc = "GPIO062 Control Register"]
pub mod gpio278;
#[doc = "GPIO27C (rw) register accessor: GPIO063 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio27c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio27c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio27c`] module"]
#[doc(alias = "GPIO27C")]
pub type Gpio27c = crate::Reg<gpio27c::Gpio27cSpec>;
#[doc = "GPIO063 Control Register"]
pub mod gpio27c;
#[doc = "GPIO280 (rw) register accessor: GPIO064 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio280::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio280::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio280`] module"]
#[doc(alias = "GPIO280")]
pub type Gpio280 = crate::Reg<gpio280::Gpio280Spec>;
#[doc = "GPIO064 Control Register"]
pub mod gpio280;
#[doc = "GPIO284 (rw) register accessor: GPIO065 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio284::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio284::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio284`] module"]
#[doc(alias = "GPIO284")]
pub type Gpio284 = crate::Reg<gpio284::Gpio284Spec>;
#[doc = "GPIO065 Control Register"]
pub mod gpio284;
#[doc = "GPIO288 (rw) register accessor: GPIO066 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio288::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio288::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio288`] module"]
#[doc(alias = "GPIO288")]
pub type Gpio288 = crate::Reg<gpio288::Gpio288Spec>;
#[doc = "GPIO066 Control Register"]
pub mod gpio288;
#[doc = "GPIO28C (rw) register accessor: GPIO067 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio28c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio28c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio28c`] module"]
#[doc(alias = "GPIO28C")]
pub type Gpio28c = crate::Reg<gpio28c::Gpio28cSpec>;
#[doc = "GPIO067 Control Register"]
pub mod gpio28c;
#[doc = "GPIO290 (rw) register accessor: GPIO068 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio290::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio290::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio290`] module"]
#[doc(alias = "GPIO290")]
pub type Gpio290 = crate::Reg<gpio290::Gpio290Spec>;
#[doc = "GPIO068 Control Register"]
pub mod gpio290;
#[doc = "GPIO294 (rw) register accessor: GPIO069 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio294::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio294::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio294`] module"]
#[doc(alias = "GPIO294")]
pub type Gpio294 = crate::Reg<gpio294::Gpio294Spec>;
#[doc = "GPIO069 Control Register"]
pub mod gpio294;
#[doc = "GPIO298 (rw) register accessor: GPIO070 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio298::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio298::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio298`] module"]
#[doc(alias = "GPIO298")]
pub type Gpio298 = crate::Reg<gpio298::Gpio298Spec>;
#[doc = "GPIO070 Control Register"]
pub mod gpio298;
#[doc = "GPIO29C (rw) register accessor: GPIO071 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio29c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio29c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio29c`] module"]
#[doc(alias = "GPIO29C")]
pub type Gpio29c = crate::Reg<gpio29c::Gpio29cSpec>;
#[doc = "GPIO071 Control Register"]
pub mod gpio29c;
#[doc = "GPIO2A0 (rw) register accessor: GPIO072 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2a0`] module"]
#[doc(alias = "GPIO2A0")]
pub type Gpio2a0 = crate::Reg<gpio2a0::Gpio2a0Spec>;
#[doc = "GPIO072 Control Register"]
pub mod gpio2a0;
#[doc = "GPIO2A4 (rw) register accessor: GPIO073 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2a4`] module"]
#[doc(alias = "GPIO2A4")]
pub type Gpio2a4 = crate::Reg<gpio2a4::Gpio2a4Spec>;
#[doc = "GPIO073 Control Register"]
pub mod gpio2a4;
#[doc = "GPIO2A8 (rw) register accessor: GPIO074 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2a8`] module"]
#[doc(alias = "GPIO2A8")]
pub type Gpio2a8 = crate::Reg<gpio2a8::Gpio2a8Spec>;
#[doc = "GPIO074 Control Register"]
pub mod gpio2a8;
#[doc = "GPIO2AC (rw) register accessor: GPIO075 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2ac`] module"]
#[doc(alias = "GPIO2AC")]
pub type Gpio2ac = crate::Reg<gpio2ac::Gpio2acSpec>;
#[doc = "GPIO075 Control Register"]
pub mod gpio2ac;
#[doc = "GPIO2B0 (rw) register accessor: GPIO076 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2b0`] module"]
#[doc(alias = "GPIO2B0")]
pub type Gpio2b0 = crate::Reg<gpio2b0::Gpio2b0Spec>;
#[doc = "GPIO076 Control Register"]
pub mod gpio2b0;
#[doc = "GPIO2B4 (rw) register accessor: GPIO077 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2b4`] module"]
#[doc(alias = "GPIO2B4")]
pub type Gpio2b4 = crate::Reg<gpio2b4::Gpio2b4Spec>;
#[doc = "GPIO077 Control Register"]
pub mod gpio2b4;
#[doc = "GPIO2B8 (rw) register accessor: GPIO078 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2b8`] module"]
#[doc(alias = "GPIO2B8")]
pub type Gpio2b8 = crate::Reg<gpio2b8::Gpio2b8Spec>;
#[doc = "GPIO078 Control Register"]
pub mod gpio2b8;
#[doc = "GPIO2BC (rw) register accessor: GPIO079 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2bc`] module"]
#[doc(alias = "GPIO2BC")]
pub type Gpio2bc = crate::Reg<gpio2bc::Gpio2bcSpec>;
#[doc = "GPIO079 Control Register"]
pub mod gpio2bc;
#[doc = "GPIO2C0 (rw) register accessor: GPIO080 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2c0`] module"]
#[doc(alias = "GPIO2C0")]
pub type Gpio2c0 = crate::Reg<gpio2c0::Gpio2c0Spec>;
#[doc = "GPIO080 Control Register"]
pub mod gpio2c0;
#[doc = "GPIO2C4 (rw) register accessor: GPIO081 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2c4`] module"]
#[doc(alias = "GPIO2C4")]
pub type Gpio2c4 = crate::Reg<gpio2c4::Gpio2c4Spec>;
#[doc = "GPIO081 Control Register"]
pub mod gpio2c4;
#[doc = "GPIO2C8 (rw) register accessor: GPIO082 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2c8`] module"]
#[doc(alias = "GPIO2C8")]
pub type Gpio2c8 = crate::Reg<gpio2c8::Gpio2c8Spec>;
#[doc = "GPIO082 Control Register"]
pub mod gpio2c8;
#[doc = "GPIO2CC (rw) register accessor: GPIO083 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2cc`] module"]
#[doc(alias = "GPIO2CC")]
pub type Gpio2cc = crate::Reg<gpio2cc::Gpio2ccSpec>;
#[doc = "GPIO083 Control Register"]
pub mod gpio2cc;
#[doc = "GPIO2D0 (rw) register accessor: GPIO084 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2d0`] module"]
#[doc(alias = "GPIO2D0")]
pub type Gpio2d0 = crate::Reg<gpio2d0::Gpio2d0Spec>;
#[doc = "GPIO084 Control Register"]
pub mod gpio2d0;
#[doc = "GPIO2D4 (rw) register accessor: GPIO085 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2d4`] module"]
#[doc(alias = "GPIO2D4")]
pub type Gpio2d4 = crate::Reg<gpio2d4::Gpio2d4Spec>;
#[doc = "GPIO085 Control Register"]
pub mod gpio2d4;
#[doc = "GPIO2D8 (rw) register accessor: GPIO086 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2d8`] module"]
#[doc(alias = "GPIO2D8")]
pub type Gpio2d8 = crate::Reg<gpio2d8::Gpio2d8Spec>;
#[doc = "GPIO086 Control Register"]
pub mod gpio2d8;
#[doc = "GPIO2DC (rw) register accessor: GPIO087 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2dc`] module"]
#[doc(alias = "GPIO2DC")]
pub type Gpio2dc = crate::Reg<gpio2dc::Gpio2dcSpec>;
#[doc = "GPIO087 Control Register"]
pub mod gpio2dc;
#[doc = "GPIO2E0 (rw) register accessor: GPIO088 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2e0`] module"]
#[doc(alias = "GPIO2E0")]
pub type Gpio2e0 = crate::Reg<gpio2e0::Gpio2e0Spec>;
#[doc = "GPIO088 Control Register"]
pub mod gpio2e0;
#[doc = "GPIO2E4 (rw) register accessor: GPIO089 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2e4`] module"]
#[doc(alias = "GPIO2E4")]
pub type Gpio2e4 = crate::Reg<gpio2e4::Gpio2e4Spec>;
#[doc = "GPIO089 Control Register"]
pub mod gpio2e4;
#[doc = "GPIO2E8 (rw) register accessor: GPIO090 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2e8`] module"]
#[doc(alias = "GPIO2E8")]
pub type Gpio2e8 = crate::Reg<gpio2e8::Gpio2e8Spec>;
#[doc = "GPIO090 Control Register"]
pub mod gpio2e8;
#[doc = "GPIO2EC (rw) register accessor: GPIO091 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2ec`] module"]
#[doc(alias = "GPIO2EC")]
pub type Gpio2ec = crate::Reg<gpio2ec::Gpio2ecSpec>;
#[doc = "GPIO091 Control Register"]
pub mod gpio2ec;
#[doc = "GPIO2F0 (rw) register accessor: GPIO092 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2f0`] module"]
#[doc(alias = "GPIO2F0")]
pub type Gpio2f0 = crate::Reg<gpio2f0::Gpio2f0Spec>;
#[doc = "GPIO092 Control Register"]
pub mod gpio2f0;
#[doc = "GPIO2F4 (rw) register accessor: GPIO093 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2f4`] module"]
#[doc(alias = "GPIO2F4")]
pub type Gpio2f4 = crate::Reg<gpio2f4::Gpio2f4Spec>;
#[doc = "GPIO093 Control Register"]
pub mod gpio2f4;
#[doc = "GPIO2F8 (rw) register accessor: GPIO094 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2f8`] module"]
#[doc(alias = "GPIO2F8")]
pub type Gpio2f8 = crate::Reg<gpio2f8::Gpio2f8Spec>;
#[doc = "GPIO094 Control Register"]
pub mod gpio2f8;
#[doc = "GPIO2FC (rw) register accessor: GPIO095 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2fc`] module"]
#[doc(alias = "GPIO2FC")]
pub type Gpio2fc = crate::Reg<gpio2fc::Gpio2fcSpec>;
#[doc = "GPIO095 Control Register"]
pub mod gpio2fc;
#[doc = "GPIO300 (rw) register accessor: GPIO096 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio300::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio300::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio300`] module"]
#[doc(alias = "GPIO300")]
pub type Gpio300 = crate::Reg<gpio300::Gpio300Spec>;
#[doc = "GPIO096 Control Register"]
pub mod gpio300;
#[doc = "GPIO304 (rw) register accessor: GPIO097 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio304::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio304::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio304`] module"]
#[doc(alias = "GPIO304")]
pub type Gpio304 = crate::Reg<gpio304::Gpio304Spec>;
#[doc = "GPIO097 Control Register"]
pub mod gpio304;
#[doc = "GPIO308 (rw) register accessor: GPIO098 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio308::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio308::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio308`] module"]
#[doc(alias = "GPIO308")]
pub type Gpio308 = crate::Reg<gpio308::Gpio308Spec>;
#[doc = "GPIO098 Control Register"]
pub mod gpio308;
#[doc = "GPIO30C (rw) register accessor: GPIO099 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio30c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio30c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio30c`] module"]
#[doc(alias = "GPIO30C")]
pub type Gpio30c = crate::Reg<gpio30c::Gpio30cSpec>;
#[doc = "GPIO099 Control Register"]
pub mod gpio30c;
#[doc = "GPIO310 (rw) register accessor: GPIO100 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio310`] module"]
#[doc(alias = "GPIO310")]
pub type Gpio310 = crate::Reg<gpio310::Gpio310Spec>;
#[doc = "GPIO100 Control Register"]
pub mod gpio310;
#[doc = "GPIO314 (rw) register accessor: GPIO101 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio314`] module"]
#[doc(alias = "GPIO314")]
pub type Gpio314 = crate::Reg<gpio314::Gpio314Spec>;
#[doc = "GPIO101 Control Register"]
pub mod gpio314;
#[doc = "GPIO318 (rw) register accessor: GPIO102 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio318::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio318::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio318`] module"]
#[doc(alias = "GPIO318")]
pub type Gpio318 = crate::Reg<gpio318::Gpio318Spec>;
#[doc = "GPIO102 Control Register"]
pub mod gpio318;
#[doc = "GPIO31C (rw) register accessor: GPIO103 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio31c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio31c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio31c`] module"]
#[doc(alias = "GPIO31C")]
pub type Gpio31c = crate::Reg<gpio31c::Gpio31cSpec>;
#[doc = "GPIO103 Control Register"]
pub mod gpio31c;
#[doc = "GPIO320 (rw) register accessor: GPIO104 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio320::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio320::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio320`] module"]
#[doc(alias = "GPIO320")]
pub type Gpio320 = crate::Reg<gpio320::Gpio320Spec>;
#[doc = "GPIO104 Control Register"]
pub mod gpio320;
#[doc = "GPIO324 (rw) register accessor: GPIO105 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio324::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio324::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio324`] module"]
#[doc(alias = "GPIO324")]
pub type Gpio324 = crate::Reg<gpio324::Gpio324Spec>;
#[doc = "GPIO105 Control Register"]
pub mod gpio324;
#[doc = "GPIO328 (rw) register accessor: GPIO106 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio328::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio328::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio328`] module"]
#[doc(alias = "GPIO328")]
pub type Gpio328 = crate::Reg<gpio328::Gpio328Spec>;
#[doc = "GPIO106 Control Register"]
pub mod gpio328;
#[doc = "GPIO32C (rw) register accessor: GPIO107 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio32c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio32c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio32c`] module"]
#[doc(alias = "GPIO32C")]
pub type Gpio32c = crate::Reg<gpio32c::Gpio32cSpec>;
#[doc = "GPIO107 Control Register"]
pub mod gpio32c;
#[doc = "GPIO330 (rw) register accessor: GPIO108 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio330::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio330::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio330`] module"]
#[doc(alias = "GPIO330")]
pub type Gpio330 = crate::Reg<gpio330::Gpio330Spec>;
#[doc = "GPIO108 Control Register"]
pub mod gpio330;
#[doc = "GPIO334 (rw) register accessor: GPIO109 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio334::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio334::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio334`] module"]
#[doc(alias = "GPIO334")]
pub type Gpio334 = crate::Reg<gpio334::Gpio334Spec>;
#[doc = "GPIO109 Control Register"]
pub mod gpio334;
#[doc = "GPIO338 (rw) register accessor: GPIO110 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio338::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio338::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio338`] module"]
#[doc(alias = "GPIO338")]
pub type Gpio338 = crate::Reg<gpio338::Gpio338Spec>;
#[doc = "GPIO110 Control Register"]
pub mod gpio338;
#[doc = "GPIO33C (rw) register accessor: GPIO111 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio33c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio33c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio33c`] module"]
#[doc(alias = "GPIO33C")]
pub type Gpio33c = crate::Reg<gpio33c::Gpio33cSpec>;
#[doc = "GPIO111 Control Register"]
pub mod gpio33c;
#[doc = "GPIO340 (rw) register accessor: GPIO112 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio340::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio340::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio340`] module"]
#[doc(alias = "GPIO340")]
pub type Gpio340 = crate::Reg<gpio340::Gpio340Spec>;
#[doc = "GPIO112 Control Register"]
pub mod gpio340;
#[doc = "GPIO344 (rw) register accessor: GPIO113 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio344::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio344::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio344`] module"]
#[doc(alias = "GPIO344")]
pub type Gpio344 = crate::Reg<gpio344::Gpio344Spec>;
#[doc = "GPIO113 Control Register"]
pub mod gpio344;
#[doc = "GPIO348 (rw) register accessor: GPIO114 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio348::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio348::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio348`] module"]
#[doc(alias = "GPIO348")]
pub type Gpio348 = crate::Reg<gpio348::Gpio348Spec>;
#[doc = "GPIO114 Control Register"]
pub mod gpio348;
#[doc = "GPIO34C (rw) register accessor: GPIO115 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio34c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio34c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio34c`] module"]
#[doc(alias = "GPIO34C")]
pub type Gpio34c = crate::Reg<gpio34c::Gpio34cSpec>;
#[doc = "GPIO115 Control Register"]
pub mod gpio34c;
#[doc = "GPIO350 (rw) register accessor: GPIO116 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio350::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio350::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio350`] module"]
#[doc(alias = "GPIO350")]
pub type Gpio350 = crate::Reg<gpio350::Gpio350Spec>;
#[doc = "GPIO116 Control Register"]
pub mod gpio350;
#[doc = "GPIO354 (rw) register accessor: GPIO117 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio354::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio354::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio354`] module"]
#[doc(alias = "GPIO354")]
pub type Gpio354 = crate::Reg<gpio354::Gpio354Spec>;
#[doc = "GPIO117 Control Register"]
pub mod gpio354;
#[doc = "GPIO358 (rw) register accessor: GPIO118 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio358::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio358::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio358`] module"]
#[doc(alias = "GPIO358")]
pub type Gpio358 = crate::Reg<gpio358::Gpio358Spec>;
#[doc = "GPIO118 Control Register"]
pub mod gpio358;
#[doc = "GPIO35C (rw) register accessor: GPIO119 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio35c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio35c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio35c`] module"]
#[doc(alias = "GPIO35C")]
pub type Gpio35c = crate::Reg<gpio35c::Gpio35cSpec>;
#[doc = "GPIO119 Control Register"]
pub mod gpio35c;
#[doc = "GPIO360 (rw) register accessor: GPIO120 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio360::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio360::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio360`] module"]
#[doc(alias = "GPIO360")]
pub type Gpio360 = crate::Reg<gpio360::Gpio360Spec>;
#[doc = "GPIO120 Control Register"]
pub mod gpio360;
#[doc = "GPIO364 (rw) register accessor: GPIO121 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio364::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio364::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio364`] module"]
#[doc(alias = "GPIO364")]
pub type Gpio364 = crate::Reg<gpio364::Gpio364Spec>;
#[doc = "GPIO121 Control Register"]
pub mod gpio364;
#[doc = "GPIO368 (rw) register accessor: GPIO122 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio368::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio368::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio368`] module"]
#[doc(alias = "GPIO368")]
pub type Gpio368 = crate::Reg<gpio368::Gpio368Spec>;
#[doc = "GPIO122 Control Register"]
pub mod gpio368;
#[doc = "GPIO36C (rw) register accessor: GPIO123 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio36c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio36c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio36c`] module"]
#[doc(alias = "GPIO36C")]
pub type Gpio36c = crate::Reg<gpio36c::Gpio36cSpec>;
#[doc = "GPIO123 Control Register"]
pub mod gpio36c;
#[doc = "GPIO370 (rw) register accessor: GPIO124 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio370::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio370::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio370`] module"]
#[doc(alias = "GPIO370")]
pub type Gpio370 = crate::Reg<gpio370::Gpio370Spec>;
#[doc = "GPIO124 Control Register"]
pub mod gpio370;
#[doc = "GPIO374 (rw) register accessor: GPIO125 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio374::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio374::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio374`] module"]
#[doc(alias = "GPIO374")]
pub type Gpio374 = crate::Reg<gpio374::Gpio374Spec>;
#[doc = "GPIO125 Control Register"]
pub mod gpio374;
#[doc = "GPIO378 (rw) register accessor: GPIO126 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio378::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio378::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio378`] module"]
#[doc(alias = "GPIO378")]
pub type Gpio378 = crate::Reg<gpio378::Gpio378Spec>;
#[doc = "GPIO126 Control Register"]
pub mod gpio378;
#[doc = "GPIO37C (rw) register accessor: GPIO127 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio37c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio37c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio37c`] module"]
#[doc(alias = "GPIO37C")]
pub type Gpio37c = crate::Reg<gpio37c::Gpio37cSpec>;
#[doc = "GPIO127 Control Register"]
pub mod gpio37c;
#[doc = "GPIO380 (rw) register accessor: GPIO128 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio380::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio380::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio380`] module"]
#[doc(alias = "GPIO380")]
pub type Gpio380 = crate::Reg<gpio380::Gpio380Spec>;
#[doc = "GPIO128 Control Register"]
pub mod gpio380;
#[doc = "GPIO384 (rw) register accessor: GPIO129 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio384::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio384::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio384`] module"]
#[doc(alias = "GPIO384")]
pub type Gpio384 = crate::Reg<gpio384::Gpio384Spec>;
#[doc = "GPIO129 Control Register"]
pub mod gpio384;
#[doc = "GPIO388 (rw) register accessor: GPIO130 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio388::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio388::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio388`] module"]
#[doc(alias = "GPIO388")]
pub type Gpio388 = crate::Reg<gpio388::Gpio388Spec>;
#[doc = "GPIO130 Control Register"]
pub mod gpio388;
#[doc = "GPIO38C (rw) register accessor: GPIO131 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio38c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio38c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio38c`] module"]
#[doc(alias = "GPIO38C")]
pub type Gpio38c = crate::Reg<gpio38c::Gpio38cSpec>;
#[doc = "GPIO131 Control Register"]
pub mod gpio38c;
#[doc = "GPIO390 (rw) register accessor: GPIO132 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio390::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio390::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio390`] module"]
#[doc(alias = "GPIO390")]
pub type Gpio390 = crate::Reg<gpio390::Gpio390Spec>;
#[doc = "GPIO132 Control Register"]
pub mod gpio390;
#[doc = "GPIO394 (rw) register accessor: GPIO133 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio394::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio394::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio394`] module"]
#[doc(alias = "GPIO394")]
pub type Gpio394 = crate::Reg<gpio394::Gpio394Spec>;
#[doc = "GPIO133 Control Register"]
pub mod gpio394;
#[doc = "GPIO398 (rw) register accessor: GPIO134 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio398::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio398::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio398`] module"]
#[doc(alias = "GPIO398")]
pub type Gpio398 = crate::Reg<gpio398::Gpio398Spec>;
#[doc = "GPIO134 Control Register"]
pub mod gpio398;
#[doc = "GPIO39C (rw) register accessor: GPIO135 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio39c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio39c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio39c`] module"]
#[doc(alias = "GPIO39C")]
pub type Gpio39c = crate::Reg<gpio39c::Gpio39cSpec>;
#[doc = "GPIO135 Control Register"]
pub mod gpio39c;
#[doc = "GPIO3A0 (rw) register accessor: GPIO136 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3a0`] module"]
#[doc(alias = "GPIO3A0")]
pub type Gpio3a0 = crate::Reg<gpio3a0::Gpio3a0Spec>;
#[doc = "GPIO136 Control Register"]
pub mod gpio3a0;
#[doc = "GPIO3A4 (rw) register accessor: GPIO137 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3a4`] module"]
#[doc(alias = "GPIO3A4")]
pub type Gpio3a4 = crate::Reg<gpio3a4::Gpio3a4Spec>;
#[doc = "GPIO137 Control Register"]
pub mod gpio3a4;
#[doc = "GPIO3A8 (rw) register accessor: GPIO138 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3a8`] module"]
#[doc(alias = "GPIO3A8")]
pub type Gpio3a8 = crate::Reg<gpio3a8::Gpio3a8Spec>;
#[doc = "GPIO138 Control Register"]
pub mod gpio3a8;
#[doc = "GPIO3AC (rw) register accessor: GPIO139 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3ac`] module"]
#[doc(alias = "GPIO3AC")]
pub type Gpio3ac = crate::Reg<gpio3ac::Gpio3acSpec>;
#[doc = "GPIO139 Control Register"]
pub mod gpio3ac;
#[doc = "GPIO3B0 (rw) register accessor: GPIO140 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3b0`] module"]
#[doc(alias = "GPIO3B0")]
pub type Gpio3b0 = crate::Reg<gpio3b0::Gpio3b0Spec>;
#[doc = "GPIO140 Control Register"]
pub mod gpio3b0;
#[doc = "GPIO3B4 (rw) register accessor: GPIO141 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3b4`] module"]
#[doc(alias = "GPIO3B4")]
pub type Gpio3b4 = crate::Reg<gpio3b4::Gpio3b4Spec>;
#[doc = "GPIO141 Control Register"]
pub mod gpio3b4;
#[doc = "GPIO3B8 (rw) register accessor: GPIO142 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3b8`] module"]
#[doc(alias = "GPIO3B8")]
pub type Gpio3b8 = crate::Reg<gpio3b8::Gpio3b8Spec>;
#[doc = "GPIO142 Control Register"]
pub mod gpio3b8;
#[doc = "GPIO3BC (rw) register accessor: GPIO143 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3bc`] module"]
#[doc(alias = "GPIO3BC")]
pub type Gpio3bc = crate::Reg<gpio3bc::Gpio3bcSpec>;
#[doc = "GPIO143 Control Register"]
pub mod gpio3bc;
#[doc = "GPIO3C0 (rw) register accessor: GPIO144 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3c0`] module"]
#[doc(alias = "GPIO3C0")]
pub type Gpio3c0 = crate::Reg<gpio3c0::Gpio3c0Spec>;
#[doc = "GPIO144 Control Register"]
pub mod gpio3c0;
#[doc = "GPIO3C4 (rw) register accessor: GPIO145 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3c4`] module"]
#[doc(alias = "GPIO3C4")]
pub type Gpio3c4 = crate::Reg<gpio3c4::Gpio3c4Spec>;
#[doc = "GPIO145 Control Register"]
pub mod gpio3c4;
#[doc = "GPIO3C8 (rw) register accessor: GPIO146 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3c8`] module"]
#[doc(alias = "GPIO3C8")]
pub type Gpio3c8 = crate::Reg<gpio3c8::Gpio3c8Spec>;
#[doc = "GPIO146 Control Register"]
pub mod gpio3c8;
#[doc = "GPIO3CC (rw) register accessor: GPIO147 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3cc`] module"]
#[doc(alias = "GPIO3CC")]
pub type Gpio3cc = crate::Reg<gpio3cc::Gpio3ccSpec>;
#[doc = "GPIO147 Control Register"]
pub mod gpio3cc;
#[doc = "GPIO3D0 (rw) register accessor: GPIO148 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3d0`] module"]
#[doc(alias = "GPIO3D0")]
pub type Gpio3d0 = crate::Reg<gpio3d0::Gpio3d0Spec>;
#[doc = "GPIO148 Control Register"]
pub mod gpio3d0;
#[doc = "GPIO3D4 (rw) register accessor: GPIO149 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3d4`] module"]
#[doc(alias = "GPIO3D4")]
pub type Gpio3d4 = crate::Reg<gpio3d4::Gpio3d4Spec>;
#[doc = "GPIO149 Control Register"]
pub mod gpio3d4;
#[doc = "GPIO3D8 (rw) register accessor: GPIO150 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3d8`] module"]
#[doc(alias = "GPIO3D8")]
pub type Gpio3d8 = crate::Reg<gpio3d8::Gpio3d8Spec>;
#[doc = "GPIO150 Control Register"]
pub mod gpio3d8;
#[doc = "GPIO3DC (rw) register accessor: GPIO151 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3dc`] module"]
#[doc(alias = "GPIO3DC")]
pub type Gpio3dc = crate::Reg<gpio3dc::Gpio3dcSpec>;
#[doc = "GPIO151 Control Register"]
pub mod gpio3dc;
#[doc = "GPIO3E0 (rw) register accessor: GPIO152 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3e0`] module"]
#[doc(alias = "GPIO3E0")]
pub type Gpio3e0 = crate::Reg<gpio3e0::Gpio3e0Spec>;
#[doc = "GPIO152 Control Register"]
pub mod gpio3e0;
#[doc = "GPIO3E4 (rw) register accessor: GPIO153 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3e4`] module"]
#[doc(alias = "GPIO3E4")]
pub type Gpio3e4 = crate::Reg<gpio3e4::Gpio3e4Spec>;
#[doc = "GPIO153 Control Register"]
pub mod gpio3e4;
#[doc = "GPIO3E8 (rw) register accessor: GPIO154 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3e8`] module"]
#[doc(alias = "GPIO3E8")]
pub type Gpio3e8 = crate::Reg<gpio3e8::Gpio3e8Spec>;
#[doc = "GPIO154 Control Register"]
pub mod gpio3e8;
#[doc = "GPIO3EC (rw) register accessor: GPIO155 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3ec`] module"]
#[doc(alias = "GPIO3EC")]
pub type Gpio3ec = crate::Reg<gpio3ec::Gpio3ecSpec>;
#[doc = "GPIO155 Control Register"]
pub mod gpio3ec;
#[doc = "GPIO3F0 (rw) register accessor: GPIO156 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3f0`] module"]
#[doc(alias = "GPIO3F0")]
pub type Gpio3f0 = crate::Reg<gpio3f0::Gpio3f0Spec>;
#[doc = "GPIO156 Control Register"]
pub mod gpio3f0;
#[doc = "GPIO3F4 (rw) register accessor: GPIO157 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3f4`] module"]
#[doc(alias = "GPIO3F4")]
pub type Gpio3f4 = crate::Reg<gpio3f4::Gpio3f4Spec>;
#[doc = "GPIO157 Control Register"]
pub mod gpio3f4;
#[doc = "GPIO3F8 (rw) register accessor: GPIO158 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3f8`] module"]
#[doc(alias = "GPIO3F8")]
pub type Gpio3f8 = crate::Reg<gpio3f8::Gpio3f8Spec>;
#[doc = "GPIO158 Control Register"]
pub mod gpio3f8;
#[doc = "GPIO3FC (rw) register accessor: GPIO159 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3fc`] module"]
#[doc(alias = "GPIO3FC")]
pub type Gpio3fc = crate::Reg<gpio3fc::Gpio3fcSpec>;
#[doc = "GPIO159 Control Register"]
pub mod gpio3fc;
#[doc = "GPIO400 (rw) register accessor: GPIO160 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio400::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio400::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio400`] module"]
#[doc(alias = "GPIO400")]
pub type Gpio400 = crate::Reg<gpio400::Gpio400Spec>;
#[doc = "GPIO160 Control Register"]
pub mod gpio400;
#[doc = "GPIO404 (rw) register accessor: GPIO161 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio404::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio404::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio404`] module"]
#[doc(alias = "GPIO404")]
pub type Gpio404 = crate::Reg<gpio404::Gpio404Spec>;
#[doc = "GPIO161 Control Register"]
pub mod gpio404;
#[doc = "GPIO408 (rw) register accessor: GPIO162 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio408::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio408::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio408`] module"]
#[doc(alias = "GPIO408")]
pub type Gpio408 = crate::Reg<gpio408::Gpio408Spec>;
#[doc = "GPIO162 Control Register"]
pub mod gpio408;
#[doc = "GPIO40C (rw) register accessor: GPIO163 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio40c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio40c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio40c`] module"]
#[doc(alias = "GPIO40C")]
pub type Gpio40c = crate::Reg<gpio40c::Gpio40cSpec>;
#[doc = "GPIO163 Control Register"]
pub mod gpio40c;
#[doc = "GPIO410 (rw) register accessor: GPIO164 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio410::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio410::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio410`] module"]
#[doc(alias = "GPIO410")]
pub type Gpio410 = crate::Reg<gpio410::Gpio410Spec>;
#[doc = "GPIO164 Control Register"]
pub mod gpio410;
#[doc = "GPIO414 (rw) register accessor: GPIO165 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio414::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio414::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio414`] module"]
#[doc(alias = "GPIO414")]
pub type Gpio414 = crate::Reg<gpio414::Gpio414Spec>;
#[doc = "GPIO165 Control Register"]
pub mod gpio414;
#[doc = "GPIO418 (rw) register accessor: GPIO166 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio418::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio418::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio418`] module"]
#[doc(alias = "GPIO418")]
pub type Gpio418 = crate::Reg<gpio418::Gpio418Spec>;
#[doc = "GPIO166 Control Register"]
pub mod gpio418;
#[doc = "GPIO41C (rw) register accessor: GPIO167 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio41c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio41c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio41c`] module"]
#[doc(alias = "GPIO41C")]
pub type Gpio41c = crate::Reg<gpio41c::Gpio41cSpec>;
#[doc = "GPIO167 Control Register"]
pub mod gpio41c;
#[doc = "GPIO420 (rw) register accessor: GPIO168 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio420::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio420::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio420`] module"]
#[doc(alias = "GPIO420")]
pub type Gpio420 = crate::Reg<gpio420::Gpio420Spec>;
#[doc = "GPIO168 Control Register"]
pub mod gpio420;
#[doc = "GPIO424 (rw) register accessor: GPIO169 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio424::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio424::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio424`] module"]
#[doc(alias = "GPIO424")]
pub type Gpio424 = crate::Reg<gpio424::Gpio424Spec>;
#[doc = "GPIO169 Control Register"]
pub mod gpio424;
#[doc = "GPIO428 (rw) register accessor: GPIO170 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio428::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio428::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio428`] module"]
#[doc(alias = "GPIO428")]
pub type Gpio428 = crate::Reg<gpio428::Gpio428Spec>;
#[doc = "GPIO170 Control Register"]
pub mod gpio428;
#[doc = "GPIO42C (rw) register accessor: GPIO171 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio42c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio42c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio42c`] module"]
#[doc(alias = "GPIO42C")]
pub type Gpio42c = crate::Reg<gpio42c::Gpio42cSpec>;
#[doc = "GPIO171 Control Register"]
pub mod gpio42c;
#[doc = "GPIO430 (rw) register accessor: GPIO172 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio430::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio430::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio430`] module"]
#[doc(alias = "GPIO430")]
pub type Gpio430 = crate::Reg<gpio430::Gpio430Spec>;
#[doc = "GPIO172 Control Register"]
pub mod gpio430;
#[doc = "GPIO434 (rw) register accessor: GPIO173 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio434::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio434::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio434`] module"]
#[doc(alias = "GPIO434")]
pub type Gpio434 = crate::Reg<gpio434::Gpio434Spec>;
#[doc = "GPIO173 Control Register"]
pub mod gpio434;
#[doc = "GPIO438 (rw) register accessor: GPIO174 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio438::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio438::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio438`] module"]
#[doc(alias = "GPIO438")]
pub type Gpio438 = crate::Reg<gpio438::Gpio438Spec>;
#[doc = "GPIO174 Control Register"]
pub mod gpio438;
#[doc = "GPIO43C (rw) register accessor: GPIO175 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio43c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio43c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio43c`] module"]
#[doc(alias = "GPIO43C")]
pub type Gpio43c = crate::Reg<gpio43c::Gpio43cSpec>;
#[doc = "GPIO175 Control Register"]
pub mod gpio43c;
#[doc = "GPIO440 (rw) register accessor: GPIO176 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio440::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio440::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio440`] module"]
#[doc(alias = "GPIO440")]
pub type Gpio440 = crate::Reg<gpio440::Gpio440Spec>;
#[doc = "GPIO176 Control Register"]
pub mod gpio440;
#[doc = "GPIO444 (rw) register accessor: GPIO177 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio444::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio444::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio444`] module"]
#[doc(alias = "GPIO444")]
pub type Gpio444 = crate::Reg<gpio444::Gpio444Spec>;
#[doc = "GPIO177 Control Register"]
pub mod gpio444;
#[doc = "GPIO448 (rw) register accessor: GPIO178 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio448::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio448::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio448`] module"]
#[doc(alias = "GPIO448")]
pub type Gpio448 = crate::Reg<gpio448::Gpio448Spec>;
#[doc = "GPIO178 Control Register"]
pub mod gpio448;
#[doc = "GPIO44C (rw) register accessor: GPIO179 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio44c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio44c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio44c`] module"]
#[doc(alias = "GPIO44C")]
pub type Gpio44c = crate::Reg<gpio44c::Gpio44cSpec>;
#[doc = "GPIO179 Control Register"]
pub mod gpio44c;
#[doc = "GPIO450 (rw) register accessor: GPIO180 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio450::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio450::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio450`] module"]
#[doc(alias = "GPIO450")]
pub type Gpio450 = crate::Reg<gpio450::Gpio450Spec>;
#[doc = "GPIO180 Control Register"]
pub mod gpio450;
#[doc = "GPIO454 (rw) register accessor: GPIO181 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio454::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio454::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio454`] module"]
#[doc(alias = "GPIO454")]
pub type Gpio454 = crate::Reg<gpio454::Gpio454Spec>;
#[doc = "GPIO181 Control Register"]
pub mod gpio454;
#[doc = "GPIO458 (rw) register accessor: GPIO182 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio458::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio458::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio458`] module"]
#[doc(alias = "GPIO458")]
pub type Gpio458 = crate::Reg<gpio458::Gpio458Spec>;
#[doc = "GPIO182 Control Register"]
pub mod gpio458;
#[doc = "GPIO45C (rw) register accessor: GPIO183 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio45c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio45c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio45c`] module"]
#[doc(alias = "GPIO45C")]
pub type Gpio45c = crate::Reg<gpio45c::Gpio45cSpec>;
#[doc = "GPIO183 Control Register"]
pub mod gpio45c;
#[doc = "GPIO460 (rw) register accessor: GPIO184 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio460::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio460::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio460`] module"]
#[doc(alias = "GPIO460")]
pub type Gpio460 = crate::Reg<gpio460::Gpio460Spec>;
#[doc = "GPIO184 Control Register"]
pub mod gpio460;
#[doc = "GPIO464 (rw) register accessor: GPIO185 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio464::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio464::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio464`] module"]
#[doc(alias = "GPIO464")]
pub type Gpio464 = crate::Reg<gpio464::Gpio464Spec>;
#[doc = "GPIO185 Control Register"]
pub mod gpio464;
#[doc = "GPIO468 (rw) register accessor: GPIO186 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio468::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio468::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio468`] module"]
#[doc(alias = "GPIO468")]
pub type Gpio468 = crate::Reg<gpio468::Gpio468Spec>;
#[doc = "GPIO186 Control Register"]
pub mod gpio468;
#[doc = "GPIO46C (rw) register accessor: GPIO187 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio46c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio46c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio46c`] module"]
#[doc(alias = "GPIO46C")]
pub type Gpio46c = crate::Reg<gpio46c::Gpio46cSpec>;
#[doc = "GPIO187 Control Register"]
pub mod gpio46c;
#[doc = "GPIO470 (rw) register accessor: GPIO188 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio470::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio470::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio470`] module"]
#[doc(alias = "GPIO470")]
pub type Gpio470 = crate::Reg<gpio470::Gpio470Spec>;
#[doc = "GPIO188 Control Register"]
pub mod gpio470;
#[doc = "GPIO474 (rw) register accessor: GPIO189 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio474::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio474::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio474`] module"]
#[doc(alias = "GPIO474")]
pub type Gpio474 = crate::Reg<gpio474::Gpio474Spec>;
#[doc = "GPIO189 Control Register"]
pub mod gpio474;
#[doc = "GPIO478 (rw) register accessor: GPIO190 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio478::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio478::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio478`] module"]
#[doc(alias = "GPIO478")]
pub type Gpio478 = crate::Reg<gpio478::Gpio478Spec>;
#[doc = "GPIO190 Control Register"]
pub mod gpio478;
#[doc = "GPIO47C (rw) register accessor: GPIO191 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio47c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio47c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio47c`] module"]
#[doc(alias = "GPIO47C")]
pub type Gpio47c = crate::Reg<gpio47c::Gpio47cSpec>;
#[doc = "GPIO191 Control Register"]
pub mod gpio47c;
#[doc = "GPIO480 (rw) register accessor: GPIO192 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio480::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio480::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio480`] module"]
#[doc(alias = "GPIO480")]
pub type Gpio480 = crate::Reg<gpio480::Gpio480Spec>;
#[doc = "GPIO192 Control Register"]
pub mod gpio480;
#[doc = "GPIO484 (rw) register accessor: GPIO193 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio484::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio484::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio484`] module"]
#[doc(alias = "GPIO484")]
pub type Gpio484 = crate::Reg<gpio484::Gpio484Spec>;
#[doc = "GPIO193 Control Register"]
pub mod gpio484;
#[doc = "GPIO800 (rw) register accessor: Master Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio800::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio800::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio800`] module"]
#[doc(alias = "GPIO800")]
pub type Gpio800 = crate::Reg<gpio800::Gpio800Spec>;
#[doc = "Master Control Register \\#1"]
pub mod gpio800;
#[doc = "GPIO804 (rw) register accessor: Master Control Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio804::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio804::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio804`] module"]
#[doc(alias = "GPIO804")]
pub type Gpio804 = crate::Reg<gpio804::Gpio804Spec>;
#[doc = "Master Control Register \\#2"]
pub mod gpio804;
#[doc = "GPIO808 (rw) register accessor: GPIO Global Privilege Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio808::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio808::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio808`] module"]
#[doc(alias = "GPIO808")]
pub type Gpio808 = crate::Reg<gpio808::Gpio808Spec>;
#[doc = "GPIO Global Privilege Control Register"]
pub mod gpio808;
#[doc = "GPIO810 (rw) register accessor: GPIO Write Privilege Control Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio810::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio810::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio810`] module"]
#[doc(alias = "GPIO810")]
pub type Gpio810 = crate::Reg<gpio810::Gpio810Spec>;
#[doc = "GPIO Write Privilege Control Register \\#0"]
pub mod gpio810;
#[doc = "GPIO814 (rw) register accessor: GPIO Write Privilege Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio814::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio814::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio814`] module"]
#[doc(alias = "GPIO814")]
pub type Gpio814 = crate::Reg<gpio814::Gpio814Spec>;
#[doc = "GPIO Write Privilege Control Register \\#1"]
pub mod gpio814;
#[doc = "GPIO818 (rw) register accessor: GPIO Write Privilege Control Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio818::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio818::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio818`] module"]
#[doc(alias = "GPIO818")]
pub type Gpio818 = crate::Reg<gpio818::Gpio818Spec>;
#[doc = "GPIO Write Privilege Control Register \\#2"]
pub mod gpio818;
#[doc = "GPIO81C (rw) register accessor: GPIO Write Privilege Control Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio81c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio81c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio81c`] module"]
#[doc(alias = "GPIO81C")]
pub type Gpio81c = crate::Reg<gpio81c::Gpio81cSpec>;
#[doc = "GPIO Write Privilege Control Register \\#3"]
pub mod gpio81c;
#[doc = "GPIO820 (rw) register accessor: GPIO Write Privilege Control Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio820::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio820::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio820`] module"]
#[doc(alias = "GPIO820")]
pub type Gpio820 = crate::Reg<gpio820::Gpio820Spec>;
#[doc = "GPIO Write Privilege Control Register \\#4"]
pub mod gpio820;
#[doc = "GPIO824 (rw) register accessor: GPIO Write Privilege Control Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio824::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio824::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio824`] module"]
#[doc(alias = "GPIO824")]
pub type Gpio824 = crate::Reg<gpio824::Gpio824Spec>;
#[doc = "GPIO Write Privilege Control Register \\#5"]
pub mod gpio824;
#[doc = "GPIO828 (rw) register accessor: GPIO Write Privilege Control Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio828::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio828::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio828`] module"]
#[doc(alias = "GPIO828")]
pub type Gpio828 = crate::Reg<gpio828::Gpio828Spec>;
#[doc = "GPIO Write Privilege Control Register \\#6"]
pub mod gpio828;
#[doc = "GPIO82C (rw) register accessor: GPIO Write Privilege Control Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio82c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio82c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio82c`] module"]
#[doc(alias = "GPIO82C")]
pub type Gpio82c = crate::Reg<gpio82c::Gpio82cSpec>;
#[doc = "GPIO Write Privilege Control Register \\#7"]
pub mod gpio82c;
#[doc = "GPIO830 (rw) register accessor: GPIO Write Privilege Control Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio830::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio830::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio830`] module"]
#[doc(alias = "GPIO830")]
pub type Gpio830 = crate::Reg<gpio830::Gpio830Spec>;
#[doc = "GPIO Write Privilege Control Register \\#8"]
pub mod gpio830;
#[doc = "GPIO834 (rw) register accessor: GPIO Write Privilege Control Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio834::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio834::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio834`] module"]
#[doc(alias = "GPIO834")]
pub type Gpio834 = crate::Reg<gpio834::Gpio834Spec>;
#[doc = "GPIO Write Privilege Control Register \\#9"]
pub mod gpio834;
#[doc = "GPIO838 (rw) register accessor: GPIO Write Privilege Control Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio838::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio838::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio838`] module"]
#[doc(alias = "GPIO838")]
pub type Gpio838 = crate::Reg<gpio838::Gpio838Spec>;
#[doc = "GPIO Write Privilege Control Register \\#10"]
pub mod gpio838;
#[doc = "GPIO83C (rw) register accessor: GPIO Write Privilege Control Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio83c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio83c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio83c`] module"]
#[doc(alias = "GPIO83C")]
pub type Gpio83c = crate::Reg<gpio83c::Gpio83cSpec>;
#[doc = "GPIO Write Privilege Control Register \\#11"]
pub mod gpio83c;
#[doc = "GPIO840 (rw) register accessor: GPIO Write Privilege Control Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio840::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio840::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio840`] module"]
#[doc(alias = "GPIO840")]
pub type Gpio840 = crate::Reg<gpio840::Gpio840Spec>;
#[doc = "GPIO Write Privilege Control Register \\#12"]
pub mod gpio840;
#[doc = "GPIO844 (rw) register accessor: GPIO Write Privilege Control Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio844::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio844::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio844`] module"]
#[doc(alias = "GPIO844")]
pub type Gpio844 = crate::Reg<gpio844::Gpio844Spec>;
#[doc = "GPIO Write Privilege Control Register \\#13"]
pub mod gpio844;
#[doc = "GPIO848 (rw) register accessor: GPIO Write Privilege Control Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio848::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio848::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio848`] module"]
#[doc(alias = "GPIO848")]
pub type Gpio848 = crate::Reg<gpio848::Gpio848Spec>;
#[doc = "GPIO Write Privilege Control Register \\#14"]
pub mod gpio848;
#[doc = "GPIO84C (rw) register accessor: GPIO Write Privilege Control Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio84c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio84c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio84c`] module"]
#[doc(alias = "GPIO84C")]
pub type Gpio84c = crate::Reg<gpio84c::Gpio84cSpec>;
#[doc = "GPIO Write Privilege Control Register \\#15"]
pub mod gpio84c;
#[doc = "GPIO850 (rw) register accessor: GPIO Write Privilege Control Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio850::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio850::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio850`] module"]
#[doc(alias = "GPIO850")]
pub type Gpio850 = crate::Reg<gpio850::Gpio850Spec>;
#[doc = "GPIO Write Privilege Control Register \\#16"]
pub mod gpio850;
#[doc = "GPIO854 (rw) register accessor: GPIO Write Privilege Control Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio854::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio854::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio854`] module"]
#[doc(alias = "GPIO854")]
pub type Gpio854 = crate::Reg<gpio854::Gpio854Spec>;
#[doc = "GPIO Write Privilege Control Register \\#17"]
pub mod gpio854;
#[doc = "GPIO858 (rw) register accessor: GPIO Write Privilege Control Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio858::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio858::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio858`] module"]
#[doc(alias = "GPIO858")]
pub type Gpio858 = crate::Reg<gpio858::Gpio858Spec>;
#[doc = "GPIO Write Privilege Control Register \\#18"]
pub mod gpio858;
#[doc = "GPIO85C (rw) register accessor: GPIO Write Privilege Control Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio85c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio85c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio85c`] module"]
#[doc(alias = "GPIO85C")]
pub type Gpio85c = crate::Reg<gpio85c::Gpio85cSpec>;
#[doc = "GPIO Write Privilege Control Register \\#19"]
pub mod gpio85c;
#[doc = "GPIO860 (rw) register accessor: GPIO Write Privilege Control Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio860::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio860::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio860`] module"]
#[doc(alias = "GPIO860")]
pub type Gpio860 = crate::Reg<gpio860::Gpio860Spec>;
#[doc = "GPIO Write Privilege Control Register \\#20"]
pub mod gpio860;
#[doc = "GPIO864 (rw) register accessor: GPIO Write Privilege Control Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio864::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio864::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio864`] module"]
#[doc(alias = "GPIO864")]
pub type Gpio864 = crate::Reg<gpio864::Gpio864Spec>;
#[doc = "GPIO Write Privilege Control Register \\#21"]
pub mod gpio864;
#[doc = "GPIO868 (rw) register accessor: GPIO Write Privilege Control Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio868::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio868::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio868`] module"]
#[doc(alias = "GPIO868")]
pub type Gpio868 = crate::Reg<gpio868::Gpio868Spec>;
#[doc = "GPIO Write Privilege Control Register \\#22"]
pub mod gpio868;
#[doc = "GPIO86C (rw) register accessor: GPIO Write Privilege Control Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio86c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio86c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio86c`] module"]
#[doc(alias = "GPIO86C")]
pub type Gpio86c = crate::Reg<gpio86c::Gpio86cSpec>;
#[doc = "GPIO Write Privilege Control Register \\#23"]
pub mod gpio86c;
#[doc = "GPIO870 (rw) register accessor: GPIO Write Privilege Control Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio870::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio870::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio870`] module"]
#[doc(alias = "GPIO870")]
pub type Gpio870 = crate::Reg<gpio870::Gpio870Spec>;
#[doc = "GPIO Write Privilege Control Register \\#24"]
pub mod gpio870;
#[doc = "GPIO874 (rw) register accessor: GPIO Write Privilege Control Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio874::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio874::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio874`] module"]
#[doc(alias = "GPIO874")]
pub type Gpio874 = crate::Reg<gpio874::Gpio874Spec>;
#[doc = "GPIO Write Privilege Control Register \\#25"]
pub mod gpio874;
#[doc = "GPIO878 (rw) register accessor: GPIO Write Privilege Control Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio878::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio878::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio878`] module"]
#[doc(alias = "GPIO878")]
pub type Gpio878 = crate::Reg<gpio878::Gpio878Spec>;
#[doc = "GPIO Write Privilege Control Register \\#26"]
pub mod gpio878;
#[doc = "GPIO87C (rw) register accessor: GPIO Write Privilege Control Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio87c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio87c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio87c`] module"]
#[doc(alias = "GPIO87C")]
pub type Gpio87c = crate::Reg<gpio87c::Gpio87cSpec>;
#[doc = "GPIO Write Privilege Control Register \\#27"]
pub mod gpio87c;
#[doc = "GPIO880 (rw) register accessor: GPIO Write Privilege Control Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio880::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio880::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio880`] module"]
#[doc(alias = "GPIO880")]
pub type Gpio880 = crate::Reg<gpio880::Gpio880Spec>;
#[doc = "GPIO Write Privilege Control Register \\#28"]
pub mod gpio880;
#[doc = "GPIO884 (rw) register accessor: GPIO Write Privilege Control Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio884::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio884::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio884`] module"]
#[doc(alias = "GPIO884")]
pub type Gpio884 = crate::Reg<gpio884::Gpio884Spec>;
#[doc = "GPIO Write Privilege Control Register \\#29"]
pub mod gpio884;
#[doc = "GPIO888 (rw) register accessor: GPIO Write Privilege Control Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio888::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio888::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio888`] module"]
#[doc(alias = "GPIO888")]
pub type Gpio888 = crate::Reg<gpio888::Gpio888Spec>;
#[doc = "GPIO Write Privilege Control Register \\#30"]
pub mod gpio888;
#[doc = "GPIO88C (rw) register accessor: GPIO Write Privilege Control Register \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio88c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio88c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio88c`] module"]
#[doc(alias = "GPIO88C")]
pub type Gpio88c = crate::Reg<gpio88c::Gpio88cSpec>;
#[doc = "GPIO Write Privilege Control Register \\#31"]
pub mod gpio88c;
#[doc = "GPIO890 (rw) register accessor: GPIO Write Privilege Control Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio890::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio890::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio890`] module"]
#[doc(alias = "GPIO890")]
pub type Gpio890 = crate::Reg<gpio890::Gpio890Spec>;
#[doc = "GPIO Write Privilege Control Register \\#32"]
pub mod gpio890;
#[doc = "GPIO894 (rw) register accessor: GPIO Write Privilege Control Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio894::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio894::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio894`] module"]
#[doc(alias = "GPIO894")]
pub type Gpio894 = crate::Reg<gpio894::Gpio894Spec>;
#[doc = "GPIO Write Privilege Control Register \\#33"]
pub mod gpio894;
#[doc = "GPIO898 (rw) register accessor: GPIO Write Privilege Control Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio898::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio898::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio898`] module"]
#[doc(alias = "GPIO898")]
pub type Gpio898 = crate::Reg<gpio898::Gpio898Spec>;
#[doc = "GPIO Write Privilege Control Register \\#34"]
pub mod gpio898;
#[doc = "GPIO89C (rw) register accessor: GPIO Write Privilege Control Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio89c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio89c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio89c`] module"]
#[doc(alias = "GPIO89C")]
pub type Gpio89c = crate::Reg<gpio89c::Gpio89cSpec>;
#[doc = "GPIO Write Privilege Control Register \\#35"]
pub mod gpio89c;
#[doc = "GPIO8A0 (rw) register accessor: GPIO Write Privilege Control Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8a0`] module"]
#[doc(alias = "GPIO8A0")]
pub type Gpio8a0 = crate::Reg<gpio8a0::Gpio8a0Spec>;
#[doc = "GPIO Write Privilege Control Register \\#36"]
pub mod gpio8a0;
#[doc = "GPIO8A4 (rw) register accessor: GPIO Write Privilege Control Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8a4`] module"]
#[doc(alias = "GPIO8A4")]
pub type Gpio8a4 = crate::Reg<gpio8a4::Gpio8a4Spec>;
#[doc = "GPIO Write Privilege Control Register \\#37"]
pub mod gpio8a4;
#[doc = "GPIO8A8 (rw) register accessor: GPIO Write Privilege Control Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8a8`] module"]
#[doc(alias = "GPIO8A8")]
pub type Gpio8a8 = crate::Reg<gpio8a8::Gpio8a8Spec>;
#[doc = "GPIO Write Privilege Control Register \\#38"]
pub mod gpio8a8;
#[doc = "GPIO8AC (rw) register accessor: GPIO Write Privilege Control Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8ac`] module"]
#[doc(alias = "GPIO8AC")]
pub type Gpio8ac = crate::Reg<gpio8ac::Gpio8acSpec>;
#[doc = "GPIO Write Privilege Control Register \\#39"]
pub mod gpio8ac;
#[doc = "GPIO8B0 (rw) register accessor: GPIO Write Privilege Control Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8b0`] module"]
#[doc(alias = "GPIO8B0")]
pub type Gpio8b0 = crate::Reg<gpio8b0::Gpio8b0Spec>;
#[doc = "GPIO Write Privilege Control Register \\#40"]
pub mod gpio8b0;
#[doc = "GPIO8B4 (rw) register accessor: GPIO Write Privilege Control Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8b4`] module"]
#[doc(alias = "GPIO8B4")]
pub type Gpio8b4 = crate::Reg<gpio8b4::Gpio8b4Spec>;
#[doc = "GPIO Write Privilege Control Register \\#41"]
pub mod gpio8b4;
#[doc = "GPIO8B8 (rw) register accessor: GPIO Write Privilege Control Register \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8b8`] module"]
#[doc(alias = "GPIO8B8")]
pub type Gpio8b8 = crate::Reg<gpio8b8::Gpio8b8Spec>;
#[doc = "GPIO Write Privilege Control Register \\#42"]
pub mod gpio8b8;
#[doc = "GPIO8BC (rw) register accessor: GPIO Write Privilege Control Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8bc`] module"]
#[doc(alias = "GPIO8BC")]
pub type Gpio8bc = crate::Reg<gpio8bc::Gpio8bcSpec>;
#[doc = "GPIO Write Privilege Control Register \\#43"]
pub mod gpio8bc;
#[doc = "GPIO8C0 (rw) register accessor: GPIO Write Privilege Control Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8c0`] module"]
#[doc(alias = "GPIO8C0")]
pub type Gpio8c0 = crate::Reg<gpio8c0::Gpio8c0Spec>;
#[doc = "GPIO Write Privilege Control Register \\#44"]
pub mod gpio8c0;
#[doc = "GPIO8C4 (rw) register accessor: GPIO Write Privilege Control Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8c4`] module"]
#[doc(alias = "GPIO8C4")]
pub type Gpio8c4 = crate::Reg<gpio8c4::Gpio8c4Spec>;
#[doc = "GPIO Write Privilege Control Register \\#45"]
pub mod gpio8c4;
#[doc = "GPIO8C8 (rw) register accessor: GPIO Write Privilege Control Register \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8c8`] module"]
#[doc(alias = "GPIO8C8")]
pub type Gpio8c8 = crate::Reg<gpio8c8::Gpio8c8Spec>;
#[doc = "GPIO Write Privilege Control Register \\#46"]
pub mod gpio8c8;
#[doc = "GPIO8CC (rw) register accessor: GPIO Write Privilege Control Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8cc`] module"]
#[doc(alias = "GPIO8CC")]
pub type Gpio8cc = crate::Reg<gpio8cc::Gpio8ccSpec>;
#[doc = "GPIO Write Privilege Control Register \\#47"]
pub mod gpio8cc;
#[doc = "GPIO8D0 (rw) register accessor: GPIO Write Privilege Control Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio8d0`] module"]
#[doc(alias = "GPIO8D0")]
pub type Gpio8d0 = crate::Reg<gpio8d0::Gpio8d0Spec>;
#[doc = "GPIO Write Privilege Control Register \\#48"]
pub mod gpio8d0;
#[doc = "GPIO910 (rw) register accessor: GPIO Read Privilege Control Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio910::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio910::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio910`] module"]
#[doc(alias = "GPIO910")]
pub type Gpio910 = crate::Reg<gpio910::Gpio910Spec>;
#[doc = "GPIO Read Privilege Control Register \\#0"]
pub mod gpio910;
#[doc = "GPIO914 (rw) register accessor: GPIO Read Privilege Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio914::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio914::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio914`] module"]
#[doc(alias = "GPIO914")]
pub type Gpio914 = crate::Reg<gpio914::Gpio914Spec>;
#[doc = "GPIO Read Privilege Control Register \\#1"]
pub mod gpio914;
#[doc = "GPIO918 (rw) register accessor: GPIO Read Privilege Control Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio918::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio918::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio918`] module"]
#[doc(alias = "GPIO918")]
pub type Gpio918 = crate::Reg<gpio918::Gpio918Spec>;
#[doc = "GPIO Read Privilege Control Register \\#2"]
pub mod gpio918;
#[doc = "GPIO91C (rw) register accessor: GPIO Read Privilege Control Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio91c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio91c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio91c`] module"]
#[doc(alias = "GPIO91C")]
pub type Gpio91c = crate::Reg<gpio91c::Gpio91cSpec>;
#[doc = "GPIO Read Privilege Control Register \\#3"]
pub mod gpio91c;
#[doc = "GPIO920 (rw) register accessor: GPIO Read Privilege Control Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio920::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio920::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio920`] module"]
#[doc(alias = "GPIO920")]
pub type Gpio920 = crate::Reg<gpio920::Gpio920Spec>;
#[doc = "GPIO Read Privilege Control Register \\#4"]
pub mod gpio920;
#[doc = "GPIO924 (rw) register accessor: GPIO Read Privilege Control Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio924::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio924::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio924`] module"]
#[doc(alias = "GPIO924")]
pub type Gpio924 = crate::Reg<gpio924::Gpio924Spec>;
#[doc = "GPIO Read Privilege Control Register \\#5"]
pub mod gpio924;
#[doc = "GPIO928 (rw) register accessor: GPIO Read Privilege Control Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio928::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio928::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio928`] module"]
#[doc(alias = "GPIO928")]
pub type Gpio928 = crate::Reg<gpio928::Gpio928Spec>;
#[doc = "GPIO Read Privilege Control Register \\#6"]
pub mod gpio928;
#[doc = "GPIO92C (rw) register accessor: GPIO Read Privilege Control Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio92c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio92c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio92c`] module"]
#[doc(alias = "GPIO92C")]
pub type Gpio92c = crate::Reg<gpio92c::Gpio92cSpec>;
#[doc = "GPIO Read Privilege Control Register \\#7"]
pub mod gpio92c;
#[doc = "GPIO930 (rw) register accessor: GPIO Read Privilege Control Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio930::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio930::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio930`] module"]
#[doc(alias = "GPIO930")]
pub type Gpio930 = crate::Reg<gpio930::Gpio930Spec>;
#[doc = "GPIO Read Privilege Control Register \\#8"]
pub mod gpio930;
#[doc = "GPIO934 (rw) register accessor: GPIO Read Privilege Control Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio934::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio934::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio934`] module"]
#[doc(alias = "GPIO934")]
pub type Gpio934 = crate::Reg<gpio934::Gpio934Spec>;
#[doc = "GPIO Read Privilege Control Register \\#9"]
pub mod gpio934;
#[doc = "GPIO938 (rw) register accessor: GPIO Read Privilege Control Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio938::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio938::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio938`] module"]
#[doc(alias = "GPIO938")]
pub type Gpio938 = crate::Reg<gpio938::Gpio938Spec>;
#[doc = "GPIO Read Privilege Control Register \\#10"]
pub mod gpio938;
#[doc = "GPIO93C (rw) register accessor: GPIO Read Privilege Control Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio93c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio93c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio93c`] module"]
#[doc(alias = "GPIO93C")]
pub type Gpio93c = crate::Reg<gpio93c::Gpio93cSpec>;
#[doc = "GPIO Read Privilege Control Register \\#11"]
pub mod gpio93c;
#[doc = "GPIO940 (rw) register accessor: GPIO Read Privilege Control Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio940::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio940::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio940`] module"]
#[doc(alias = "GPIO940")]
pub type Gpio940 = crate::Reg<gpio940::Gpio940Spec>;
#[doc = "GPIO Read Privilege Control Register \\#12"]
pub mod gpio940;
#[doc = "GPIO944 (rw) register accessor: GPIO Read Privilege Control Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio944::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio944::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio944`] module"]
#[doc(alias = "GPIO944")]
pub type Gpio944 = crate::Reg<gpio944::Gpio944Spec>;
#[doc = "GPIO Read Privilege Control Register \\#13"]
pub mod gpio944;
#[doc = "GPIO948 (rw) register accessor: GPIO Read Privilege Control Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio948::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio948::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio948`] module"]
#[doc(alias = "GPIO948")]
pub type Gpio948 = crate::Reg<gpio948::Gpio948Spec>;
#[doc = "GPIO Read Privilege Control Register \\#14"]
pub mod gpio948;
#[doc = "GPIO94C (rw) register accessor: GPIO Read Privilege Control Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio94c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio94c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio94c`] module"]
#[doc(alias = "GPIO94C")]
pub type Gpio94c = crate::Reg<gpio94c::Gpio94cSpec>;
#[doc = "GPIO Read Privilege Control Register \\#15"]
pub mod gpio94c;
#[doc = "GPIO950 (rw) register accessor: GPIO Read Privilege Control Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio950::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio950::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio950`] module"]
#[doc(alias = "GPIO950")]
pub type Gpio950 = crate::Reg<gpio950::Gpio950Spec>;
#[doc = "GPIO Read Privilege Control Register \\#16"]
pub mod gpio950;
#[doc = "GPIO954 (rw) register accessor: GPIO Read Privilege Control Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio954::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio954::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio954`] module"]
#[doc(alias = "GPIO954")]
pub type Gpio954 = crate::Reg<gpio954::Gpio954Spec>;
#[doc = "GPIO Read Privilege Control Register \\#17"]
pub mod gpio954;
#[doc = "GPIO958 (rw) register accessor: GPIO Read Privilege Control Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio958::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio958::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio958`] module"]
#[doc(alias = "GPIO958")]
pub type Gpio958 = crate::Reg<gpio958::Gpio958Spec>;
#[doc = "GPIO Read Privilege Control Register \\#18"]
pub mod gpio958;
#[doc = "GPIO95C (rw) register accessor: GPIO Read Privilege Control Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio95c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio95c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio95c`] module"]
#[doc(alias = "GPIO95C")]
pub type Gpio95c = crate::Reg<gpio95c::Gpio95cSpec>;
#[doc = "GPIO Read Privilege Control Register \\#19"]
pub mod gpio95c;
#[doc = "GPIO960 (rw) register accessor: GPIO Read Privilege Control Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio960::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio960::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio960`] module"]
#[doc(alias = "GPIO960")]
pub type Gpio960 = crate::Reg<gpio960::Gpio960Spec>;
#[doc = "GPIO Read Privilege Control Register \\#20"]
pub mod gpio960;
#[doc = "GPIO964 (rw) register accessor: GPIO Read Privilege Control Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio964::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio964::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio964`] module"]
#[doc(alias = "GPIO964")]
pub type Gpio964 = crate::Reg<gpio964::Gpio964Spec>;
#[doc = "GPIO Read Privilege Control Register \\#21"]
pub mod gpio964;
#[doc = "GPIO968 (rw) register accessor: GPIO Read Privilege Control Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio968::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio968::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio968`] module"]
#[doc(alias = "GPIO968")]
pub type Gpio968 = crate::Reg<gpio968::Gpio968Spec>;
#[doc = "GPIO Read Privilege Control Register \\#22"]
pub mod gpio968;
#[doc = "GPIO96C (rw) register accessor: GPIO Read Privilege Control Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio96c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio96c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio96c`] module"]
#[doc(alias = "GPIO96C")]
pub type Gpio96c = crate::Reg<gpio96c::Gpio96cSpec>;
#[doc = "GPIO Read Privilege Control Register \\#23"]
pub mod gpio96c;
#[doc = "GPIO970 (rw) register accessor: GPIO Read Privilege Control Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio970::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio970::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio970`] module"]
#[doc(alias = "GPIO970")]
pub type Gpio970 = crate::Reg<gpio970::Gpio970Spec>;
#[doc = "GPIO Read Privilege Control Register \\#24"]
pub mod gpio970;
#[doc = "GPIO974 (rw) register accessor: GPIO Read Privilege Control Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio974::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio974::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio974`] module"]
#[doc(alias = "GPIO974")]
pub type Gpio974 = crate::Reg<gpio974::Gpio974Spec>;
#[doc = "GPIO Read Privilege Control Register \\#25"]
pub mod gpio974;
#[doc = "GPIO978 (rw) register accessor: GPIO Read Privilege Control Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio978::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio978::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio978`] module"]
#[doc(alias = "GPIO978")]
pub type Gpio978 = crate::Reg<gpio978::Gpio978Spec>;
#[doc = "GPIO Read Privilege Control Register \\#26"]
pub mod gpio978;
#[doc = "GPIO97C (rw) register accessor: GPIO Read Privilege Control Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio97c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio97c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio97c`] module"]
#[doc(alias = "GPIO97C")]
pub type Gpio97c = crate::Reg<gpio97c::Gpio97cSpec>;
#[doc = "GPIO Read Privilege Control Register \\#27"]
pub mod gpio97c;
#[doc = "GPIO980 (rw) register accessor: GPIO Read Privilege Control Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio980::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio980::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio980`] module"]
#[doc(alias = "GPIO980")]
pub type Gpio980 = crate::Reg<gpio980::Gpio980Spec>;
#[doc = "GPIO Read Privilege Control Register \\#28"]
pub mod gpio980;
#[doc = "GPIO984 (rw) register accessor: GPIO Read Privilege Control Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio984::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio984::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio984`] module"]
#[doc(alias = "GPIO984")]
pub type Gpio984 = crate::Reg<gpio984::Gpio984Spec>;
#[doc = "GPIO Read Privilege Control Register \\#29"]
pub mod gpio984;
#[doc = "GPIO988 (rw) register accessor: GPIO Read Privilege Control Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio988::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio988::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio988`] module"]
#[doc(alias = "GPIO988")]
pub type Gpio988 = crate::Reg<gpio988::Gpio988Spec>;
#[doc = "GPIO Read Privilege Control Register \\#30"]
pub mod gpio988;
#[doc = "GPIO98C (rw) register accessor: GPIO Read Privilege Control Register \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio98c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio98c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio98c`] module"]
#[doc(alias = "GPIO98C")]
pub type Gpio98c = crate::Reg<gpio98c::Gpio98cSpec>;
#[doc = "GPIO Read Privilege Control Register \\#31"]
pub mod gpio98c;
#[doc = "GPIO990 (rw) register accessor: GPIO Read Privilege Control Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio990::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio990::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio990`] module"]
#[doc(alias = "GPIO990")]
pub type Gpio990 = crate::Reg<gpio990::Gpio990Spec>;
#[doc = "GPIO Read Privilege Control Register \\#32"]
pub mod gpio990;
#[doc = "GPIO994 (rw) register accessor: GPIO Read Privilege Control Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio994::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio994::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio994`] module"]
#[doc(alias = "GPIO994")]
pub type Gpio994 = crate::Reg<gpio994::Gpio994Spec>;
#[doc = "GPIO Read Privilege Control Register \\#33"]
pub mod gpio994;
#[doc = "GPIO998 (rw) register accessor: GPIO Read Privilege Control Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio998::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio998::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio998`] module"]
#[doc(alias = "GPIO998")]
pub type Gpio998 = crate::Reg<gpio998::Gpio998Spec>;
#[doc = "GPIO Read Privilege Control Register \\#34"]
pub mod gpio998;
#[doc = "GPIO99C (rw) register accessor: GPIO Read Privilege Control Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio99c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio99c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio99c`] module"]
#[doc(alias = "GPIO99C")]
pub type Gpio99c = crate::Reg<gpio99c::Gpio99cSpec>;
#[doc = "GPIO Read Privilege Control Register \\#35"]
pub mod gpio99c;
#[doc = "GPIO9A0 (rw) register accessor: GPIO Read Privilege Control Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9a0`] module"]
#[doc(alias = "GPIO9A0")]
pub type Gpio9a0 = crate::Reg<gpio9a0::Gpio9a0Spec>;
#[doc = "GPIO Read Privilege Control Register \\#36"]
pub mod gpio9a0;
#[doc = "GPIO9A4 (rw) register accessor: GPIO Read Privilege Control Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9a4`] module"]
#[doc(alias = "GPIO9A4")]
pub type Gpio9a4 = crate::Reg<gpio9a4::Gpio9a4Spec>;
#[doc = "GPIO Read Privilege Control Register \\#37"]
pub mod gpio9a4;
#[doc = "GPIO9A8 (rw) register accessor: GPIO Read Privilege Control Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9a8`] module"]
#[doc(alias = "GPIO9A8")]
pub type Gpio9a8 = crate::Reg<gpio9a8::Gpio9a8Spec>;
#[doc = "GPIO Read Privilege Control Register \\#38"]
pub mod gpio9a8;
#[doc = "GPIO9AC (rw) register accessor: GPIO Read Privilege Control Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9ac`] module"]
#[doc(alias = "GPIO9AC")]
pub type Gpio9ac = crate::Reg<gpio9ac::Gpio9acSpec>;
#[doc = "GPIO Read Privilege Control Register \\#39"]
pub mod gpio9ac;
#[doc = "GPIO9B0 (rw) register accessor: GPIO Read Privilege Control Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9b0`] module"]
#[doc(alias = "GPIO9B0")]
pub type Gpio9b0 = crate::Reg<gpio9b0::Gpio9b0Spec>;
#[doc = "GPIO Read Privilege Control Register \\#40"]
pub mod gpio9b0;
#[doc = "GPIO9B4 (rw) register accessor: GPIO Read Privilege Control Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9b4`] module"]
#[doc(alias = "GPIO9B4")]
pub type Gpio9b4 = crate::Reg<gpio9b4::Gpio9b4Spec>;
#[doc = "GPIO Read Privilege Control Register \\#41"]
pub mod gpio9b4;
#[doc = "GPIO9B8 (rw) register accessor: GPIO Read Privilege Control Register \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9b8`] module"]
#[doc(alias = "GPIO9B8")]
pub type Gpio9b8 = crate::Reg<gpio9b8::Gpio9b8Spec>;
#[doc = "GPIO Read Privilege Control Register \\#42"]
pub mod gpio9b8;
#[doc = "GPIO9BC (rw) register accessor: GPIO Read Privilege Control Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9bc`] module"]
#[doc(alias = "GPIO9BC")]
pub type Gpio9bc = crate::Reg<gpio9bc::Gpio9bcSpec>;
#[doc = "GPIO Read Privilege Control Register \\#43"]
pub mod gpio9bc;
#[doc = "GPIO9C0 (rw) register accessor: GPIO Read Privilege Control Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9c0`] module"]
#[doc(alias = "GPIO9C0")]
pub type Gpio9c0 = crate::Reg<gpio9c0::Gpio9c0Spec>;
#[doc = "GPIO Read Privilege Control Register \\#44"]
pub mod gpio9c0;
#[doc = "GPIO9C4 (rw) register accessor: GPIO Read Privilege Control Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9c4`] module"]
#[doc(alias = "GPIO9C4")]
pub type Gpio9c4 = crate::Reg<gpio9c4::Gpio9c4Spec>;
#[doc = "GPIO Read Privilege Control Register \\#45"]
pub mod gpio9c4;
#[doc = "GPIO9C8 (rw) register accessor: GPIO Read Privilege Control Register \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9c8`] module"]
#[doc(alias = "GPIO9C8")]
pub type Gpio9c8 = crate::Reg<gpio9c8::Gpio9c8Spec>;
#[doc = "GPIO Read Privilege Control Register \\#46"]
pub mod gpio9c8;
#[doc = "GPIO9CC (rw) register accessor: GPIO Read Privilege Control Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9cc`] module"]
#[doc(alias = "GPIO9CC")]
pub type Gpio9cc = crate::Reg<gpio9cc::Gpio9ccSpec>;
#[doc = "GPIO Read Privilege Control Register \\#47"]
pub mod gpio9cc;
#[doc = "GPIO9D0 (rw) register accessor: GPIO Read Privilege Control Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio9d0`] module"]
#[doc(alias = "GPIO9D0")]
pub type Gpio9d0 = crate::Reg<gpio9d0::Gpio9d0Spec>;
#[doc = "GPIO Read Privilege Control Register \\#48"]
pub mod gpio9d0;
#[doc = "GPIOA14 (rw) register accessor: GPIO Interrupt Target Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa14`] module"]
#[doc(alias = "GPIOA14")]
pub type Gpioa14 = crate::Reg<gpioa14::Gpioa14Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#1"]
pub mod gpioa14;
#[doc = "GPIOA18 (rw) register accessor: GPIO Interrupt Target Control Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa18`] module"]
#[doc(alias = "GPIOA18")]
pub type Gpioa18 = crate::Reg<gpioa18::Gpioa18Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#2"]
pub mod gpioa18;
#[doc = "GPIOA1C (rw) register accessor: GPIO Interrupt Target Control Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa1c`] module"]
#[doc(alias = "GPIOA1C")]
pub type Gpioa1c = crate::Reg<gpioa1c::Gpioa1cSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#3"]
pub mod gpioa1c;
#[doc = "GPIOA20 (rw) register accessor: GPIO Interrupt Target Control Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa20`] module"]
#[doc(alias = "GPIOA20")]
pub type Gpioa20 = crate::Reg<gpioa20::Gpioa20Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#4"]
pub mod gpioa20;
#[doc = "GPIOA24 (rw) register accessor: GPIO Interrupt Target Control Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa24`] module"]
#[doc(alias = "GPIOA24")]
pub type Gpioa24 = crate::Reg<gpioa24::Gpioa24Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#5"]
pub mod gpioa24;
#[doc = "GPIOA28 (rw) register accessor: GPIO Interrupt Target Control Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa28`] module"]
#[doc(alias = "GPIOA28")]
pub type Gpioa28 = crate::Reg<gpioa28::Gpioa28Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#6"]
pub mod gpioa28;
#[doc = "GPIOA2C (rw) register accessor: GPIO Interrupt Target Control Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa2c`] module"]
#[doc(alias = "GPIOA2C")]
pub type Gpioa2c = crate::Reg<gpioa2c::Gpioa2cSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#7"]
pub mod gpioa2c;
#[doc = "GPIOA30 (rw) register accessor: GPIO Interrupt Target Control Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa30`] module"]
#[doc(alias = "GPIOA30")]
pub type Gpioa30 = crate::Reg<gpioa30::Gpioa30Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#8"]
pub mod gpioa30;
#[doc = "GPIOA34 (rw) register accessor: GPIO Interrupt Target Control Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa34`] module"]
#[doc(alias = "GPIOA34")]
pub type Gpioa34 = crate::Reg<gpioa34::Gpioa34Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#9"]
pub mod gpioa34;
#[doc = "GPIOA38 (rw) register accessor: GPIO Interrupt Target Control Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa38`] module"]
#[doc(alias = "GPIOA38")]
pub type Gpioa38 = crate::Reg<gpioa38::Gpioa38Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#10"]
pub mod gpioa38;
#[doc = "GPIOA3C (rw) register accessor: GPIO Interrupt Target Control Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa3c`] module"]
#[doc(alias = "GPIOA3C")]
pub type Gpioa3c = crate::Reg<gpioa3c::Gpioa3cSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#11"]
pub mod gpioa3c;
#[doc = "GPIOA40 (rw) register accessor: GPIO Interrupt Target Control Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa40`] module"]
#[doc(alias = "GPIOA40")]
pub type Gpioa40 = crate::Reg<gpioa40::Gpioa40Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#12"]
pub mod gpioa40;
#[doc = "GPIOA44 (rw) register accessor: GPIO Interrupt Target Control Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa44`] module"]
#[doc(alias = "GPIOA44")]
pub type Gpioa44 = crate::Reg<gpioa44::Gpioa44Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#13"]
pub mod gpioa44;
#[doc = "GPIOA48 (rw) register accessor: GPIO Interrupt Target Control Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa48`] module"]
#[doc(alias = "GPIOA48")]
pub type Gpioa48 = crate::Reg<gpioa48::Gpioa48Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#14"]
pub mod gpioa48;
#[doc = "GPIOA4C (rw) register accessor: GPIO Interrupt Target Control Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa4c`] module"]
#[doc(alias = "GPIOA4C")]
pub type Gpioa4c = crate::Reg<gpioa4c::Gpioa4cSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#15"]
pub mod gpioa4c;
#[doc = "GPIOA50 (rw) register accessor: GPIO Interrupt Target Control Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa50`] module"]
#[doc(alias = "GPIOA50")]
pub type Gpioa50 = crate::Reg<gpioa50::Gpioa50Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#16"]
pub mod gpioa50;
#[doc = "GPIOA54 (rw) register accessor: GPIO Interrupt Target Control Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa54`] module"]
#[doc(alias = "GPIOA54")]
pub type Gpioa54 = crate::Reg<gpioa54::Gpioa54Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#17"]
pub mod gpioa54;
#[doc = "GPIOA58 (rw) register accessor: GPIO Interrupt Target Control Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa58`] module"]
#[doc(alias = "GPIOA58")]
pub type Gpioa58 = crate::Reg<gpioa58::Gpioa58Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#18"]
pub mod gpioa58;
#[doc = "GPIOA5C (rw) register accessor: GPIO Interrupt Target Control Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa5c`] module"]
#[doc(alias = "GPIOA5C")]
pub type Gpioa5c = crate::Reg<gpioa5c::Gpioa5cSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#19"]
pub mod gpioa5c;
#[doc = "GPIOA60 (rw) register accessor: GPIO Interrupt Target Control Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa60`] module"]
#[doc(alias = "GPIOA60")]
pub type Gpioa60 = crate::Reg<gpioa60::Gpioa60Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#20"]
pub mod gpioa60;
#[doc = "GPIOA64 (rw) register accessor: GPIO Interrupt Target Control Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa64`] module"]
#[doc(alias = "GPIOA64")]
pub type Gpioa64 = crate::Reg<gpioa64::Gpioa64Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#21"]
pub mod gpioa64;
#[doc = "GPIOA68 (rw) register accessor: GPIO Interrupt Target Control Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa68`] module"]
#[doc(alias = "GPIOA68")]
pub type Gpioa68 = crate::Reg<gpioa68::Gpioa68Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#22"]
pub mod gpioa68;
#[doc = "GPIOA6C (rw) register accessor: GPIO Interrupt Target Control Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa6c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa6c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa6c`] module"]
#[doc(alias = "GPIOA6C")]
pub type Gpioa6c = crate::Reg<gpioa6c::Gpioa6cSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#23"]
pub mod gpioa6c;
#[doc = "GPIOA70 (rw) register accessor: GPIO Interrupt Target Control Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa70`] module"]
#[doc(alias = "GPIOA70")]
pub type Gpioa70 = crate::Reg<gpioa70::Gpioa70Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#24"]
pub mod gpioa70;
#[doc = "GPIOA74 (rw) register accessor: GPIO Interrupt Target Control Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa74`] module"]
#[doc(alias = "GPIOA74")]
pub type Gpioa74 = crate::Reg<gpioa74::Gpioa74Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#25"]
pub mod gpioa74;
#[doc = "GPIOA78 (rw) register accessor: GPIO Interrupt Target Control Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa78`] module"]
#[doc(alias = "GPIOA78")]
pub type Gpioa78 = crate::Reg<gpioa78::Gpioa78Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#26"]
pub mod gpioa78;
#[doc = "GPIOA7C (rw) register accessor: GPIO Interrupt Target Control Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa7c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa7c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa7c`] module"]
#[doc(alias = "GPIOA7C")]
pub type Gpioa7c = crate::Reg<gpioa7c::Gpioa7cSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#27"]
pub mod gpioa7c;
#[doc = "GPIOA80 (rw) register accessor: GPIO Interrupt Target Control Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa80`] module"]
#[doc(alias = "GPIOA80")]
pub type Gpioa80 = crate::Reg<gpioa80::Gpioa80Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#28"]
pub mod gpioa80;
#[doc = "GPIOA84 (rw) register accessor: GPIO Interrupt Target Control Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa84`] module"]
#[doc(alias = "GPIOA84")]
pub type Gpioa84 = crate::Reg<gpioa84::Gpioa84Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#29"]
pub mod gpioa84;
#[doc = "GPIOA88 (rw) register accessor: GPIO Interrupt Target Control Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa88`] module"]
#[doc(alias = "GPIOA88")]
pub type Gpioa88 = crate::Reg<gpioa88::Gpioa88Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#30"]
pub mod gpioa88;
#[doc = "GPIOA8C (rw) register accessor: GPIO Interrupt Target Control Register \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa8c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa8c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa8c`] module"]
#[doc(alias = "GPIOA8C")]
pub type Gpioa8c = crate::Reg<gpioa8c::Gpioa8cSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#31"]
pub mod gpioa8c;
#[doc = "GPIOA90 (rw) register accessor: GPIO Interrupt Target Control Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa90`] module"]
#[doc(alias = "GPIOA90")]
pub type Gpioa90 = crate::Reg<gpioa90::Gpioa90Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#32"]
pub mod gpioa90;
#[doc = "GPIOA94 (rw) register accessor: GPIO Interrupt Target Control Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa94`] module"]
#[doc(alias = "GPIOA94")]
pub type Gpioa94 = crate::Reg<gpioa94::Gpioa94Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#33"]
pub mod gpioa94;
#[doc = "GPIOA98 (rw) register accessor: GPIO Interrupt Target Control Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa98`] module"]
#[doc(alias = "GPIOA98")]
pub type Gpioa98 = crate::Reg<gpioa98::Gpioa98Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#34"]
pub mod gpioa98;
#[doc = "GPIOA9C (rw) register accessor: GPIO Interrupt Target Control Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioa9c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioa9c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioa9c`] module"]
#[doc(alias = "GPIOA9C")]
pub type Gpioa9c = crate::Reg<gpioa9c::Gpioa9cSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#35"]
pub mod gpioa9c;
#[doc = "GPIOAA0 (rw) register accessor: GPIO Interrupt Target Control Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioaa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioaa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioaa0`] module"]
#[doc(alias = "GPIOAA0")]
pub type Gpioaa0 = crate::Reg<gpioaa0::Gpioaa0Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#36"]
pub mod gpioaa0;
#[doc = "GPIOAA4 (rw) register accessor: GPIO Interrupt Target Control Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioaa4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioaa4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioaa4`] module"]
#[doc(alias = "GPIOAA4")]
pub type Gpioaa4 = crate::Reg<gpioaa4::Gpioaa4Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#37"]
pub mod gpioaa4;
#[doc = "GPIOAA8 (rw) register accessor: GPIO Interrupt Target Control Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioaa8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioaa8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioaa8`] module"]
#[doc(alias = "GPIOAA8")]
pub type Gpioaa8 = crate::Reg<gpioaa8::Gpioaa8Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#38"]
pub mod gpioaa8;
#[doc = "GPIOAAC (rw) register accessor: GPIO Interrupt Target Control Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioaac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioaac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioaac`] module"]
#[doc(alias = "GPIOAAC")]
pub type Gpioaac = crate::Reg<gpioaac::GpioaacSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#39"]
pub mod gpioaac;
#[doc = "GPIOAB0 (rw) register accessor: GPIO Interrupt Target Control Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioab0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioab0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioab0`] module"]
#[doc(alias = "GPIOAB0")]
pub type Gpioab0 = crate::Reg<gpioab0::Gpioab0Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#40"]
pub mod gpioab0;
#[doc = "GPIOAB4 (rw) register accessor: GPIO Interrupt Target Control Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioab4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioab4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioab4`] module"]
#[doc(alias = "GPIOAB4")]
pub type Gpioab4 = crate::Reg<gpioab4::Gpioab4Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#41"]
pub mod gpioab4;
#[doc = "GPIOAB8 (rw) register accessor: GPIO Interrupt Target Control Register \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioab8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioab8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioab8`] module"]
#[doc(alias = "GPIOAB8")]
pub type Gpioab8 = crate::Reg<gpioab8::Gpioab8Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#42"]
pub mod gpioab8;
#[doc = "GPIOABC (rw) register accessor: GPIO Interrupt Target Control Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioabc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioabc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioabc`] module"]
#[doc(alias = "GPIOABC")]
pub type Gpioabc = crate::Reg<gpioabc::GpioabcSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#43"]
pub mod gpioabc;
#[doc = "GPIOAC0 (rw) register accessor: GPIO Interrupt Target Control Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioac0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioac0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioac0`] module"]
#[doc(alias = "GPIOAC0")]
pub type Gpioac0 = crate::Reg<gpioac0::Gpioac0Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#44"]
pub mod gpioac0;
#[doc = "GPIOAC4 (rw) register accessor: GPIO Interrupt Target Control Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioac4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioac4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioac4`] module"]
#[doc(alias = "GPIOAC4")]
pub type Gpioac4 = crate::Reg<gpioac4::Gpioac4Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#45"]
pub mod gpioac4;
#[doc = "GPIOAC8 (rw) register accessor: GPIO Interrupt Target Control Register \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioac8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioac8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioac8`] module"]
#[doc(alias = "GPIOAC8")]
pub type Gpioac8 = crate::Reg<gpioac8::Gpioac8Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#46"]
pub mod gpioac8;
#[doc = "GPIOACC (rw) register accessor: GPIO Interrupt Target Control Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioacc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioacc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioacc`] module"]
#[doc(alias = "GPIOACC")]
pub type Gpioacc = crate::Reg<gpioacc::GpioaccSpec>;
#[doc = "GPIO Interrupt Target Control Register \\#47"]
pub mod gpioacc;
#[doc = "GPIOAD0 (rw) register accessor: GPIO Interrupt Target Control Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioad0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioad0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioad0`] module"]
#[doc(alias = "GPIOAD0")]
pub type Gpioad0 = crate::Reg<gpioad0::Gpioad0Spec>;
#[doc = "GPIO Interrupt Target Control Register \\#48"]
pub mod gpioad0;
#[doc = "GPIOB10 (rw) register accessor: Write Privilege Reset Tolerance Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob10`] module"]
#[doc(alias = "GPIOB10")]
pub type Gpiob10 = crate::Reg<gpiob10::Gpiob10Spec>;
#[doc = "Write Privilege Reset Tolerance Register \\#1"]
pub mod gpiob10;
#[doc = "GPIOB14 (rw) register accessor: Write Privilege Reset Tolerance Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob14`] module"]
#[doc(alias = "GPIOB14")]
pub type Gpiob14 = crate::Reg<gpiob14::Gpiob14Spec>;
#[doc = "Write Privilege Reset Tolerance Register \\#2"]
pub mod gpiob14;
#[doc = "GPIOB18 (rw) register accessor: Write Privilege Reset Tolerance Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob18`] module"]
#[doc(alias = "GPIOB18")]
pub type Gpiob18 = crate::Reg<gpiob18::Gpiob18Spec>;
#[doc = "Write Privilege Reset Tolerance Register \\#3"]
pub mod gpiob18;
#[doc = "GPIOB1C (rw) register accessor: Write Privilege Reset Tolerance Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob1c`] module"]
#[doc(alias = "GPIOB1C")]
pub type Gpiob1c = crate::Reg<gpiob1c::Gpiob1cSpec>;
#[doc = "Write Privilege Reset Tolerance Register \\#4"]
pub mod gpiob1c;
#[doc = "GPIOB20 (rw) register accessor: Write Privilege Reset Tolerance Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob20`] module"]
#[doc(alias = "GPIOB20")]
pub type Gpiob20 = crate::Reg<gpiob20::Gpiob20Spec>;
#[doc = "Write Privilege Reset Tolerance Register \\#5"]
pub mod gpiob20;
#[doc = "GPIOB24 (rw) register accessor: Write Privilege Reset Tolerance Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob24`] module"]
#[doc(alias = "GPIOB24")]
pub type Gpiob24 = crate::Reg<gpiob24::Gpiob24Spec>;
#[doc = "Write Privilege Reset Tolerance Register \\#6"]
pub mod gpiob24;
#[doc = "GPIOB28 (rw) register accessor: Write Privilege Reset Tolerance Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiob28`] module"]
#[doc(alias = "GPIOB28")]
pub type Gpiob28 = crate::Reg<gpiob28::Gpiob28Spec>;
#[doc = "Write Privilege Reset Tolerance Register \\#7"]
pub mod gpiob28;
#[doc = "GPIOC10 (rw) register accessor: Read Privilege Reset Tolerance Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc10`] module"]
#[doc(alias = "GPIOC10")]
pub type Gpioc10 = crate::Reg<gpioc10::Gpioc10Spec>;
#[doc = "Read Privilege Reset Tolerance Register \\#1"]
pub mod gpioc10;
#[doc = "GPIOC14 (rw) register accessor: Read Privilege Reset Tolerance Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc14`] module"]
#[doc(alias = "GPIOC14")]
pub type Gpioc14 = crate::Reg<gpioc14::Gpioc14Spec>;
#[doc = "Read Privilege Reset Tolerance Register \\#2"]
pub mod gpioc14;
#[doc = "GPIOC18 (rw) register accessor: Read Privilege Reset Tolerance Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc18`] module"]
#[doc(alias = "GPIOC18")]
pub type Gpioc18 = crate::Reg<gpioc18::Gpioc18Spec>;
#[doc = "Read Privilege Reset Tolerance Register \\#3"]
pub mod gpioc18;
#[doc = "GPIOC1C (rw) register accessor: Read Privilege Reset Tolerance Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc1c`] module"]
#[doc(alias = "GPIOC1C")]
pub type Gpioc1c = crate::Reg<gpioc1c::Gpioc1cSpec>;
#[doc = "Read Privilege Reset Tolerance Register \\#4"]
pub mod gpioc1c;
#[doc = "GPIOC20 (rw) register accessor: Read Privilege Reset Tolerance Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc20`] module"]
#[doc(alias = "GPIOC20")]
pub type Gpioc20 = crate::Reg<gpioc20::Gpioc20Spec>;
#[doc = "Read Privilege Reset Tolerance Register \\#5"]
pub mod gpioc20;
#[doc = "GPIOC24 (rw) register accessor: Read Privilege Reset Tolerance Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc24`] module"]
#[doc(alias = "GPIOC24")]
pub type Gpioc24 = crate::Reg<gpioc24::Gpioc24Spec>;
#[doc = "Read Privilege Reset Tolerance Register \\#6"]
pub mod gpioc24;
#[doc = "GPIOC28 (rw) register accessor: Read Privilege Reset Tolerance Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioc28`] module"]
#[doc(alias = "GPIOC28")]
pub type Gpioc28 = crate::Reg<gpioc28::Gpioc28Spec>;
#[doc = "Read Privilege Reset Tolerance Register \\#7"]
pub mod gpioc28;
#[doc = "GPIOD10 (rw) register accessor: Write Privilege Write Protection Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiod10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod10`] module"]
#[doc(alias = "GPIOD10")]
pub type Gpiod10 = crate::Reg<gpiod10::Gpiod10Spec>;
#[doc = "Write Privilege Write Protection Register \\#1"]
pub mod gpiod10;
#[doc = "GPIOD14 (rw) register accessor: Write Privilege Write Protection Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiod14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod14`] module"]
#[doc(alias = "GPIOD14")]
pub type Gpiod14 = crate::Reg<gpiod14::Gpiod14Spec>;
#[doc = "Write Privilege Write Protection Register \\#2"]
pub mod gpiod14;
#[doc = "GPIOD18 (rw) register accessor: Write Privilege Write Protection Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiod18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod18`] module"]
#[doc(alias = "GPIOD18")]
pub type Gpiod18 = crate::Reg<gpiod18::Gpiod18Spec>;
#[doc = "Write Privilege Write Protection Register \\#3"]
pub mod gpiod18;
#[doc = "GPIOD1C (rw) register accessor: Write Privilege Write Protection Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiod1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod1c`] module"]
#[doc(alias = "GPIOD1C")]
pub type Gpiod1c = crate::Reg<gpiod1c::Gpiod1cSpec>;
#[doc = "Write Privilege Write Protection Register \\#4"]
pub mod gpiod1c;
#[doc = "GPIOD20 (rw) register accessor: Write Privilege Write Protection Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiod20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod20`] module"]
#[doc(alias = "GPIOD20")]
pub type Gpiod20 = crate::Reg<gpiod20::Gpiod20Spec>;
#[doc = "Write Privilege Write Protection Register \\#5"]
pub mod gpiod20;
#[doc = "GPIOD24 (rw) register accessor: Write Privilege Write Protection Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiod24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod24`] module"]
#[doc(alias = "GPIOD24")]
pub type Gpiod24 = crate::Reg<gpiod24::Gpiod24Spec>;
#[doc = "Write Privilege Write Protection Register \\#6"]
pub mod gpiod24;
#[doc = "GPIOD28 (rw) register accessor: Write Privilege Write Protection Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiod28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiod28`] module"]
#[doc(alias = "GPIOD28")]
pub type Gpiod28 = crate::Reg<gpiod28::Gpiod28Spec>;
#[doc = "Write Privilege Write Protection Register \\#7"]
pub mod gpiod28;
#[doc = "GPIOE10 (rw) register accessor: Read Privilege Write Protection Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioe10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe10`] module"]
#[doc(alias = "GPIOE10")]
pub type Gpioe10 = crate::Reg<gpioe10::Gpioe10Spec>;
#[doc = "Read Privilege Write Protection Register \\#1"]
pub mod gpioe10;
#[doc = "GPIOE14 (rw) register accessor: Read Privilege Write Protection Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioe14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe14`] module"]
#[doc(alias = "GPIOE14")]
pub type Gpioe14 = crate::Reg<gpioe14::Gpioe14Spec>;
#[doc = "Read Privilege Write Protection Register \\#2"]
pub mod gpioe14;
#[doc = "GPIOE18 (rw) register accessor: Read Privilege Write Protection Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioe18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe18`] module"]
#[doc(alias = "GPIOE18")]
pub type Gpioe18 = crate::Reg<gpioe18::Gpioe18Spec>;
#[doc = "Read Privilege Write Protection Register \\#3"]
pub mod gpioe18;
#[doc = "GPIOE1C (rw) register accessor: Read Privilege Write Protection Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioe1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe1c`] module"]
#[doc(alias = "GPIOE1C")]
pub type Gpioe1c = crate::Reg<gpioe1c::Gpioe1cSpec>;
#[doc = "Read Privilege Write Protection Register \\#4"]
pub mod gpioe1c;
#[doc = "GPIOE20 (rw) register accessor: Read Privilege Write Protection Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioe20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe20`] module"]
#[doc(alias = "GPIOE20")]
pub type Gpioe20 = crate::Reg<gpioe20::Gpioe20Spec>;
#[doc = "Read Privilege Write Protection Register \\#5"]
pub mod gpioe20;
#[doc = "GPIOE24 (rw) register accessor: Read Privilege Write Protection Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioe24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe24`] module"]
#[doc(alias = "GPIOE24")]
pub type Gpioe24 = crate::Reg<gpioe24::Gpioe24Spec>;
#[doc = "Read Privilege Write Protection Register \\#6"]
pub mod gpioe24;
#[doc = "GPIOE28 (rw) register accessor: Read Privilege Write Protection Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioe28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioe28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioe28`] module"]
#[doc(alias = "GPIOE28")]
pub type Gpioe28 = crate::Reg<gpioe28::Gpioe28Spec>;
#[doc = "Read Privilege Write Protection Register \\#7"]
pub mod gpioe28;
