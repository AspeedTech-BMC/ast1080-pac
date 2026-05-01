#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pric_io000: PricIo000,
    pric_io004: PricIo004,
    pric_io008: PricIo008,
    pric_io00c: PricIo00c,
    pric_io010: PricIo010,
    pric_io014: PricIo014,
    _reserved6: [u8; 0xe8],
    pric_io100: PricIo100,
    pric_io104: PricIo104,
    pric_io108: PricIo108,
    pric_io10c: PricIo10c,
    pric_io110: PricIo110,
    pric_io114: PricIo114,
    _reserved12: [u8; 0xe8],
    pric_io200: PricIo200,
    pric_io204: PricIo204,
    pric_io208: PricIo208,
    pric_io20c: PricIo20c,
    pric_io210: PricIo210,
    pric_io214: PricIo214,
    pric_io218: PricIo218,
    pric_io21c: PricIo21c,
    pric_io220: PricIo220,
    pric_io224: PricIo224,
    pric_io228: PricIo228,
    pric_io22c: PricIo22c,
    pric_io230: PricIo230,
    pric_io234: PricIo234,
    pric_io238: PricIo238,
    pric_io23c: PricIo23c,
    pric_io240: PricIo240,
    pric_io244: PricIo244,
    pric_io248: PricIo248,
    pric_io24c: PricIo24c,
    pric_io250: PricIo250,
    pric_io254: PricIo254,
    pric_io258: PricIo258,
    pric_io25c: PricIo25c,
    pric_io260: PricIo260,
    pric_io264: PricIo264,
    pric_io268: PricIo268,
    pric_io26c: PricIo26c,
    pric_io270: PricIo270,
    pric_io274: PricIo274,
    pric_io278: PricIo278,
    pric_io27c: PricIo27c,
    pric_io280: PricIo280,
    pric_io284: PricIo284,
    pric_io288: PricIo288,
    pric_io28c: PricIo28c,
    pric_io290: PricIo290,
    pric_io294: PricIo294,
    pric_io298: PricIo298,
    pric_io29c: PricIo29c,
    pric_io2a0: PricIo2a0,
    pric_io2a4: PricIo2a4,
    pric_io2a8: PricIo2a8,
    pric_io2ac: PricIo2ac,
    pric_io2b0: PricIo2b0,
    pric_io2b4: PricIo2b4,
    pric_io2b8: PricIo2b8,
    pric_io2bc: PricIo2bc,
    pric_io2c0: PricIo2c0,
    pric_io2c4: PricIo2c4,
    pric_io2c8: PricIo2c8,
    pric_io2cc: PricIo2cc,
    pric_io2d0: PricIo2d0,
    pric_io2d4: PricIo2d4,
    pric_io2d8: PricIo2d8,
    pric_io2dc: PricIo2dc,
    pric_io2e0: PricIo2e0,
    pric_io2e4: PricIo2e4,
    pric_io2e8: PricIo2e8,
    pric_io2ec: PricIo2ec,
    pric_io2f0: PricIo2f0,
    pric_io2f4: PricIo2f4,
    pric_io2f8: PricIo2f8,
    pric_io2fc: PricIo2fc,
    pric_io300: PricIo300,
    pric_io304: PricIo304,
    pric_io308: PricIo308,
    pric_io30c: PricIo30c,
    pric_io310: PricIo310,
    pric_io314: PricIo314,
    pric_io318: PricIo318,
    pric_io31c: PricIo31c,
    pric_io320: PricIo320,
    pric_io324: PricIo324,
    pric_io328: PricIo328,
    pric_io32c: PricIo32c,
    pric_io330: PricIo330,
    pric_io334: PricIo334,
    pric_io338: PricIo338,
    pric_io33c: PricIo33c,
    pric_io340: PricIo340,
    pric_io344: PricIo344,
    pric_io348: PricIo348,
    pric_io34c: PricIo34c,
    pric_io350: PricIo350,
    pric_io354: PricIo354,
    pric_io358: PricIo358,
    pric_io35c: PricIo35c,
    pric_io360: PricIo360,
    pric_io364: PricIo364,
    pric_io368: PricIo368,
    pric_io36c: PricIo36c,
    pric_io370: PricIo370,
    pric_io374: PricIo374,
    pric_io378: PricIo378,
    pric_io37c: PricIo37c,
    pric_io380: PricIo380,
    pric_io384: PricIo384,
    pric_io388: PricIo388,
    pric_io38c: PricIo38c,
    pric_io390: PricIo390,
    pric_io394: PricIo394,
    pric_io398: PricIo398,
    pric_io39c: PricIo39c,
    pric_io3a0: PricIo3a0,
    pric_io3a4: PricIo3a4,
    pric_io3a8: PricIo3a8,
    pric_io3ac: PricIo3ac,
    pric_io3b0: PricIo3b0,
    pric_io3b4: PricIo3b4,
    pric_io3b8: PricIo3b8,
    pric_io3bc: PricIo3bc,
    pric_io3c0: PricIo3c0,
    pric_io3c4: PricIo3c4,
    pric_io3c8: PricIo3c8,
    pric_io3cc: PricIo3cc,
    pric_io3d0: PricIo3d0,
    pric_io3d4: PricIo3d4,
    pric_io3d8: PricIo3d8,
    pric_io3dc: PricIo3dc,
    pric_io3e0: PricIo3e0,
    pric_io3e4: PricIo3e4,
    pric_io3e8: PricIo3e8,
    pric_io3ec: PricIo3ec,
    pric_io3f0: PricIo3f0,
    pric_io3f4: PricIo3f4,
    pric_io3f8: PricIo3f8,
    pric_io3fc: PricIo3fc,
    pric_io400: PricIo400,
    pric_io404: PricIo404,
    pric_io408: PricIo408,
    pric_io40c: PricIo40c,
    pric_io410: PricIo410,
    pric_io414: PricIo414,
    pric_io418: PricIo418,
    pric_io41c: PricIo41c,
    pric_io420: PricIo420,
    pric_io424: PricIo424,
    pric_io428: PricIo428,
    pric_io42c: PricIo42c,
    pric_io430: PricIo430,
    pric_io434: PricIo434,
    pric_io438: PricIo438,
    pric_io43c: PricIo43c,
    pric_io440: PricIo440,
    pric_io444: PricIo444,
    pric_io448: PricIo448,
    pric_io44c: PricIo44c,
    pric_io450: PricIo450,
    pric_io454: PricIo454,
    pric_io458: PricIo458,
    pric_io45c: PricIo45c,
    pric_io460: PricIo460,
    pric_io464: PricIo464,
    pric_io468: PricIo468,
    pric_io46c: PricIo46c,
    pric_io470: PricIo470,
    pric_io474: PricIo474,
    pric_io478: PricIo478,
    pric_io47c: PricIo47c,
    pric_io480: PricIo480,
    pric_io484: PricIo484,
    pric_io488: PricIo488,
    pric_io48c: PricIo48c,
    pric_io490: PricIo490,
    pric_io494: PricIo494,
    pric_io498: PricIo498,
    pric_io49c: PricIo49c,
    pric_io4a0: PricIo4a0,
    pric_io4a4: PricIo4a4,
    pric_io4a8: PricIo4a8,
    pric_io4ac: PricIo4ac,
    pric_io4b0: PricIo4b0,
    pric_io4b4: PricIo4b4,
    pric_io4b8: PricIo4b8,
    pric_io4bc: PricIo4bc,
    pric_io4c0: PricIo4c0,
    pric_io4c4: PricIo4c4,
    pric_io4c8: PricIo4c8,
    pric_io4cc: PricIo4cc,
    pric_io4d0: PricIo4d0,
    pric_io4d4: PricIo4d4,
    pric_io4d8: PricIo4d8,
    pric_io4dc: PricIo4dc,
    pric_io4e0: PricIo4e0,
    pric_io4e4: PricIo4e4,
    pric_io4e8: PricIo4e8,
    pric_io4ec: PricIo4ec,
    pric_io4f0: PricIo4f0,
    pric_io4f4: PricIo4f4,
    pric_io4f8: PricIo4f8,
    pric_io4fc: PricIo4fc,
}
impl RegisterBlock {
    #[doc = "0x00 - Master Write Group Setting Register \\#0"]
    #[inline(always)]
    pub const fn pric_io000(&self) -> &PricIo000 {
        &self.pric_io000
    }
    #[doc = "0x04 - Master Write Group Setting Register \\#1"]
    #[inline(always)]
    pub const fn pric_io004(&self) -> &PricIo004 {
        &self.pric_io004
    }
    #[doc = "0x08 - Master Write Group Setting Register \\#2"]
    #[inline(always)]
    pub const fn pric_io008(&self) -> &PricIo008 {
        &self.pric_io008
    }
    #[doc = "0x0c - Master Write Group Setting Register \\#3"]
    #[inline(always)]
    pub const fn pric_io00c(&self) -> &PricIo00c {
        &self.pric_io00c
    }
    #[doc = "0x10 - Master Write Group Setting Register \\#4"]
    #[inline(always)]
    pub const fn pric_io010(&self) -> &PricIo010 {
        &self.pric_io010
    }
    #[doc = "0x14 - Master Write Group Setting Register \\#5"]
    #[inline(always)]
    pub const fn pric_io014(&self) -> &PricIo014 {
        &self.pric_io014
    }
    #[doc = "0x100 - Master Read Group Setting Register \\#0"]
    #[inline(always)]
    pub const fn pric_io100(&self) -> &PricIo100 {
        &self.pric_io100
    }
    #[doc = "0x104 - Master Read Group Setting Register \\#1"]
    #[inline(always)]
    pub const fn pric_io104(&self) -> &PricIo104 {
        &self.pric_io104
    }
    #[doc = "0x108 - Master Read Group Setting Register \\#2"]
    #[inline(always)]
    pub const fn pric_io108(&self) -> &PricIo108 {
        &self.pric_io108
    }
    #[doc = "0x10c - Master Read Group Setting Register \\#3"]
    #[inline(always)]
    pub const fn pric_io10c(&self) -> &PricIo10c {
        &self.pric_io10c
    }
    #[doc = "0x110 - Master Read Group Setting Register \\#4"]
    #[inline(always)]
    pub const fn pric_io110(&self) -> &PricIo110 {
        &self.pric_io110
    }
    #[doc = "0x114 - Master Read Group Setting Register \\#5"]
    #[inline(always)]
    pub const fn pric_io114(&self) -> &PricIo114 {
        &self.pric_io114
    }
    #[doc = "0x200 - Slave Write Group Setting Register \\#0"]
    #[inline(always)]
    pub const fn pric_io200(&self) -> &PricIo200 {
        &self.pric_io200
    }
    #[doc = "0x204 - Slave Write Group Setting Register \\#1"]
    #[inline(always)]
    pub const fn pric_io204(&self) -> &PricIo204 {
        &self.pric_io204
    }
    #[doc = "0x208 - Slave Write Group Setting Register \\#2"]
    #[inline(always)]
    pub const fn pric_io208(&self) -> &PricIo208 {
        &self.pric_io208
    }
    #[doc = "0x20c - Slave Write Group Setting Register \\#3"]
    #[inline(always)]
    pub const fn pric_io20c(&self) -> &PricIo20c {
        &self.pric_io20c
    }
    #[doc = "0x210 - Slave Write Group Setting Register \\#4"]
    #[inline(always)]
    pub const fn pric_io210(&self) -> &PricIo210 {
        &self.pric_io210
    }
    #[doc = "0x214 - Slave Write Group Setting Register \\#5"]
    #[inline(always)]
    pub const fn pric_io214(&self) -> &PricIo214 {
        &self.pric_io214
    }
    #[doc = "0x218 - Slave Write Group Setting Register \\#6"]
    #[inline(always)]
    pub const fn pric_io218(&self) -> &PricIo218 {
        &self.pric_io218
    }
    #[doc = "0x21c - Slave Write Group Setting Register \\#7"]
    #[inline(always)]
    pub const fn pric_io21c(&self) -> &PricIo21c {
        &self.pric_io21c
    }
    #[doc = "0x220 - Slave Write Group Setting Register \\#8"]
    #[inline(always)]
    pub const fn pric_io220(&self) -> &PricIo220 {
        &self.pric_io220
    }
    #[doc = "0x224 - Slave Write Group Setting Register \\#9"]
    #[inline(always)]
    pub const fn pric_io224(&self) -> &PricIo224 {
        &self.pric_io224
    }
    #[doc = "0x228 - Slave Write Group Setting Register \\#10"]
    #[inline(always)]
    pub const fn pric_io228(&self) -> &PricIo228 {
        &self.pric_io228
    }
    #[doc = "0x22c - Slave Write Group Setting Register \\#11"]
    #[inline(always)]
    pub const fn pric_io22c(&self) -> &PricIo22c {
        &self.pric_io22c
    }
    #[doc = "0x230 - Slave Write Group Setting Register \\#12"]
    #[inline(always)]
    pub const fn pric_io230(&self) -> &PricIo230 {
        &self.pric_io230
    }
    #[doc = "0x234 - Slave Write Group Setting Register \\#13"]
    #[inline(always)]
    pub const fn pric_io234(&self) -> &PricIo234 {
        &self.pric_io234
    }
    #[doc = "0x238 - Slave Write Group Setting Register \\#14"]
    #[inline(always)]
    pub const fn pric_io238(&self) -> &PricIo238 {
        &self.pric_io238
    }
    #[doc = "0x23c - Slave Write Group Setting Register \\#15"]
    #[inline(always)]
    pub const fn pric_io23c(&self) -> &PricIo23c {
        &self.pric_io23c
    }
    #[doc = "0x240 - Slave Write Group Setting Register \\#16"]
    #[inline(always)]
    pub const fn pric_io240(&self) -> &PricIo240 {
        &self.pric_io240
    }
    #[doc = "0x244 - Slave Write Group Setting Register \\#17"]
    #[inline(always)]
    pub const fn pric_io244(&self) -> &PricIo244 {
        &self.pric_io244
    }
    #[doc = "0x248 - Slave Write Group Setting Register \\#18"]
    #[inline(always)]
    pub const fn pric_io248(&self) -> &PricIo248 {
        &self.pric_io248
    }
    #[doc = "0x24c - Slave Write Group Setting Register \\#19"]
    #[inline(always)]
    pub const fn pric_io24c(&self) -> &PricIo24c {
        &self.pric_io24c
    }
    #[doc = "0x250 - Slave Write Group Setting Register \\#20"]
    #[inline(always)]
    pub const fn pric_io250(&self) -> &PricIo250 {
        &self.pric_io250
    }
    #[doc = "0x254 - Slave Write Group Setting Register \\#21"]
    #[inline(always)]
    pub const fn pric_io254(&self) -> &PricIo254 {
        &self.pric_io254
    }
    #[doc = "0x258 - Slave Write Group Setting Register \\#22"]
    #[inline(always)]
    pub const fn pric_io258(&self) -> &PricIo258 {
        &self.pric_io258
    }
    #[doc = "0x25c - Slave Write Group Setting Register \\#23"]
    #[inline(always)]
    pub const fn pric_io25c(&self) -> &PricIo25c {
        &self.pric_io25c
    }
    #[doc = "0x260 - Slave Write Group Setting Register \\#24"]
    #[inline(always)]
    pub const fn pric_io260(&self) -> &PricIo260 {
        &self.pric_io260
    }
    #[doc = "0x264 - Slave Write Group Setting Register \\#25"]
    #[inline(always)]
    pub const fn pric_io264(&self) -> &PricIo264 {
        &self.pric_io264
    }
    #[doc = "0x268 - Slave Write Group Setting Register \\#26"]
    #[inline(always)]
    pub const fn pric_io268(&self) -> &PricIo268 {
        &self.pric_io268
    }
    #[doc = "0x26c - Slave Write Group Setting Register \\#27"]
    #[inline(always)]
    pub const fn pric_io26c(&self) -> &PricIo26c {
        &self.pric_io26c
    }
    #[doc = "0x270 - Slave Write Group Setting Register \\#28"]
    #[inline(always)]
    pub const fn pric_io270(&self) -> &PricIo270 {
        &self.pric_io270
    }
    #[doc = "0x274 - Slave Write Group Setting Register \\#29"]
    #[inline(always)]
    pub const fn pric_io274(&self) -> &PricIo274 {
        &self.pric_io274
    }
    #[doc = "0x278 - Slave Write Group Setting Register \\#30"]
    #[inline(always)]
    pub const fn pric_io278(&self) -> &PricIo278 {
        &self.pric_io278
    }
    #[doc = "0x27c - Slave Write Group Setting Register \\#31"]
    #[inline(always)]
    pub const fn pric_io27c(&self) -> &PricIo27c {
        &self.pric_io27c
    }
    #[doc = "0x280 - Slave Write Group Setting Register \\#32"]
    #[inline(always)]
    pub const fn pric_io280(&self) -> &PricIo280 {
        &self.pric_io280
    }
    #[doc = "0x284 - Slave Write Group Setting Register \\#33"]
    #[inline(always)]
    pub const fn pric_io284(&self) -> &PricIo284 {
        &self.pric_io284
    }
    #[doc = "0x288 - Slave Write Group Setting Register \\#34"]
    #[inline(always)]
    pub const fn pric_io288(&self) -> &PricIo288 {
        &self.pric_io288
    }
    #[doc = "0x28c - Slave Write Group Setting Register \\#35"]
    #[inline(always)]
    pub const fn pric_io28c(&self) -> &PricIo28c {
        &self.pric_io28c
    }
    #[doc = "0x290 - Slave Write Group Setting Register \\#36"]
    #[inline(always)]
    pub const fn pric_io290(&self) -> &PricIo290 {
        &self.pric_io290
    }
    #[doc = "0x294 - Slave Write Group Setting Register \\#37"]
    #[inline(always)]
    pub const fn pric_io294(&self) -> &PricIo294 {
        &self.pric_io294
    }
    #[doc = "0x298 - Slave Write Group Setting Register \\#38"]
    #[inline(always)]
    pub const fn pric_io298(&self) -> &PricIo298 {
        &self.pric_io298
    }
    #[doc = "0x29c - Slave Write Group Setting Register \\#39"]
    #[inline(always)]
    pub const fn pric_io29c(&self) -> &PricIo29c {
        &self.pric_io29c
    }
    #[doc = "0x2a0 - Slave Write Group Setting Register \\#40"]
    #[inline(always)]
    pub const fn pric_io2a0(&self) -> &PricIo2a0 {
        &self.pric_io2a0
    }
    #[doc = "0x2a4 - Slave Write Group Setting Register \\#41"]
    #[inline(always)]
    pub const fn pric_io2a4(&self) -> &PricIo2a4 {
        &self.pric_io2a4
    }
    #[doc = "0x2a8 - Slave Write Group Setting Register \\#42"]
    #[inline(always)]
    pub const fn pric_io2a8(&self) -> &PricIo2a8 {
        &self.pric_io2a8
    }
    #[doc = "0x2ac - Slave Write Group Setting Register \\#43"]
    #[inline(always)]
    pub const fn pric_io2ac(&self) -> &PricIo2ac {
        &self.pric_io2ac
    }
    #[doc = "0x2b0 - Slave Write Group Setting Register \\#44"]
    #[inline(always)]
    pub const fn pric_io2b0(&self) -> &PricIo2b0 {
        &self.pric_io2b0
    }
    #[doc = "0x2b4 - Slave Write Group Setting Register \\#45"]
    #[inline(always)]
    pub const fn pric_io2b4(&self) -> &PricIo2b4 {
        &self.pric_io2b4
    }
    #[doc = "0x2b8 - Slave Write Group Setting Register \\#46"]
    #[inline(always)]
    pub const fn pric_io2b8(&self) -> &PricIo2b8 {
        &self.pric_io2b8
    }
    #[doc = "0x2bc - Slave Write Group Setting Register \\#47"]
    #[inline(always)]
    pub const fn pric_io2bc(&self) -> &PricIo2bc {
        &self.pric_io2bc
    }
    #[doc = "0x2c0 - Slave Write Group Setting Register \\#48"]
    #[inline(always)]
    pub const fn pric_io2c0(&self) -> &PricIo2c0 {
        &self.pric_io2c0
    }
    #[doc = "0x2c4 - Slave Write Group Setting Register \\#49"]
    #[inline(always)]
    pub const fn pric_io2c4(&self) -> &PricIo2c4 {
        &self.pric_io2c4
    }
    #[doc = "0x2c8 - Slave Write Group Setting Register \\#50"]
    #[inline(always)]
    pub const fn pric_io2c8(&self) -> &PricIo2c8 {
        &self.pric_io2c8
    }
    #[doc = "0x2cc - Slave Write Group Setting Register \\#51"]
    #[inline(always)]
    pub const fn pric_io2cc(&self) -> &PricIo2cc {
        &self.pric_io2cc
    }
    #[doc = "0x2d0 - Slave Write Group Setting Register \\#52"]
    #[inline(always)]
    pub const fn pric_io2d0(&self) -> &PricIo2d0 {
        &self.pric_io2d0
    }
    #[doc = "0x2d4 - Slave Write Group Setting Register \\#53"]
    #[inline(always)]
    pub const fn pric_io2d4(&self) -> &PricIo2d4 {
        &self.pric_io2d4
    }
    #[doc = "0x2d8 - Slave Write Group Setting Register \\#54"]
    #[inline(always)]
    pub const fn pric_io2d8(&self) -> &PricIo2d8 {
        &self.pric_io2d8
    }
    #[doc = "0x2dc - Slave Write Group Setting Register \\#55"]
    #[inline(always)]
    pub const fn pric_io2dc(&self) -> &PricIo2dc {
        &self.pric_io2dc
    }
    #[doc = "0x2e0 - Slave Write Group Setting Register \\#56"]
    #[inline(always)]
    pub const fn pric_io2e0(&self) -> &PricIo2e0 {
        &self.pric_io2e0
    }
    #[doc = "0x2e4 - Slave Write Group Setting Register \\#57"]
    #[inline(always)]
    pub const fn pric_io2e4(&self) -> &PricIo2e4 {
        &self.pric_io2e4
    }
    #[doc = "0x2e8 - Slave Write Group Setting Register \\#58"]
    #[inline(always)]
    pub const fn pric_io2e8(&self) -> &PricIo2e8 {
        &self.pric_io2e8
    }
    #[doc = "0x2ec - Slave Write Group Setting Register \\#59"]
    #[inline(always)]
    pub const fn pric_io2ec(&self) -> &PricIo2ec {
        &self.pric_io2ec
    }
    #[doc = "0x2f0 - Slave Write Group Setting Register \\#60"]
    #[inline(always)]
    pub const fn pric_io2f0(&self) -> &PricIo2f0 {
        &self.pric_io2f0
    }
    #[doc = "0x2f4 - Slave Write Group Setting Register \\#61"]
    #[inline(always)]
    pub const fn pric_io2f4(&self) -> &PricIo2f4 {
        &self.pric_io2f4
    }
    #[doc = "0x2f8 - Slave Write Group Setting Register \\#62"]
    #[inline(always)]
    pub const fn pric_io2f8(&self) -> &PricIo2f8 {
        &self.pric_io2f8
    }
    #[doc = "0x2fc - Slave Write Group Setting Register \\#63"]
    #[inline(always)]
    pub const fn pric_io2fc(&self) -> &PricIo2fc {
        &self.pric_io2fc
    }
    #[doc = "0x300 - Slave Read Group Setting Register \\#0"]
    #[inline(always)]
    pub const fn pric_io300(&self) -> &PricIo300 {
        &self.pric_io300
    }
    #[doc = "0x304 - Slave Read Group Setting Register \\#1"]
    #[inline(always)]
    pub const fn pric_io304(&self) -> &PricIo304 {
        &self.pric_io304
    }
    #[doc = "0x308 - Slave Read Group Setting Register \\#2"]
    #[inline(always)]
    pub const fn pric_io308(&self) -> &PricIo308 {
        &self.pric_io308
    }
    #[doc = "0x30c - Slave Read Group Setting Register \\#3"]
    #[inline(always)]
    pub const fn pric_io30c(&self) -> &PricIo30c {
        &self.pric_io30c
    }
    #[doc = "0x310 - Slave Read Group Setting Register \\#4"]
    #[inline(always)]
    pub const fn pric_io310(&self) -> &PricIo310 {
        &self.pric_io310
    }
    #[doc = "0x314 - Slave Read Group Setting Register \\#5"]
    #[inline(always)]
    pub const fn pric_io314(&self) -> &PricIo314 {
        &self.pric_io314
    }
    #[doc = "0x318 - Slave Read Group Setting Register \\#6"]
    #[inline(always)]
    pub const fn pric_io318(&self) -> &PricIo318 {
        &self.pric_io318
    }
    #[doc = "0x31c - Slave Read Group Setting Register \\#7"]
    #[inline(always)]
    pub const fn pric_io31c(&self) -> &PricIo31c {
        &self.pric_io31c
    }
    #[doc = "0x320 - Slave Read Group Setting Register \\#8"]
    #[inline(always)]
    pub const fn pric_io320(&self) -> &PricIo320 {
        &self.pric_io320
    }
    #[doc = "0x324 - Slave Read Group Setting Register \\#9"]
    #[inline(always)]
    pub const fn pric_io324(&self) -> &PricIo324 {
        &self.pric_io324
    }
    #[doc = "0x328 - Slave Read Group Setting Register \\#10"]
    #[inline(always)]
    pub const fn pric_io328(&self) -> &PricIo328 {
        &self.pric_io328
    }
    #[doc = "0x32c - Slave Read Group Setting Register \\#11"]
    #[inline(always)]
    pub const fn pric_io32c(&self) -> &PricIo32c {
        &self.pric_io32c
    }
    #[doc = "0x330 - Slave Read Group Setting Register \\#12"]
    #[inline(always)]
    pub const fn pric_io330(&self) -> &PricIo330 {
        &self.pric_io330
    }
    #[doc = "0x334 - Slave Read Group Setting Register \\#13"]
    #[inline(always)]
    pub const fn pric_io334(&self) -> &PricIo334 {
        &self.pric_io334
    }
    #[doc = "0x338 - Slave Read Group Setting Register \\#14"]
    #[inline(always)]
    pub const fn pric_io338(&self) -> &PricIo338 {
        &self.pric_io338
    }
    #[doc = "0x33c - Slave Read Group Setting Register \\#15"]
    #[inline(always)]
    pub const fn pric_io33c(&self) -> &PricIo33c {
        &self.pric_io33c
    }
    #[doc = "0x340 - Slave Read Group Setting Register \\#16"]
    #[inline(always)]
    pub const fn pric_io340(&self) -> &PricIo340 {
        &self.pric_io340
    }
    #[doc = "0x344 - Slave Read Group Setting Register \\#17"]
    #[inline(always)]
    pub const fn pric_io344(&self) -> &PricIo344 {
        &self.pric_io344
    }
    #[doc = "0x348 - Slave Read Group Setting Register \\#18"]
    #[inline(always)]
    pub const fn pric_io348(&self) -> &PricIo348 {
        &self.pric_io348
    }
    #[doc = "0x34c - Slave Read Group Setting Register \\#19"]
    #[inline(always)]
    pub const fn pric_io34c(&self) -> &PricIo34c {
        &self.pric_io34c
    }
    #[doc = "0x350 - Slave Read Group Setting Register \\#20"]
    #[inline(always)]
    pub const fn pric_io350(&self) -> &PricIo350 {
        &self.pric_io350
    }
    #[doc = "0x354 - Slave Read Group Setting Register \\#21"]
    #[inline(always)]
    pub const fn pric_io354(&self) -> &PricIo354 {
        &self.pric_io354
    }
    #[doc = "0x358 - Slave Read Group Setting Register \\#22"]
    #[inline(always)]
    pub const fn pric_io358(&self) -> &PricIo358 {
        &self.pric_io358
    }
    #[doc = "0x35c - Slave Read Group Setting Register \\#23"]
    #[inline(always)]
    pub const fn pric_io35c(&self) -> &PricIo35c {
        &self.pric_io35c
    }
    #[doc = "0x360 - Slave Read Group Setting Register \\#24"]
    #[inline(always)]
    pub const fn pric_io360(&self) -> &PricIo360 {
        &self.pric_io360
    }
    #[doc = "0x364 - Slave Read Group Setting Register \\#25"]
    #[inline(always)]
    pub const fn pric_io364(&self) -> &PricIo364 {
        &self.pric_io364
    }
    #[doc = "0x368 - Slave Read Group Setting Register \\#26"]
    #[inline(always)]
    pub const fn pric_io368(&self) -> &PricIo368 {
        &self.pric_io368
    }
    #[doc = "0x36c - Slave Read Group Setting Register \\#27"]
    #[inline(always)]
    pub const fn pric_io36c(&self) -> &PricIo36c {
        &self.pric_io36c
    }
    #[doc = "0x370 - Slave Read Group Setting Register \\#28"]
    #[inline(always)]
    pub const fn pric_io370(&self) -> &PricIo370 {
        &self.pric_io370
    }
    #[doc = "0x374 - Slave Read Group Setting Register \\#29"]
    #[inline(always)]
    pub const fn pric_io374(&self) -> &PricIo374 {
        &self.pric_io374
    }
    #[doc = "0x378 - Slave Read Group Setting Register \\#30"]
    #[inline(always)]
    pub const fn pric_io378(&self) -> &PricIo378 {
        &self.pric_io378
    }
    #[doc = "0x37c - Slave Read Group Setting Register \\#31"]
    #[inline(always)]
    pub const fn pric_io37c(&self) -> &PricIo37c {
        &self.pric_io37c
    }
    #[doc = "0x380 - Slave Read Group Setting Register \\#32"]
    #[inline(always)]
    pub const fn pric_io380(&self) -> &PricIo380 {
        &self.pric_io380
    }
    #[doc = "0x384 - Slave Read Group Setting Register \\#33"]
    #[inline(always)]
    pub const fn pric_io384(&self) -> &PricIo384 {
        &self.pric_io384
    }
    #[doc = "0x388 - Slave Read Group Setting Register \\#34"]
    #[inline(always)]
    pub const fn pric_io388(&self) -> &PricIo388 {
        &self.pric_io388
    }
    #[doc = "0x38c - Slave Read Group Setting Register \\#35"]
    #[inline(always)]
    pub const fn pric_io38c(&self) -> &PricIo38c {
        &self.pric_io38c
    }
    #[doc = "0x390 - Slave Read Group Setting Register \\#36"]
    #[inline(always)]
    pub const fn pric_io390(&self) -> &PricIo390 {
        &self.pric_io390
    }
    #[doc = "0x394 - Slave Read Group Setting Register \\#37"]
    #[inline(always)]
    pub const fn pric_io394(&self) -> &PricIo394 {
        &self.pric_io394
    }
    #[doc = "0x398 - Slave Read Group Setting Register \\#38"]
    #[inline(always)]
    pub const fn pric_io398(&self) -> &PricIo398 {
        &self.pric_io398
    }
    #[doc = "0x39c - Slave Read Group Setting Register \\#39"]
    #[inline(always)]
    pub const fn pric_io39c(&self) -> &PricIo39c {
        &self.pric_io39c
    }
    #[doc = "0x3a0 - Slave Read Group Setting Register \\#40"]
    #[inline(always)]
    pub const fn pric_io3a0(&self) -> &PricIo3a0 {
        &self.pric_io3a0
    }
    #[doc = "0x3a4 - Slave Read Group Setting Register \\#41"]
    #[inline(always)]
    pub const fn pric_io3a4(&self) -> &PricIo3a4 {
        &self.pric_io3a4
    }
    #[doc = "0x3a8 - Slave Read Group Setting Register \\#42"]
    #[inline(always)]
    pub const fn pric_io3a8(&self) -> &PricIo3a8 {
        &self.pric_io3a8
    }
    #[doc = "0x3ac - Slave Read Group Setting Register \\#43"]
    #[inline(always)]
    pub const fn pric_io3ac(&self) -> &PricIo3ac {
        &self.pric_io3ac
    }
    #[doc = "0x3b0 - Slave Read Group Setting Register \\#44"]
    #[inline(always)]
    pub const fn pric_io3b0(&self) -> &PricIo3b0 {
        &self.pric_io3b0
    }
    #[doc = "0x3b4 - Slave Read Group Setting Register \\#45"]
    #[inline(always)]
    pub const fn pric_io3b4(&self) -> &PricIo3b4 {
        &self.pric_io3b4
    }
    #[doc = "0x3b8 - Slave Read Group Setting Register \\#46"]
    #[inline(always)]
    pub const fn pric_io3b8(&self) -> &PricIo3b8 {
        &self.pric_io3b8
    }
    #[doc = "0x3bc - Slave Read Group Setting Register \\#47"]
    #[inline(always)]
    pub const fn pric_io3bc(&self) -> &PricIo3bc {
        &self.pric_io3bc
    }
    #[doc = "0x3c0 - Slave Read Group Setting Register \\#48"]
    #[inline(always)]
    pub const fn pric_io3c0(&self) -> &PricIo3c0 {
        &self.pric_io3c0
    }
    #[doc = "0x3c4 - Slave Read Group Setting Register \\#49"]
    #[inline(always)]
    pub const fn pric_io3c4(&self) -> &PricIo3c4 {
        &self.pric_io3c4
    }
    #[doc = "0x3c8 - Slave Read Group Setting Register \\#50"]
    #[inline(always)]
    pub const fn pric_io3c8(&self) -> &PricIo3c8 {
        &self.pric_io3c8
    }
    #[doc = "0x3cc - Slave Read Group Setting Register \\#51"]
    #[inline(always)]
    pub const fn pric_io3cc(&self) -> &PricIo3cc {
        &self.pric_io3cc
    }
    #[doc = "0x3d0 - Slave Read Group Setting Register \\#52"]
    #[inline(always)]
    pub const fn pric_io3d0(&self) -> &PricIo3d0 {
        &self.pric_io3d0
    }
    #[doc = "0x3d4 - Slave Read Group Setting Register \\#53"]
    #[inline(always)]
    pub const fn pric_io3d4(&self) -> &PricIo3d4 {
        &self.pric_io3d4
    }
    #[doc = "0x3d8 - Slave Read Group Setting Register \\#54"]
    #[inline(always)]
    pub const fn pric_io3d8(&self) -> &PricIo3d8 {
        &self.pric_io3d8
    }
    #[doc = "0x3dc - Slave Read Group Setting Register \\#55"]
    #[inline(always)]
    pub const fn pric_io3dc(&self) -> &PricIo3dc {
        &self.pric_io3dc
    }
    #[doc = "0x3e0 - Slave Read Group Setting Register \\#56"]
    #[inline(always)]
    pub const fn pric_io3e0(&self) -> &PricIo3e0 {
        &self.pric_io3e0
    }
    #[doc = "0x3e4 - Slave Read Group Setting Register \\#57"]
    #[inline(always)]
    pub const fn pric_io3e4(&self) -> &PricIo3e4 {
        &self.pric_io3e4
    }
    #[doc = "0x3e8 - Slave Read Group Setting Register \\#58"]
    #[inline(always)]
    pub const fn pric_io3e8(&self) -> &PricIo3e8 {
        &self.pric_io3e8
    }
    #[doc = "0x3ec - Slave Read Group Setting Register \\#59"]
    #[inline(always)]
    pub const fn pric_io3ec(&self) -> &PricIo3ec {
        &self.pric_io3ec
    }
    #[doc = "0x3f0 - Slave Read Group Setting Register \\#60"]
    #[inline(always)]
    pub const fn pric_io3f0(&self) -> &PricIo3f0 {
        &self.pric_io3f0
    }
    #[doc = "0x3f4 - Slave Read Group Setting Register \\#61"]
    #[inline(always)]
    pub const fn pric_io3f4(&self) -> &PricIo3f4 {
        &self.pric_io3f4
    }
    #[doc = "0x3f8 - Slave Read Group Setting Register \\#62"]
    #[inline(always)]
    pub const fn pric_io3f8(&self) -> &PricIo3f8 {
        &self.pric_io3f8
    }
    #[doc = "0x3fc - Slave Read Group Setting Register \\#63"]
    #[inline(always)]
    pub const fn pric_io3fc(&self) -> &PricIo3fc {
        &self.pric_io3fc
    }
    #[doc = "0x400 - Memory Region Proection for AHB Masters Register \\#0"]
    #[inline(always)]
    pub const fn pric_io400(&self) -> &PricIo400 {
        &self.pric_io400
    }
    #[doc = "0x404 - Memory Region Proection for AHB Masters Register \\#1"]
    #[inline(always)]
    pub const fn pric_io404(&self) -> &PricIo404 {
        &self.pric_io404
    }
    #[doc = "0x408 - Memory Region Proection for AHB Masters Register \\#2"]
    #[inline(always)]
    pub const fn pric_io408(&self) -> &PricIo408 {
        &self.pric_io408
    }
    #[doc = "0x40c - Memory Region Proection for AHB Masters Register \\#3"]
    #[inline(always)]
    pub const fn pric_io40c(&self) -> &PricIo40c {
        &self.pric_io40c
    }
    #[doc = "0x410 - Memory Region Proection for AHB Masters Register \\#4"]
    #[inline(always)]
    pub const fn pric_io410(&self) -> &PricIo410 {
        &self.pric_io410
    }
    #[doc = "0x414 - Memory Region Proection for AHB Masters Register \\#5"]
    #[inline(always)]
    pub const fn pric_io414(&self) -> &PricIo414 {
        &self.pric_io414
    }
    #[doc = "0x418 - Memory Region Proection for AHB Masters Register \\#6"]
    #[inline(always)]
    pub const fn pric_io418(&self) -> &PricIo418 {
        &self.pric_io418
    }
    #[doc = "0x41c - Memory Region Proection for AHB Masters Register \\#7"]
    #[inline(always)]
    pub const fn pric_io41c(&self) -> &PricIo41c {
        &self.pric_io41c
    }
    #[doc = "0x420 - Memory Region Proection for AHB Masters Register \\#8"]
    #[inline(always)]
    pub const fn pric_io420(&self) -> &PricIo420 {
        &self.pric_io420
    }
    #[doc = "0x424 - Memory Region Proection for AHB Masters Register \\#9"]
    #[inline(always)]
    pub const fn pric_io424(&self) -> &PricIo424 {
        &self.pric_io424
    }
    #[doc = "0x428 - Memory Region Proection for AHB Masters Register \\#10"]
    #[inline(always)]
    pub const fn pric_io428(&self) -> &PricIo428 {
        &self.pric_io428
    }
    #[doc = "0x42c - Memory Region Proection for AHB Masters Register \\#11"]
    #[inline(always)]
    pub const fn pric_io42c(&self) -> &PricIo42c {
        &self.pric_io42c
    }
    #[doc = "0x430 - Memory Region Proection for AHB Masters Register \\#12"]
    #[inline(always)]
    pub const fn pric_io430(&self) -> &PricIo430 {
        &self.pric_io430
    }
    #[doc = "0x434 - Memory Region Proection for AHB Masters Register \\#13"]
    #[inline(always)]
    pub const fn pric_io434(&self) -> &PricIo434 {
        &self.pric_io434
    }
    #[doc = "0x438 - Memory Region Proection for AHB Masters Register \\#14"]
    #[inline(always)]
    pub const fn pric_io438(&self) -> &PricIo438 {
        &self.pric_io438
    }
    #[doc = "0x43c - Memory Region Proection for AHB Masters Register \\#15"]
    #[inline(always)]
    pub const fn pric_io43c(&self) -> &PricIo43c {
        &self.pric_io43c
    }
    #[doc = "0x440 - Memory Region Proection for AHB Masters Register \\#16"]
    #[inline(always)]
    pub const fn pric_io440(&self) -> &PricIo440 {
        &self.pric_io440
    }
    #[doc = "0x444 - Memory Region Proection for AHB Masters Register \\#17"]
    #[inline(always)]
    pub const fn pric_io444(&self) -> &PricIo444 {
        &self.pric_io444
    }
    #[doc = "0x448 - Memory Region Proection for AHB Masters Register \\#18"]
    #[inline(always)]
    pub const fn pric_io448(&self) -> &PricIo448 {
        &self.pric_io448
    }
    #[doc = "0x44c - Memory Region Proection for AHB Masters Register \\#19"]
    #[inline(always)]
    pub const fn pric_io44c(&self) -> &PricIo44c {
        &self.pric_io44c
    }
    #[doc = "0x450 - Memory Region Proection for AHB Masters Register \\#20"]
    #[inline(always)]
    pub const fn pric_io450(&self) -> &PricIo450 {
        &self.pric_io450
    }
    #[doc = "0x454 - Memory Region Proection for AHB Masters Register \\#21"]
    #[inline(always)]
    pub const fn pric_io454(&self) -> &PricIo454 {
        &self.pric_io454
    }
    #[doc = "0x458 - Memory Region Proection for AHB Masters Register \\#22"]
    #[inline(always)]
    pub const fn pric_io458(&self) -> &PricIo458 {
        &self.pric_io458
    }
    #[doc = "0x45c - Memory Region Proection for AHB Masters Register \\#23"]
    #[inline(always)]
    pub const fn pric_io45c(&self) -> &PricIo45c {
        &self.pric_io45c
    }
    #[doc = "0x460 - Memory Region Proection for AHB Masters Register \\#24"]
    #[inline(always)]
    pub const fn pric_io460(&self) -> &PricIo460 {
        &self.pric_io460
    }
    #[doc = "0x464 - Memory Region Proection for AHB Masters Register \\#25"]
    #[inline(always)]
    pub const fn pric_io464(&self) -> &PricIo464 {
        &self.pric_io464
    }
    #[doc = "0x468 - Memory Region Proection for AHB Masters Register \\#26"]
    #[inline(always)]
    pub const fn pric_io468(&self) -> &PricIo468 {
        &self.pric_io468
    }
    #[doc = "0x46c - Memory Region Proection for AHB Masters Register \\#27"]
    #[inline(always)]
    pub const fn pric_io46c(&self) -> &PricIo46c {
        &self.pric_io46c
    }
    #[doc = "0x470 - Memory Region Proection for AHB Masters Register \\#28"]
    #[inline(always)]
    pub const fn pric_io470(&self) -> &PricIo470 {
        &self.pric_io470
    }
    #[doc = "0x474 - Memory Region Proection for AHB Masters Register \\#29"]
    #[inline(always)]
    pub const fn pric_io474(&self) -> &PricIo474 {
        &self.pric_io474
    }
    #[doc = "0x478 - Memory Region Proection for AHB Masters Register \\#20"]
    #[inline(always)]
    pub const fn pric_io478(&self) -> &PricIo478 {
        &self.pric_io478
    }
    #[doc = "0x47c - Memory Region Proection for AHB Masters Register \\#21"]
    #[inline(always)]
    pub const fn pric_io47c(&self) -> &PricIo47c {
        &self.pric_io47c
    }
    #[doc = "0x480 - Memory Region Proection for AHB Masters Register \\#32"]
    #[inline(always)]
    pub const fn pric_io480(&self) -> &PricIo480 {
        &self.pric_io480
    }
    #[doc = "0x484 - Memory Region Proection for AHB Masters Register \\#33"]
    #[inline(always)]
    pub const fn pric_io484(&self) -> &PricIo484 {
        &self.pric_io484
    }
    #[doc = "0x488 - Memory Region Proection for AHB Masters Register \\#34"]
    #[inline(always)]
    pub const fn pric_io488(&self) -> &PricIo488 {
        &self.pric_io488
    }
    #[doc = "0x48c - Memory Region Proection for AHB Masters Register \\#35"]
    #[inline(always)]
    pub const fn pric_io48c(&self) -> &PricIo48c {
        &self.pric_io48c
    }
    #[doc = "0x490 - Memory Region Proection for AHB Masters Register \\#36"]
    #[inline(always)]
    pub const fn pric_io490(&self) -> &PricIo490 {
        &self.pric_io490
    }
    #[doc = "0x494 - Memory Region Proection for AHB Masters Register \\#37"]
    #[inline(always)]
    pub const fn pric_io494(&self) -> &PricIo494 {
        &self.pric_io494
    }
    #[doc = "0x498 - Memory Region Proection for AHB Masters Register \\#38"]
    #[inline(always)]
    pub const fn pric_io498(&self) -> &PricIo498 {
        &self.pric_io498
    }
    #[doc = "0x49c - Memory Region Proection for AHB Masters Register \\#39"]
    #[inline(always)]
    pub const fn pric_io49c(&self) -> &PricIo49c {
        &self.pric_io49c
    }
    #[doc = "0x4a0 - Memory Region Proection for AHB Masters Register \\#40"]
    #[inline(always)]
    pub const fn pric_io4a0(&self) -> &PricIo4a0 {
        &self.pric_io4a0
    }
    #[doc = "0x4a4 - Memory Region Proection for AHB Masters Register \\#41"]
    #[inline(always)]
    pub const fn pric_io4a4(&self) -> &PricIo4a4 {
        &self.pric_io4a4
    }
    #[doc = "0x4a8 - Memory Region Proection for AHB Masters Register \\#42"]
    #[inline(always)]
    pub const fn pric_io4a8(&self) -> &PricIo4a8 {
        &self.pric_io4a8
    }
    #[doc = "0x4ac - Memory Region Proection for AHB Masters Register \\#43"]
    #[inline(always)]
    pub const fn pric_io4ac(&self) -> &PricIo4ac {
        &self.pric_io4ac
    }
    #[doc = "0x4b0 - Memory Region Proection for AHB Masters Register \\#44"]
    #[inline(always)]
    pub const fn pric_io4b0(&self) -> &PricIo4b0 {
        &self.pric_io4b0
    }
    #[doc = "0x4b4 - Memory Region Proection for AHB Masters Register \\#45"]
    #[inline(always)]
    pub const fn pric_io4b4(&self) -> &PricIo4b4 {
        &self.pric_io4b4
    }
    #[doc = "0x4b8 - Memory Region Proection for AHB Masters Register \\#46"]
    #[inline(always)]
    pub const fn pric_io4b8(&self) -> &PricIo4b8 {
        &self.pric_io4b8
    }
    #[doc = "0x4bc - Memory Region Proection for AHB Masters Register \\#47"]
    #[inline(always)]
    pub const fn pric_io4bc(&self) -> &PricIo4bc {
        &self.pric_io4bc
    }
    #[doc = "0x4c0 - Memory Region Proection for AHB Masters Register \\#48"]
    #[inline(always)]
    pub const fn pric_io4c0(&self) -> &PricIo4c0 {
        &self.pric_io4c0
    }
    #[doc = "0x4c4 - Memory Region Proection for AHB Masters Register \\#49"]
    #[inline(always)]
    pub const fn pric_io4c4(&self) -> &PricIo4c4 {
        &self.pric_io4c4
    }
    #[doc = "0x4c8 - Memory Region Proection for AHB Masters Register \\#40"]
    #[inline(always)]
    pub const fn pric_io4c8(&self) -> &PricIo4c8 {
        &self.pric_io4c8
    }
    #[doc = "0x4cc - Memory Region Proection for AHB Masters Register \\#41"]
    #[inline(always)]
    pub const fn pric_io4cc(&self) -> &PricIo4cc {
        &self.pric_io4cc
    }
    #[doc = "0x4d0 - Memory Region Proection for AHB Masters Register \\#52"]
    #[inline(always)]
    pub const fn pric_io4d0(&self) -> &PricIo4d0 {
        &self.pric_io4d0
    }
    #[doc = "0x4d4 - Memory Region Proection for AHB Masters Register \\#53"]
    #[inline(always)]
    pub const fn pric_io4d4(&self) -> &PricIo4d4 {
        &self.pric_io4d4
    }
    #[doc = "0x4d8 - Memory Region Proection for AHB Masters Register \\#54"]
    #[inline(always)]
    pub const fn pric_io4d8(&self) -> &PricIo4d8 {
        &self.pric_io4d8
    }
    #[doc = "0x4dc - Memory Region Proection for AHB Masters Register \\#55"]
    #[inline(always)]
    pub const fn pric_io4dc(&self) -> &PricIo4dc {
        &self.pric_io4dc
    }
    #[doc = "0x4e0 - Memory Region Proection for AHB Masters Register \\#56"]
    #[inline(always)]
    pub const fn pric_io4e0(&self) -> &PricIo4e0 {
        &self.pric_io4e0
    }
    #[doc = "0x4e4 - Memory Region Proection for AHB Masters Register \\#57"]
    #[inline(always)]
    pub const fn pric_io4e4(&self) -> &PricIo4e4 {
        &self.pric_io4e4
    }
    #[doc = "0x4e8 - Memory Region Proection for AHB Masters Register \\#58"]
    #[inline(always)]
    pub const fn pric_io4e8(&self) -> &PricIo4e8 {
        &self.pric_io4e8
    }
    #[doc = "0x4ec - Memory Region Proection for AHB Masters Register \\#59"]
    #[inline(always)]
    pub const fn pric_io4ec(&self) -> &PricIo4ec {
        &self.pric_io4ec
    }
    #[doc = "0x4f0 - Memory Region Proection for AHB Masters Register \\#60"]
    #[inline(always)]
    pub const fn pric_io4f0(&self) -> &PricIo4f0 {
        &self.pric_io4f0
    }
    #[doc = "0x4f4 - Memory Region Proection for AHB Masters Register \\#61"]
    #[inline(always)]
    pub const fn pric_io4f4(&self) -> &PricIo4f4 {
        &self.pric_io4f4
    }
    #[doc = "0x4f8 - Memory Region Proection for AHB Masters Register \\#62"]
    #[inline(always)]
    pub const fn pric_io4f8(&self) -> &PricIo4f8 {
        &self.pric_io4f8
    }
    #[doc = "0x4fc - Memory Region Proection for AHB Masters Register \\#63"]
    #[inline(always)]
    pub const fn pric_io4fc(&self) -> &PricIo4fc {
        &self.pric_io4fc
    }
}
#[doc = "PRIC_IO000 (rw) register accessor: Master Write Group Setting Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io000`] module"]
#[doc(alias = "PRIC_IO000")]
pub type PricIo000 = crate::Reg<pric_io000::PricIo000Spec>;
#[doc = "Master Write Group Setting Register \\#0"]
pub mod pric_io000;
#[doc = "PRIC_IO004 (rw) register accessor: Master Write Group Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io004`] module"]
#[doc(alias = "PRIC_IO004")]
pub type PricIo004 = crate::Reg<pric_io004::PricIo004Spec>;
#[doc = "Master Write Group Setting Register \\#1"]
pub mod pric_io004;
#[doc = "PRIC_IO008 (rw) register accessor: Master Write Group Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io008`] module"]
#[doc(alias = "PRIC_IO008")]
pub type PricIo008 = crate::Reg<pric_io008::PricIo008Spec>;
#[doc = "Master Write Group Setting Register \\#2"]
pub mod pric_io008;
#[doc = "PRIC_IO00C (rw) register accessor: Master Write Group Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io00c`] module"]
#[doc(alias = "PRIC_IO00C")]
pub type PricIo00c = crate::Reg<pric_io00c::PricIo00cSpec>;
#[doc = "Master Write Group Setting Register \\#3"]
pub mod pric_io00c;
#[doc = "PRIC_IO010 (rw) register accessor: Master Write Group Setting Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io010`] module"]
#[doc(alias = "PRIC_IO010")]
pub type PricIo010 = crate::Reg<pric_io010::PricIo010Spec>;
#[doc = "Master Write Group Setting Register \\#4"]
pub mod pric_io010;
#[doc = "PRIC_IO014 (rw) register accessor: Master Write Group Setting Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io014`] module"]
#[doc(alias = "PRIC_IO014")]
pub type PricIo014 = crate::Reg<pric_io014::PricIo014Spec>;
#[doc = "Master Write Group Setting Register \\#5"]
pub mod pric_io014;
#[doc = "PRIC_IO100 (rw) register accessor: Master Read Group Setting Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io100`] module"]
#[doc(alias = "PRIC_IO100")]
pub type PricIo100 = crate::Reg<pric_io100::PricIo100Spec>;
#[doc = "Master Read Group Setting Register \\#0"]
pub mod pric_io100;
#[doc = "PRIC_IO104 (rw) register accessor: Master Read Group Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io104`] module"]
#[doc(alias = "PRIC_IO104")]
pub type PricIo104 = crate::Reg<pric_io104::PricIo104Spec>;
#[doc = "Master Read Group Setting Register \\#1"]
pub mod pric_io104;
#[doc = "PRIC_IO108 (rw) register accessor: Master Read Group Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io108`] module"]
#[doc(alias = "PRIC_IO108")]
pub type PricIo108 = crate::Reg<pric_io108::PricIo108Spec>;
#[doc = "Master Read Group Setting Register \\#2"]
pub mod pric_io108;
#[doc = "PRIC_IO10C (rw) register accessor: Master Read Group Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io10c`] module"]
#[doc(alias = "PRIC_IO10C")]
pub type PricIo10c = crate::Reg<pric_io10c::PricIo10cSpec>;
#[doc = "Master Read Group Setting Register \\#3"]
pub mod pric_io10c;
#[doc = "PRIC_IO110 (rw) register accessor: Master Read Group Setting Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io110`] module"]
#[doc(alias = "PRIC_IO110")]
pub type PricIo110 = crate::Reg<pric_io110::PricIo110Spec>;
#[doc = "Master Read Group Setting Register \\#4"]
pub mod pric_io110;
#[doc = "PRIC_IO114 (rw) register accessor: Master Read Group Setting Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io114`] module"]
#[doc(alias = "PRIC_IO114")]
pub type PricIo114 = crate::Reg<pric_io114::PricIo114Spec>;
#[doc = "Master Read Group Setting Register \\#5"]
pub mod pric_io114;
#[doc = "PRIC_IO200 (rw) register accessor: Slave Write Group Setting Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io200`] module"]
#[doc(alias = "PRIC_IO200")]
pub type PricIo200 = crate::Reg<pric_io200::PricIo200Spec>;
#[doc = "Slave Write Group Setting Register \\#0"]
pub mod pric_io200;
#[doc = "PRIC_IO204 (rw) register accessor: Slave Write Group Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io204`] module"]
#[doc(alias = "PRIC_IO204")]
pub type PricIo204 = crate::Reg<pric_io204::PricIo204Spec>;
#[doc = "Slave Write Group Setting Register \\#1"]
pub mod pric_io204;
#[doc = "PRIC_IO208 (rw) register accessor: Slave Write Group Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io208`] module"]
#[doc(alias = "PRIC_IO208")]
pub type PricIo208 = crate::Reg<pric_io208::PricIo208Spec>;
#[doc = "Slave Write Group Setting Register \\#2"]
pub mod pric_io208;
#[doc = "PRIC_IO20C (rw) register accessor: Slave Write Group Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io20c`] module"]
#[doc(alias = "PRIC_IO20C")]
pub type PricIo20c = crate::Reg<pric_io20c::PricIo20cSpec>;
#[doc = "Slave Write Group Setting Register \\#3"]
pub mod pric_io20c;
#[doc = "PRIC_IO210 (rw) register accessor: Slave Write Group Setting Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io210`] module"]
#[doc(alias = "PRIC_IO210")]
pub type PricIo210 = crate::Reg<pric_io210::PricIo210Spec>;
#[doc = "Slave Write Group Setting Register \\#4"]
pub mod pric_io210;
#[doc = "PRIC_IO214 (rw) register accessor: Slave Write Group Setting Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io214`] module"]
#[doc(alias = "PRIC_IO214")]
pub type PricIo214 = crate::Reg<pric_io214::PricIo214Spec>;
#[doc = "Slave Write Group Setting Register \\#5"]
pub mod pric_io214;
#[doc = "PRIC_IO218 (rw) register accessor: Slave Write Group Setting Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io218`] module"]
#[doc(alias = "PRIC_IO218")]
pub type PricIo218 = crate::Reg<pric_io218::PricIo218Spec>;
#[doc = "Slave Write Group Setting Register \\#6"]
pub mod pric_io218;
#[doc = "PRIC_IO21C (rw) register accessor: Slave Write Group Setting Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io21c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io21c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io21c`] module"]
#[doc(alias = "PRIC_IO21C")]
pub type PricIo21c = crate::Reg<pric_io21c::PricIo21cSpec>;
#[doc = "Slave Write Group Setting Register \\#7"]
pub mod pric_io21c;
#[doc = "PRIC_IO220 (rw) register accessor: Slave Write Group Setting Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io220`] module"]
#[doc(alias = "PRIC_IO220")]
pub type PricIo220 = crate::Reg<pric_io220::PricIo220Spec>;
#[doc = "Slave Write Group Setting Register \\#8"]
pub mod pric_io220;
#[doc = "PRIC_IO224 (rw) register accessor: Slave Write Group Setting Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io224`] module"]
#[doc(alias = "PRIC_IO224")]
pub type PricIo224 = crate::Reg<pric_io224::PricIo224Spec>;
#[doc = "Slave Write Group Setting Register \\#9"]
pub mod pric_io224;
#[doc = "PRIC_IO228 (rw) register accessor: Slave Write Group Setting Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io228`] module"]
#[doc(alias = "PRIC_IO228")]
pub type PricIo228 = crate::Reg<pric_io228::PricIo228Spec>;
#[doc = "Slave Write Group Setting Register \\#10"]
pub mod pric_io228;
#[doc = "PRIC_IO22C (rw) register accessor: Slave Write Group Setting Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io22c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io22c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io22c`] module"]
#[doc(alias = "PRIC_IO22C")]
pub type PricIo22c = crate::Reg<pric_io22c::PricIo22cSpec>;
#[doc = "Slave Write Group Setting Register \\#11"]
pub mod pric_io22c;
#[doc = "PRIC_IO230 (rw) register accessor: Slave Write Group Setting Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io230`] module"]
#[doc(alias = "PRIC_IO230")]
pub type PricIo230 = crate::Reg<pric_io230::PricIo230Spec>;
#[doc = "Slave Write Group Setting Register \\#12"]
pub mod pric_io230;
#[doc = "PRIC_IO234 (rw) register accessor: Slave Write Group Setting Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io234`] module"]
#[doc(alias = "PRIC_IO234")]
pub type PricIo234 = crate::Reg<pric_io234::PricIo234Spec>;
#[doc = "Slave Write Group Setting Register \\#13"]
pub mod pric_io234;
#[doc = "PRIC_IO238 (rw) register accessor: Slave Write Group Setting Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io238`] module"]
#[doc(alias = "PRIC_IO238")]
pub type PricIo238 = crate::Reg<pric_io238::PricIo238Spec>;
#[doc = "Slave Write Group Setting Register \\#14"]
pub mod pric_io238;
#[doc = "PRIC_IO23C (rw) register accessor: Slave Write Group Setting Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io23c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io23c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io23c`] module"]
#[doc(alias = "PRIC_IO23C")]
pub type PricIo23c = crate::Reg<pric_io23c::PricIo23cSpec>;
#[doc = "Slave Write Group Setting Register \\#15"]
pub mod pric_io23c;
#[doc = "PRIC_IO240 (rw) register accessor: Slave Write Group Setting Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io240`] module"]
#[doc(alias = "PRIC_IO240")]
pub type PricIo240 = crate::Reg<pric_io240::PricIo240Spec>;
#[doc = "Slave Write Group Setting Register \\#16"]
pub mod pric_io240;
#[doc = "PRIC_IO244 (rw) register accessor: Slave Write Group Setting Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io244`] module"]
#[doc(alias = "PRIC_IO244")]
pub type PricIo244 = crate::Reg<pric_io244::PricIo244Spec>;
#[doc = "Slave Write Group Setting Register \\#17"]
pub mod pric_io244;
#[doc = "PRIC_IO248 (rw) register accessor: Slave Write Group Setting Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io248`] module"]
#[doc(alias = "PRIC_IO248")]
pub type PricIo248 = crate::Reg<pric_io248::PricIo248Spec>;
#[doc = "Slave Write Group Setting Register \\#18"]
pub mod pric_io248;
#[doc = "PRIC_IO24C (rw) register accessor: Slave Write Group Setting Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io24c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io24c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io24c`] module"]
#[doc(alias = "PRIC_IO24C")]
pub type PricIo24c = crate::Reg<pric_io24c::PricIo24cSpec>;
#[doc = "Slave Write Group Setting Register \\#19"]
pub mod pric_io24c;
#[doc = "PRIC_IO250 (rw) register accessor: Slave Write Group Setting Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io250`] module"]
#[doc(alias = "PRIC_IO250")]
pub type PricIo250 = crate::Reg<pric_io250::PricIo250Spec>;
#[doc = "Slave Write Group Setting Register \\#20"]
pub mod pric_io250;
#[doc = "PRIC_IO254 (rw) register accessor: Slave Write Group Setting Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io254`] module"]
#[doc(alias = "PRIC_IO254")]
pub type PricIo254 = crate::Reg<pric_io254::PricIo254Spec>;
#[doc = "Slave Write Group Setting Register \\#21"]
pub mod pric_io254;
#[doc = "PRIC_IO258 (rw) register accessor: Slave Write Group Setting Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io258::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io258::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io258`] module"]
#[doc(alias = "PRIC_IO258")]
pub type PricIo258 = crate::Reg<pric_io258::PricIo258Spec>;
#[doc = "Slave Write Group Setting Register \\#22"]
pub mod pric_io258;
#[doc = "PRIC_IO25C (rw) register accessor: Slave Write Group Setting Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io25c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io25c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io25c`] module"]
#[doc(alias = "PRIC_IO25C")]
pub type PricIo25c = crate::Reg<pric_io25c::PricIo25cSpec>;
#[doc = "Slave Write Group Setting Register \\#23"]
pub mod pric_io25c;
#[doc = "PRIC_IO260 (rw) register accessor: Slave Write Group Setting Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io260::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io260::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io260`] module"]
#[doc(alias = "PRIC_IO260")]
pub type PricIo260 = crate::Reg<pric_io260::PricIo260Spec>;
#[doc = "Slave Write Group Setting Register \\#24"]
pub mod pric_io260;
#[doc = "PRIC_IO264 (rw) register accessor: Slave Write Group Setting Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io264::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io264::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io264`] module"]
#[doc(alias = "PRIC_IO264")]
pub type PricIo264 = crate::Reg<pric_io264::PricIo264Spec>;
#[doc = "Slave Write Group Setting Register \\#25"]
pub mod pric_io264;
#[doc = "PRIC_IO268 (rw) register accessor: Slave Write Group Setting Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io268::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io268::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io268`] module"]
#[doc(alias = "PRIC_IO268")]
pub type PricIo268 = crate::Reg<pric_io268::PricIo268Spec>;
#[doc = "Slave Write Group Setting Register \\#26"]
pub mod pric_io268;
#[doc = "PRIC_IO26C (rw) register accessor: Slave Write Group Setting Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io26c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io26c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io26c`] module"]
#[doc(alias = "PRIC_IO26C")]
pub type PricIo26c = crate::Reg<pric_io26c::PricIo26cSpec>;
#[doc = "Slave Write Group Setting Register \\#27"]
pub mod pric_io26c;
#[doc = "PRIC_IO270 (rw) register accessor: Slave Write Group Setting Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io270::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io270::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io270`] module"]
#[doc(alias = "PRIC_IO270")]
pub type PricIo270 = crate::Reg<pric_io270::PricIo270Spec>;
#[doc = "Slave Write Group Setting Register \\#28"]
pub mod pric_io270;
#[doc = "PRIC_IO274 (rw) register accessor: Slave Write Group Setting Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io274::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io274::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io274`] module"]
#[doc(alias = "PRIC_IO274")]
pub type PricIo274 = crate::Reg<pric_io274::PricIo274Spec>;
#[doc = "Slave Write Group Setting Register \\#29"]
pub mod pric_io274;
#[doc = "PRIC_IO278 (rw) register accessor: Slave Write Group Setting Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io278::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io278::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io278`] module"]
#[doc(alias = "PRIC_IO278")]
pub type PricIo278 = crate::Reg<pric_io278::PricIo278Spec>;
#[doc = "Slave Write Group Setting Register \\#30"]
pub mod pric_io278;
#[doc = "PRIC_IO27C (rw) register accessor: Slave Write Group Setting Register \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io27c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io27c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io27c`] module"]
#[doc(alias = "PRIC_IO27C")]
pub type PricIo27c = crate::Reg<pric_io27c::PricIo27cSpec>;
#[doc = "Slave Write Group Setting Register \\#31"]
pub mod pric_io27c;
#[doc = "PRIC_IO280 (rw) register accessor: Slave Write Group Setting Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io280::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io280::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io280`] module"]
#[doc(alias = "PRIC_IO280")]
pub type PricIo280 = crate::Reg<pric_io280::PricIo280Spec>;
#[doc = "Slave Write Group Setting Register \\#32"]
pub mod pric_io280;
#[doc = "PRIC_IO284 (rw) register accessor: Slave Write Group Setting Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io284::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io284::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io284`] module"]
#[doc(alias = "PRIC_IO284")]
pub type PricIo284 = crate::Reg<pric_io284::PricIo284Spec>;
#[doc = "Slave Write Group Setting Register \\#33"]
pub mod pric_io284;
#[doc = "PRIC_IO288 (rw) register accessor: Slave Write Group Setting Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io288::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io288::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io288`] module"]
#[doc(alias = "PRIC_IO288")]
pub type PricIo288 = crate::Reg<pric_io288::PricIo288Spec>;
#[doc = "Slave Write Group Setting Register \\#34"]
pub mod pric_io288;
#[doc = "PRIC_IO28C (rw) register accessor: Slave Write Group Setting Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io28c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io28c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io28c`] module"]
#[doc(alias = "PRIC_IO28C")]
pub type PricIo28c = crate::Reg<pric_io28c::PricIo28cSpec>;
#[doc = "Slave Write Group Setting Register \\#35"]
pub mod pric_io28c;
#[doc = "PRIC_IO290 (rw) register accessor: Slave Write Group Setting Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io290::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io290::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io290`] module"]
#[doc(alias = "PRIC_IO290")]
pub type PricIo290 = crate::Reg<pric_io290::PricIo290Spec>;
#[doc = "Slave Write Group Setting Register \\#36"]
pub mod pric_io290;
#[doc = "PRIC_IO294 (rw) register accessor: Slave Write Group Setting Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io294::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io294::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io294`] module"]
#[doc(alias = "PRIC_IO294")]
pub type PricIo294 = crate::Reg<pric_io294::PricIo294Spec>;
#[doc = "Slave Write Group Setting Register \\#37"]
pub mod pric_io294;
#[doc = "PRIC_IO298 (rw) register accessor: Slave Write Group Setting Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io298::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io298::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io298`] module"]
#[doc(alias = "PRIC_IO298")]
pub type PricIo298 = crate::Reg<pric_io298::PricIo298Spec>;
#[doc = "Slave Write Group Setting Register \\#38"]
pub mod pric_io298;
#[doc = "PRIC_IO29C (rw) register accessor: Slave Write Group Setting Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io29c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io29c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io29c`] module"]
#[doc(alias = "PRIC_IO29C")]
pub type PricIo29c = crate::Reg<pric_io29c::PricIo29cSpec>;
#[doc = "Slave Write Group Setting Register \\#39"]
pub mod pric_io29c;
#[doc = "PRIC_IO2A0 (rw) register accessor: Slave Write Group Setting Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2a0`] module"]
#[doc(alias = "PRIC_IO2A0")]
pub type PricIo2a0 = crate::Reg<pric_io2a0::PricIo2a0Spec>;
#[doc = "Slave Write Group Setting Register \\#40"]
pub mod pric_io2a0;
#[doc = "PRIC_IO2A4 (rw) register accessor: Slave Write Group Setting Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2a4`] module"]
#[doc(alias = "PRIC_IO2A4")]
pub type PricIo2a4 = crate::Reg<pric_io2a4::PricIo2a4Spec>;
#[doc = "Slave Write Group Setting Register \\#41"]
pub mod pric_io2a4;
#[doc = "PRIC_IO2A8 (rw) register accessor: Slave Write Group Setting Register \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2a8`] module"]
#[doc(alias = "PRIC_IO2A8")]
pub type PricIo2a8 = crate::Reg<pric_io2a8::PricIo2a8Spec>;
#[doc = "Slave Write Group Setting Register \\#42"]
pub mod pric_io2a8;
#[doc = "PRIC_IO2AC (rw) register accessor: Slave Write Group Setting Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2ac`] module"]
#[doc(alias = "PRIC_IO2AC")]
pub type PricIo2ac = crate::Reg<pric_io2ac::PricIo2acSpec>;
#[doc = "Slave Write Group Setting Register \\#43"]
pub mod pric_io2ac;
#[doc = "PRIC_IO2B0 (rw) register accessor: Slave Write Group Setting Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2b0`] module"]
#[doc(alias = "PRIC_IO2B0")]
pub type PricIo2b0 = crate::Reg<pric_io2b0::PricIo2b0Spec>;
#[doc = "Slave Write Group Setting Register \\#44"]
pub mod pric_io2b0;
#[doc = "PRIC_IO2B4 (rw) register accessor: Slave Write Group Setting Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2b4`] module"]
#[doc(alias = "PRIC_IO2B4")]
pub type PricIo2b4 = crate::Reg<pric_io2b4::PricIo2b4Spec>;
#[doc = "Slave Write Group Setting Register \\#45"]
pub mod pric_io2b4;
#[doc = "PRIC_IO2B8 (rw) register accessor: Slave Write Group Setting Register \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2b8`] module"]
#[doc(alias = "PRIC_IO2B8")]
pub type PricIo2b8 = crate::Reg<pric_io2b8::PricIo2b8Spec>;
#[doc = "Slave Write Group Setting Register \\#46"]
pub mod pric_io2b8;
#[doc = "PRIC_IO2BC (rw) register accessor: Slave Write Group Setting Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2bc`] module"]
#[doc(alias = "PRIC_IO2BC")]
pub type PricIo2bc = crate::Reg<pric_io2bc::PricIo2bcSpec>;
#[doc = "Slave Write Group Setting Register \\#47"]
pub mod pric_io2bc;
#[doc = "PRIC_IO2C0 (rw) register accessor: Slave Write Group Setting Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2c0`] module"]
#[doc(alias = "PRIC_IO2C0")]
pub type PricIo2c0 = crate::Reg<pric_io2c0::PricIo2c0Spec>;
#[doc = "Slave Write Group Setting Register \\#48"]
pub mod pric_io2c0;
#[doc = "PRIC_IO2C4 (rw) register accessor: Slave Write Group Setting Register \\#49\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2c4`] module"]
#[doc(alias = "PRIC_IO2C4")]
pub type PricIo2c4 = crate::Reg<pric_io2c4::PricIo2c4Spec>;
#[doc = "Slave Write Group Setting Register \\#49"]
pub mod pric_io2c4;
#[doc = "PRIC_IO2C8 (rw) register accessor: Slave Write Group Setting Register \\#50\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2c8`] module"]
#[doc(alias = "PRIC_IO2C8")]
pub type PricIo2c8 = crate::Reg<pric_io2c8::PricIo2c8Spec>;
#[doc = "Slave Write Group Setting Register \\#50"]
pub mod pric_io2c8;
#[doc = "PRIC_IO2CC (rw) register accessor: Slave Write Group Setting Register \\#51\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2cc`] module"]
#[doc(alias = "PRIC_IO2CC")]
pub type PricIo2cc = crate::Reg<pric_io2cc::PricIo2ccSpec>;
#[doc = "Slave Write Group Setting Register \\#51"]
pub mod pric_io2cc;
#[doc = "PRIC_IO2D0 (rw) register accessor: Slave Write Group Setting Register \\#52\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2d0`] module"]
#[doc(alias = "PRIC_IO2D0")]
pub type PricIo2d0 = crate::Reg<pric_io2d0::PricIo2d0Spec>;
#[doc = "Slave Write Group Setting Register \\#52"]
pub mod pric_io2d0;
#[doc = "PRIC_IO2D4 (rw) register accessor: Slave Write Group Setting Register \\#53\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2d4`] module"]
#[doc(alias = "PRIC_IO2D4")]
pub type PricIo2d4 = crate::Reg<pric_io2d4::PricIo2d4Spec>;
#[doc = "Slave Write Group Setting Register \\#53"]
pub mod pric_io2d4;
#[doc = "PRIC_IO2D8 (rw) register accessor: Slave Write Group Setting Register \\#54\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2d8`] module"]
#[doc(alias = "PRIC_IO2D8")]
pub type PricIo2d8 = crate::Reg<pric_io2d8::PricIo2d8Spec>;
#[doc = "Slave Write Group Setting Register \\#54"]
pub mod pric_io2d8;
#[doc = "PRIC_IO2DC (rw) register accessor: Slave Write Group Setting Register \\#55\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2dc`] module"]
#[doc(alias = "PRIC_IO2DC")]
pub type PricIo2dc = crate::Reg<pric_io2dc::PricIo2dcSpec>;
#[doc = "Slave Write Group Setting Register \\#55"]
pub mod pric_io2dc;
#[doc = "PRIC_IO2E0 (rw) register accessor: Slave Write Group Setting Register \\#56\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2e0`] module"]
#[doc(alias = "PRIC_IO2E0")]
pub type PricIo2e0 = crate::Reg<pric_io2e0::PricIo2e0Spec>;
#[doc = "Slave Write Group Setting Register \\#56"]
pub mod pric_io2e0;
#[doc = "PRIC_IO2E4 (rw) register accessor: Slave Write Group Setting Register \\#57\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2e4`] module"]
#[doc(alias = "PRIC_IO2E4")]
pub type PricIo2e4 = crate::Reg<pric_io2e4::PricIo2e4Spec>;
#[doc = "Slave Write Group Setting Register \\#57"]
pub mod pric_io2e4;
#[doc = "PRIC_IO2E8 (rw) register accessor: Slave Write Group Setting Register \\#58\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2e8`] module"]
#[doc(alias = "PRIC_IO2E8")]
pub type PricIo2e8 = crate::Reg<pric_io2e8::PricIo2e8Spec>;
#[doc = "Slave Write Group Setting Register \\#58"]
pub mod pric_io2e8;
#[doc = "PRIC_IO2EC (rw) register accessor: Slave Write Group Setting Register \\#59\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2ec`] module"]
#[doc(alias = "PRIC_IO2EC")]
pub type PricIo2ec = crate::Reg<pric_io2ec::PricIo2ecSpec>;
#[doc = "Slave Write Group Setting Register \\#59"]
pub mod pric_io2ec;
#[doc = "PRIC_IO2F0 (rw) register accessor: Slave Write Group Setting Register \\#60\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2f0`] module"]
#[doc(alias = "PRIC_IO2F0")]
pub type PricIo2f0 = crate::Reg<pric_io2f0::PricIo2f0Spec>;
#[doc = "Slave Write Group Setting Register \\#60"]
pub mod pric_io2f0;
#[doc = "PRIC_IO2F4 (rw) register accessor: Slave Write Group Setting Register \\#61\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2f4`] module"]
#[doc(alias = "PRIC_IO2F4")]
pub type PricIo2f4 = crate::Reg<pric_io2f4::PricIo2f4Spec>;
#[doc = "Slave Write Group Setting Register \\#61"]
pub mod pric_io2f4;
#[doc = "PRIC_IO2F8 (rw) register accessor: Slave Write Group Setting Register \\#62\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2f8`] module"]
#[doc(alias = "PRIC_IO2F8")]
pub type PricIo2f8 = crate::Reg<pric_io2f8::PricIo2f8Spec>;
#[doc = "Slave Write Group Setting Register \\#62"]
pub mod pric_io2f8;
#[doc = "PRIC_IO2FC (rw) register accessor: Slave Write Group Setting Register \\#63\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io2fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io2fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io2fc`] module"]
#[doc(alias = "PRIC_IO2FC")]
pub type PricIo2fc = crate::Reg<pric_io2fc::PricIo2fcSpec>;
#[doc = "Slave Write Group Setting Register \\#63"]
pub mod pric_io2fc;
#[doc = "PRIC_IO300 (rw) register accessor: Slave Read Group Setting Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io300::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io300::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io300`] module"]
#[doc(alias = "PRIC_IO300")]
pub type PricIo300 = crate::Reg<pric_io300::PricIo300Spec>;
#[doc = "Slave Read Group Setting Register \\#0"]
pub mod pric_io300;
#[doc = "PRIC_IO304 (rw) register accessor: Slave Read Group Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io304::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io304::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io304`] module"]
#[doc(alias = "PRIC_IO304")]
pub type PricIo304 = crate::Reg<pric_io304::PricIo304Spec>;
#[doc = "Slave Read Group Setting Register \\#1"]
pub mod pric_io304;
#[doc = "PRIC_IO308 (rw) register accessor: Slave Read Group Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io308::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io308::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io308`] module"]
#[doc(alias = "PRIC_IO308")]
pub type PricIo308 = crate::Reg<pric_io308::PricIo308Spec>;
#[doc = "Slave Read Group Setting Register \\#2"]
pub mod pric_io308;
#[doc = "PRIC_IO30C (rw) register accessor: Slave Read Group Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io30c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io30c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io30c`] module"]
#[doc(alias = "PRIC_IO30C")]
pub type PricIo30c = crate::Reg<pric_io30c::PricIo30cSpec>;
#[doc = "Slave Read Group Setting Register \\#3"]
pub mod pric_io30c;
#[doc = "PRIC_IO310 (rw) register accessor: Slave Read Group Setting Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io310`] module"]
#[doc(alias = "PRIC_IO310")]
pub type PricIo310 = crate::Reg<pric_io310::PricIo310Spec>;
#[doc = "Slave Read Group Setting Register \\#4"]
pub mod pric_io310;
#[doc = "PRIC_IO314 (rw) register accessor: Slave Read Group Setting Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io314`] module"]
#[doc(alias = "PRIC_IO314")]
pub type PricIo314 = crate::Reg<pric_io314::PricIo314Spec>;
#[doc = "Slave Read Group Setting Register \\#5"]
pub mod pric_io314;
#[doc = "PRIC_IO318 (rw) register accessor: Slave Read Group Setting Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io318::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io318::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io318`] module"]
#[doc(alias = "PRIC_IO318")]
pub type PricIo318 = crate::Reg<pric_io318::PricIo318Spec>;
#[doc = "Slave Read Group Setting Register \\#6"]
pub mod pric_io318;
#[doc = "PRIC_IO31C (rw) register accessor: Slave Read Group Setting Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io31c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io31c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io31c`] module"]
#[doc(alias = "PRIC_IO31C")]
pub type PricIo31c = crate::Reg<pric_io31c::PricIo31cSpec>;
#[doc = "Slave Read Group Setting Register \\#7"]
pub mod pric_io31c;
#[doc = "PRIC_IO320 (rw) register accessor: Slave Read Group Setting Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io320::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io320::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io320`] module"]
#[doc(alias = "PRIC_IO320")]
pub type PricIo320 = crate::Reg<pric_io320::PricIo320Spec>;
#[doc = "Slave Read Group Setting Register \\#8"]
pub mod pric_io320;
#[doc = "PRIC_IO324 (rw) register accessor: Slave Read Group Setting Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io324::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io324::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io324`] module"]
#[doc(alias = "PRIC_IO324")]
pub type PricIo324 = crate::Reg<pric_io324::PricIo324Spec>;
#[doc = "Slave Read Group Setting Register \\#9"]
pub mod pric_io324;
#[doc = "PRIC_IO328 (rw) register accessor: Slave Read Group Setting Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io328::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io328::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io328`] module"]
#[doc(alias = "PRIC_IO328")]
pub type PricIo328 = crate::Reg<pric_io328::PricIo328Spec>;
#[doc = "Slave Read Group Setting Register \\#10"]
pub mod pric_io328;
#[doc = "PRIC_IO32C (rw) register accessor: Slave Read Group Setting Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io32c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io32c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io32c`] module"]
#[doc(alias = "PRIC_IO32C")]
pub type PricIo32c = crate::Reg<pric_io32c::PricIo32cSpec>;
#[doc = "Slave Read Group Setting Register \\#11"]
pub mod pric_io32c;
#[doc = "PRIC_IO330 (rw) register accessor: Slave Read Group Setting Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io330::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io330::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io330`] module"]
#[doc(alias = "PRIC_IO330")]
pub type PricIo330 = crate::Reg<pric_io330::PricIo330Spec>;
#[doc = "Slave Read Group Setting Register \\#12"]
pub mod pric_io330;
#[doc = "PRIC_IO334 (rw) register accessor: Slave Read Group Setting Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io334::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io334::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io334`] module"]
#[doc(alias = "PRIC_IO334")]
pub type PricIo334 = crate::Reg<pric_io334::PricIo334Spec>;
#[doc = "Slave Read Group Setting Register \\#13"]
pub mod pric_io334;
#[doc = "PRIC_IO338 (rw) register accessor: Slave Read Group Setting Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io338::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io338::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io338`] module"]
#[doc(alias = "PRIC_IO338")]
pub type PricIo338 = crate::Reg<pric_io338::PricIo338Spec>;
#[doc = "Slave Read Group Setting Register \\#14"]
pub mod pric_io338;
#[doc = "PRIC_IO33C (rw) register accessor: Slave Read Group Setting Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io33c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io33c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io33c`] module"]
#[doc(alias = "PRIC_IO33C")]
pub type PricIo33c = crate::Reg<pric_io33c::PricIo33cSpec>;
#[doc = "Slave Read Group Setting Register \\#15"]
pub mod pric_io33c;
#[doc = "PRIC_IO340 (rw) register accessor: Slave Read Group Setting Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io340::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io340::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io340`] module"]
#[doc(alias = "PRIC_IO340")]
pub type PricIo340 = crate::Reg<pric_io340::PricIo340Spec>;
#[doc = "Slave Read Group Setting Register \\#16"]
pub mod pric_io340;
#[doc = "PRIC_IO344 (rw) register accessor: Slave Read Group Setting Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io344::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io344::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io344`] module"]
#[doc(alias = "PRIC_IO344")]
pub type PricIo344 = crate::Reg<pric_io344::PricIo344Spec>;
#[doc = "Slave Read Group Setting Register \\#17"]
pub mod pric_io344;
#[doc = "PRIC_IO348 (rw) register accessor: Slave Read Group Setting Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io348::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io348::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io348`] module"]
#[doc(alias = "PRIC_IO348")]
pub type PricIo348 = crate::Reg<pric_io348::PricIo348Spec>;
#[doc = "Slave Read Group Setting Register \\#18"]
pub mod pric_io348;
#[doc = "PRIC_IO34C (rw) register accessor: Slave Read Group Setting Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io34c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io34c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io34c`] module"]
#[doc(alias = "PRIC_IO34C")]
pub type PricIo34c = crate::Reg<pric_io34c::PricIo34cSpec>;
#[doc = "Slave Read Group Setting Register \\#19"]
pub mod pric_io34c;
#[doc = "PRIC_IO350 (rw) register accessor: Slave Read Group Setting Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io350::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io350::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io350`] module"]
#[doc(alias = "PRIC_IO350")]
pub type PricIo350 = crate::Reg<pric_io350::PricIo350Spec>;
#[doc = "Slave Read Group Setting Register \\#20"]
pub mod pric_io350;
#[doc = "PRIC_IO354 (rw) register accessor: Slave Read Group Setting Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io354::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io354::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io354`] module"]
#[doc(alias = "PRIC_IO354")]
pub type PricIo354 = crate::Reg<pric_io354::PricIo354Spec>;
#[doc = "Slave Read Group Setting Register \\#21"]
pub mod pric_io354;
#[doc = "PRIC_IO358 (rw) register accessor: Slave Read Group Setting Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io358::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io358::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io358`] module"]
#[doc(alias = "PRIC_IO358")]
pub type PricIo358 = crate::Reg<pric_io358::PricIo358Spec>;
#[doc = "Slave Read Group Setting Register \\#22"]
pub mod pric_io358;
#[doc = "PRIC_IO35C (rw) register accessor: Slave Read Group Setting Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io35c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io35c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io35c`] module"]
#[doc(alias = "PRIC_IO35C")]
pub type PricIo35c = crate::Reg<pric_io35c::PricIo35cSpec>;
#[doc = "Slave Read Group Setting Register \\#23"]
pub mod pric_io35c;
#[doc = "PRIC_IO360 (rw) register accessor: Slave Read Group Setting Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io360::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io360::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io360`] module"]
#[doc(alias = "PRIC_IO360")]
pub type PricIo360 = crate::Reg<pric_io360::PricIo360Spec>;
#[doc = "Slave Read Group Setting Register \\#24"]
pub mod pric_io360;
#[doc = "PRIC_IO364 (rw) register accessor: Slave Read Group Setting Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io364::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io364::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io364`] module"]
#[doc(alias = "PRIC_IO364")]
pub type PricIo364 = crate::Reg<pric_io364::PricIo364Spec>;
#[doc = "Slave Read Group Setting Register \\#25"]
pub mod pric_io364;
#[doc = "PRIC_IO368 (rw) register accessor: Slave Read Group Setting Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io368::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io368::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io368`] module"]
#[doc(alias = "PRIC_IO368")]
pub type PricIo368 = crate::Reg<pric_io368::PricIo368Spec>;
#[doc = "Slave Read Group Setting Register \\#26"]
pub mod pric_io368;
#[doc = "PRIC_IO36C (rw) register accessor: Slave Read Group Setting Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io36c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io36c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io36c`] module"]
#[doc(alias = "PRIC_IO36C")]
pub type PricIo36c = crate::Reg<pric_io36c::PricIo36cSpec>;
#[doc = "Slave Read Group Setting Register \\#27"]
pub mod pric_io36c;
#[doc = "PRIC_IO370 (rw) register accessor: Slave Read Group Setting Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io370::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io370::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io370`] module"]
#[doc(alias = "PRIC_IO370")]
pub type PricIo370 = crate::Reg<pric_io370::PricIo370Spec>;
#[doc = "Slave Read Group Setting Register \\#28"]
pub mod pric_io370;
#[doc = "PRIC_IO374 (rw) register accessor: Slave Read Group Setting Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io374::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io374::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io374`] module"]
#[doc(alias = "PRIC_IO374")]
pub type PricIo374 = crate::Reg<pric_io374::PricIo374Spec>;
#[doc = "Slave Read Group Setting Register \\#29"]
pub mod pric_io374;
#[doc = "PRIC_IO378 (rw) register accessor: Slave Read Group Setting Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io378::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io378::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io378`] module"]
#[doc(alias = "PRIC_IO378")]
pub type PricIo378 = crate::Reg<pric_io378::PricIo378Spec>;
#[doc = "Slave Read Group Setting Register \\#30"]
pub mod pric_io378;
#[doc = "PRIC_IO37C (rw) register accessor: Slave Read Group Setting Register \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io37c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io37c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io37c`] module"]
#[doc(alias = "PRIC_IO37C")]
pub type PricIo37c = crate::Reg<pric_io37c::PricIo37cSpec>;
#[doc = "Slave Read Group Setting Register \\#31"]
pub mod pric_io37c;
#[doc = "PRIC_IO380 (rw) register accessor: Slave Read Group Setting Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io380::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io380::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io380`] module"]
#[doc(alias = "PRIC_IO380")]
pub type PricIo380 = crate::Reg<pric_io380::PricIo380Spec>;
#[doc = "Slave Read Group Setting Register \\#32"]
pub mod pric_io380;
#[doc = "PRIC_IO384 (rw) register accessor: Slave Read Group Setting Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io384::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io384::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io384`] module"]
#[doc(alias = "PRIC_IO384")]
pub type PricIo384 = crate::Reg<pric_io384::PricIo384Spec>;
#[doc = "Slave Read Group Setting Register \\#33"]
pub mod pric_io384;
#[doc = "PRIC_IO388 (rw) register accessor: Slave Read Group Setting Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io388::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io388::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io388`] module"]
#[doc(alias = "PRIC_IO388")]
pub type PricIo388 = crate::Reg<pric_io388::PricIo388Spec>;
#[doc = "Slave Read Group Setting Register \\#34"]
pub mod pric_io388;
#[doc = "PRIC_IO38C (rw) register accessor: Slave Read Group Setting Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io38c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io38c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io38c`] module"]
#[doc(alias = "PRIC_IO38C")]
pub type PricIo38c = crate::Reg<pric_io38c::PricIo38cSpec>;
#[doc = "Slave Read Group Setting Register \\#35"]
pub mod pric_io38c;
#[doc = "PRIC_IO390 (rw) register accessor: Slave Read Group Setting Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io390::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io390::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io390`] module"]
#[doc(alias = "PRIC_IO390")]
pub type PricIo390 = crate::Reg<pric_io390::PricIo390Spec>;
#[doc = "Slave Read Group Setting Register \\#36"]
pub mod pric_io390;
#[doc = "PRIC_IO394 (rw) register accessor: Slave Read Group Setting Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io394::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io394::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io394`] module"]
#[doc(alias = "PRIC_IO394")]
pub type PricIo394 = crate::Reg<pric_io394::PricIo394Spec>;
#[doc = "Slave Read Group Setting Register \\#37"]
pub mod pric_io394;
#[doc = "PRIC_IO398 (rw) register accessor: Slave Read Group Setting Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io398::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io398::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io398`] module"]
#[doc(alias = "PRIC_IO398")]
pub type PricIo398 = crate::Reg<pric_io398::PricIo398Spec>;
#[doc = "Slave Read Group Setting Register \\#38"]
pub mod pric_io398;
#[doc = "PRIC_IO39C (rw) register accessor: Slave Read Group Setting Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io39c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io39c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io39c`] module"]
#[doc(alias = "PRIC_IO39C")]
pub type PricIo39c = crate::Reg<pric_io39c::PricIo39cSpec>;
#[doc = "Slave Read Group Setting Register \\#39"]
pub mod pric_io39c;
#[doc = "PRIC_IO3A0 (rw) register accessor: Slave Read Group Setting Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3a0`] module"]
#[doc(alias = "PRIC_IO3A0")]
pub type PricIo3a0 = crate::Reg<pric_io3a0::PricIo3a0Spec>;
#[doc = "Slave Read Group Setting Register \\#40"]
pub mod pric_io3a0;
#[doc = "PRIC_IO3A4 (rw) register accessor: Slave Read Group Setting Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3a4`] module"]
#[doc(alias = "PRIC_IO3A4")]
pub type PricIo3a4 = crate::Reg<pric_io3a4::PricIo3a4Spec>;
#[doc = "Slave Read Group Setting Register \\#41"]
pub mod pric_io3a4;
#[doc = "PRIC_IO3A8 (rw) register accessor: Slave Read Group Setting Register \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3a8`] module"]
#[doc(alias = "PRIC_IO3A8")]
pub type PricIo3a8 = crate::Reg<pric_io3a8::PricIo3a8Spec>;
#[doc = "Slave Read Group Setting Register \\#42"]
pub mod pric_io3a8;
#[doc = "PRIC_IO3AC (rw) register accessor: Slave Read Group Setting Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3ac`] module"]
#[doc(alias = "PRIC_IO3AC")]
pub type PricIo3ac = crate::Reg<pric_io3ac::PricIo3acSpec>;
#[doc = "Slave Read Group Setting Register \\#43"]
pub mod pric_io3ac;
#[doc = "PRIC_IO3B0 (rw) register accessor: Slave Read Group Setting Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3b0`] module"]
#[doc(alias = "PRIC_IO3B0")]
pub type PricIo3b0 = crate::Reg<pric_io3b0::PricIo3b0Spec>;
#[doc = "Slave Read Group Setting Register \\#44"]
pub mod pric_io3b0;
#[doc = "PRIC_IO3B4 (rw) register accessor: Slave Read Group Setting Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3b4`] module"]
#[doc(alias = "PRIC_IO3B4")]
pub type PricIo3b4 = crate::Reg<pric_io3b4::PricIo3b4Spec>;
#[doc = "Slave Read Group Setting Register \\#45"]
pub mod pric_io3b4;
#[doc = "PRIC_IO3B8 (rw) register accessor: Slave Read Group Setting Register \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3b8`] module"]
#[doc(alias = "PRIC_IO3B8")]
pub type PricIo3b8 = crate::Reg<pric_io3b8::PricIo3b8Spec>;
#[doc = "Slave Read Group Setting Register \\#46"]
pub mod pric_io3b8;
#[doc = "PRIC_IO3BC (rw) register accessor: Slave Read Group Setting Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3bc`] module"]
#[doc(alias = "PRIC_IO3BC")]
pub type PricIo3bc = crate::Reg<pric_io3bc::PricIo3bcSpec>;
#[doc = "Slave Read Group Setting Register \\#47"]
pub mod pric_io3bc;
#[doc = "PRIC_IO3C0 (rw) register accessor: Slave Read Group Setting Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3c0`] module"]
#[doc(alias = "PRIC_IO3C0")]
pub type PricIo3c0 = crate::Reg<pric_io3c0::PricIo3c0Spec>;
#[doc = "Slave Read Group Setting Register \\#48"]
pub mod pric_io3c0;
#[doc = "PRIC_IO3C4 (rw) register accessor: Slave Read Group Setting Register \\#49\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3c4`] module"]
#[doc(alias = "PRIC_IO3C4")]
pub type PricIo3c4 = crate::Reg<pric_io3c4::PricIo3c4Spec>;
#[doc = "Slave Read Group Setting Register \\#49"]
pub mod pric_io3c4;
#[doc = "PRIC_IO3C8 (rw) register accessor: Slave Read Group Setting Register \\#50\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3c8`] module"]
#[doc(alias = "PRIC_IO3C8")]
pub type PricIo3c8 = crate::Reg<pric_io3c8::PricIo3c8Spec>;
#[doc = "Slave Read Group Setting Register \\#50"]
pub mod pric_io3c8;
#[doc = "PRIC_IO3CC (rw) register accessor: Slave Read Group Setting Register \\#51\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3cc`] module"]
#[doc(alias = "PRIC_IO3CC")]
pub type PricIo3cc = crate::Reg<pric_io3cc::PricIo3ccSpec>;
#[doc = "Slave Read Group Setting Register \\#51"]
pub mod pric_io3cc;
#[doc = "PRIC_IO3D0 (rw) register accessor: Slave Read Group Setting Register \\#52\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3d0`] module"]
#[doc(alias = "PRIC_IO3D0")]
pub type PricIo3d0 = crate::Reg<pric_io3d0::PricIo3d0Spec>;
#[doc = "Slave Read Group Setting Register \\#52"]
pub mod pric_io3d0;
#[doc = "PRIC_IO3D4 (rw) register accessor: Slave Read Group Setting Register \\#53\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3d4`] module"]
#[doc(alias = "PRIC_IO3D4")]
pub type PricIo3d4 = crate::Reg<pric_io3d4::PricIo3d4Spec>;
#[doc = "Slave Read Group Setting Register \\#53"]
pub mod pric_io3d4;
#[doc = "PRIC_IO3D8 (rw) register accessor: Slave Read Group Setting Register \\#54\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3d8`] module"]
#[doc(alias = "PRIC_IO3D8")]
pub type PricIo3d8 = crate::Reg<pric_io3d8::PricIo3d8Spec>;
#[doc = "Slave Read Group Setting Register \\#54"]
pub mod pric_io3d8;
#[doc = "PRIC_IO3DC (rw) register accessor: Slave Read Group Setting Register \\#55\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3dc`] module"]
#[doc(alias = "PRIC_IO3DC")]
pub type PricIo3dc = crate::Reg<pric_io3dc::PricIo3dcSpec>;
#[doc = "Slave Read Group Setting Register \\#55"]
pub mod pric_io3dc;
#[doc = "PRIC_IO3E0 (rw) register accessor: Slave Read Group Setting Register \\#56\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3e0`] module"]
#[doc(alias = "PRIC_IO3E0")]
pub type PricIo3e0 = crate::Reg<pric_io3e0::PricIo3e0Spec>;
#[doc = "Slave Read Group Setting Register \\#56"]
pub mod pric_io3e0;
#[doc = "PRIC_IO3E4 (rw) register accessor: Slave Read Group Setting Register \\#57\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3e4`] module"]
#[doc(alias = "PRIC_IO3E4")]
pub type PricIo3e4 = crate::Reg<pric_io3e4::PricIo3e4Spec>;
#[doc = "Slave Read Group Setting Register \\#57"]
pub mod pric_io3e4;
#[doc = "PRIC_IO3E8 (rw) register accessor: Slave Read Group Setting Register \\#58\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3e8`] module"]
#[doc(alias = "PRIC_IO3E8")]
pub type PricIo3e8 = crate::Reg<pric_io3e8::PricIo3e8Spec>;
#[doc = "Slave Read Group Setting Register \\#58"]
pub mod pric_io3e8;
#[doc = "PRIC_IO3EC (rw) register accessor: Slave Read Group Setting Register \\#59\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3ec`] module"]
#[doc(alias = "PRIC_IO3EC")]
pub type PricIo3ec = crate::Reg<pric_io3ec::PricIo3ecSpec>;
#[doc = "Slave Read Group Setting Register \\#59"]
pub mod pric_io3ec;
#[doc = "PRIC_IO3F0 (rw) register accessor: Slave Read Group Setting Register \\#60\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3f0`] module"]
#[doc(alias = "PRIC_IO3F0")]
pub type PricIo3f0 = crate::Reg<pric_io3f0::PricIo3f0Spec>;
#[doc = "Slave Read Group Setting Register \\#60"]
pub mod pric_io3f0;
#[doc = "PRIC_IO3F4 (rw) register accessor: Slave Read Group Setting Register \\#61\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3f4`] module"]
#[doc(alias = "PRIC_IO3F4")]
pub type PricIo3f4 = crate::Reg<pric_io3f4::PricIo3f4Spec>;
#[doc = "Slave Read Group Setting Register \\#61"]
pub mod pric_io3f4;
#[doc = "PRIC_IO3F8 (rw) register accessor: Slave Read Group Setting Register \\#62\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3f8`] module"]
#[doc(alias = "PRIC_IO3F8")]
pub type PricIo3f8 = crate::Reg<pric_io3f8::PricIo3f8Spec>;
#[doc = "Slave Read Group Setting Register \\#62"]
pub mod pric_io3f8;
#[doc = "PRIC_IO3FC (rw) register accessor: Slave Read Group Setting Register \\#63\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io3fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io3fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io3fc`] module"]
#[doc(alias = "PRIC_IO3FC")]
pub type PricIo3fc = crate::Reg<pric_io3fc::PricIo3fcSpec>;
#[doc = "Slave Read Group Setting Register \\#63"]
pub mod pric_io3fc;
#[doc = "PRIC_IO400 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io400::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io400::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io400`] module"]
#[doc(alias = "PRIC_IO400")]
pub type PricIo400 = crate::Reg<pric_io400::PricIo400Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#0"]
pub mod pric_io400;
#[doc = "PRIC_IO410 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io410::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io410::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io410`] module"]
#[doc(alias = "PRIC_IO410")]
pub type PricIo410 = crate::Reg<pric_io410::PricIo410Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#4"]
pub mod pric_io410;
#[doc = "PRIC_IO420 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io420::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io420::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io420`] module"]
#[doc(alias = "PRIC_IO420")]
pub type PricIo420 = crate::Reg<pric_io420::PricIo420Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#8"]
pub mod pric_io420;
#[doc = "PRIC_IO430 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io430::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io430::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io430`] module"]
#[doc(alias = "PRIC_IO430")]
pub type PricIo430 = crate::Reg<pric_io430::PricIo430Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#12"]
pub mod pric_io430;
#[doc = "PRIC_IO440 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io440::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io440::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io440`] module"]
#[doc(alias = "PRIC_IO440")]
pub type PricIo440 = crate::Reg<pric_io440::PricIo440Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#16"]
pub mod pric_io440;
#[doc = "PRIC_IO450 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io450::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io450::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io450`] module"]
#[doc(alias = "PRIC_IO450")]
pub type PricIo450 = crate::Reg<pric_io450::PricIo450Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#20"]
pub mod pric_io450;
#[doc = "PRIC_IO460 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io460::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io460::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io460`] module"]
#[doc(alias = "PRIC_IO460")]
pub type PricIo460 = crate::Reg<pric_io460::PricIo460Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#24"]
pub mod pric_io460;
#[doc = "PRIC_IO470 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io470::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io470::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io470`] module"]
#[doc(alias = "PRIC_IO470")]
pub type PricIo470 = crate::Reg<pric_io470::PricIo470Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#28"]
pub mod pric_io470;
#[doc = "PRIC_IO480 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io480::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io480::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io480`] module"]
#[doc(alias = "PRIC_IO480")]
pub type PricIo480 = crate::Reg<pric_io480::PricIo480Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#32"]
pub mod pric_io480;
#[doc = "PRIC_IO490 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io490::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io490::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io490`] module"]
#[doc(alias = "PRIC_IO490")]
pub type PricIo490 = crate::Reg<pric_io490::PricIo490Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#36"]
pub mod pric_io490;
#[doc = "PRIC_IO4A0 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4a0`] module"]
#[doc(alias = "PRIC_IO4A0")]
pub type PricIo4a0 = crate::Reg<pric_io4a0::PricIo4a0Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#40"]
pub mod pric_io4a0;
#[doc = "PRIC_IO4B0 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4b0`] module"]
#[doc(alias = "PRIC_IO4B0")]
pub type PricIo4b0 = crate::Reg<pric_io4b0::PricIo4b0Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#44"]
pub mod pric_io4b0;
#[doc = "PRIC_IO4C0 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4c0`] module"]
#[doc(alias = "PRIC_IO4C0")]
pub type PricIo4c0 = crate::Reg<pric_io4c0::PricIo4c0Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#48"]
pub mod pric_io4c0;
#[doc = "PRIC_IO4D0 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#52\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4d0`] module"]
#[doc(alias = "PRIC_IO4D0")]
pub type PricIo4d0 = crate::Reg<pric_io4d0::PricIo4d0Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#52"]
pub mod pric_io4d0;
#[doc = "PRIC_IO4E0 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#56\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4e0`] module"]
#[doc(alias = "PRIC_IO4E0")]
pub type PricIo4e0 = crate::Reg<pric_io4e0::PricIo4e0Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#56"]
pub mod pric_io4e0;
#[doc = "PRIC_IO4F0 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#60\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4f0`] module"]
#[doc(alias = "PRIC_IO4F0")]
pub type PricIo4f0 = crate::Reg<pric_io4f0::PricIo4f0Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#60"]
pub mod pric_io4f0;
#[doc = "PRIC_IO404 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io404::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io404::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io404`] module"]
#[doc(alias = "PRIC_IO404")]
pub type PricIo404 = crate::Reg<pric_io404::PricIo404Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#1"]
pub mod pric_io404;
#[doc = "PRIC_IO414 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io414::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io414::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io414`] module"]
#[doc(alias = "PRIC_IO414")]
pub type PricIo414 = crate::Reg<pric_io414::PricIo414Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#5"]
pub mod pric_io414;
#[doc = "PRIC_IO424 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io424::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io424::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io424`] module"]
#[doc(alias = "PRIC_IO424")]
pub type PricIo424 = crate::Reg<pric_io424::PricIo424Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#9"]
pub mod pric_io424;
#[doc = "PRIC_IO434 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io434::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io434::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io434`] module"]
#[doc(alias = "PRIC_IO434")]
pub type PricIo434 = crate::Reg<pric_io434::PricIo434Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#13"]
pub mod pric_io434;
#[doc = "PRIC_IO444 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io444::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io444::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io444`] module"]
#[doc(alias = "PRIC_IO444")]
pub type PricIo444 = crate::Reg<pric_io444::PricIo444Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#17"]
pub mod pric_io444;
#[doc = "PRIC_IO454 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io454::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io454::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io454`] module"]
#[doc(alias = "PRIC_IO454")]
pub type PricIo454 = crate::Reg<pric_io454::PricIo454Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#21"]
pub mod pric_io454;
#[doc = "PRIC_IO464 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io464::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io464::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io464`] module"]
#[doc(alias = "PRIC_IO464")]
pub type PricIo464 = crate::Reg<pric_io464::PricIo464Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#25"]
pub mod pric_io464;
#[doc = "PRIC_IO474 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io474::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io474::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io474`] module"]
#[doc(alias = "PRIC_IO474")]
pub type PricIo474 = crate::Reg<pric_io474::PricIo474Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#29"]
pub mod pric_io474;
#[doc = "PRIC_IO484 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io484::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io484::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io484`] module"]
#[doc(alias = "PRIC_IO484")]
pub type PricIo484 = crate::Reg<pric_io484::PricIo484Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#33"]
pub mod pric_io484;
#[doc = "PRIC_IO494 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io494::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io494::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io494`] module"]
#[doc(alias = "PRIC_IO494")]
pub type PricIo494 = crate::Reg<pric_io494::PricIo494Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#37"]
pub mod pric_io494;
#[doc = "PRIC_IO4A4 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4a4`] module"]
#[doc(alias = "PRIC_IO4A4")]
pub type PricIo4a4 = crate::Reg<pric_io4a4::PricIo4a4Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#41"]
pub mod pric_io4a4;
#[doc = "PRIC_IO4B4 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4b4`] module"]
#[doc(alias = "PRIC_IO4B4")]
pub type PricIo4b4 = crate::Reg<pric_io4b4::PricIo4b4Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#45"]
pub mod pric_io4b4;
#[doc = "PRIC_IO4C4 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#49\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4c4`] module"]
#[doc(alias = "PRIC_IO4C4")]
pub type PricIo4c4 = crate::Reg<pric_io4c4::PricIo4c4Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#49"]
pub mod pric_io4c4;
#[doc = "PRIC_IO4D4 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#53\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4d4`] module"]
#[doc(alias = "PRIC_IO4D4")]
pub type PricIo4d4 = crate::Reg<pric_io4d4::PricIo4d4Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#53"]
pub mod pric_io4d4;
#[doc = "PRIC_IO4E4 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#57\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4e4`] module"]
#[doc(alias = "PRIC_IO4E4")]
pub type PricIo4e4 = crate::Reg<pric_io4e4::PricIo4e4Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#57"]
pub mod pric_io4e4;
#[doc = "PRIC_IO4F4 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#61\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4f4`] module"]
#[doc(alias = "PRIC_IO4F4")]
pub type PricIo4f4 = crate::Reg<pric_io4f4::PricIo4f4Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#61"]
pub mod pric_io4f4;
#[doc = "PRIC_IO408 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io408::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io408::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io408`] module"]
#[doc(alias = "PRIC_IO408")]
pub type PricIo408 = crate::Reg<pric_io408::PricIo408Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#2"]
pub mod pric_io408;
#[doc = "PRIC_IO418 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io418::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io418::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io418`] module"]
#[doc(alias = "PRIC_IO418")]
pub type PricIo418 = crate::Reg<pric_io418::PricIo418Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#6"]
pub mod pric_io418;
#[doc = "PRIC_IO428 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io428::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io428::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io428`] module"]
#[doc(alias = "PRIC_IO428")]
pub type PricIo428 = crate::Reg<pric_io428::PricIo428Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#10"]
pub mod pric_io428;
#[doc = "PRIC_IO438 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io438::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io438::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io438`] module"]
#[doc(alias = "PRIC_IO438")]
pub type PricIo438 = crate::Reg<pric_io438::PricIo438Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#14"]
pub mod pric_io438;
#[doc = "PRIC_IO448 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io448::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io448::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io448`] module"]
#[doc(alias = "PRIC_IO448")]
pub type PricIo448 = crate::Reg<pric_io448::PricIo448Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#18"]
pub mod pric_io448;
#[doc = "PRIC_IO458 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io458::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io458::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io458`] module"]
#[doc(alias = "PRIC_IO458")]
pub type PricIo458 = crate::Reg<pric_io458::PricIo458Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#22"]
pub mod pric_io458;
#[doc = "PRIC_IO468 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io468::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io468::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io468`] module"]
#[doc(alias = "PRIC_IO468")]
pub type PricIo468 = crate::Reg<pric_io468::PricIo468Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#26"]
pub mod pric_io468;
#[doc = "PRIC_IO478 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io478::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io478::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io478`] module"]
#[doc(alias = "PRIC_IO478")]
pub type PricIo478 = crate::Reg<pric_io478::PricIo478Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#20"]
pub mod pric_io478;
#[doc = "PRIC_IO488 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io488::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io488::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io488`] module"]
#[doc(alias = "PRIC_IO488")]
pub type PricIo488 = crate::Reg<pric_io488::PricIo488Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#34"]
pub mod pric_io488;
#[doc = "PRIC_IO498 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io498::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io498::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io498`] module"]
#[doc(alias = "PRIC_IO498")]
pub type PricIo498 = crate::Reg<pric_io498::PricIo498Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#38"]
pub mod pric_io498;
#[doc = "PRIC_IO4A8 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4a8`] module"]
#[doc(alias = "PRIC_IO4A8")]
pub type PricIo4a8 = crate::Reg<pric_io4a8::PricIo4a8Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#42"]
pub mod pric_io4a8;
#[doc = "PRIC_IO4B8 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4b8`] module"]
#[doc(alias = "PRIC_IO4B8")]
pub type PricIo4b8 = crate::Reg<pric_io4b8::PricIo4b8Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#46"]
pub mod pric_io4b8;
#[doc = "PRIC_IO4C8 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4c8`] module"]
#[doc(alias = "PRIC_IO4C8")]
pub type PricIo4c8 = crate::Reg<pric_io4c8::PricIo4c8Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#40"]
pub mod pric_io4c8;
#[doc = "PRIC_IO4D8 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#54\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4d8`] module"]
#[doc(alias = "PRIC_IO4D8")]
pub type PricIo4d8 = crate::Reg<pric_io4d8::PricIo4d8Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#54"]
pub mod pric_io4d8;
#[doc = "PRIC_IO4E8 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#58\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4e8`] module"]
#[doc(alias = "PRIC_IO4E8")]
pub type PricIo4e8 = crate::Reg<pric_io4e8::PricIo4e8Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#58"]
pub mod pric_io4e8;
#[doc = "PRIC_IO4F8 (rw) register accessor: Memory Region Proection for AHB Masters Register \\#62\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4f8`] module"]
#[doc(alias = "PRIC_IO4F8")]
pub type PricIo4f8 = crate::Reg<pric_io4f8::PricIo4f8Spec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#62"]
pub mod pric_io4f8;
#[doc = "PRIC_IO40C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io40c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io40c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io40c`] module"]
#[doc(alias = "PRIC_IO40C")]
pub type PricIo40c = crate::Reg<pric_io40c::PricIo40cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#3"]
pub mod pric_io40c;
#[doc = "PRIC_IO41C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io41c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io41c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io41c`] module"]
#[doc(alias = "PRIC_IO41C")]
pub type PricIo41c = crate::Reg<pric_io41c::PricIo41cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#7"]
pub mod pric_io41c;
#[doc = "PRIC_IO42C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io42c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io42c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io42c`] module"]
#[doc(alias = "PRIC_IO42C")]
pub type PricIo42c = crate::Reg<pric_io42c::PricIo42cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#11"]
pub mod pric_io42c;
#[doc = "PRIC_IO43C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io43c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io43c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io43c`] module"]
#[doc(alias = "PRIC_IO43C")]
pub type PricIo43c = crate::Reg<pric_io43c::PricIo43cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#15"]
pub mod pric_io43c;
#[doc = "PRIC_IO44C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io44c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io44c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io44c`] module"]
#[doc(alias = "PRIC_IO44C")]
pub type PricIo44c = crate::Reg<pric_io44c::PricIo44cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#19"]
pub mod pric_io44c;
#[doc = "PRIC_IO45C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io45c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io45c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io45c`] module"]
#[doc(alias = "PRIC_IO45C")]
pub type PricIo45c = crate::Reg<pric_io45c::PricIo45cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#23"]
pub mod pric_io45c;
#[doc = "PRIC_IO46C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io46c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io46c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io46c`] module"]
#[doc(alias = "PRIC_IO46C")]
pub type PricIo46c = crate::Reg<pric_io46c::PricIo46cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#27"]
pub mod pric_io46c;
#[doc = "PRIC_IO47C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io47c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io47c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io47c`] module"]
#[doc(alias = "PRIC_IO47C")]
pub type PricIo47c = crate::Reg<pric_io47c::PricIo47cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#21"]
pub mod pric_io47c;
#[doc = "PRIC_IO48C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io48c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io48c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io48c`] module"]
#[doc(alias = "PRIC_IO48C")]
pub type PricIo48c = crate::Reg<pric_io48c::PricIo48cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#35"]
pub mod pric_io48c;
#[doc = "PRIC_IO49C (rw) register accessor: Memory Region Proection for AHB Masters Register \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io49c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io49c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io49c`] module"]
#[doc(alias = "PRIC_IO49C")]
pub type PricIo49c = crate::Reg<pric_io49c::PricIo49cSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#39"]
pub mod pric_io49c;
#[doc = "PRIC_IO4AC (rw) register accessor: Memory Region Proection for AHB Masters Register \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4ac`] module"]
#[doc(alias = "PRIC_IO4AC")]
pub type PricIo4ac = crate::Reg<pric_io4ac::PricIo4acSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#43"]
pub mod pric_io4ac;
#[doc = "PRIC_IO4BC (rw) register accessor: Memory Region Proection for AHB Masters Register \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4bc`] module"]
#[doc(alias = "PRIC_IO4BC")]
pub type PricIo4bc = crate::Reg<pric_io4bc::PricIo4bcSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#47"]
pub mod pric_io4bc;
#[doc = "PRIC_IO4CC (rw) register accessor: Memory Region Proection for AHB Masters Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4cc`] module"]
#[doc(alias = "PRIC_IO4CC")]
pub type PricIo4cc = crate::Reg<pric_io4cc::PricIo4ccSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#41"]
pub mod pric_io4cc;
#[doc = "PRIC_IO4DC (rw) register accessor: Memory Region Proection for AHB Masters Register \\#55\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4dc`] module"]
#[doc(alias = "PRIC_IO4DC")]
pub type PricIo4dc = crate::Reg<pric_io4dc::PricIo4dcSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#55"]
pub mod pric_io4dc;
#[doc = "PRIC_IO4EC (rw) register accessor: Memory Region Proection for AHB Masters Register \\#59\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4ec`] module"]
#[doc(alias = "PRIC_IO4EC")]
pub type PricIo4ec = crate::Reg<pric_io4ec::PricIo4ecSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#59"]
pub mod pric_io4ec;
#[doc = "PRIC_IO4FC (rw) register accessor: Memory Region Proection for AHB Masters Register \\#63\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pric_io4fc`] module"]
#[doc(alias = "PRIC_IO4FC")]
pub type PricIo4fc = crate::Reg<pric_io4fc::PricIo4fcSpec>;
#[doc = "Memory Region Proection for AHB Masters Register \\#63"]
pub mod pric_io4fc;
