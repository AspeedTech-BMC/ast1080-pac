#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    scu000: Scu000,
    _reserved1: [u8; 0x0c],
    scu010: Scu010,
    scu014: Scu014,
    _reserved3: [u8; 0x08],
    scu020: Scu020,
    scu024: Scu024,
    scu028: Scu028,
    scu02c: Scu02c,
    scu030: Scu030,
    scu034: Scu034,
    _reserved9: [u8; 0x08],
    scu040: Scu040,
    scu044: Scu044,
    scu048: Scu048,
    scu04c: Scu04c,
    scu050: Scu050,
    scu054: Scu054,
    _reserved15: [u8; 0x08],
    scu060: Scu060,
    scu064: Scu064,
    scu068: Scu068,
    scu06c: Scu06c,
    scu070: Scu070,
    scu074: Scu074,
    scu078: Scu078,
    scu07c: Scu07c,
    scu080: Scu080,
    scu084: Scu084,
    scu088: Scu088,
    scu08c: Scu08c,
    scu090: Scu090,
    scu094: Scu094,
    scu098: Scu098,
    scu09c: Scu09c,
    scu0a0: Scu0a0,
    scu0a4: Scu0a4,
    scu0a8: Scu0a8,
    scu0ac: Scu0ac,
    _reserved35: [u8; 0x0c],
    scu0bc: Scu0bc,
    scu0c0: Scu0c0,
    scu0c4: Scu0c4,
    scu0c8: Scu0c8,
    scu0cc: Scu0cc,
    scu0d0: Scu0d0,
    scu0d4: Scu0d4,
    _reserved42: [u8; 0x18],
    scu0f0: Scu0f0,
    scu0f4: Scu0f4,
    _reserved44: [u8; 0x08],
    scu100: Scu100,
    scu104: Scu104,
    _reserved46: [u8; 0x08],
    scu110: Scu110,
    scu114: Scu114,
    _reserved48: [u8; 0x08],
    scu120: Scu120,
    scu124: Scu124,
    scu128: Scu128,
    scu12c: Scu12c,
    scu130: Scu130,
    scu134: Scu134,
    _reserved54: [u8; 0x08],
    scu140: Scu140,
    scu144: Scu144,
    scu148: Scu148,
    scu14c: Scu14c,
    scu150: Scu150,
    scu154: Scu154,
    scu158: Scu158,
    scu15c: Scu15c,
    scu160: Scu160,
    _reserved63: [u8; 0x1c],
    scu180: Scu180,
    scu184: Scu184,
    scu188: Scu188,
    scu18c: Scu18c,
    scu190: Scu190,
    scu194: Scu194,
    scu198: Scu198,
    scu19c: Scu19c,
    scu1a0: Scu1a0,
    scu1a4: Scu1a4,
    scu1a8: Scu1a8,
    scu1ac: Scu1ac,
    scu1b0: Scu1b0,
    scu1b4: Scu1b4,
    scu1b8: Scu1b8,
    scu1bc: Scu1bc,
    scu1c0: Scu1c0,
    scu1c4: Scu1c4,
    scu1c8: Scu1c8,
    scu1cc: Scu1cc,
    scu1d0: Scu1d0,
    scu1d4: Scu1d4,
    scu1d8: Scu1d8,
    scu1dc: Scu1dc,
    scu1e0: Scu1e0,
    scu1e4: Scu1e4,
    scu1e8: Scu1e8,
    scu1ec: Scu1ec,
    scu1f0: Scu1f0,
    scu1f4: Scu1f4,
    scu1f8: Scu1f8,
    scu1fc: Scu1fc,
    scu200: Scu200,
    scu204: Scu204,
    _reserved97: [u8; 0x08],
    scu210: Scu210,
    scu214: Scu214,
    scu218: Scu218,
    scu21c: Scu21c,
    scu220: Scu220,
    scu224: Scu224,
    _reserved103: [u8; 0x08],
    scu230: Scu230,
    scu234: Scu234,
    scu238: Scu238,
    scu23c: Scu23c,
    scu240: Scu240,
    scu244: Scu244,
    _reserved109: [u8; 0x08],
    scu250: Scu250,
    scu254: Scu254,
    scu258: Scu258,
    scu25c: Scu25c,
    scu260: Scu260,
    scu264: Scu264,
    _reserved115: [u8; 0x08],
    scu270: Scu270,
    scu274: Scu274,
    scu278: Scu278,
    scu27c: Scu27c,
    scu280: Scu280,
    scu284: Scu284,
    _reserved121: [u8; 0x18],
    scu2a0: Scu2a0,
    scu2a4: Scu2a4,
    scu2a8: Scu2a8,
    scu2ac: Scu2ac,
    scu2b0: Scu2b0,
    scu2b4: Scu2b4,
    scu2b8: Scu2b8,
    scu2bc: Scu2bc,
    _reserved129: [u8; 0x30],
    scu2f0: Scu2f0,
    scu2f4: Scu2f4,
    scu2f8: Scu2f8,
    _reserved132: [u8; 0x04],
    scu300: Scu300,
    scu304: Scu304,
    _reserved134: [u8; 0x08],
    scu310: Scu310,
    scu314: Scu314,
    scu318: Scu318,
    scu31c: Scu31c,
    scu320: Scu320,
    scu324: Scu324,
    _reserved140: [u8; 0x04],
    scu32c: Scu32c,
    scu330: Scu330,
    scu334: Scu334,
    _reserved143: [u8; 0x48],
    scu380: Scu380,
    scu384: Scu384,
    scu388: Scu388,
    _reserved146: [u8; 0x04],
    scu390: Scu390,
    scu394: Scu394,
    scu398: Scu398,
    _reserved149: [u8; 0x04],
    scu3a0: Scu3a0,
    scu3a4: Scu3a4,
    _reserved151: [u8; 0x08],
    scu3b0: Scu3b0,
    scu3b4: Scu3b4,
    scu3b8: Scu3b8,
    scu3bc: Scu3bc,
    scu3c0: Scu3c0,
    _reserved156: [u8; 0x3c],
    scu400: Scu400,
    scu404: Scu404,
    scu408: Scu408,
    scu40c: Scu40c,
    scu410: Scu410,
    scu414: Scu414,
    scu418: Scu418,
    scu41c: Scu41c,
    scu420: Scu420,
    scu424: Scu424,
    scu428: Scu428,
    scu42c: Scu42c,
    scu430: Scu430,
    scu434: Scu434,
    scu438: Scu438,
    scu43c: Scu43c,
    scu440: Scu440,
    scu444: Scu444,
    scu448: Scu448,
    scu44c: Scu44c,
    scu450: Scu450,
    scu454: Scu454,
    scu458: Scu458,
    scu45c: Scu45c,
    scu460: Scu460,
    _reserved181: [u8; 0x1c],
    scu480: Scu480,
    scu484: Scu484,
    scu488: Scu488,
    scu48c: Scu48c,
    scu490: Scu490,
    scu494: Scu494,
    scu498: Scu498,
    scu49c: Scu49c,
    scu4a0: Scu4a0,
    scu4a4: Scu4a4,
    scu4a8: Scu4a8,
    scu4ac: Scu4ac,
    scu4b0: Scu4b0,
    scu4b4: Scu4b4,
    scu4b8: Scu4b8,
    scu4bc: Scu4bc,
    scu4c0: Scu4c0,
    scu4c4: Scu4c4,
    scu4c8: Scu4c8,
    scu4cc: Scu4cc,
    scu4d0: Scu4d0,
    scu4d4: Scu4d4,
    scu4d8: Scu4d8,
    scu4dc: Scu4dc,
    scu4e0: Scu4e0,
    scu4e4: Scu4e4,
    scu4e8: Scu4e8,
    scu4ec: Scu4ec,
    scu4f0: Scu4f0,
    scu4f4: Scu4f4,
    scu4f8: Scu4f8,
    scu4fc: Scu4fc,
    scu500: Scu500,
    scu504: Scu504,
    scu508: Scu508,
    scu50c: Scu50c,
    scu510: Scu510,
    scu514: Scu514,
    scu518: Scu518,
    scu51c: Scu51c,
    scu520: Scu520,
    scu524: Scu524,
    scu528: Scu528,
    scu52c: Scu52c,
    scu530: Scu530,
    scu534: Scu534,
    scu538: Scu538,
    scu53c: Scu53c,
    scu540: Scu540,
    scu544: Scu544,
    scu548: Scu548,
    scu54c: Scu54c,
    scu550: Scu550,
    scu554: Scu554,
    scu558: Scu558,
    scu55c: Scu55c,
    scu560: Scu560,
    scu564: Scu564,
    scu568: Scu568,
    scu56c: Scu56c,
    scu570: Scu570,
    scu574: Scu574,
    scu578: Scu578,
    scu57c: Scu57c,
    scu580: Scu580,
    scu584: Scu584,
    scu588: Scu588,
    scu58c: Scu58c,
    scu590: Scu590,
    scu594: Scu594,
    scu598: Scu598,
    scu59c: Scu59c,
    scu5a0: Scu5a0,
    scu5a4: Scu5a4,
    scu5a8: Scu5a8,
    scu5ac: Scu5ac,
    scu5b0: Scu5b0,
    scu5b4: Scu5b4,
    scu5b8: Scu5b8,
    scu5bc: Scu5bc,
    scu5c0: Scu5c0,
    scu5c4: Scu5c4,
    scu5c8: Scu5c8,
    scu5cc: Scu5cc,
    scu5d0: Scu5d0,
    scu5d4: Scu5d4,
    scu5d8: Scu5d8,
    scu5dc: Scu5dc,
    scu5e0: Scu5e0,
    scu5e4: Scu5e4,
    scu5e8: Scu5e8,
    scu5ec: Scu5ec,
    scu5f0: Scu5f0,
    scu5f4: Scu5f4,
    scu5f8: Scu5f8,
    scu5fc: Scu5fc,
    scu600: Scu600,
    _reserved278: [u8; 0x7c],
    scu680: Scu680,
    scu684: Scu684,
    scu688: Scu688,
    scu68c: Scu68c,
    _reserved282: [u8; 0x6c],
    scu6fc: Scu6fc,
    scu700: Scu700,
    scu704: Scu704,
    scu708: Scu708,
    scu70c: Scu70c,
    scu710: Scu710,
    scu714: Scu714,
    scu718: Scu718,
    _reserved290: [u8; 0x24],
    scu740: Scu740,
    scu744: Scu744,
    scu748: Scu748,
    scu74c: Scu74c,
    scu750: Scu750,
    scu754: Scu754,
    scu758: Scu758,
    _reserved297: [u8; 0x24],
    scu780: Scu780,
    scu784: Scu784,
    scu788: Scu788,
    scu78c: Scu78c,
    scu790: Scu790,
    scu794: Scu794,
    scu798: Scu798,
    _reserved304: [u8; 0x24],
    scu7c0: Scu7c0,
    scu7c4: Scu7c4,
    scu7c8: Scu7c8,
    scu7cc: Scu7cc,
    scu7d0: Scu7d0,
    scu7d4: Scu7d4,
    scu7d8: Scu7d8,
    _reserved311: [u8; 0x24],
    scu800: Scu800,
    scu804: Scu804,
    scu808: Scu808,
    scu80c: Scu80c,
    scu810: Scu810,
    scu814: Scu814,
    scu818: Scu818,
    scu81c: Scu81c,
    scu820: Scu820,
    scu824: Scu824,
    scu828: Scu828,
    scu82c: Scu82c,
    scu830: Scu830,
    scu834: Scu834,
    scu838: Scu838,
    scu83c: Scu83c,
    scu840: Scu840,
    scu844: Scu844,
    scu848: Scu848,
    scu84c: Scu84c,
    scu850: Scu850,
    scu854: Scu854,
    scu858: Scu858,
    scu85c: Scu85c,
    scu860: Scu860,
    scu864: Scu864,
    scu868: Scu868,
    scu86c: Scu86c,
    scu870: Scu870,
    scu874: Scu874,
    scu878: Scu878,
    scu87c: Scu87c,
    scu880: Scu880,
    scu884: Scu884,
    scu888: Scu888,
    scu88c: Scu88c,
    scu890: Scu890,
    scu894: Scu894,
    scu898: Scu898,
    scu89c: Scu89c,
    scu8a0: Scu8a0,
    scu8a4: Scu8a4,
    scu8a8: Scu8a8,
    scu8ac: Scu8ac,
    scu8b0: Scu8b0,
    scu8b4: Scu8b4,
    scu8b8: Scu8b8,
    scu8bc: Scu8bc,
    scu8c0: Scu8c0,
    scu8c4: Scu8c4,
    scu8c8: Scu8c8,
    scu8cc: Scu8cc,
    scu8d0: Scu8d0,
    scu8d4: Scu8d4,
    scu8d8: Scu8d8,
    scu8dc: Scu8dc,
    scu8e0: Scu8e0,
    scu8e4: Scu8e4,
    scu8e8: Scu8e8,
    scu8ec: Scu8ec,
    scu8f0: Scu8f0,
    scu8f4: Scu8f4,
    scu8f8: Scu8f8,
    scu8fc: Scu8fc,
    scu900: Scu900,
    scu904: Scu904,
    scu908: Scu908,
    scu90c: Scu90c,
    scu910: Scu910,
    scu914: Scu914,
    scu918: Scu918,
    _reserved382: [u8; 0x04],
    scu920: Scu920,
    scu924: Scu924,
    scu928: Scu928,
    scu92c: Scu92c,
    scu930: Scu930,
    scu934: Scu934,
    _reserved388: [u8; 0x08],
    scu940: Scu940,
    _reserved389: [u8; 0x04],
    scu948: Scu948,
    scu94c: Scu94c,
    scu950: Scu950,
    scu954: Scu954,
    scu958: Scu958,
    _reserved394: [u8; 0x18],
    scu974: Scu974,
    _reserved395: [u8; 0x08],
    scu980: Scu980,
    scu984: Scu984,
    scu988: Scu988,
    scu98c: Scu98c,
    scu990: Scu990,
    scu994: Scu994,
    scu998: Scu998,
    scu99c: Scu99c,
    scu9a0: Scu9a0,
    scu9a4: Scu9a4,
    scu9a8: Scu9a8,
    scu9ac: Scu9ac,
    scu9b0: Scu9b0,
    scu9b4: Scu9b4,
    scu9b8: Scu9b8,
    scu9bc: Scu9bc,
    _reserved411: [u8; 0x0140],
    scub00: Scub00,
    scub04: Scub04,
    scub08: Scub08,
    scub0c: Scub0c,
    scub10: Scub10,
    scub14: Scub14,
    scub18: Scub18,
    scub1c: Scub1c,
    scub20: Scub20,
    scub24: Scub24,
    scub28: Scub28,
    scub2c: Scub2c,
    scub30: Scub30,
    scub34: Scub34,
    scub38: Scub38,
    scub3c: Scub3c,
    scub40: Scub40,
    scub44: Scub44,
    scub48: Scub48,
    scub4c: Scub4c,
    scub50: Scub50,
    scub54: Scub54,
    scub58: Scub58,
    scub5c: Scub5c,
    scub60: Scub60,
    scub64: Scub64,
    scub68: Scub68,
    scub6c: Scub6c,
    scub70: Scub70,
    scub74: Scub74,
    scub78: Scub78,
    scub7c: Scub7c,
    scub80: Scub80,
    _reserved444: [u8; 0x1c],
    scuba0: Scuba0,
    scuba4: Scuba4,
    scuba8: Scuba8,
    scubac: Scubac,
    scubb0: Scubb0,
    scubb4: Scubb4,
    scubb8: Scubb8,
    scubbc: Scubbc,
    scubc0: Scubc0,
    scubc4: Scubc4,
    scubc8: Scubc8,
    scubcc: Scubcc,
    scubd0: Scubd0,
    scubd4: Scubd4,
    scubd8: Scubd8,
    scubdc: Scubdc,
    scube0: Scube0,
    scube4: Scube4,
    scube8: Scube8,
    scubec: Scubec,
    scubf0: Scubf0,
    scubf4: Scubf4,
    scubf8: Scubf8,
    scubfc: Scubfc,
    _reserved468: [u8; 0x04],
    scuc04: Scuc04,
    scuc08: Scuc08,
    scuc0c: Scuc0c,
    _reserved471: [u8; 0x04],
    scuc14: Scuc14,
    scuc18: Scuc18,
    scuc1c: Scuc1c,
    _reserved474: [u8; 0x08],
    scuc28: Scuc28,
    _reserved475: [u8; 0x08],
    scuc34: Scuc34,
    _reserved476: [u8; 0x10],
    scuc48: Scuc48,
    scuc4c: Scuc4c,
    scuc50: Scuc50,
    _reserved479: [u8; 0x08],
    scuc5c: Scuc5c,
    _reserved480: [u8; 0x24],
    scuc84: Scuc84,
    scuc88: Scuc88,
    scuc8c: Scuc8c,
    _reserved483: [u8; 0x04],
    scuc94: Scuc94,
    scuc98: Scuc98,
    scuc9c: Scuc9c,
    _reserved486: [u8; 0x08],
    scuca8: Scuca8,
    _reserved487: [u8; 0x08],
    scucb4: Scucb4,
    _reserved488: [u8; 0x10],
    scucc8: Scucc8,
    scuccc: Scuccc,
    scucd0: Scucd0,
    _reserved491: [u8; 0x08],
    scucdc: Scucdc,
    _reserved492: [u8; 0x24],
    scud04: Scud04,
    scud08: Scud08,
    scud0c: Scud0c,
    _reserved495: [u8; 0x04],
    scud14: Scud14,
    scud18: Scud18,
    scud1c: Scud1c,
    _reserved498: [u8; 0x08],
    scud28: Scud28,
    _reserved499: [u8; 0x08],
    scud34: Scud34,
    _reserved500: [u8; 0x10],
    scud48: Scud48,
    scud4c: Scud4c,
    scud50: Scud50,
    _reserved503: [u8; 0x08],
    scud5c: Scud5c,
    _reserved504: [u8; 0xa0],
    scue00: Scue00,
    scue04: Scue04,
    scue08: Scue08,
    scue0c: Scue0c,
    scue10: Scue10,
    scue14: Scue14,
    scue18: Scue18,
    scue1c: Scue1c,
    _reserved512: [u8; 0x04],
    scue24: Scue24,
    scue28: Scue28,
    scue2c: Scue2c,
    scue30: Scue30,
    scue34: Scue34,
    scue38: Scue38,
    scue3c: Scue3c,
    scue40: Scue40,
    scue44: Scue44,
    scue48: Scue48,
    scue4c: Scue4c,
    scue50: Scue50,
    scue54: Scue54,
    _reserved525: [u8; 0x04],
    scue5c: Scue5c,
    scue60: Scue60,
    scue64: Scue64,
    scue68: Scue68,
    _reserved529: [u8; 0x04],
    scue70: Scue70,
    _reserved530: [u8; 0x04],
    scue78: Scue78,
    _reserved531: [u8; 0x84],
    scuf00: Scuf00,
    scuf04: Scuf04,
    scuf08: Scuf08,
    scuf0c: Scuf0c,
    scuf10: Scuf10,
    scuf14: Scuf14,
    scuf18: Scuf18,
    scuf1c: Scuf1c,
    scuf20: Scuf20,
    scuf24: Scuf24,
    scuf28: Scuf28,
    scuf2c: Scuf2c,
    scuf30: Scuf30,
    scuf34: Scuf34,
    scuf38: Scuf38,
    scuf3c: Scuf3c,
    _reserved547: [u8; 0x08],
    scuf48: Scuf48,
    scuf4c: Scuf4c,
    scuf50: Scuf50,
    _reserved550: [u8; 0x08],
    scuf5c: Scuf5c,
    scuf60: Scuf60,
    scuf64: Scuf64,
    scuf68: Scuf68,
    scuf6c: Scuf6c,
    scuf70: Scuf70,
    _reserved556: [u8; 0x04],
    scuf78: Scuf78,
}
impl RegisterBlock {
    #[doc = "0x00 - Silicon Revision ID Register"]
    #[inline(always)]
    pub const fn scu000(&self) -> &Scu000 {
        &self.scu000
    }
    #[doc = "0x10 - Hardware/OTP Strap 1 Register"]
    #[inline(always)]
    pub const fn scu010(&self) -> &Scu010 {
        &self.scu010
    }
    #[doc = "0x14 - Hareware/OTP Strap Clear 1 Register"]
    #[inline(always)]
    pub const fn scu014(&self) -> &Scu014 {
        &self.scu014
    }
    #[doc = "0x20 - Hardware/OTP Strap Write Protection 1 Register"]
    #[inline(always)]
    pub const fn scu020(&self) -> &Scu020 {
        &self.scu020
    }
    #[doc = "0x24 - Hardware/OTP Strap Secure Control Register 1"]
    #[inline(always)]
    pub const fn scu024(&self) -> &Scu024 {
        &self.scu024
    }
    #[doc = "0x28 - Hardware/OTP Strap Secure Control Register 2"]
    #[inline(always)]
    pub const fn scu028(&self) -> &Scu028 {
        &self.scu028
    }
    #[doc = "0x2c - Hardware/OTP Strap Secure Control Register 3"]
    #[inline(always)]
    pub const fn scu02c(&self) -> &Scu02c {
        &self.scu02c
    }
    #[doc = "0x30 - Hardware/OTP Strap 2 Register"]
    #[inline(always)]
    pub const fn scu030(&self) -> &Scu030 {
        &self.scu030
    }
    #[doc = "0x34 - Hareware/OTP Strap Clear 2 Register"]
    #[inline(always)]
    pub const fn scu034(&self) -> &Scu034 {
        &self.scu034
    }
    #[doc = "0x40 - Hardware/OTP Strap Write Protection 2 Register"]
    #[inline(always)]
    pub const fn scu040(&self) -> &Scu040 {
        &self.scu040
    }
    #[doc = "0x44 - Hardware/OTP Strap 2 Secure Control Register 1"]
    #[inline(always)]
    pub const fn scu044(&self) -> &Scu044 {
        &self.scu044
    }
    #[doc = "0x48 - Hardware/OTP Strap 2 Secure Control Register 2"]
    #[inline(always)]
    pub const fn scu048(&self) -> &Scu048 {
        &self.scu048
    }
    #[doc = "0x4c - Hardware/OTP Strap 2 Secure Control Register 3"]
    #[inline(always)]
    pub const fn scu04c(&self) -> &Scu04c {
        &self.scu04c
    }
    #[doc = "0x50 - Hardware/OTP Strap 3 Register"]
    #[inline(always)]
    pub const fn scu050(&self) -> &Scu050 {
        &self.scu050
    }
    #[doc = "0x54 - Hareware/OTP Strap Clear 3 Register"]
    #[inline(always)]
    pub const fn scu054(&self) -> &Scu054 {
        &self.scu054
    }
    #[doc = "0x60 - Hardware/OTP Strap Write Protection 3 Register"]
    #[inline(always)]
    pub const fn scu060(&self) -> &Scu060 {
        &self.scu060
    }
    #[doc = "0x64 - Hardware/OTP Strap 3 Secure Control Register 1"]
    #[inline(always)]
    pub const fn scu064(&self) -> &Scu064 {
        &self.scu064
    }
    #[doc = "0x68 - Hardware/OTP Strap 3 Secure Control Register 2"]
    #[inline(always)]
    pub const fn scu068(&self) -> &Scu068 {
        &self.scu068
    }
    #[doc = "0x6c - Hardware/OTP Strap 3 Secure Control Register 3"]
    #[inline(always)]
    pub const fn scu06c(&self) -> &Scu06c {
        &self.scu06c
    }
    #[doc = "0x70 - System Reset Event Log Set 1 Register"]
    #[inline(always)]
    pub const fn scu070(&self) -> &Scu070 {
        &self.scu070
    }
    #[doc = "0x74 - System Reset Event Log Set 1 Secure Register 1"]
    #[inline(always)]
    pub const fn scu074(&self) -> &Scu074 {
        &self.scu074
    }
    #[doc = "0x78 - System Reset Event Log Set 1 Secure Register 2"]
    #[inline(always)]
    pub const fn scu078(&self) -> &Scu078 {
        &self.scu078
    }
    #[doc = "0x7c - System Reset Event Log Set 1 Secure Register 3"]
    #[inline(always)]
    pub const fn scu07c(&self) -> &Scu07c {
        &self.scu07c
    }
    #[doc = "0x80 - System Reset Event Log Set 2 Register"]
    #[inline(always)]
    pub const fn scu080(&self) -> &Scu080 {
        &self.scu080
    }
    #[doc = "0x84 - System Reset Event Log Set 2 Secure Register 1"]
    #[inline(always)]
    pub const fn scu084(&self) -> &Scu084 {
        &self.scu084
    }
    #[doc = "0x88 - System Reset Event Log Set 2 Secure Register 2"]
    #[inline(always)]
    pub const fn scu088(&self) -> &Scu088 {
        &self.scu088
    }
    #[doc = "0x8c - System Reset Event Log Set 2 Secure Register 3"]
    #[inline(always)]
    pub const fn scu08c(&self) -> &Scu08c {
        &self.scu08c
    }
    #[doc = "0x90 - System Reset Event Log Set 3 Register"]
    #[inline(always)]
    pub const fn scu090(&self) -> &Scu090 {
        &self.scu090
    }
    #[doc = "0x94 - System Reset Event Log Set 3 Secure Register 1"]
    #[inline(always)]
    pub const fn scu094(&self) -> &Scu094 {
        &self.scu094
    }
    #[doc = "0x98 - System Reset Event Log Set 3 Secure Register 2"]
    #[inline(always)]
    pub const fn scu098(&self) -> &Scu098 {
        &self.scu098
    }
    #[doc = "0x9c - System Reset Event Log Set 3 Secure Register 3"]
    #[inline(always)]
    pub const fn scu09c(&self) -> &Scu09c {
        &self.scu09c
    }
    #[doc = "0xa0 - System Reset Event Log Set 4 Register"]
    #[inline(always)]
    pub const fn scu0a0(&self) -> &Scu0a0 {
        &self.scu0a0
    }
    #[doc = "0xa4 - System Reset Event Log Set 4 Secure Register 1"]
    #[inline(always)]
    pub const fn scu0a4(&self) -> &Scu0a4 {
        &self.scu0a4
    }
    #[doc = "0xa8 - System Reset Event Log Set 4 Secure Register 2"]
    #[inline(always)]
    pub const fn scu0a8(&self) -> &Scu0a8 {
        &self.scu0a8
    }
    #[doc = "0xac - System Reset Event Log Set 4 Secure Register 3"]
    #[inline(always)]
    pub const fn scu0ac(&self) -> &Scu0ac {
        &self.scu0ac
    }
    #[doc = "0xbc - Debug UART Baudrate"]
    #[inline(always)]
    pub const fn scu0bc(&self) -> &Scu0bc {
        &self.scu0bc
    }
    #[doc = "0xc0 - Misc. Control Set 1 Register"]
    #[inline(always)]
    pub const fn scu0c0(&self) -> &Scu0c0 {
        &self.scu0c0
    }
    #[doc = "0xc4 - Debug Selection Register"]
    #[inline(always)]
    pub const fn scu0c4(&self) -> &Scu0c4 {
        &self.scu0c4
    }
    #[doc = "0xc8 - Debug Disable Register"]
    #[inline(always)]
    pub const fn scu0c8(&self) -> &Scu0c8 {
        &self.scu0c8
    }
    #[doc = "0xcc - HeartBeat Control Register"]
    #[inline(always)]
    pub const fn scu0cc(&self) -> &Scu0cc {
        &self.scu0cc
    }
    #[doc = "0xd0 - Analog Mux Mode Register"]
    #[inline(always)]
    pub const fn scu0d0(&self) -> &Scu0d0 {
        &self.scu0d0
    }
    #[doc = "0xd4 - SPI Mode Register"]
    #[inline(always)]
    pub const fn scu0d4(&self) -> &Scu0d4 {
        &self.scu0d4
    }
    #[doc = "0xf0 - Random Number Generator Control Register"]
    #[inline(always)]
    pub const fn scu0f0(&self) -> &Scu0f0 {
        &self.scu0f0
    }
    #[doc = "0xf4 - Random Number Generator Data Register"]
    #[inline(always)]
    pub const fn scu0f4(&self) -> &Scu0f4 {
        &self.scu0f4
    }
    #[doc = "0x100 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn scu100(&self) -> &Scu100 {
        &self.scu100
    }
    #[doc = "0x104 - Interrupt Control Register"]
    #[inline(always)]
    pub const fn scu104(&self) -> &Scu104 {
        &self.scu104
    }
    #[doc = "0x110 - MCU Control and Status Register"]
    #[inline(always)]
    pub const fn scu110(&self) -> &Scu110 {
        &self.scu110
    }
    #[doc = "0x114 - MCU Reset Vector"]
    #[inline(always)]
    pub const fn scu114(&self) -> &Scu114 {
        &self.scu114
    }
    #[doc = "0x120 - CPTRA Page Register 0"]
    #[inline(always)]
    pub const fn scu120(&self) -> &Scu120 {
        &self.scu120
    }
    #[doc = "0x124 - CPTRA Page Register 1"]
    #[inline(always)]
    pub const fn scu124(&self) -> &Scu124 {
        &self.scu124
    }
    #[doc = "0x128 - CPTRA Page Register 2"]
    #[inline(always)]
    pub const fn scu128(&self) -> &Scu128 {
        &self.scu128
    }
    #[doc = "0x12c - CPTRA Page Register 3"]
    #[inline(always)]
    pub const fn scu12c(&self) -> &Scu12c {
        &self.scu12c
    }
    #[doc = "0x130 - CPTRA Page Register 4"]
    #[inline(always)]
    pub const fn scu130(&self) -> &Scu130 {
        &self.scu130
    }
    #[doc = "0x134 - CPTRA Page Register 5"]
    #[inline(always)]
    pub const fn scu134(&self) -> &Scu134 {
        &self.scu134
    }
    #[doc = "0x140 - Caliptra Config Register 0"]
    #[inline(always)]
    pub const fn scu140(&self) -> &Scu140 {
        &self.scu140
    }
    #[doc = "0x144 - Caliptra Config Register 1"]
    #[inline(always)]
    pub const fn scu144(&self) -> &Scu144 {
        &self.scu144
    }
    #[doc = "0x148 - Caliptra Config Register 2"]
    #[inline(always)]
    pub const fn scu148(&self) -> &Scu148 {
        &self.scu148
    }
    #[doc = "0x14c - Caliptra Config Register 3"]
    #[inline(always)]
    pub const fn scu14c(&self) -> &Scu14c {
        &self.scu14c
    }
    #[doc = "0x150 - Caliptra Config Register 4"]
    #[inline(always)]
    pub const fn scu150(&self) -> &Scu150 {
        &self.scu150
    }
    #[doc = "0x154 - Caliptra Config Register 5"]
    #[inline(always)]
    pub const fn scu154(&self) -> &Scu154 {
        &self.scu154
    }
    #[doc = "0x158 - Caliptra Config Register 6"]
    #[inline(always)]
    pub const fn scu158(&self) -> &Scu158 {
        &self.scu158
    }
    #[doc = "0x15c - Caliptra Config Register 7"]
    #[inline(always)]
    pub const fn scu15c(&self) -> &Scu15c {
        &self.scu15c
    }
    #[doc = "0x160 - Caliptra Config Register 8"]
    #[inline(always)]
    pub const fn scu160(&self) -> &Scu160 {
        &self.scu160
    }
    #[doc = "0x180 - SCU\\_CPU\\_SCRATCH\\_1"]
    #[inline(always)]
    pub const fn scu180(&self) -> &Scu180 {
        &self.scu180
    }
    #[doc = "0x184 - SCU\\_CPU\\_SCRATCH\\_2"]
    #[inline(always)]
    pub const fn scu184(&self) -> &Scu184 {
        &self.scu184
    }
    #[doc = "0x188 - SCU\\_CPU\\_SCRATCH\\_3"]
    #[inline(always)]
    pub const fn scu188(&self) -> &Scu188 {
        &self.scu188
    }
    #[doc = "0x18c - SCU\\_CPU\\_SCRATCH\\_4"]
    #[inline(always)]
    pub const fn scu18c(&self) -> &Scu18c {
        &self.scu18c
    }
    #[doc = "0x190 - SCU\\_CPU\\_SCRATCH\\_5"]
    #[inline(always)]
    pub const fn scu190(&self) -> &Scu190 {
        &self.scu190
    }
    #[doc = "0x194 - SCU\\_CPU\\_SCRATCH\\_6"]
    #[inline(always)]
    pub const fn scu194(&self) -> &Scu194 {
        &self.scu194
    }
    #[doc = "0x198 - SCU\\_CPU\\_SCRATCH\\_7"]
    #[inline(always)]
    pub const fn scu198(&self) -> &Scu198 {
        &self.scu198
    }
    #[doc = "0x19c - SCU\\_CPU\\_SCRATCH\\_8"]
    #[inline(always)]
    pub const fn scu19c(&self) -> &Scu19c {
        &self.scu19c
    }
    #[doc = "0x1a0 - SCU\\_CPU\\_SCRATCH\\_9"]
    #[inline(always)]
    pub const fn scu1a0(&self) -> &Scu1a0 {
        &self.scu1a0
    }
    #[doc = "0x1a4 - SCU\\_CPU\\_SCRATCH\\_10"]
    #[inline(always)]
    pub const fn scu1a4(&self) -> &Scu1a4 {
        &self.scu1a4
    }
    #[doc = "0x1a8 - SCU\\_CPU\\_SCRATCH\\_11"]
    #[inline(always)]
    pub const fn scu1a8(&self) -> &Scu1a8 {
        &self.scu1a8
    }
    #[doc = "0x1ac - SCU\\_CPU\\_SCRATCH\\_12"]
    #[inline(always)]
    pub const fn scu1ac(&self) -> &Scu1ac {
        &self.scu1ac
    }
    #[doc = "0x1b0 - SCU\\_CPU\\_SCRATCH\\_13"]
    #[inline(always)]
    pub const fn scu1b0(&self) -> &Scu1b0 {
        &self.scu1b0
    }
    #[doc = "0x1b4 - SCU\\_CPU\\_SCRATCH\\_14"]
    #[inline(always)]
    pub const fn scu1b4(&self) -> &Scu1b4 {
        &self.scu1b4
    }
    #[doc = "0x1b8 - SCU\\_CPU\\_SCRATCH\\_15"]
    #[inline(always)]
    pub const fn scu1b8(&self) -> &Scu1b8 {
        &self.scu1b8
    }
    #[doc = "0x1bc - SCU\\_CPU\\_SCRATCH\\_16"]
    #[inline(always)]
    pub const fn scu1bc(&self) -> &Scu1bc {
        &self.scu1bc
    }
    #[doc = "0x1c0 - SCU\\_CPU\\_SCRATCH\\_17"]
    #[inline(always)]
    pub const fn scu1c0(&self) -> &Scu1c0 {
        &self.scu1c0
    }
    #[doc = "0x1c4 - SCU\\_CPU\\_SCRATCH\\_18"]
    #[inline(always)]
    pub const fn scu1c4(&self) -> &Scu1c4 {
        &self.scu1c4
    }
    #[doc = "0x1c8 - SCU\\_CPU\\_SCRATCH\\_19"]
    #[inline(always)]
    pub const fn scu1c8(&self) -> &Scu1c8 {
        &self.scu1c8
    }
    #[doc = "0x1cc - SCU\\_CPU\\_SCRATCH\\_20"]
    #[inline(always)]
    pub const fn scu1cc(&self) -> &Scu1cc {
        &self.scu1cc
    }
    #[doc = "0x1d0 - SCU\\_CPU\\_SCRATCH\\_21"]
    #[inline(always)]
    pub const fn scu1d0(&self) -> &Scu1d0 {
        &self.scu1d0
    }
    #[doc = "0x1d4 - SCU\\_CPU\\_SCRATCH\\_22"]
    #[inline(always)]
    pub const fn scu1d4(&self) -> &Scu1d4 {
        &self.scu1d4
    }
    #[doc = "0x1d8 - SCU\\_CPU\\_SCRATCH\\_23"]
    #[inline(always)]
    pub const fn scu1d8(&self) -> &Scu1d8 {
        &self.scu1d8
    }
    #[doc = "0x1dc - SCU\\_CPU\\_SCRATCH\\_24"]
    #[inline(always)]
    pub const fn scu1dc(&self) -> &Scu1dc {
        &self.scu1dc
    }
    #[doc = "0x1e0 - SCU\\_CPU\\_SCRATCH\\_25"]
    #[inline(always)]
    pub const fn scu1e0(&self) -> &Scu1e0 {
        &self.scu1e0
    }
    #[doc = "0x1e4 - SCU\\_CPU\\_SCRATCH\\_26"]
    #[inline(always)]
    pub const fn scu1e4(&self) -> &Scu1e4 {
        &self.scu1e4
    }
    #[doc = "0x1e8 - SCU\\_CPU\\_SCRATCH\\_27"]
    #[inline(always)]
    pub const fn scu1e8(&self) -> &Scu1e8 {
        &self.scu1e8
    }
    #[doc = "0x1ec - SCU\\_CPU\\_SCRATCH\\_28"]
    #[inline(always)]
    pub const fn scu1ec(&self) -> &Scu1ec {
        &self.scu1ec
    }
    #[doc = "0x1f0 - SCU\\_CPU\\_SCRATCH\\_29"]
    #[inline(always)]
    pub const fn scu1f0(&self) -> &Scu1f0 {
        &self.scu1f0
    }
    #[doc = "0x1f4 - SCU\\_CPU\\_SCRATCH\\_30"]
    #[inline(always)]
    pub const fn scu1f4(&self) -> &Scu1f4 {
        &self.scu1f4
    }
    #[doc = "0x1f8 - SCU\\_CPU\\_SCRATCH\\_31"]
    #[inline(always)]
    pub const fn scu1f8(&self) -> &Scu1f8 {
        &self.scu1f8
    }
    #[doc = "0x1fc - SCU\\_CPU\\_SCRATCH\\_32"]
    #[inline(always)]
    pub const fn scu1fc(&self) -> &Scu1fc {
        &self.scu1fc
    }
    #[doc = "0x200 - System Reset Control 1 Register"]
    #[inline(always)]
    pub const fn scu200(&self) -> &Scu200 {
        &self.scu200
    }
    #[doc = "0x204 - System Reset Clear 1 Register"]
    #[inline(always)]
    pub const fn scu204(&self) -> &Scu204 {
        &self.scu204
    }
    #[doc = "0x210 - System Reset Lock 1 Register"]
    #[inline(always)]
    pub const fn scu210(&self) -> &Scu210 {
        &self.scu210
    }
    #[doc = "0x214 - System Reset Secure 1 Register 1"]
    #[inline(always)]
    pub const fn scu214(&self) -> &Scu214 {
        &self.scu214
    }
    #[doc = "0x218 - System Reset Secure 1 Register 2"]
    #[inline(always)]
    pub const fn scu218(&self) -> &Scu218 {
        &self.scu218
    }
    #[doc = "0x21c - System Reset Secure 1 Register 3"]
    #[inline(always)]
    pub const fn scu21c(&self) -> &Scu21c {
        &self.scu21c
    }
    #[doc = "0x220 - System Reset Control 2 Register"]
    #[inline(always)]
    pub const fn scu220(&self) -> &Scu220 {
        &self.scu220
    }
    #[doc = "0x224 - System Reset Clear 2 Register"]
    #[inline(always)]
    pub const fn scu224(&self) -> &Scu224 {
        &self.scu224
    }
    #[doc = "0x230 - System Reset Lock 2 Register"]
    #[inline(always)]
    pub const fn scu230(&self) -> &Scu230 {
        &self.scu230
    }
    #[doc = "0x234 - System Reset Secure 2 Register 1"]
    #[inline(always)]
    pub const fn scu234(&self) -> &Scu234 {
        &self.scu234
    }
    #[doc = "0x238 - System Reset Secure 2 Register 2"]
    #[inline(always)]
    pub const fn scu238(&self) -> &Scu238 {
        &self.scu238
    }
    #[doc = "0x23c - System Reset Secure 2 Register 3"]
    #[inline(always)]
    pub const fn scu23c(&self) -> &Scu23c {
        &self.scu23c
    }
    #[doc = "0x240 - Clock Stop Control 1 Register"]
    #[inline(always)]
    pub const fn scu240(&self) -> &Scu240 {
        &self.scu240
    }
    #[doc = "0x244 - Clock Stop Clear 1 Register"]
    #[inline(always)]
    pub const fn scu244(&self) -> &Scu244 {
        &self.scu244
    }
    #[doc = "0x250 - Clock Stop Lock 1 Register"]
    #[inline(always)]
    pub const fn scu250(&self) -> &Scu250 {
        &self.scu250
    }
    #[doc = "0x254 - Clock Stop Secure 1 Register 1"]
    #[inline(always)]
    pub const fn scu254(&self) -> &Scu254 {
        &self.scu254
    }
    #[doc = "0x258 - Clock Stop Secure 1 Register 2"]
    #[inline(always)]
    pub const fn scu258(&self) -> &Scu258 {
        &self.scu258
    }
    #[doc = "0x25c - Clock Stop Secure 1 Register 3"]
    #[inline(always)]
    pub const fn scu25c(&self) -> &Scu25c {
        &self.scu25c
    }
    #[doc = "0x260 - Clock Stop Control 2 Register"]
    #[inline(always)]
    pub const fn scu260(&self) -> &Scu260 {
        &self.scu260
    }
    #[doc = "0x264 - Clock Stop Clear 2 Register"]
    #[inline(always)]
    pub const fn scu264(&self) -> &Scu264 {
        &self.scu264
    }
    #[doc = "0x270 - Clock Stop Lock 2 Register"]
    #[inline(always)]
    pub const fn scu270(&self) -> &Scu270 {
        &self.scu270
    }
    #[doc = "0x274 - Clock Stop Secure 2 Register 1"]
    #[inline(always)]
    pub const fn scu274(&self) -> &Scu274 {
        &self.scu274
    }
    #[doc = "0x278 - Clock Stop Secure 2 Register 2"]
    #[inline(always)]
    pub const fn scu278(&self) -> &Scu278 {
        &self.scu278
    }
    #[doc = "0x27c - Clock Stop Secure 2 Register 3"]
    #[inline(always)]
    pub const fn scu27c(&self) -> &Scu27c {
        &self.scu27c
    }
    #[doc = "0x280 - Clock Selection 1 Register"]
    #[inline(always)]
    pub const fn scu280(&self) -> &Scu280 {
        &self.scu280
    }
    #[doc = "0x284 - Clock Selection 2 Register"]
    #[inline(always)]
    pub const fn scu284(&self) -> &Scu284 {
        &self.scu284
    }
    #[doc = "0x2a0 - Clock Selection Lock 1 Register"]
    #[inline(always)]
    pub const fn scu2a0(&self) -> &Scu2a0 {
        &self.scu2a0
    }
    #[doc = "0x2a4 - Clock Selection Secure 1 Register 1"]
    #[inline(always)]
    pub const fn scu2a4(&self) -> &Scu2a4 {
        &self.scu2a4
    }
    #[doc = "0x2a8 - Clock Selection Secure 1 Register 2"]
    #[inline(always)]
    pub const fn scu2a8(&self) -> &Scu2a8 {
        &self.scu2a8
    }
    #[doc = "0x2ac - Clock Selection Secure 1 Register 3"]
    #[inline(always)]
    pub const fn scu2ac(&self) -> &Scu2ac {
        &self.scu2ac
    }
    #[doc = "0x2b0 - Clock Selection Lock 2 Register"]
    #[inline(always)]
    pub const fn scu2b0(&self) -> &Scu2b0 {
        &self.scu2b0
    }
    #[doc = "0x2b4 - Clock Selection Secure 2 Register 1"]
    #[inline(always)]
    pub const fn scu2b4(&self) -> &Scu2b4 {
        &self.scu2b4
    }
    #[doc = "0x2b8 - Clock Selection Secure 2 Register 2"]
    #[inline(always)]
    pub const fn scu2b8(&self) -> &Scu2b8 {
        &self.scu2b8
    }
    #[doc = "0x2bc - Clock Selection Secure 2 Register 3"]
    #[inline(always)]
    pub const fn scu2bc(&self) -> &Scu2bc {
        &self.scu2bc
    }
    #[doc = "0x2f0 - EXTRST\\# Reset Selection 1 Register"]
    #[inline(always)]
    pub const fn scu2f0(&self) -> &Scu2f0 {
        &self.scu2f0
    }
    #[doc = "0x2f4 - EXTRST\\# Reset Selection 2 Register"]
    #[inline(always)]
    pub const fn scu2f4(&self) -> &Scu2f4 {
        &self.scu2f4
    }
    #[doc = "0x2f8 - EXTRST\\# Reset Selection 3 Register"]
    #[inline(always)]
    pub const fn scu2f8(&self) -> &Scu2f8 {
        &self.scu2f8
    }
    #[doc = "0x300 - HPLL Parameter Register"]
    #[inline(always)]
    pub const fn scu300(&self) -> &Scu300 {
        &self.scu300
    }
    #[doc = "0x304 - HPLL Extended Parameter Register"]
    #[inline(always)]
    pub const fn scu304(&self) -> &Scu304 {
        &self.scu304
    }
    #[doc = "0x310 - DIPLL Parameter Register"]
    #[inline(always)]
    pub const fn scu310(&self) -> &Scu310 {
        &self.scu310
    }
    #[doc = "0x314 - DIPLL Parameter Register 2"]
    #[inline(always)]
    pub const fn scu314(&self) -> &Scu314 {
        &self.scu314
    }
    #[doc = "0x318 - DIPLL Parameter Register 3"]
    #[inline(always)]
    pub const fn scu318(&self) -> &Scu318 {
        &self.scu318
    }
    #[doc = "0x31c - DIPLL Parameter Register 4"]
    #[inline(always)]
    pub const fn scu31c(&self) -> &Scu31c {
        &self.scu31c
    }
    #[doc = "0x320 - DIPLL Parameter Register 5"]
    #[inline(always)]
    pub const fn scu320(&self) -> &Scu320 {
        &self.scu320
    }
    #[doc = "0x324 - DIPLL Parameter Register 6"]
    #[inline(always)]
    pub const fn scu324(&self) -> &Scu324 {
        &self.scu324
    }
    #[doc = "0x32c - DIPLL Parameter Register 8"]
    #[inline(always)]
    pub const fn scu32c(&self) -> &Scu32c {
        &self.scu32c
    }
    #[doc = "0x330 - UARTCLK Generation Register"]
    #[inline(always)]
    pub const fn scu330(&self) -> &Scu330 {
        &self.scu330
    }
    #[doc = "0x334 - HUARTCLK Generation Register"]
    #[inline(always)]
    pub const fn scu334(&self) -> &Scu334 {
        &self.scu334
    }
    #[doc = "0x380 - Clock Duty Measurement Control Register"]
    #[inline(always)]
    pub const fn scu380(&self) -> &Scu380 {
        &self.scu380
    }
    #[doc = "0x384 - Clock Duty Selection Register"]
    #[inline(always)]
    pub const fn scu384(&self) -> &Scu384 {
        &self.scu384
    }
    #[doc = "0x388 - Clock Duty Measurement Result Register"]
    #[inline(always)]
    pub const fn scu388(&self) -> &Scu388 {
        &self.scu388
    }
    #[doc = "0x390 - MAC0/1 Interface Clock Delay Setting"]
    #[inline(always)]
    pub const fn scu390(&self) -> &Scu390 {
        &self.scu390
    }
    #[doc = "0x394 - MAC0/1 Interface Clock Delay 100M Setting"]
    #[inline(always)]
    pub const fn scu394(&self) -> &Scu394 {
        &self.scu394
    }
    #[doc = "0x398 - MAC0/1 Interface Clock Delay 10M Setting"]
    #[inline(always)]
    pub const fn scu398(&self) -> &Scu398 {
        &self.scu398
    }
    #[doc = "0x3a0 - Frequency Counter Control Register"]
    #[inline(always)]
    pub const fn scu3a0(&self) -> &Scu3a0 {
        &self.scu3a0
    }
    #[doc = "0x3a4 - Frequency Counter Comparison"]
    #[inline(always)]
    pub const fn scu3a4(&self) -> &Scu3a4 {
        &self.scu3a4
    }
    #[doc = "0x3b0 - USB Controler Register"]
    #[inline(always)]
    pub const fn scu3b0(&self) -> &Scu3b0 {
        &self.scu3b0
    }
    #[doc = "0x3b4 - USB Controler Lock Register"]
    #[inline(always)]
    pub const fn scu3b4(&self) -> &Scu3b4 {
        &self.scu3b4
    }
    #[doc = "0x3b8 - USB Controler Secure Register 1"]
    #[inline(always)]
    pub const fn scu3b8(&self) -> &Scu3b8 {
        &self.scu3b8
    }
    #[doc = "0x3bc - USB Controler Secure Register 2"]
    #[inline(always)]
    pub const fn scu3bc(&self) -> &Scu3bc {
        &self.scu3bc
    }
    #[doc = "0x3c0 - USB Controler Secure Register 3"]
    #[inline(always)]
    pub const fn scu3c0(&self) -> &Scu3c0 {
        &self.scu3c0
    }
    #[doc = "0x400 - Multi-Function Pin Control \\#1"]
    #[inline(always)]
    pub const fn scu400(&self) -> &Scu400 {
        &self.scu400
    }
    #[doc = "0x404 - Multi-Function Pin Control \\#2"]
    #[inline(always)]
    pub const fn scu404(&self) -> &Scu404 {
        &self.scu404
    }
    #[doc = "0x408 - Multi-Function Pin Control \\#3"]
    #[inline(always)]
    pub const fn scu408(&self) -> &Scu408 {
        &self.scu408
    }
    #[doc = "0x40c - Multi-Function Pin Control \\#4"]
    #[inline(always)]
    pub const fn scu40c(&self) -> &Scu40c {
        &self.scu40c
    }
    #[doc = "0x410 - Multi-Function Pin Control \\#5"]
    #[inline(always)]
    pub const fn scu410(&self) -> &Scu410 {
        &self.scu410
    }
    #[doc = "0x414 - Multi-Function Pin Control \\#6"]
    #[inline(always)]
    pub const fn scu414(&self) -> &Scu414 {
        &self.scu414
    }
    #[doc = "0x418 - Multi-Function Pin Control \\#7"]
    #[inline(always)]
    pub const fn scu418(&self) -> &Scu418 {
        &self.scu418
    }
    #[doc = "0x41c - Multi-Function Pin Control \\#8"]
    #[inline(always)]
    pub const fn scu41c(&self) -> &Scu41c {
        &self.scu41c
    }
    #[doc = "0x420 - Multi-Function Pin Control \\#9"]
    #[inline(always)]
    pub const fn scu420(&self) -> &Scu420 {
        &self.scu420
    }
    #[doc = "0x424 - Multi-Function Pin Control \\#10"]
    #[inline(always)]
    pub const fn scu424(&self) -> &Scu424 {
        &self.scu424
    }
    #[doc = "0x428 - Multi-Function Pin Control \\#11"]
    #[inline(always)]
    pub const fn scu428(&self) -> &Scu428 {
        &self.scu428
    }
    #[doc = "0x42c - Multi-Function Pin Control \\#12"]
    #[inline(always)]
    pub const fn scu42c(&self) -> &Scu42c {
        &self.scu42c
    }
    #[doc = "0x430 - Multi-Function Pin Control \\#13"]
    #[inline(always)]
    pub const fn scu430(&self) -> &Scu430 {
        &self.scu430
    }
    #[doc = "0x434 - Multi-Function Pin Control \\#14"]
    #[inline(always)]
    pub const fn scu434(&self) -> &Scu434 {
        &self.scu434
    }
    #[doc = "0x438 - Multi-Function Pin Control \\#15"]
    #[inline(always)]
    pub const fn scu438(&self) -> &Scu438 {
        &self.scu438
    }
    #[doc = "0x43c - Multi-Function Pin Control \\#16"]
    #[inline(always)]
    pub const fn scu43c(&self) -> &Scu43c {
        &self.scu43c
    }
    #[doc = "0x440 - Multi-Function Pin Control \\#17"]
    #[inline(always)]
    pub const fn scu440(&self) -> &Scu440 {
        &self.scu440
    }
    #[doc = "0x444 - Multi-Function Pin Control \\#18"]
    #[inline(always)]
    pub const fn scu444(&self) -> &Scu444 {
        &self.scu444
    }
    #[doc = "0x448 - Multi-Function Pin Control \\#19"]
    #[inline(always)]
    pub const fn scu448(&self) -> &Scu448 {
        &self.scu448
    }
    #[doc = "0x44c - Multi-Function Pin Control \\#20"]
    #[inline(always)]
    pub const fn scu44c(&self) -> &Scu44c {
        &self.scu44c
    }
    #[doc = "0x450 - Multi-Function Pin Control \\#21"]
    #[inline(always)]
    pub const fn scu450(&self) -> &Scu450 {
        &self.scu450
    }
    #[doc = "0x454 - Multi-Function Pin Control \\#22"]
    #[inline(always)]
    pub const fn scu454(&self) -> &Scu454 {
        &self.scu454
    }
    #[doc = "0x458 - Multi-Function Pin Control \\#23"]
    #[inline(always)]
    pub const fn scu458(&self) -> &Scu458 {
        &self.scu458
    }
    #[doc = "0x45c - Multi-Function Pin Control \\#24"]
    #[inline(always)]
    pub const fn scu45c(&self) -> &Scu45c {
        &self.scu45c
    }
    #[doc = "0x460 - Multi-Function Pin Control \\#25"]
    #[inline(always)]
    pub const fn scu460(&self) -> &Scu460 {
        &self.scu460
    }
    #[doc = "0x480 - IO Control \\#1"]
    #[inline(always)]
    pub const fn scu480(&self) -> &Scu480 {
        &self.scu480
    }
    #[doc = "0x484 - IO Control \\#2"]
    #[inline(always)]
    pub const fn scu484(&self) -> &Scu484 {
        &self.scu484
    }
    #[doc = "0x488 - IO Control \\#3"]
    #[inline(always)]
    pub const fn scu488(&self) -> &Scu488 {
        &self.scu488
    }
    #[doc = "0x48c - IO Control \\#4"]
    #[inline(always)]
    pub const fn scu48c(&self) -> &Scu48c {
        &self.scu48c
    }
    #[doc = "0x490 - IO Control \\#5"]
    #[inline(always)]
    pub const fn scu490(&self) -> &Scu490 {
        &self.scu490
    }
    #[doc = "0x494 - IO Control \\#6"]
    #[inline(always)]
    pub const fn scu494(&self) -> &Scu494 {
        &self.scu494
    }
    #[doc = "0x498 - IO Control \\#7"]
    #[inline(always)]
    pub const fn scu498(&self) -> &Scu498 {
        &self.scu498
    }
    #[doc = "0x49c - IO Control \\#8"]
    #[inline(always)]
    pub const fn scu49c(&self) -> &Scu49c {
        &self.scu49c
    }
    #[doc = "0x4a0 - IO Control \\#9"]
    #[inline(always)]
    pub const fn scu4a0(&self) -> &Scu4a0 {
        &self.scu4a0
    }
    #[doc = "0x4a4 - IO Control \\#10"]
    #[inline(always)]
    pub const fn scu4a4(&self) -> &Scu4a4 {
        &self.scu4a4
    }
    #[doc = "0x4a8 - IO Control \\#11"]
    #[inline(always)]
    pub const fn scu4a8(&self) -> &Scu4a8 {
        &self.scu4a8
    }
    #[doc = "0x4ac - IO Control \\#12"]
    #[inline(always)]
    pub const fn scu4ac(&self) -> &Scu4ac {
        &self.scu4ac
    }
    #[doc = "0x4b0 - IO Control \\#13"]
    #[inline(always)]
    pub const fn scu4b0(&self) -> &Scu4b0 {
        &self.scu4b0
    }
    #[doc = "0x4b4 - IO Control \\#14"]
    #[inline(always)]
    pub const fn scu4b4(&self) -> &Scu4b4 {
        &self.scu4b4
    }
    #[doc = "0x4b8 - IO Control \\#15"]
    #[inline(always)]
    pub const fn scu4b8(&self) -> &Scu4b8 {
        &self.scu4b8
    }
    #[doc = "0x4bc - IO Control \\#16"]
    #[inline(always)]
    pub const fn scu4bc(&self) -> &Scu4bc {
        &self.scu4bc
    }
    #[doc = "0x4c0 - IO Control \\#17"]
    #[inline(always)]
    pub const fn scu4c0(&self) -> &Scu4c0 {
        &self.scu4c0
    }
    #[doc = "0x4c4 - IO Control \\#18"]
    #[inline(always)]
    pub const fn scu4c4(&self) -> &Scu4c4 {
        &self.scu4c4
    }
    #[doc = "0x4c8 - IO Control \\#19"]
    #[inline(always)]
    pub const fn scu4c8(&self) -> &Scu4c8 {
        &self.scu4c8
    }
    #[doc = "0x4cc - IO Control \\#20"]
    #[inline(always)]
    pub const fn scu4cc(&self) -> &Scu4cc {
        &self.scu4cc
    }
    #[doc = "0x4d0 - IO Control \\#21"]
    #[inline(always)]
    pub const fn scu4d0(&self) -> &Scu4d0 {
        &self.scu4d0
    }
    #[doc = "0x4d4 - IO Control \\#22"]
    #[inline(always)]
    pub const fn scu4d4(&self) -> &Scu4d4 {
        &self.scu4d4
    }
    #[doc = "0x4d8 - IO Control \\#23"]
    #[inline(always)]
    pub const fn scu4d8(&self) -> &Scu4d8 {
        &self.scu4d8
    }
    #[doc = "0x4dc - IO Control \\#24"]
    #[inline(always)]
    pub const fn scu4dc(&self) -> &Scu4dc {
        &self.scu4dc
    }
    #[doc = "0x4e0 - IO Control \\#25"]
    #[inline(always)]
    pub const fn scu4e0(&self) -> &Scu4e0 {
        &self.scu4e0
    }
    #[doc = "0x4e4 - IO Control \\#26"]
    #[inline(always)]
    pub const fn scu4e4(&self) -> &Scu4e4 {
        &self.scu4e4
    }
    #[doc = "0x4e8 - IO Control \\#27"]
    #[inline(always)]
    pub const fn scu4e8(&self) -> &Scu4e8 {
        &self.scu4e8
    }
    #[doc = "0x4ec - IO Control \\#28"]
    #[inline(always)]
    pub const fn scu4ec(&self) -> &Scu4ec {
        &self.scu4ec
    }
    #[doc = "0x4f0 - IO Control \\#29"]
    #[inline(always)]
    pub const fn scu4f0(&self) -> &Scu4f0 {
        &self.scu4f0
    }
    #[doc = "0x4f4 - IO Control \\#30"]
    #[inline(always)]
    pub const fn scu4f4(&self) -> &Scu4f4 {
        &self.scu4f4
    }
    #[doc = "0x4f8 - IO Control \\#31"]
    #[inline(always)]
    pub const fn scu4f8(&self) -> &Scu4f8 {
        &self.scu4f8
    }
    #[doc = "0x4fc - IO Control \\#32"]
    #[inline(always)]
    pub const fn scu4fc(&self) -> &Scu4fc {
        &self.scu4fc
    }
    #[doc = "0x500 - IO Control \\#33"]
    #[inline(always)]
    pub const fn scu500(&self) -> &Scu500 {
        &self.scu500
    }
    #[doc = "0x504 - IO Control \\#34"]
    #[inline(always)]
    pub const fn scu504(&self) -> &Scu504 {
        &self.scu504
    }
    #[doc = "0x508 - IO Control \\#35"]
    #[inline(always)]
    pub const fn scu508(&self) -> &Scu508 {
        &self.scu508
    }
    #[doc = "0x50c - IO Control \\#36"]
    #[inline(always)]
    pub const fn scu50c(&self) -> &Scu50c {
        &self.scu50c
    }
    #[doc = "0x510 - IO Control \\#37"]
    #[inline(always)]
    pub const fn scu510(&self) -> &Scu510 {
        &self.scu510
    }
    #[doc = "0x514 - IO Control \\#38"]
    #[inline(always)]
    pub const fn scu514(&self) -> &Scu514 {
        &self.scu514
    }
    #[doc = "0x518 - IO Control \\#39"]
    #[inline(always)]
    pub const fn scu518(&self) -> &Scu518 {
        &self.scu518
    }
    #[doc = "0x51c - IO Control \\#40"]
    #[inline(always)]
    pub const fn scu51c(&self) -> &Scu51c {
        &self.scu51c
    }
    #[doc = "0x520 - IO Control \\#41"]
    #[inline(always)]
    pub const fn scu520(&self) -> &Scu520 {
        &self.scu520
    }
    #[doc = "0x524 - IO Control \\#42"]
    #[inline(always)]
    pub const fn scu524(&self) -> &Scu524 {
        &self.scu524
    }
    #[doc = "0x528 - IO Control \\#43"]
    #[inline(always)]
    pub const fn scu528(&self) -> &Scu528 {
        &self.scu528
    }
    #[doc = "0x52c - IO Control \\#44"]
    #[inline(always)]
    pub const fn scu52c(&self) -> &Scu52c {
        &self.scu52c
    }
    #[doc = "0x530 - IO Control \\#45"]
    #[inline(always)]
    pub const fn scu530(&self) -> &Scu530 {
        &self.scu530
    }
    #[doc = "0x534 - IO Control \\#46"]
    #[inline(always)]
    pub const fn scu534(&self) -> &Scu534 {
        &self.scu534
    }
    #[doc = "0x538 - IO Control \\#47"]
    #[inline(always)]
    pub const fn scu538(&self) -> &Scu538 {
        &self.scu538
    }
    #[doc = "0x53c - IO Control \\#48"]
    #[inline(always)]
    pub const fn scu53c(&self) -> &Scu53c {
        &self.scu53c
    }
    #[doc = "0x540 - IO Control \\#49"]
    #[inline(always)]
    pub const fn scu540(&self) -> &Scu540 {
        &self.scu540
    }
    #[doc = "0x544 - IO Control \\#50"]
    #[inline(always)]
    pub const fn scu544(&self) -> &Scu544 {
        &self.scu544
    }
    #[doc = "0x548 - IO Control \\#51"]
    #[inline(always)]
    pub const fn scu548(&self) -> &Scu548 {
        &self.scu548
    }
    #[doc = "0x54c - IO Control \\#52"]
    #[inline(always)]
    pub const fn scu54c(&self) -> &Scu54c {
        &self.scu54c
    }
    #[doc = "0x550 - IO Control \\#53"]
    #[inline(always)]
    pub const fn scu550(&self) -> &Scu550 {
        &self.scu550
    }
    #[doc = "0x554 - IO Control \\#54"]
    #[inline(always)]
    pub const fn scu554(&self) -> &Scu554 {
        &self.scu554
    }
    #[doc = "0x558 - IO Control \\#55"]
    #[inline(always)]
    pub const fn scu558(&self) -> &Scu558 {
        &self.scu558
    }
    #[doc = "0x55c - IO Control \\#56"]
    #[inline(always)]
    pub const fn scu55c(&self) -> &Scu55c {
        &self.scu55c
    }
    #[doc = "0x560 - IO Control \\#57"]
    #[inline(always)]
    pub const fn scu560(&self) -> &Scu560 {
        &self.scu560
    }
    #[doc = "0x564 - IO Control \\#58"]
    #[inline(always)]
    pub const fn scu564(&self) -> &Scu564 {
        &self.scu564
    }
    #[doc = "0x568 - IO Control \\#59"]
    #[inline(always)]
    pub const fn scu568(&self) -> &Scu568 {
        &self.scu568
    }
    #[doc = "0x56c - IO Control \\#60"]
    #[inline(always)]
    pub const fn scu56c(&self) -> &Scu56c {
        &self.scu56c
    }
    #[doc = "0x570 - IO Control \\#61"]
    #[inline(always)]
    pub const fn scu570(&self) -> &Scu570 {
        &self.scu570
    }
    #[doc = "0x574 - IO Control \\#62"]
    #[inline(always)]
    pub const fn scu574(&self) -> &Scu574 {
        &self.scu574
    }
    #[doc = "0x578 - IO Control \\#63"]
    #[inline(always)]
    pub const fn scu578(&self) -> &Scu578 {
        &self.scu578
    }
    #[doc = "0x57c - IO Control \\#64"]
    #[inline(always)]
    pub const fn scu57c(&self) -> &Scu57c {
        &self.scu57c
    }
    #[doc = "0x580 - IO Control \\#65"]
    #[inline(always)]
    pub const fn scu580(&self) -> &Scu580 {
        &self.scu580
    }
    #[doc = "0x584 - IO Control \\#66"]
    #[inline(always)]
    pub const fn scu584(&self) -> &Scu584 {
        &self.scu584
    }
    #[doc = "0x588 - IO Control \\#67"]
    #[inline(always)]
    pub const fn scu588(&self) -> &Scu588 {
        &self.scu588
    }
    #[doc = "0x58c - IO Control \\#68"]
    #[inline(always)]
    pub const fn scu58c(&self) -> &Scu58c {
        &self.scu58c
    }
    #[doc = "0x590 - IO Control \\#69"]
    #[inline(always)]
    pub const fn scu590(&self) -> &Scu590 {
        &self.scu590
    }
    #[doc = "0x594 - IO Control \\#70"]
    #[inline(always)]
    pub const fn scu594(&self) -> &Scu594 {
        &self.scu594
    }
    #[doc = "0x598 - IO Control \\#71"]
    #[inline(always)]
    pub const fn scu598(&self) -> &Scu598 {
        &self.scu598
    }
    #[doc = "0x59c - IO Control \\#72"]
    #[inline(always)]
    pub const fn scu59c(&self) -> &Scu59c {
        &self.scu59c
    }
    #[doc = "0x5a0 - IO Control \\#73"]
    #[inline(always)]
    pub const fn scu5a0(&self) -> &Scu5a0 {
        &self.scu5a0
    }
    #[doc = "0x5a4 - IO Control \\#74"]
    #[inline(always)]
    pub const fn scu5a4(&self) -> &Scu5a4 {
        &self.scu5a4
    }
    #[doc = "0x5a8 - IO Control \\#75"]
    #[inline(always)]
    pub const fn scu5a8(&self) -> &Scu5a8 {
        &self.scu5a8
    }
    #[doc = "0x5ac - IO Control \\#76"]
    #[inline(always)]
    pub const fn scu5ac(&self) -> &Scu5ac {
        &self.scu5ac
    }
    #[doc = "0x5b0 - IO Control \\#77"]
    #[inline(always)]
    pub const fn scu5b0(&self) -> &Scu5b0 {
        &self.scu5b0
    }
    #[doc = "0x5b4 - IO Control \\#78"]
    #[inline(always)]
    pub const fn scu5b4(&self) -> &Scu5b4 {
        &self.scu5b4
    }
    #[doc = "0x5b8 - IO Control \\#79"]
    #[inline(always)]
    pub const fn scu5b8(&self) -> &Scu5b8 {
        &self.scu5b8
    }
    #[doc = "0x5bc - IO Control \\#80"]
    #[inline(always)]
    pub const fn scu5bc(&self) -> &Scu5bc {
        &self.scu5bc
    }
    #[doc = "0x5c0 - IO Control \\#81"]
    #[inline(always)]
    pub const fn scu5c0(&self) -> &Scu5c0 {
        &self.scu5c0
    }
    #[doc = "0x5c4 - IO Control \\#82"]
    #[inline(always)]
    pub const fn scu5c4(&self) -> &Scu5c4 {
        &self.scu5c4
    }
    #[doc = "0x5c8 - IO Control \\#83"]
    #[inline(always)]
    pub const fn scu5c8(&self) -> &Scu5c8 {
        &self.scu5c8
    }
    #[doc = "0x5cc - IO Control \\#84"]
    #[inline(always)]
    pub const fn scu5cc(&self) -> &Scu5cc {
        &self.scu5cc
    }
    #[doc = "0x5d0 - IO Control \\#85"]
    #[inline(always)]
    pub const fn scu5d0(&self) -> &Scu5d0 {
        &self.scu5d0
    }
    #[doc = "0x5d4 - IO Control \\#86"]
    #[inline(always)]
    pub const fn scu5d4(&self) -> &Scu5d4 {
        &self.scu5d4
    }
    #[doc = "0x5d8 - IO Control \\#87"]
    #[inline(always)]
    pub const fn scu5d8(&self) -> &Scu5d8 {
        &self.scu5d8
    }
    #[doc = "0x5dc - IO Control \\#88"]
    #[inline(always)]
    pub const fn scu5dc(&self) -> &Scu5dc {
        &self.scu5dc
    }
    #[doc = "0x5e0 - IO Control \\#89"]
    #[inline(always)]
    pub const fn scu5e0(&self) -> &Scu5e0 {
        &self.scu5e0
    }
    #[doc = "0x5e4 - IO Control \\#90"]
    #[inline(always)]
    pub const fn scu5e4(&self) -> &Scu5e4 {
        &self.scu5e4
    }
    #[doc = "0x5e8 - IO Control \\#91"]
    #[inline(always)]
    pub const fn scu5e8(&self) -> &Scu5e8 {
        &self.scu5e8
    }
    #[doc = "0x5ec - IO Control \\#92"]
    #[inline(always)]
    pub const fn scu5ec(&self) -> &Scu5ec {
        &self.scu5ec
    }
    #[doc = "0x5f0 - IO Control \\#93"]
    #[inline(always)]
    pub const fn scu5f0(&self) -> &Scu5f0 {
        &self.scu5f0
    }
    #[doc = "0x5f4 - IO Control \\#94"]
    #[inline(always)]
    pub const fn scu5f4(&self) -> &Scu5f4 {
        &self.scu5f4
    }
    #[doc = "0x5f8 - IO Control \\#95"]
    #[inline(always)]
    pub const fn scu5f8(&self) -> &Scu5f8 {
        &self.scu5f8
    }
    #[doc = "0x5fc - IO Control \\#96"]
    #[inline(always)]
    pub const fn scu5fc(&self) -> &Scu5fc {
        &self.scu5fc
    }
    #[doc = "0x600 - IO Control \\#97"]
    #[inline(always)]
    pub const fn scu600(&self) -> &Scu600 {
        &self.scu600
    }
    #[doc = "0x680 - IO Control 0 Regiser"]
    #[inline(always)]
    pub const fn scu680(&self) -> &Scu680 {
        &self.scu680
    }
    #[doc = "0x684 - HRAM IO Control 1 Register"]
    #[inline(always)]
    pub const fn scu684(&self) -> &Scu684 {
        &self.scu684
    }
    #[doc = "0x688 - HRAM IO Control 2 Register"]
    #[inline(always)]
    pub const fn scu688(&self) -> &Scu688 {
        &self.scu688
    }
    #[doc = "0x68c - HRAM IO Control 3 Register"]
    #[inline(always)]
    pub const fn scu68c(&self) -> &Scu68c {
        &self.scu68c
    }
    #[doc = "0x6fc - GPIO Passthrough Debounce Register"]
    #[inline(always)]
    pub const fn scu6fc(&self) -> &Scu6fc {
        &self.scu6fc
    }
    #[doc = "0x700 - IO Lock 1 Register 1"]
    #[inline(always)]
    pub const fn scu700(&self) -> &Scu700 {
        &self.scu700
    }
    #[doc = "0x704 - IO Lock Register 2"]
    #[inline(always)]
    pub const fn scu704(&self) -> &Scu704 {
        &self.scu704
    }
    #[doc = "0x708 - IO Lock Register 3"]
    #[inline(always)]
    pub const fn scu708(&self) -> &Scu708 {
        &self.scu708
    }
    #[doc = "0x70c - IO Lock Register 4"]
    #[inline(always)]
    pub const fn scu70c(&self) -> &Scu70c {
        &self.scu70c
    }
    #[doc = "0x710 - IO Lock Register 5"]
    #[inline(always)]
    pub const fn scu710(&self) -> &Scu710 {
        &self.scu710
    }
    #[doc = "0x714 - IO Lock Register 6"]
    #[inline(always)]
    pub const fn scu714(&self) -> &Scu714 {
        &self.scu714
    }
    #[doc = "0x718 - IO Lock Register 7"]
    #[inline(always)]
    pub const fn scu718(&self) -> &Scu718 {
        &self.scu718
    }
    #[doc = "0x740 - IO Secure 1 Register 1"]
    #[inline(always)]
    pub const fn scu740(&self) -> &Scu740 {
        &self.scu740
    }
    #[doc = "0x744 - IO Secure 1 Register 2"]
    #[inline(always)]
    pub const fn scu744(&self) -> &Scu744 {
        &self.scu744
    }
    #[doc = "0x748 - IO Secure 1 Register 3"]
    #[inline(always)]
    pub const fn scu748(&self) -> &Scu748 {
        &self.scu748
    }
    #[doc = "0x74c - IO Secure 1 Register 4"]
    #[inline(always)]
    pub const fn scu74c(&self) -> &Scu74c {
        &self.scu74c
    }
    #[doc = "0x750 - IO Secure 1 Register 5"]
    #[inline(always)]
    pub const fn scu750(&self) -> &Scu750 {
        &self.scu750
    }
    #[doc = "0x754 - IO Secure 1 Register 6"]
    #[inline(always)]
    pub const fn scu754(&self) -> &Scu754 {
        &self.scu754
    }
    #[doc = "0x758 - IO Secure 1 Register 7"]
    #[inline(always)]
    pub const fn scu758(&self) -> &Scu758 {
        &self.scu758
    }
    #[doc = "0x780 - IO SECURE 2 Register 1"]
    #[inline(always)]
    pub const fn scu780(&self) -> &Scu780 {
        &self.scu780
    }
    #[doc = "0x784 - IO SECURE 2 Register 2"]
    #[inline(always)]
    pub const fn scu784(&self) -> &Scu784 {
        &self.scu784
    }
    #[doc = "0x788 - IO SECURE 2 Register 3"]
    #[inline(always)]
    pub const fn scu788(&self) -> &Scu788 {
        &self.scu788
    }
    #[doc = "0x78c - IO SECURE 2 Register 4"]
    #[inline(always)]
    pub const fn scu78c(&self) -> &Scu78c {
        &self.scu78c
    }
    #[doc = "0x790 - IO SECURE 2 Register 5"]
    #[inline(always)]
    pub const fn scu790(&self) -> &Scu790 {
        &self.scu790
    }
    #[doc = "0x794 - IO SECURE 2 Register 6"]
    #[inline(always)]
    pub const fn scu794(&self) -> &Scu794 {
        &self.scu794
    }
    #[doc = "0x798 - IO SECURE 2 Register 7"]
    #[inline(always)]
    pub const fn scu798(&self) -> &Scu798 {
        &self.scu798
    }
    #[doc = "0x7c0 - IO Secure 3 Register 1"]
    #[inline(always)]
    pub const fn scu7c0(&self) -> &Scu7c0 {
        &self.scu7c0
    }
    #[doc = "0x7c4 - IO Secure 3 Register 2"]
    #[inline(always)]
    pub const fn scu7c4(&self) -> &Scu7c4 {
        &self.scu7c4
    }
    #[doc = "0x7c8 - IO Secure 3 Register 3"]
    #[inline(always)]
    pub const fn scu7c8(&self) -> &Scu7c8 {
        &self.scu7c8
    }
    #[doc = "0x7cc - IO Secure 3 Register 4"]
    #[inline(always)]
    pub const fn scu7cc(&self) -> &Scu7cc {
        &self.scu7cc
    }
    #[doc = "0x7d0 - IO Secure 3 Register 5"]
    #[inline(always)]
    pub const fn scu7d0(&self) -> &Scu7d0 {
        &self.scu7d0
    }
    #[doc = "0x7d4 - IO Secure 3 Register 6"]
    #[inline(always)]
    pub const fn scu7d4(&self) -> &Scu7d4 {
        &self.scu7d4
    }
    #[doc = "0x7d8 - IO Secure 3 Register 7"]
    #[inline(always)]
    pub const fn scu7d8(&self) -> &Scu7d8 {
        &self.scu7d8
    }
    #[doc = "0x800 - Scratch register for MCU 0"]
    #[inline(always)]
    pub const fn scu800(&self) -> &Scu800 {
        &self.scu800
    }
    #[doc = "0x804 - Scratch register for MCU 1"]
    #[inline(always)]
    pub const fn scu804(&self) -> &Scu804 {
        &self.scu804
    }
    #[doc = "0x808 - Scratch register for MCU 2"]
    #[inline(always)]
    pub const fn scu808(&self) -> &Scu808 {
        &self.scu808
    }
    #[doc = "0x80c - Scratch register for MCU 3"]
    #[inline(always)]
    pub const fn scu80c(&self) -> &Scu80c {
        &self.scu80c
    }
    #[doc = "0x810 - Scratch register for MCU 4"]
    #[inline(always)]
    pub const fn scu810(&self) -> &Scu810 {
        &self.scu810
    }
    #[doc = "0x814 - Scratch register for MCU 5"]
    #[inline(always)]
    pub const fn scu814(&self) -> &Scu814 {
        &self.scu814
    }
    #[doc = "0x818 - Scratch register for MCU 6"]
    #[inline(always)]
    pub const fn scu818(&self) -> &Scu818 {
        &self.scu818
    }
    #[doc = "0x81c - Scratch register for MCU 7"]
    #[inline(always)]
    pub const fn scu81c(&self) -> &Scu81c {
        &self.scu81c
    }
    #[doc = "0x820 - Scratch register for MCU 8"]
    #[inline(always)]
    pub const fn scu820(&self) -> &Scu820 {
        &self.scu820
    }
    #[doc = "0x824 - Scratch register for MCU 9"]
    #[inline(always)]
    pub const fn scu824(&self) -> &Scu824 {
        &self.scu824
    }
    #[doc = "0x828 - Scratch register for MCU 10"]
    #[inline(always)]
    pub const fn scu828(&self) -> &Scu828 {
        &self.scu828
    }
    #[doc = "0x82c - Scratch register for MCU 11"]
    #[inline(always)]
    pub const fn scu82c(&self) -> &Scu82c {
        &self.scu82c
    }
    #[doc = "0x830 - Scratch register for MCU 12"]
    #[inline(always)]
    pub const fn scu830(&self) -> &Scu830 {
        &self.scu830
    }
    #[doc = "0x834 - Scratch register for MCU 13"]
    #[inline(always)]
    pub const fn scu834(&self) -> &Scu834 {
        &self.scu834
    }
    #[doc = "0x838 - Scratch register for MCU 14"]
    #[inline(always)]
    pub const fn scu838(&self) -> &Scu838 {
        &self.scu838
    }
    #[doc = "0x83c - Scratch register for MCU 15"]
    #[inline(always)]
    pub const fn scu83c(&self) -> &Scu83c {
        &self.scu83c
    }
    #[doc = "0x840 - Scratch register for MCU 16"]
    #[inline(always)]
    pub const fn scu840(&self) -> &Scu840 {
        &self.scu840
    }
    #[doc = "0x844 - Scratch register for MCU 17"]
    #[inline(always)]
    pub const fn scu844(&self) -> &Scu844 {
        &self.scu844
    }
    #[doc = "0x848 - Scratch register for MCU 18"]
    #[inline(always)]
    pub const fn scu848(&self) -> &Scu848 {
        &self.scu848
    }
    #[doc = "0x84c - Scratch register for MCU 19"]
    #[inline(always)]
    pub const fn scu84c(&self) -> &Scu84c {
        &self.scu84c
    }
    #[doc = "0x850 - Scratch register for MCU 20"]
    #[inline(always)]
    pub const fn scu850(&self) -> &Scu850 {
        &self.scu850
    }
    #[doc = "0x854 - Scratch register for MCU 21"]
    #[inline(always)]
    pub const fn scu854(&self) -> &Scu854 {
        &self.scu854
    }
    #[doc = "0x858 - Scratch register for MCU 22"]
    #[inline(always)]
    pub const fn scu858(&self) -> &Scu858 {
        &self.scu858
    }
    #[doc = "0x85c - Scratch register for MCU 23"]
    #[inline(always)]
    pub const fn scu85c(&self) -> &Scu85c {
        &self.scu85c
    }
    #[doc = "0x860 - Scratch register for MCU 24"]
    #[inline(always)]
    pub const fn scu860(&self) -> &Scu860 {
        &self.scu860
    }
    #[doc = "0x864 - Scratch register for MCU 25"]
    #[inline(always)]
    pub const fn scu864(&self) -> &Scu864 {
        &self.scu864
    }
    #[doc = "0x868 - Scratch register for MCU 26"]
    #[inline(always)]
    pub const fn scu868(&self) -> &Scu868 {
        &self.scu868
    }
    #[doc = "0x86c - Scratch register for MCU 27"]
    #[inline(always)]
    pub const fn scu86c(&self) -> &Scu86c {
        &self.scu86c
    }
    #[doc = "0x870 - Scratch register for MCU 28"]
    #[inline(always)]
    pub const fn scu870(&self) -> &Scu870 {
        &self.scu870
    }
    #[doc = "0x874 - Scratch register for MCU 29"]
    #[inline(always)]
    pub const fn scu874(&self) -> &Scu874 {
        &self.scu874
    }
    #[doc = "0x878 - Scratch register for MCU 30"]
    #[inline(always)]
    pub const fn scu878(&self) -> &Scu878 {
        &self.scu878
    }
    #[doc = "0x87c - Scratch register for MCU 31"]
    #[inline(always)]
    pub const fn scu87c(&self) -> &Scu87c {
        &self.scu87c
    }
    #[doc = "0x880 - Scratch register for MCU 32"]
    #[inline(always)]
    pub const fn scu880(&self) -> &Scu880 {
        &self.scu880
    }
    #[doc = "0x884 - Scratch register for MCU 33"]
    #[inline(always)]
    pub const fn scu884(&self) -> &Scu884 {
        &self.scu884
    }
    #[doc = "0x888 - Scratch register for MCU 34"]
    #[inline(always)]
    pub const fn scu888(&self) -> &Scu888 {
        &self.scu888
    }
    #[doc = "0x88c - Scratch register for MCU 35"]
    #[inline(always)]
    pub const fn scu88c(&self) -> &Scu88c {
        &self.scu88c
    }
    #[doc = "0x890 - Scratch register for MCU 36"]
    #[inline(always)]
    pub const fn scu890(&self) -> &Scu890 {
        &self.scu890
    }
    #[doc = "0x894 - Scratch register for MCU 37"]
    #[inline(always)]
    pub const fn scu894(&self) -> &Scu894 {
        &self.scu894
    }
    #[doc = "0x898 - Scratch register for MCU 38"]
    #[inline(always)]
    pub const fn scu898(&self) -> &Scu898 {
        &self.scu898
    }
    #[doc = "0x89c - Scratch register for MCU 39"]
    #[inline(always)]
    pub const fn scu89c(&self) -> &Scu89c {
        &self.scu89c
    }
    #[doc = "0x8a0 - Scratch register for MCU 40"]
    #[inline(always)]
    pub const fn scu8a0(&self) -> &Scu8a0 {
        &self.scu8a0
    }
    #[doc = "0x8a4 - Scratch register for MCU 41"]
    #[inline(always)]
    pub const fn scu8a4(&self) -> &Scu8a4 {
        &self.scu8a4
    }
    #[doc = "0x8a8 - Scratch register for MCU 42"]
    #[inline(always)]
    pub const fn scu8a8(&self) -> &Scu8a8 {
        &self.scu8a8
    }
    #[doc = "0x8ac - Scratch register for MCU 43"]
    #[inline(always)]
    pub const fn scu8ac(&self) -> &Scu8ac {
        &self.scu8ac
    }
    #[doc = "0x8b0 - Scratch register for MCU 44"]
    #[inline(always)]
    pub const fn scu8b0(&self) -> &Scu8b0 {
        &self.scu8b0
    }
    #[doc = "0x8b4 - Scratch register for MCU 45"]
    #[inline(always)]
    pub const fn scu8b4(&self) -> &Scu8b4 {
        &self.scu8b4
    }
    #[doc = "0x8b8 - Scratch register for MCU 46"]
    #[inline(always)]
    pub const fn scu8b8(&self) -> &Scu8b8 {
        &self.scu8b8
    }
    #[doc = "0x8bc - Scratch register for MCU 47"]
    #[inline(always)]
    pub const fn scu8bc(&self) -> &Scu8bc {
        &self.scu8bc
    }
    #[doc = "0x8c0 - Scratch register for MCU 48"]
    #[inline(always)]
    pub const fn scu8c0(&self) -> &Scu8c0 {
        &self.scu8c0
    }
    #[doc = "0x8c4 - Scratch register for MCU 49"]
    #[inline(always)]
    pub const fn scu8c4(&self) -> &Scu8c4 {
        &self.scu8c4
    }
    #[doc = "0x8c8 - Scratch register for MCU 50"]
    #[inline(always)]
    pub const fn scu8c8(&self) -> &Scu8c8 {
        &self.scu8c8
    }
    #[doc = "0x8cc - Scratch register for MCU 51"]
    #[inline(always)]
    pub const fn scu8cc(&self) -> &Scu8cc {
        &self.scu8cc
    }
    #[doc = "0x8d0 - Scratch register for MCU 52"]
    #[inline(always)]
    pub const fn scu8d0(&self) -> &Scu8d0 {
        &self.scu8d0
    }
    #[doc = "0x8d4 - Scratch register for MCU 53"]
    #[inline(always)]
    pub const fn scu8d4(&self) -> &Scu8d4 {
        &self.scu8d4
    }
    #[doc = "0x8d8 - Scratch register for MCU 54"]
    #[inline(always)]
    pub const fn scu8d8(&self) -> &Scu8d8 {
        &self.scu8d8
    }
    #[doc = "0x8dc - Scratch register for MCU 55"]
    #[inline(always)]
    pub const fn scu8dc(&self) -> &Scu8dc {
        &self.scu8dc
    }
    #[doc = "0x8e0 - Scratch register for MCU 56"]
    #[inline(always)]
    pub const fn scu8e0(&self) -> &Scu8e0 {
        &self.scu8e0
    }
    #[doc = "0x8e4 - Scratch register for MCU 57"]
    #[inline(always)]
    pub const fn scu8e4(&self) -> &Scu8e4 {
        &self.scu8e4
    }
    #[doc = "0x8e8 - Scratch register for MCU 58"]
    #[inline(always)]
    pub const fn scu8e8(&self) -> &Scu8e8 {
        &self.scu8e8
    }
    #[doc = "0x8ec - Scratch register for MCU 59"]
    #[inline(always)]
    pub const fn scu8ec(&self) -> &Scu8ec {
        &self.scu8ec
    }
    #[doc = "0x8f0 - Scratch register for MCU 60"]
    #[inline(always)]
    pub const fn scu8f0(&self) -> &Scu8f0 {
        &self.scu8f0
    }
    #[doc = "0x8f4 - Scratch register for MCU 61"]
    #[inline(always)]
    pub const fn scu8f4(&self) -> &Scu8f4 {
        &self.scu8f4
    }
    #[doc = "0x8f8 - Scratch register for MCU 62"]
    #[inline(always)]
    pub const fn scu8f8(&self) -> &Scu8f8 {
        &self.scu8f8
    }
    #[doc = "0x8fc - Scratch register for MCU 63"]
    #[inline(always)]
    pub const fn scu8fc(&self) -> &Scu8fc {
        &self.scu8fc
    }
    #[doc = "0x900 - \\PSP\\ Service Processor Control Register 1"]
    #[inline(always)]
    pub const fn scu900(&self) -> &Scu900 {
        &self.scu900
    }
    #[doc = "0x904 - \\PSP\\ Service Processor Control Register 2"]
    #[inline(always)]
    pub const fn scu904(&self) -> &Scu904 {
        &self.scu904
    }
    #[doc = "0x908 - \\PSP\\ Service Processor Control Register 3"]
    #[inline(always)]
    pub const fn scu908(&self) -> &Scu908 {
        &self.scu908
    }
    #[doc = "0x90c - \\PSP\\ Service Processor Control Register 4"]
    #[inline(always)]
    pub const fn scu90c(&self) -> &Scu90c {
        &self.scu90c
    }
    #[doc = "0x910 - \\PSP\\ Service Processor Control Register 5"]
    #[inline(always)]
    pub const fn scu910(&self) -> &Scu910 {
        &self.scu910
    }
    #[doc = "0x914 - \\PSP\\ Service Processor Control Register 6"]
    #[inline(always)]
    pub const fn scu914(&self) -> &Scu914 {
        &self.scu914
    }
    #[doc = "0x918 - \\PSP\\ Service Processor Control Register 7"]
    #[inline(always)]
    pub const fn scu918(&self) -> &Scu918 {
        &self.scu918
    }
    #[doc = "0x920 - \\PSP\\ Service Processor REMAP Base Register 0"]
    #[inline(always)]
    pub const fn scu920(&self) -> &Scu920 {
        &self.scu920
    }
    #[doc = "0x924 - \\PSP\\ Service Processor REMAP Size Register 0"]
    #[inline(always)]
    pub const fn scu924(&self) -> &Scu924 {
        &self.scu924
    }
    #[doc = "0x928 - \\PSP\\ Service Processor REMAP Base Register 1"]
    #[inline(always)]
    pub const fn scu928(&self) -> &Scu928 {
        &self.scu928
    }
    #[doc = "0x92c - \\PSP\\ Service Processor REMAP Size Register 1"]
    #[inline(always)]
    pub const fn scu92c(&self) -> &Scu92c {
        &self.scu92c
    }
    #[doc = "0x930 - \\PSP\\ Service Processor REMAP Base Register 2"]
    #[inline(always)]
    pub const fn scu930(&self) -> &Scu930 {
        &self.scu930
    }
    #[doc = "0x934 - \\PSP\\ Service Processor REMAP Size Register 2"]
    #[inline(always)]
    pub const fn scu934(&self) -> &Scu934 {
        &self.scu934
    }
    #[doc = "0x940 - \\SSP\\ Service Processor Control Register"]
    #[inline(always)]
    pub const fn scu940(&self) -> &Scu940 {
        &self.scu940
    }
    #[doc = "0x948 - \\SSP\\ Service Processor Control Register 3"]
    #[inline(always)]
    pub const fn scu948(&self) -> &Scu948 {
        &self.scu948
    }
    #[doc = "0x94c - \\SSP\\ Service Processor Control Register 4"]
    #[inline(always)]
    pub const fn scu94c(&self) -> &Scu94c {
        &self.scu94c
    }
    #[doc = "0x950 - \\SSP\\ Service Processor Control Register 5"]
    #[inline(always)]
    pub const fn scu950(&self) -> &Scu950 {
        &self.scu950
    }
    #[doc = "0x954 - \\SSP\\ Service Processor Control Register 6"]
    #[inline(always)]
    pub const fn scu954(&self) -> &Scu954 {
        &self.scu954
    }
    #[doc = "0x958 - \\SSP\\ Service Processor Control Register 7"]
    #[inline(always)]
    pub const fn scu958(&self) -> &Scu958 {
        &self.scu958
    }
    #[doc = "0x974 - \\SSP\\ Service Processor REMAP Size Register 2"]
    #[inline(always)]
    pub const fn scu974(&self) -> &Scu974 {
        &self.scu974
    }
    #[doc = "0x980 - EFUSE Control Register"]
    #[inline(always)]
    pub const fn scu980(&self) -> &Scu980 {
        &self.scu980
    }
    #[doc = "0x984 - EFUSE Data Register"]
    #[inline(always)]
    pub const fn scu984(&self) -> &Scu984 {
        &self.scu984
    }
    #[doc = "0x988 - EFUSE Command Register"]
    #[inline(always)]
    pub const fn scu988(&self) -> &Scu988 {
        &self.scu988
    }
    #[doc = "0x98c - EFUSE Program Pattern Register"]
    #[inline(always)]
    pub const fn scu98c(&self) -> &Scu98c {
        &self.scu98c
    }
    #[doc = "0x990 - Chip Unique ID 0"]
    #[inline(always)]
    pub const fn scu990(&self) -> &Scu990 {
        &self.scu990
    }
    #[doc = "0x994 - Chip Unique ID 1"]
    #[inline(always)]
    pub const fn scu994(&self) -> &Scu994 {
        &self.scu994
    }
    #[doc = "0x998 - Reserved Read Only ID 0"]
    #[inline(always)]
    pub const fn scu998(&self) -> &Scu998 {
        &self.scu998
    }
    #[doc = "0x99c - Reserved Read Only ID 1"]
    #[inline(always)]
    pub const fn scu99c(&self) -> &Scu99c {
        &self.scu99c
    }
    #[doc = "0x9a0 - Reserved Read Only ID 2"]
    #[inline(always)]
    pub const fn scu9a0(&self) -> &Scu9a0 {
        &self.scu9a0
    }
    #[doc = "0x9a4 - Reserved Read Only ID 3"]
    #[inline(always)]
    pub const fn scu9a4(&self) -> &Scu9a4 {
        &self.scu9a4
    }
    #[doc = "0x9a8 - Reserved Read Only ID 4"]
    #[inline(always)]
    pub const fn scu9a8(&self) -> &Scu9a8 {
        &self.scu9a8
    }
    #[doc = "0x9ac - Reserved Read Only ID 5"]
    #[inline(always)]
    pub const fn scu9ac(&self) -> &Scu9ac {
        &self.scu9ac
    }
    #[doc = "0x9b0 - EFUSE PGM Timing Register 0"]
    #[inline(always)]
    pub const fn scu9b0(&self) -> &Scu9b0 {
        &self.scu9b0
    }
    #[doc = "0x9b4 - EFUSE PGM Timing Register 1"]
    #[inline(always)]
    pub const fn scu9b4(&self) -> &Scu9b4 {
        &self.scu9b4
    }
    #[doc = "0x9b8 - EFUSE Read Timing Register 0"]
    #[inline(always)]
    pub const fn scu9b8(&self) -> &Scu9b8 {
        &self.scu9b8
    }
    #[doc = "0x9bc - EFUSE Read Timing Register 1"]
    #[inline(always)]
    pub const fn scu9bc(&self) -> &Scu9bc {
        &self.scu9bc
    }
    #[doc = "0xb00 - SW PUF Register 0"]
    #[inline(always)]
    pub const fn scub00(&self) -> &Scub00 {
        &self.scub00
    }
    #[doc = "0xb04 - SW PUF Register 1"]
    #[inline(always)]
    pub const fn scub04(&self) -> &Scub04 {
        &self.scub04
    }
    #[doc = "0xb08 - SW PUF Register 2"]
    #[inline(always)]
    pub const fn scub08(&self) -> &Scub08 {
        &self.scub08
    }
    #[doc = "0xb0c - SW PUF Register 3"]
    #[inline(always)]
    pub const fn scub0c(&self) -> &Scub0c {
        &self.scub0c
    }
    #[doc = "0xb10 - SW PUF Register 4"]
    #[inline(always)]
    pub const fn scub10(&self) -> &Scub10 {
        &self.scub10
    }
    #[doc = "0xb14 - SW PUF Register 5"]
    #[inline(always)]
    pub const fn scub14(&self) -> &Scub14 {
        &self.scub14
    }
    #[doc = "0xb18 - SW PUF Register 6"]
    #[inline(always)]
    pub const fn scub18(&self) -> &Scub18 {
        &self.scub18
    }
    #[doc = "0xb1c - SW PUF Register 7"]
    #[inline(always)]
    pub const fn scub1c(&self) -> &Scub1c {
        &self.scub1c
    }
    #[doc = "0xb20 - SW PUF Register 8"]
    #[inline(always)]
    pub const fn scub20(&self) -> &Scub20 {
        &self.scub20
    }
    #[doc = "0xb24 - SW PUF Register 9"]
    #[inline(always)]
    pub const fn scub24(&self) -> &Scub24 {
        &self.scub24
    }
    #[doc = "0xb28 - SW PUF Register 10"]
    #[inline(always)]
    pub const fn scub28(&self) -> &Scub28 {
        &self.scub28
    }
    #[doc = "0xb2c - SW PUF Register 11"]
    #[inline(always)]
    pub const fn scub2c(&self) -> &Scub2c {
        &self.scub2c
    }
    #[doc = "0xb30 - SW PUF Register 12"]
    #[inline(always)]
    pub const fn scub30(&self) -> &Scub30 {
        &self.scub30
    }
    #[doc = "0xb34 - SW PUF Register 13"]
    #[inline(always)]
    pub const fn scub34(&self) -> &Scub34 {
        &self.scub34
    }
    #[doc = "0xb38 - SW PUF Register 14"]
    #[inline(always)]
    pub const fn scub38(&self) -> &Scub38 {
        &self.scub38
    }
    #[doc = "0xb3c - SW PUF Register 15"]
    #[inline(always)]
    pub const fn scub3c(&self) -> &Scub3c {
        &self.scub3c
    }
    #[doc = "0xb40 - SW PUF Register 16"]
    #[inline(always)]
    pub const fn scub40(&self) -> &Scub40 {
        &self.scub40
    }
    #[doc = "0xb44 - SW PUF Register 17"]
    #[inline(always)]
    pub const fn scub44(&self) -> &Scub44 {
        &self.scub44
    }
    #[doc = "0xb48 - SW PUF Register 18"]
    #[inline(always)]
    pub const fn scub48(&self) -> &Scub48 {
        &self.scub48
    }
    #[doc = "0xb4c - SW PUF Register 19"]
    #[inline(always)]
    pub const fn scub4c(&self) -> &Scub4c {
        &self.scub4c
    }
    #[doc = "0xb50 - SW PUF Register 20"]
    #[inline(always)]
    pub const fn scub50(&self) -> &Scub50 {
        &self.scub50
    }
    #[doc = "0xb54 - SW PUF Register 21"]
    #[inline(always)]
    pub const fn scub54(&self) -> &Scub54 {
        &self.scub54
    }
    #[doc = "0xb58 - SW PUF Register 22"]
    #[inline(always)]
    pub const fn scub58(&self) -> &Scub58 {
        &self.scub58
    }
    #[doc = "0xb5c - SW PUF Register 23"]
    #[inline(always)]
    pub const fn scub5c(&self) -> &Scub5c {
        &self.scub5c
    }
    #[doc = "0xb60 - SW PUF Register 24"]
    #[inline(always)]
    pub const fn scub60(&self) -> &Scub60 {
        &self.scub60
    }
    #[doc = "0xb64 - SW PUF Register 25"]
    #[inline(always)]
    pub const fn scub64(&self) -> &Scub64 {
        &self.scub64
    }
    #[doc = "0xb68 - SW PUF Register 26"]
    #[inline(always)]
    pub const fn scub68(&self) -> &Scub68 {
        &self.scub68
    }
    #[doc = "0xb6c - SW PUF Register 27"]
    #[inline(always)]
    pub const fn scub6c(&self) -> &Scub6c {
        &self.scub6c
    }
    #[doc = "0xb70 - SW PUF Register 28"]
    #[inline(always)]
    pub const fn scub70(&self) -> &Scub70 {
        &self.scub70
    }
    #[doc = "0xb74 - SW PUF Register 29"]
    #[inline(always)]
    pub const fn scub74(&self) -> &Scub74 {
        &self.scub74
    }
    #[doc = "0xb78 - SW PUF Register 30"]
    #[inline(always)]
    pub const fn scub78(&self) -> &Scub78 {
        &self.scub78
    }
    #[doc = "0xb7c - SW PUF Register 31"]
    #[inline(always)]
    pub const fn scub7c(&self) -> &Scub7c {
        &self.scub7c
    }
    #[doc = "0xb80 - PUF Control Register"]
    #[inline(always)]
    pub const fn scub80(&self) -> &Scub80 {
        &self.scub80
    }
    #[doc = "0xba0 - HW PUF Register 8"]
    #[inline(always)]
    pub const fn scuba0(&self) -> &Scuba0 {
        &self.scuba0
    }
    #[doc = "0xba4 - HW PUF Register 9"]
    #[inline(always)]
    pub const fn scuba4(&self) -> &Scuba4 {
        &self.scuba4
    }
    #[doc = "0xba8 - HW PUF Register 10"]
    #[inline(always)]
    pub const fn scuba8(&self) -> &Scuba8 {
        &self.scuba8
    }
    #[doc = "0xbac - HW PUF Register 11"]
    #[inline(always)]
    pub const fn scubac(&self) -> &Scubac {
        &self.scubac
    }
    #[doc = "0xbb0 - HW PUF Register 12"]
    #[inline(always)]
    pub const fn scubb0(&self) -> &Scubb0 {
        &self.scubb0
    }
    #[doc = "0xbb4 - HW PUF Register 13"]
    #[inline(always)]
    pub const fn scubb4(&self) -> &Scubb4 {
        &self.scubb4
    }
    #[doc = "0xbb8 - HW PUF Register 14"]
    #[inline(always)]
    pub const fn scubb8(&self) -> &Scubb8 {
        &self.scubb8
    }
    #[doc = "0xbbc - HW PUF Register 15"]
    #[inline(always)]
    pub const fn scubbc(&self) -> &Scubbc {
        &self.scubbc
    }
    #[doc = "0xbc0 - HW PUF Register 16"]
    #[inline(always)]
    pub const fn scubc0(&self) -> &Scubc0 {
        &self.scubc0
    }
    #[doc = "0xbc4 - HW PUF Register 17"]
    #[inline(always)]
    pub const fn scubc4(&self) -> &Scubc4 {
        &self.scubc4
    }
    #[doc = "0xbc8 - HW PUF Register 18"]
    #[inline(always)]
    pub const fn scubc8(&self) -> &Scubc8 {
        &self.scubc8
    }
    #[doc = "0xbcc - HW PUF Register 19"]
    #[inline(always)]
    pub const fn scubcc(&self) -> &Scubcc {
        &self.scubcc
    }
    #[doc = "0xbd0 - HW PUF Register 20"]
    #[inline(always)]
    pub const fn scubd0(&self) -> &Scubd0 {
        &self.scubd0
    }
    #[doc = "0xbd4 - HW PUF Register 21"]
    #[inline(always)]
    pub const fn scubd4(&self) -> &Scubd4 {
        &self.scubd4
    }
    #[doc = "0xbd8 - HW PUF Register 22"]
    #[inline(always)]
    pub const fn scubd8(&self) -> &Scubd8 {
        &self.scubd8
    }
    #[doc = "0xbdc - HW PUF Register 23"]
    #[inline(always)]
    pub const fn scubdc(&self) -> &Scubdc {
        &self.scubdc
    }
    #[doc = "0xbe0 - HW PUF Register 24"]
    #[inline(always)]
    pub const fn scube0(&self) -> &Scube0 {
        &self.scube0
    }
    #[doc = "0xbe4 - HW PUF Register 25"]
    #[inline(always)]
    pub const fn scube4(&self) -> &Scube4 {
        &self.scube4
    }
    #[doc = "0xbe8 - HW PUF Register 26"]
    #[inline(always)]
    pub const fn scube8(&self) -> &Scube8 {
        &self.scube8
    }
    #[doc = "0xbec - HW PUF Register 27"]
    #[inline(always)]
    pub const fn scubec(&self) -> &Scubec {
        &self.scubec
    }
    #[doc = "0xbf0 - HW PUF Register 28"]
    #[inline(always)]
    pub const fn scubf0(&self) -> &Scubf0 {
        &self.scubf0
    }
    #[doc = "0xbf4 - HW PUF Register 29"]
    #[inline(always)]
    pub const fn scubf4(&self) -> &Scubf4 {
        &self.scubf4
    }
    #[doc = "0xbf8 - HW PUF Register 30"]
    #[inline(always)]
    pub const fn scubf8(&self) -> &Scubf8 {
        &self.scubf8
    }
    #[doc = "0xbfc - HW PUF Register 31"]
    #[inline(always)]
    pub const fn scubfc(&self) -> &Scubfc {
        &self.scubfc
    }
    #[doc = "0xc04 - Secure1 Control 2 Register"]
    #[inline(always)]
    pub const fn scuc04(&self) -> &Scuc04 {
        &self.scuc04
    }
    #[doc = "0xc08 - Secure1 Control 3 Register"]
    #[inline(always)]
    pub const fn scuc08(&self) -> &Scuc08 {
        &self.scuc08
    }
    #[doc = "0xc0c - Secure1 Control 4 Register"]
    #[inline(always)]
    pub const fn scuc0c(&self) -> &Scuc0c {
        &self.scuc0c
    }
    #[doc = "0xc14 - Secure1 Control 6 Register"]
    #[inline(always)]
    pub const fn scuc14(&self) -> &Scuc14 {
        &self.scuc14
    }
    #[doc = "0xc18 - Secure1 Control 7 Register"]
    #[inline(always)]
    pub const fn scuc18(&self) -> &Scuc18 {
        &self.scuc18
    }
    #[doc = "0xc1c - Secure1 Control 8 Register"]
    #[inline(always)]
    pub const fn scuc1c(&self) -> &Scuc1c {
        &self.scuc1c
    }
    #[doc = "0xc28 - Secure1 Control 11 Register"]
    #[inline(always)]
    pub const fn scuc28(&self) -> &Scuc28 {
        &self.scuc28
    }
    #[doc = "0xc34 - Secure1 Control 14 Register"]
    #[inline(always)]
    pub const fn scuc34(&self) -> &Scuc34 {
        &self.scuc34
    }
    #[doc = "0xc48 - Secure1 Control 19 Register"]
    #[inline(always)]
    pub const fn scuc48(&self) -> &Scuc48 {
        &self.scuc48
    }
    #[doc = "0xc4c - Secure1 Control 20 Register"]
    #[inline(always)]
    pub const fn scuc4c(&self) -> &Scuc4c {
        &self.scuc4c
    }
    #[doc = "0xc50 - Secure1 Control 21 Register"]
    #[inline(always)]
    pub const fn scuc50(&self) -> &Scuc50 {
        &self.scuc50
    }
    #[doc = "0xc5c - Secure1 Control 24 Register"]
    #[inline(always)]
    pub const fn scuc5c(&self) -> &Scuc5c {
        &self.scuc5c
    }
    #[doc = "0xc84 - Secure2 Control 2 Register"]
    #[inline(always)]
    pub const fn scuc84(&self) -> &Scuc84 {
        &self.scuc84
    }
    #[doc = "0xc88 - Secure2 Control 3 Register"]
    #[inline(always)]
    pub const fn scuc88(&self) -> &Scuc88 {
        &self.scuc88
    }
    #[doc = "0xc8c - Secure2 Control 4 Register"]
    #[inline(always)]
    pub const fn scuc8c(&self) -> &Scuc8c {
        &self.scuc8c
    }
    #[doc = "0xc94 - Secure2 Control 6 Register"]
    #[inline(always)]
    pub const fn scuc94(&self) -> &Scuc94 {
        &self.scuc94
    }
    #[doc = "0xc98 - Secure2 Control 7 Register"]
    #[inline(always)]
    pub const fn scuc98(&self) -> &Scuc98 {
        &self.scuc98
    }
    #[doc = "0xc9c - Secure2 Control 7 Register"]
    #[inline(always)]
    pub const fn scuc9c(&self) -> &Scuc9c {
        &self.scuc9c
    }
    #[doc = "0xca8 - Secure2 Control 11 Register"]
    #[inline(always)]
    pub const fn scuca8(&self) -> &Scuca8 {
        &self.scuca8
    }
    #[doc = "0xcb4 - Secure2 Control 14 Register"]
    #[inline(always)]
    pub const fn scucb4(&self) -> &Scucb4 {
        &self.scucb4
    }
    #[doc = "0xcc8 - Secure2 Control 19 Register"]
    #[inline(always)]
    pub const fn scucc8(&self) -> &Scucc8 {
        &self.scucc8
    }
    #[doc = "0xccc - Secure2 Control 20 Register"]
    #[inline(always)]
    pub const fn scuccc(&self) -> &Scuccc {
        &self.scuccc
    }
    #[doc = "0xcd0 - Secure2 Control 21 Register"]
    #[inline(always)]
    pub const fn scucd0(&self) -> &Scucd0 {
        &self.scucd0
    }
    #[doc = "0xcdc - Secure2 Control 24 Register"]
    #[inline(always)]
    pub const fn scucdc(&self) -> &Scucdc {
        &self.scucdc
    }
    #[doc = "0xd04 - Secure3 Control 2 Register"]
    #[inline(always)]
    pub const fn scud04(&self) -> &Scud04 {
        &self.scud04
    }
    #[doc = "0xd08 - Secure3 Control 3 Register"]
    #[inline(always)]
    pub const fn scud08(&self) -> &Scud08 {
        &self.scud08
    }
    #[doc = "0xd0c - Secure3 Control 4 Register"]
    #[inline(always)]
    pub const fn scud0c(&self) -> &Scud0c {
        &self.scud0c
    }
    #[doc = "0xd14 - Secure3 Control 6 Register"]
    #[inline(always)]
    pub const fn scud14(&self) -> &Scud14 {
        &self.scud14
    }
    #[doc = "0xd18 - Secure3 Control 7 Register"]
    #[inline(always)]
    pub const fn scud18(&self) -> &Scud18 {
        &self.scud18
    }
    #[doc = "0xd1c - Secure3 Control 7 Register"]
    #[inline(always)]
    pub const fn scud1c(&self) -> &Scud1c {
        &self.scud1c
    }
    #[doc = "0xd28 - Secure3 Control 11 Register"]
    #[inline(always)]
    pub const fn scud28(&self) -> &Scud28 {
        &self.scud28
    }
    #[doc = "0xd34 - Secure3 Control 14 Register"]
    #[inline(always)]
    pub const fn scud34(&self) -> &Scud34 {
        &self.scud34
    }
    #[doc = "0xd48 - Secure3 Control 19 Register"]
    #[inline(always)]
    pub const fn scud48(&self) -> &Scud48 {
        &self.scud48
    }
    #[doc = "0xd4c - Secure3 Control 20 Register"]
    #[inline(always)]
    pub const fn scud4c(&self) -> &Scud4c {
        &self.scud4c
    }
    #[doc = "0xd50 - Secure3 Control 21 Register"]
    #[inline(always)]
    pub const fn scud50(&self) -> &Scud50 {
        &self.scud50
    }
    #[doc = "0xd5c - Secure3 Control 24 Register"]
    #[inline(always)]
    pub const fn scud5c(&self) -> &Scud5c {
        &self.scud5c
    }
    #[doc = "0xe00 - Write Protection 1 Register"]
    #[inline(always)]
    pub const fn scue00(&self) -> &Scue00 {
        &self.scue00
    }
    #[doc = "0xe04 - Write Protection 2 Register"]
    #[inline(always)]
    pub const fn scue04(&self) -> &Scue04 {
        &self.scue04
    }
    #[doc = "0xe08 - Write Protection 3 Register"]
    #[inline(always)]
    pub const fn scue08(&self) -> &Scue08 {
        &self.scue08
    }
    #[doc = "0xe0c - Write Protection 4 Register"]
    #[inline(always)]
    pub const fn scue0c(&self) -> &Scue0c {
        &self.scue0c
    }
    #[doc = "0xe10 - Write Protection 5 Register"]
    #[inline(always)]
    pub const fn scue10(&self) -> &Scue10 {
        &self.scue10
    }
    #[doc = "0xe14 - Write Protection 6 Register"]
    #[inline(always)]
    pub const fn scue14(&self) -> &Scue14 {
        &self.scue14
    }
    #[doc = "0xe18 - Write Protection 7 Register"]
    #[inline(always)]
    pub const fn scue18(&self) -> &Scue18 {
        &self.scue18
    }
    #[doc = "0xe1c - Write Protection 8 Register"]
    #[inline(always)]
    pub const fn scue1c(&self) -> &Scue1c {
        &self.scue1c
    }
    #[doc = "0xe24 - Write Protection 10 Register"]
    #[inline(always)]
    pub const fn scue24(&self) -> &Scue24 {
        &self.scue24
    }
    #[doc = "0xe28 - Write Protection 11 Register"]
    #[inline(always)]
    pub const fn scue28(&self) -> &Scue28 {
        &self.scue28
    }
    #[doc = "0xe2c - Write Protection 12 Register"]
    #[inline(always)]
    pub const fn scue2c(&self) -> &Scue2c {
        &self.scue2c
    }
    #[doc = "0xe30 - Write Protection 13 Register"]
    #[inline(always)]
    pub const fn scue30(&self) -> &Scue30 {
        &self.scue30
    }
    #[doc = "0xe34 - Write Protection 14 Register"]
    #[inline(always)]
    pub const fn scue34(&self) -> &Scue34 {
        &self.scue34
    }
    #[doc = "0xe38 - Write Protection 15 Register"]
    #[inline(always)]
    pub const fn scue38(&self) -> &Scue38 {
        &self.scue38
    }
    #[doc = "0xe3c - Write Protection 16 Register"]
    #[inline(always)]
    pub const fn scue3c(&self) -> &Scue3c {
        &self.scue3c
    }
    #[doc = "0xe40 - Write Protection 17 Register"]
    #[inline(always)]
    pub const fn scue40(&self) -> &Scue40 {
        &self.scue40
    }
    #[doc = "0xe44 - Write Protection 18 Register"]
    #[inline(always)]
    pub const fn scue44(&self) -> &Scue44 {
        &self.scue44
    }
    #[doc = "0xe48 - Write Protection 19 Register"]
    #[inline(always)]
    pub const fn scue48(&self) -> &Scue48 {
        &self.scue48
    }
    #[doc = "0xe4c - Write Protection 20 Register"]
    #[inline(always)]
    pub const fn scue4c(&self) -> &Scue4c {
        &self.scue4c
    }
    #[doc = "0xe50 - Write Protection 21 Register"]
    #[inline(always)]
    pub const fn scue50(&self) -> &Scue50 {
        &self.scue50
    }
    #[doc = "0xe54 - Write Protection 22 Register"]
    #[inline(always)]
    pub const fn scue54(&self) -> &Scue54 {
        &self.scue54
    }
    #[doc = "0xe5c - Write Protection 23 Register"]
    #[inline(always)]
    pub const fn scue5c(&self) -> &Scue5c {
        &self.scue5c
    }
    #[doc = "0xe60 - Write Protection 24 Register"]
    #[inline(always)]
    pub const fn scue60(&self) -> &Scue60 {
        &self.scue60
    }
    #[doc = "0xe64 - Write Protection 26 Register"]
    #[inline(always)]
    pub const fn scue64(&self) -> &Scue64 {
        &self.scue64
    }
    #[doc = "0xe68 - Write Protection 27 Register"]
    #[inline(always)]
    pub const fn scue68(&self) -> &Scue68 {
        &self.scue68
    }
    #[doc = "0xe70 - Write Protection 29 Register"]
    #[inline(always)]
    pub const fn scue70(&self) -> &Scue70 {
        &self.scue70
    }
    #[doc = "0xe78 - Write Protection 31 Register"]
    #[inline(always)]
    pub const fn scue78(&self) -> &Scue78 {
        &self.scue78
    }
    #[doc = "0xf00 - Reset Control 1 Register"]
    #[inline(always)]
    pub const fn scuf00(&self) -> &Scuf00 {
        &self.scuf00
    }
    #[doc = "0xf04 - Reset Control 2 Register"]
    #[inline(always)]
    pub const fn scuf04(&self) -> &Scuf04 {
        &self.scuf04
    }
    #[doc = "0xf08 - Reset Control 3 Register"]
    #[inline(always)]
    pub const fn scuf08(&self) -> &Scuf08 {
        &self.scuf08
    }
    #[doc = "0xf0c - Reset Control 4 Register"]
    #[inline(always)]
    pub const fn scuf0c(&self) -> &Scuf0c {
        &self.scuf0c
    }
    #[doc = "0xf10 - Reset Control 5 Register"]
    #[inline(always)]
    pub const fn scuf10(&self) -> &Scuf10 {
        &self.scuf10
    }
    #[doc = "0xf14 - Reset Control 6 Register"]
    #[inline(always)]
    pub const fn scuf14(&self) -> &Scuf14 {
        &self.scuf14
    }
    #[doc = "0xf18 - Reset Control 7 Register"]
    #[inline(always)]
    pub const fn scuf18(&self) -> &Scuf18 {
        &self.scuf18
    }
    #[doc = "0xf1c - Reset Control 8 Register"]
    #[inline(always)]
    pub const fn scuf1c(&self) -> &Scuf1c {
        &self.scuf1c
    }
    #[doc = "0xf20 - Reset Control 9 Register"]
    #[inline(always)]
    pub const fn scuf20(&self) -> &Scuf20 {
        &self.scuf20
    }
    #[doc = "0xf24 - Reset Control 10 Register"]
    #[inline(always)]
    pub const fn scuf24(&self) -> &Scuf24 {
        &self.scuf24
    }
    #[doc = "0xf28 - Reset Control 11 Register"]
    #[inline(always)]
    pub const fn scuf28(&self) -> &Scuf28 {
        &self.scuf28
    }
    #[doc = "0xf2c - Reset Control 12 Register"]
    #[inline(always)]
    pub const fn scuf2c(&self) -> &Scuf2c {
        &self.scuf2c
    }
    #[doc = "0xf30 - Reset Control 13 Register"]
    #[inline(always)]
    pub const fn scuf30(&self) -> &Scuf30 {
        &self.scuf30
    }
    #[doc = "0xf34 - Reset Control 14 Register"]
    #[inline(always)]
    pub const fn scuf34(&self) -> &Scuf34 {
        &self.scuf34
    }
    #[doc = "0xf38 - Reset Control 15 Register"]
    #[inline(always)]
    pub const fn scuf38(&self) -> &Scuf38 {
        &self.scuf38
    }
    #[doc = "0xf3c - Reset Control 16 Register"]
    #[inline(always)]
    pub const fn scuf3c(&self) -> &Scuf3c {
        &self.scuf3c
    }
    #[doc = "0xf48 - Reset Control 19 Register"]
    #[inline(always)]
    pub const fn scuf48(&self) -> &Scuf48 {
        &self.scuf48
    }
    #[doc = "0xf4c - Reset Control 20 Register"]
    #[inline(always)]
    pub const fn scuf4c(&self) -> &Scuf4c {
        &self.scuf4c
    }
    #[doc = "0xf50 - Reset Control 21 Register"]
    #[inline(always)]
    pub const fn scuf50(&self) -> &Scuf50 {
        &self.scuf50
    }
    #[doc = "0xf5c - Reset Control 24 Register"]
    #[inline(always)]
    pub const fn scuf5c(&self) -> &Scuf5c {
        &self.scuf5c
    }
    #[doc = "0xf60 - Reset Control 25 Register"]
    #[inline(always)]
    pub const fn scuf60(&self) -> &Scuf60 {
        &self.scuf60
    }
    #[doc = "0xf64 - Reset Control 26 Register"]
    #[inline(always)]
    pub const fn scuf64(&self) -> &Scuf64 {
        &self.scuf64
    }
    #[doc = "0xf68 - Reset Control 27 Register"]
    #[inline(always)]
    pub const fn scuf68(&self) -> &Scuf68 {
        &self.scuf68
    }
    #[doc = "0xf6c - Reset Control 28 Register"]
    #[inline(always)]
    pub const fn scuf6c(&self) -> &Scuf6c {
        &self.scuf6c
    }
    #[doc = "0xf70 - Reset Control 29 Register"]
    #[inline(always)]
    pub const fn scuf70(&self) -> &Scuf70 {
        &self.scuf70
    }
    #[doc = "0xf78 - Reset Control 31 Register"]
    #[inline(always)]
    pub const fn scuf78(&self) -> &Scuf78 {
        &self.scuf78
    }
}
#[doc = "SCU000 (rw) register accessor: Silicon Revision ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu000`] module"]
#[doc(alias = "SCU000")]
pub type Scu000 = crate::Reg<scu000::Scu000Spec>;
#[doc = "Silicon Revision ID Register"]
pub mod scu000;
#[doc = "SCU010 (rw) register accessor: Hardware/OTP Strap 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu010`] module"]
#[doc(alias = "SCU010")]
pub type Scu010 = crate::Reg<scu010::Scu010Spec>;
#[doc = "Hardware/OTP Strap 1 Register"]
pub mod scu010;
#[doc = "SCU014 (rw) register accessor: Hareware/OTP Strap Clear 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu014`] module"]
#[doc(alias = "SCU014")]
pub type Scu014 = crate::Reg<scu014::Scu014Spec>;
#[doc = "Hareware/OTP Strap Clear 1 Register"]
pub mod scu014;
#[doc = "SCU020 (rw) register accessor: Hardware/OTP Strap Write Protection 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu020`] module"]
#[doc(alias = "SCU020")]
pub type Scu020 = crate::Reg<scu020::Scu020Spec>;
#[doc = "Hardware/OTP Strap Write Protection 1 Register"]
pub mod scu020;
#[doc = "SCU024 (rw) register accessor: Hardware/OTP Strap Secure Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu024`] module"]
#[doc(alias = "SCU024")]
pub type Scu024 = crate::Reg<scu024::Scu024Spec>;
#[doc = "Hardware/OTP Strap Secure Control Register 1"]
pub mod scu024;
#[doc = "SCU028 (rw) register accessor: Hardware/OTP Strap Secure Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu028`] module"]
#[doc(alias = "SCU028")]
pub type Scu028 = crate::Reg<scu028::Scu028Spec>;
#[doc = "Hardware/OTP Strap Secure Control Register 2"]
pub mod scu028;
#[doc = "SCU02C (rw) register accessor: Hardware/OTP Strap Secure Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu02c`] module"]
#[doc(alias = "SCU02C")]
pub type Scu02c = crate::Reg<scu02c::Scu02cSpec>;
#[doc = "Hardware/OTP Strap Secure Control Register 3"]
pub mod scu02c;
#[doc = "SCU030 (rw) register accessor: Hardware/OTP Strap 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu030`] module"]
#[doc(alias = "SCU030")]
pub type Scu030 = crate::Reg<scu030::Scu030Spec>;
#[doc = "Hardware/OTP Strap 2 Register"]
pub mod scu030;
#[doc = "SCU034 (rw) register accessor: Hareware/OTP Strap Clear 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu034`] module"]
#[doc(alias = "SCU034")]
pub type Scu034 = crate::Reg<scu034::Scu034Spec>;
#[doc = "Hareware/OTP Strap Clear 2 Register"]
pub mod scu034;
#[doc = "SCU040 (rw) register accessor: Hardware/OTP Strap Write Protection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu040`] module"]
#[doc(alias = "SCU040")]
pub type Scu040 = crate::Reg<scu040::Scu040Spec>;
#[doc = "Hardware/OTP Strap Write Protection 2 Register"]
pub mod scu040;
#[doc = "SCU044 (rw) register accessor: Hardware/OTP Strap 2 Secure Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu044`] module"]
#[doc(alias = "SCU044")]
pub type Scu044 = crate::Reg<scu044::Scu044Spec>;
#[doc = "Hardware/OTP Strap 2 Secure Control Register 1"]
pub mod scu044;
#[doc = "SCU048 (rw) register accessor: Hardware/OTP Strap 2 Secure Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu048`] module"]
#[doc(alias = "SCU048")]
pub type Scu048 = crate::Reg<scu048::Scu048Spec>;
#[doc = "Hardware/OTP Strap 2 Secure Control Register 2"]
pub mod scu048;
#[doc = "SCU04C (rw) register accessor: Hardware/OTP Strap 2 Secure Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu04c`] module"]
#[doc(alias = "SCU04C")]
pub type Scu04c = crate::Reg<scu04c::Scu04cSpec>;
#[doc = "Hardware/OTP Strap 2 Secure Control Register 3"]
pub mod scu04c;
#[doc = "SCU050 (rw) register accessor: Hardware/OTP Strap 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu050`] module"]
#[doc(alias = "SCU050")]
pub type Scu050 = crate::Reg<scu050::Scu050Spec>;
#[doc = "Hardware/OTP Strap 3 Register"]
pub mod scu050;
#[doc = "SCU054 (rw) register accessor: Hareware/OTP Strap Clear 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu054`] module"]
#[doc(alias = "SCU054")]
pub type Scu054 = crate::Reg<scu054::Scu054Spec>;
#[doc = "Hareware/OTP Strap Clear 3 Register"]
pub mod scu054;
#[doc = "SCU060 (rw) register accessor: Hardware/OTP Strap Write Protection 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu060`] module"]
#[doc(alias = "SCU060")]
pub type Scu060 = crate::Reg<scu060::Scu060Spec>;
#[doc = "Hardware/OTP Strap Write Protection 3 Register"]
pub mod scu060;
#[doc = "SCU064 (rw) register accessor: Hardware/OTP Strap 3 Secure Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu064`] module"]
#[doc(alias = "SCU064")]
pub type Scu064 = crate::Reg<scu064::Scu064Spec>;
#[doc = "Hardware/OTP Strap 3 Secure Control Register 1"]
pub mod scu064;
#[doc = "SCU068 (rw) register accessor: Hardware/OTP Strap 3 Secure Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu068`] module"]
#[doc(alias = "SCU068")]
pub type Scu068 = crate::Reg<scu068::Scu068Spec>;
#[doc = "Hardware/OTP Strap 3 Secure Control Register 2"]
pub mod scu068;
#[doc = "SCU06C (rw) register accessor: Hardware/OTP Strap 3 Secure Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu06c`] module"]
#[doc(alias = "SCU06C")]
pub type Scu06c = crate::Reg<scu06c::Scu06cSpec>;
#[doc = "Hardware/OTP Strap 3 Secure Control Register 3"]
pub mod scu06c;
#[doc = "SCU070 (rw) register accessor: System Reset Event Log Set 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu070`] module"]
#[doc(alias = "SCU070")]
pub type Scu070 = crate::Reg<scu070::Scu070Spec>;
#[doc = "System Reset Event Log Set 1 Register"]
pub mod scu070;
#[doc = "SCU074 (rw) register accessor: System Reset Event Log Set 1 Secure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu074`] module"]
#[doc(alias = "SCU074")]
pub type Scu074 = crate::Reg<scu074::Scu074Spec>;
#[doc = "System Reset Event Log Set 1 Secure Register 1"]
pub mod scu074;
#[doc = "SCU078 (rw) register accessor: System Reset Event Log Set 1 Secure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu078`] module"]
#[doc(alias = "SCU078")]
pub type Scu078 = crate::Reg<scu078::Scu078Spec>;
#[doc = "System Reset Event Log Set 1 Secure Register 2"]
pub mod scu078;
#[doc = "SCU07C (rw) register accessor: System Reset Event Log Set 1 Secure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu07c`] module"]
#[doc(alias = "SCU07C")]
pub type Scu07c = crate::Reg<scu07c::Scu07cSpec>;
#[doc = "System Reset Event Log Set 1 Secure Register 3"]
pub mod scu07c;
#[doc = "SCU080 (rw) register accessor: System Reset Event Log Set 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu080`] module"]
#[doc(alias = "SCU080")]
pub type Scu080 = crate::Reg<scu080::Scu080Spec>;
#[doc = "System Reset Event Log Set 2 Register"]
pub mod scu080;
#[doc = "SCU084 (rw) register accessor: System Reset Event Log Set 2 Secure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu084`] module"]
#[doc(alias = "SCU084")]
pub type Scu084 = crate::Reg<scu084::Scu084Spec>;
#[doc = "System Reset Event Log Set 2 Secure Register 1"]
pub mod scu084;
#[doc = "SCU088 (rw) register accessor: System Reset Event Log Set 2 Secure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu088`] module"]
#[doc(alias = "SCU088")]
pub type Scu088 = crate::Reg<scu088::Scu088Spec>;
#[doc = "System Reset Event Log Set 2 Secure Register 2"]
pub mod scu088;
#[doc = "SCU08C (rw) register accessor: System Reset Event Log Set 2 Secure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu08c`] module"]
#[doc(alias = "SCU08C")]
pub type Scu08c = crate::Reg<scu08c::Scu08cSpec>;
#[doc = "System Reset Event Log Set 2 Secure Register 3"]
pub mod scu08c;
#[doc = "SCU090 (rw) register accessor: System Reset Event Log Set 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu090`] module"]
#[doc(alias = "SCU090")]
pub type Scu090 = crate::Reg<scu090::Scu090Spec>;
#[doc = "System Reset Event Log Set 3 Register"]
pub mod scu090;
#[doc = "SCU094 (rw) register accessor: System Reset Event Log Set 3 Secure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu094`] module"]
#[doc(alias = "SCU094")]
pub type Scu094 = crate::Reg<scu094::Scu094Spec>;
#[doc = "System Reset Event Log Set 3 Secure Register 1"]
pub mod scu094;
#[doc = "SCU098 (rw) register accessor: System Reset Event Log Set 3 Secure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu098`] module"]
#[doc(alias = "SCU098")]
pub type Scu098 = crate::Reg<scu098::Scu098Spec>;
#[doc = "System Reset Event Log Set 3 Secure Register 2"]
pub mod scu098;
#[doc = "SCU09C (rw) register accessor: System Reset Event Log Set 3 Secure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu09c`] module"]
#[doc(alias = "SCU09C")]
pub type Scu09c = crate::Reg<scu09c::Scu09cSpec>;
#[doc = "System Reset Event Log Set 3 Secure Register 3"]
pub mod scu09c;
#[doc = "SCU0A0 (rw) register accessor: System Reset Event Log Set 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0a0`] module"]
#[doc(alias = "SCU0A0")]
pub type Scu0a0 = crate::Reg<scu0a0::Scu0a0Spec>;
#[doc = "System Reset Event Log Set 4 Register"]
pub mod scu0a0;
#[doc = "SCU0A4 (rw) register accessor: System Reset Event Log Set 4 Secure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0a4`] module"]
#[doc(alias = "SCU0A4")]
pub type Scu0a4 = crate::Reg<scu0a4::Scu0a4Spec>;
#[doc = "System Reset Event Log Set 4 Secure Register 1"]
pub mod scu0a4;
#[doc = "SCU0A8 (rw) register accessor: System Reset Event Log Set 4 Secure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0a8`] module"]
#[doc(alias = "SCU0A8")]
pub type Scu0a8 = crate::Reg<scu0a8::Scu0a8Spec>;
#[doc = "System Reset Event Log Set 4 Secure Register 2"]
pub mod scu0a8;
#[doc = "SCU0AC (rw) register accessor: System Reset Event Log Set 4 Secure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0ac`] module"]
#[doc(alias = "SCU0AC")]
pub type Scu0ac = crate::Reg<scu0ac::Scu0acSpec>;
#[doc = "System Reset Event Log Set 4 Secure Register 3"]
pub mod scu0ac;
#[doc = "SCU0BC (rw) register accessor: Debug UART Baudrate\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0bc`] module"]
#[doc(alias = "SCU0BC")]
pub type Scu0bc = crate::Reg<scu0bc::Scu0bcSpec>;
#[doc = "Debug UART Baudrate"]
pub mod scu0bc;
#[doc = "SCU0C0 (rw) register accessor: Misc. Control Set 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0c0`] module"]
#[doc(alias = "SCU0C0")]
pub type Scu0c0 = crate::Reg<scu0c0::Scu0c0Spec>;
#[doc = "Misc. Control Set 1 Register"]
pub mod scu0c0;
#[doc = "SCU0C4 (rw) register accessor: Debug Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0c4`] module"]
#[doc(alias = "SCU0C4")]
pub type Scu0c4 = crate::Reg<scu0c4::Scu0c4Spec>;
#[doc = "Debug Selection Register"]
pub mod scu0c4;
#[doc = "SCU0C8 (rw) register accessor: Debug Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0c8`] module"]
#[doc(alias = "SCU0C8")]
pub type Scu0c8 = crate::Reg<scu0c8::Scu0c8Spec>;
#[doc = "Debug Disable Register"]
pub mod scu0c8;
#[doc = "SCU0CC (rw) register accessor: HeartBeat Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0cc`] module"]
#[doc(alias = "SCU0CC")]
pub type Scu0cc = crate::Reg<scu0cc::Scu0ccSpec>;
#[doc = "HeartBeat Control Register"]
pub mod scu0cc;
#[doc = "SCU0D0 (rw) register accessor: Analog Mux Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0d0`] module"]
#[doc(alias = "SCU0D0")]
pub type Scu0d0 = crate::Reg<scu0d0::Scu0d0Spec>;
#[doc = "Analog Mux Mode Register"]
pub mod scu0d0;
#[doc = "SCU0D4 (rw) register accessor: SPI Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0d4`] module"]
#[doc(alias = "SCU0D4")]
pub type Scu0d4 = crate::Reg<scu0d4::Scu0d4Spec>;
#[doc = "SPI Mode Register"]
pub mod scu0d4;
#[doc = "SCU0F0 (rw) register accessor: Random Number Generator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0f0`] module"]
#[doc(alias = "SCU0F0")]
pub type Scu0f0 = crate::Reg<scu0f0::Scu0f0Spec>;
#[doc = "Random Number Generator Control Register"]
pub mod scu0f0;
#[doc = "SCU0F4 (rw) register accessor: Random Number Generator Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0f4`] module"]
#[doc(alias = "SCU0F4")]
pub type Scu0f4 = crate::Reg<scu0f4::Scu0f4Spec>;
#[doc = "Random Number Generator Data Register"]
pub mod scu0f4;
#[doc = "SCU100 (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu100`] module"]
#[doc(alias = "SCU100")]
pub type Scu100 = crate::Reg<scu100::Scu100Spec>;
#[doc = "Interrupt Status Register"]
pub mod scu100;
#[doc = "SCU104 (rw) register accessor: Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu104`] module"]
#[doc(alias = "SCU104")]
pub type Scu104 = crate::Reg<scu104::Scu104Spec>;
#[doc = "Interrupt Control Register"]
pub mod scu104;
#[doc = "SCU110 (rw) register accessor: MCU Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu110`] module"]
#[doc(alias = "SCU110")]
pub type Scu110 = crate::Reg<scu110::Scu110Spec>;
#[doc = "MCU Control and Status Register"]
pub mod scu110;
#[doc = "SCU114 (rw) register accessor: MCU Reset Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`scu114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu114`] module"]
#[doc(alias = "SCU114")]
pub type Scu114 = crate::Reg<scu114::Scu114Spec>;
#[doc = "MCU Reset Vector"]
pub mod scu114;
#[doc = "SCU120 (rw) register accessor: CPTRA Page Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu120`] module"]
#[doc(alias = "SCU120")]
pub type Scu120 = crate::Reg<scu120::Scu120Spec>;
#[doc = "CPTRA Page Register 0"]
pub mod scu120;
#[doc = "SCU124 (rw) register accessor: CPTRA Page Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu124`] module"]
#[doc(alias = "SCU124")]
pub type Scu124 = crate::Reg<scu124::Scu124Spec>;
#[doc = "CPTRA Page Register 1"]
pub mod scu124;
#[doc = "SCU128 (rw) register accessor: CPTRA Page Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu128`] module"]
#[doc(alias = "SCU128")]
pub type Scu128 = crate::Reg<scu128::Scu128Spec>;
#[doc = "CPTRA Page Register 2"]
pub mod scu128;
#[doc = "SCU12C (rw) register accessor: CPTRA Page Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu12c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu12c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu12c`] module"]
#[doc(alias = "SCU12C")]
pub type Scu12c = crate::Reg<scu12c::Scu12cSpec>;
#[doc = "CPTRA Page Register 3"]
pub mod scu12c;
#[doc = "SCU130 (rw) register accessor: CPTRA Page Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu130`] module"]
#[doc(alias = "SCU130")]
pub type Scu130 = crate::Reg<scu130::Scu130Spec>;
#[doc = "CPTRA Page Register 4"]
pub mod scu130;
#[doc = "SCU134 (rw) register accessor: CPTRA Page Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu134`] module"]
#[doc(alias = "SCU134")]
pub type Scu134 = crate::Reg<scu134::Scu134Spec>;
#[doc = "CPTRA Page Register 5"]
pub mod scu134;
#[doc = "SCU140 (rw) register accessor: Caliptra Config Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu140`] module"]
#[doc(alias = "SCU140")]
pub type Scu140 = crate::Reg<scu140::Scu140Spec>;
#[doc = "Caliptra Config Register 0"]
pub mod scu140;
#[doc = "SCU144 (rw) register accessor: Caliptra Config Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu144`] module"]
#[doc(alias = "SCU144")]
pub type Scu144 = crate::Reg<scu144::Scu144Spec>;
#[doc = "Caliptra Config Register 1"]
pub mod scu144;
#[doc = "SCU148 (rw) register accessor: Caliptra Config Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu148`] module"]
#[doc(alias = "SCU148")]
pub type Scu148 = crate::Reg<scu148::Scu148Spec>;
#[doc = "Caliptra Config Register 2"]
pub mod scu148;
#[doc = "SCU14C (rw) register accessor: Caliptra Config Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu14c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu14c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu14c`] module"]
#[doc(alias = "SCU14C")]
pub type Scu14c = crate::Reg<scu14c::Scu14cSpec>;
#[doc = "Caliptra Config Register 3"]
pub mod scu14c;
#[doc = "SCU150 (rw) register accessor: Caliptra Config Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu150`] module"]
#[doc(alias = "SCU150")]
pub type Scu150 = crate::Reg<scu150::Scu150Spec>;
#[doc = "Caliptra Config Register 4"]
pub mod scu150;
#[doc = "SCU154 (rw) register accessor: Caliptra Config Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu154`] module"]
#[doc(alias = "SCU154")]
pub type Scu154 = crate::Reg<scu154::Scu154Spec>;
#[doc = "Caliptra Config Register 5"]
pub mod scu154;
#[doc = "SCU158 (rw) register accessor: Caliptra Config Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu158`] module"]
#[doc(alias = "SCU158")]
pub type Scu158 = crate::Reg<scu158::Scu158Spec>;
#[doc = "Caliptra Config Register 6"]
pub mod scu158;
#[doc = "SCU15C (rw) register accessor: Caliptra Config Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu15c`] module"]
#[doc(alias = "SCU15C")]
pub type Scu15c = crate::Reg<scu15c::Scu15cSpec>;
#[doc = "Caliptra Config Register 7"]
pub mod scu15c;
#[doc = "SCU160 (rw) register accessor: Caliptra Config Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu160`] module"]
#[doc(alias = "SCU160")]
pub type Scu160 = crate::Reg<scu160::Scu160Spec>;
#[doc = "Caliptra Config Register 8"]
pub mod scu160;
#[doc = "SCU180 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu180`] module"]
#[doc(alias = "SCU180")]
pub type Scu180 = crate::Reg<scu180::Scu180Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_1"]
pub mod scu180;
#[doc = "SCU184 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu184`] module"]
#[doc(alias = "SCU184")]
pub type Scu184 = crate::Reg<scu184::Scu184Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_2"]
pub mod scu184;
#[doc = "SCU188 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu188`] module"]
#[doc(alias = "SCU188")]
pub type Scu188 = crate::Reg<scu188::Scu188Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_3"]
pub mod scu188;
#[doc = "SCU18C (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu18c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu18c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu18c`] module"]
#[doc(alias = "SCU18C")]
pub type Scu18c = crate::Reg<scu18c::Scu18cSpec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_4"]
pub mod scu18c;
#[doc = "SCU190 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu190`] module"]
#[doc(alias = "SCU190")]
pub type Scu190 = crate::Reg<scu190::Scu190Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_5"]
pub mod scu190;
#[doc = "SCU194 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu194`] module"]
#[doc(alias = "SCU194")]
pub type Scu194 = crate::Reg<scu194::Scu194Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_6"]
pub mod scu194;
#[doc = "SCU198 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu198`] module"]
#[doc(alias = "SCU198")]
pub type Scu198 = crate::Reg<scu198::Scu198Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_7"]
pub mod scu198;
#[doc = "SCU19C (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu19c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu19c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu19c`] module"]
#[doc(alias = "SCU19C")]
pub type Scu19c = crate::Reg<scu19c::Scu19cSpec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_8"]
pub mod scu19c;
#[doc = "SCU1A0 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1a0`] module"]
#[doc(alias = "SCU1A0")]
pub type Scu1a0 = crate::Reg<scu1a0::Scu1a0Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_9"]
pub mod scu1a0;
#[doc = "SCU1A4 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_10\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1a4`] module"]
#[doc(alias = "SCU1A4")]
pub type Scu1a4 = crate::Reg<scu1a4::Scu1a4Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_10"]
pub mod scu1a4;
#[doc = "SCU1A8 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1a8`] module"]
#[doc(alias = "SCU1A8")]
pub type Scu1a8 = crate::Reg<scu1a8::Scu1a8Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_11"]
pub mod scu1a8;
#[doc = "SCU1AC (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_12\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1ac`] module"]
#[doc(alias = "SCU1AC")]
pub type Scu1ac = crate::Reg<scu1ac::Scu1acSpec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_12"]
pub mod scu1ac;
#[doc = "SCU1B0 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1b0`] module"]
#[doc(alias = "SCU1B0")]
pub type Scu1b0 = crate::Reg<scu1b0::Scu1b0Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_13"]
pub mod scu1b0;
#[doc = "SCU1B4 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1b4`] module"]
#[doc(alias = "SCU1B4")]
pub type Scu1b4 = crate::Reg<scu1b4::Scu1b4Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_14"]
pub mod scu1b4;
#[doc = "SCU1B8 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_15\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1b8`] module"]
#[doc(alias = "SCU1B8")]
pub type Scu1b8 = crate::Reg<scu1b8::Scu1b8Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_15"]
pub mod scu1b8;
#[doc = "SCU1BC (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1bc`] module"]
#[doc(alias = "SCU1BC")]
pub type Scu1bc = crate::Reg<scu1bc::Scu1bcSpec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_16"]
pub mod scu1bc;
#[doc = "SCU1C0 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_17\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1c0`] module"]
#[doc(alias = "SCU1C0")]
pub type Scu1c0 = crate::Reg<scu1c0::Scu1c0Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_17"]
pub mod scu1c0;
#[doc = "SCU1C4 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_18\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1c4`] module"]
#[doc(alias = "SCU1C4")]
pub type Scu1c4 = crate::Reg<scu1c4::Scu1c4Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_18"]
pub mod scu1c4;
#[doc = "SCU1C8 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1c8`] module"]
#[doc(alias = "SCU1C8")]
pub type Scu1c8 = crate::Reg<scu1c8::Scu1c8Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_19"]
pub mod scu1c8;
#[doc = "SCU1CC (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_20\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1cc`] module"]
#[doc(alias = "SCU1CC")]
pub type Scu1cc = crate::Reg<scu1cc::Scu1ccSpec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_20"]
pub mod scu1cc;
#[doc = "SCU1D0 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_21\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1d0`] module"]
#[doc(alias = "SCU1D0")]
pub type Scu1d0 = crate::Reg<scu1d0::Scu1d0Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_21"]
pub mod scu1d0;
#[doc = "SCU1D4 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_22\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1d4`] module"]
#[doc(alias = "SCU1D4")]
pub type Scu1d4 = crate::Reg<scu1d4::Scu1d4Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_22"]
pub mod scu1d4;
#[doc = "SCU1D8 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1d8`] module"]
#[doc(alias = "SCU1D8")]
pub type Scu1d8 = crate::Reg<scu1d8::Scu1d8Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_23"]
pub mod scu1d8;
#[doc = "SCU1DC (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1dc`] module"]
#[doc(alias = "SCU1DC")]
pub type Scu1dc = crate::Reg<scu1dc::Scu1dcSpec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_24"]
pub mod scu1dc;
#[doc = "SCU1E0 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1e0`] module"]
#[doc(alias = "SCU1E0")]
pub type Scu1e0 = crate::Reg<scu1e0::Scu1e0Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_25"]
pub mod scu1e0;
#[doc = "SCU1E4 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_26\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1e4`] module"]
#[doc(alias = "SCU1E4")]
pub type Scu1e4 = crate::Reg<scu1e4::Scu1e4Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_26"]
pub mod scu1e4;
#[doc = "SCU1E8 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_27\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1e8`] module"]
#[doc(alias = "SCU1E8")]
pub type Scu1e8 = crate::Reg<scu1e8::Scu1e8Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_27"]
pub mod scu1e8;
#[doc = "SCU1EC (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_28\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1ec`] module"]
#[doc(alias = "SCU1EC")]
pub type Scu1ec = crate::Reg<scu1ec::Scu1ecSpec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_28"]
pub mod scu1ec;
#[doc = "SCU1F0 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_29\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1f0`] module"]
#[doc(alias = "SCU1F0")]
pub type Scu1f0 = crate::Reg<scu1f0::Scu1f0Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_29"]
pub mod scu1f0;
#[doc = "SCU1F4 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_30\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1f4`] module"]
#[doc(alias = "SCU1F4")]
pub type Scu1f4 = crate::Reg<scu1f4::Scu1f4Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_30"]
pub mod scu1f4;
#[doc = "SCU1F8 (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_31\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1f8`] module"]
#[doc(alias = "SCU1F8")]
pub type Scu1f8 = crate::Reg<scu1f8::Scu1f8Spec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_31"]
pub mod scu1f8;
#[doc = "SCU1FC (rw) register accessor: SCU\\_CPU\\_SCRATCH\\_32\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu1fc`] module"]
#[doc(alias = "SCU1FC")]
pub type Scu1fc = crate::Reg<scu1fc::Scu1fcSpec>;
#[doc = "SCU\\_CPU\\_SCRATCH\\_32"]
pub mod scu1fc;
#[doc = "SCU200 (rw) register accessor: System Reset Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu200`] module"]
#[doc(alias = "SCU200")]
pub type Scu200 = crate::Reg<scu200::Scu200Spec>;
#[doc = "System Reset Control 1 Register"]
pub mod scu200;
#[doc = "SCU204 (rw) register accessor: System Reset Clear 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu204`] module"]
#[doc(alias = "SCU204")]
pub type Scu204 = crate::Reg<scu204::Scu204Spec>;
#[doc = "System Reset Clear 1 Register"]
pub mod scu204;
#[doc = "SCU210 (rw) register accessor: System Reset Lock 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu210`] module"]
#[doc(alias = "SCU210")]
pub type Scu210 = crate::Reg<scu210::Scu210Spec>;
#[doc = "System Reset Lock 1 Register"]
pub mod scu210;
#[doc = "SCU214 (rw) register accessor: System Reset Secure 1 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu214`] module"]
#[doc(alias = "SCU214")]
pub type Scu214 = crate::Reg<scu214::Scu214Spec>;
#[doc = "System Reset Secure 1 Register 1"]
pub mod scu214;
#[doc = "SCU218 (rw) register accessor: System Reset Secure 1 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu218`] module"]
#[doc(alias = "SCU218")]
pub type Scu218 = crate::Reg<scu218::Scu218Spec>;
#[doc = "System Reset Secure 1 Register 2"]
pub mod scu218;
#[doc = "SCU21C (rw) register accessor: System Reset Secure 1 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu21c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu21c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu21c`] module"]
#[doc(alias = "SCU21C")]
pub type Scu21c = crate::Reg<scu21c::Scu21cSpec>;
#[doc = "System Reset Secure 1 Register 3"]
pub mod scu21c;
#[doc = "SCU220 (rw) register accessor: System Reset Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu220`] module"]
#[doc(alias = "SCU220")]
pub type Scu220 = crate::Reg<scu220::Scu220Spec>;
#[doc = "System Reset Control 2 Register"]
pub mod scu220;
#[doc = "SCU224 (rw) register accessor: System Reset Clear 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu224`] module"]
#[doc(alias = "SCU224")]
pub type Scu224 = crate::Reg<scu224::Scu224Spec>;
#[doc = "System Reset Clear 2 Register"]
pub mod scu224;
#[doc = "SCU230 (rw) register accessor: System Reset Lock 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu230`] module"]
#[doc(alias = "SCU230")]
pub type Scu230 = crate::Reg<scu230::Scu230Spec>;
#[doc = "System Reset Lock 2 Register"]
pub mod scu230;
#[doc = "SCU234 (rw) register accessor: System Reset Secure 2 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu234`] module"]
#[doc(alias = "SCU234")]
pub type Scu234 = crate::Reg<scu234::Scu234Spec>;
#[doc = "System Reset Secure 2 Register 1"]
pub mod scu234;
#[doc = "SCU238 (rw) register accessor: System Reset Secure 2 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu238`] module"]
#[doc(alias = "SCU238")]
pub type Scu238 = crate::Reg<scu238::Scu238Spec>;
#[doc = "System Reset Secure 2 Register 2"]
pub mod scu238;
#[doc = "SCU23C (rw) register accessor: System Reset Secure 2 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu23c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu23c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu23c`] module"]
#[doc(alias = "SCU23C")]
pub type Scu23c = crate::Reg<scu23c::Scu23cSpec>;
#[doc = "System Reset Secure 2 Register 3"]
pub mod scu23c;
#[doc = "SCU240 (rw) register accessor: Clock Stop Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu240`] module"]
#[doc(alias = "SCU240")]
pub type Scu240 = crate::Reg<scu240::Scu240Spec>;
#[doc = "Clock Stop Control 1 Register"]
pub mod scu240;
#[doc = "SCU244 (rw) register accessor: Clock Stop Clear 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu244`] module"]
#[doc(alias = "SCU244")]
pub type Scu244 = crate::Reg<scu244::Scu244Spec>;
#[doc = "Clock Stop Clear 1 Register"]
pub mod scu244;
#[doc = "SCU250 (rw) register accessor: Clock Stop Lock 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu250`] module"]
#[doc(alias = "SCU250")]
pub type Scu250 = crate::Reg<scu250::Scu250Spec>;
#[doc = "Clock Stop Lock 1 Register"]
pub mod scu250;
#[doc = "SCU254 (rw) register accessor: Clock Stop Secure 1 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu254`] module"]
#[doc(alias = "SCU254")]
pub type Scu254 = crate::Reg<scu254::Scu254Spec>;
#[doc = "Clock Stop Secure 1 Register 1"]
pub mod scu254;
#[doc = "SCU258 (rw) register accessor: Clock Stop Secure 1 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu258::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu258::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu258`] module"]
#[doc(alias = "SCU258")]
pub type Scu258 = crate::Reg<scu258::Scu258Spec>;
#[doc = "Clock Stop Secure 1 Register 2"]
pub mod scu258;
#[doc = "SCU25C (rw) register accessor: Clock Stop Secure 1 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu25c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu25c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu25c`] module"]
#[doc(alias = "SCU25C")]
pub type Scu25c = crate::Reg<scu25c::Scu25cSpec>;
#[doc = "Clock Stop Secure 1 Register 3"]
pub mod scu25c;
#[doc = "SCU260 (rw) register accessor: Clock Stop Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu260::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu260::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu260`] module"]
#[doc(alias = "SCU260")]
pub type Scu260 = crate::Reg<scu260::Scu260Spec>;
#[doc = "Clock Stop Control 2 Register"]
pub mod scu260;
#[doc = "SCU264 (rw) register accessor: Clock Stop Clear 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu264::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu264::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu264`] module"]
#[doc(alias = "SCU264")]
pub type Scu264 = crate::Reg<scu264::Scu264Spec>;
#[doc = "Clock Stop Clear 2 Register"]
pub mod scu264;
#[doc = "SCU270 (rw) register accessor: Clock Stop Lock 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu270::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu270::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu270`] module"]
#[doc(alias = "SCU270")]
pub type Scu270 = crate::Reg<scu270::Scu270Spec>;
#[doc = "Clock Stop Lock 2 Register"]
pub mod scu270;
#[doc = "SCU274 (rw) register accessor: Clock Stop Secure 2 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu274::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu274::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu274`] module"]
#[doc(alias = "SCU274")]
pub type Scu274 = crate::Reg<scu274::Scu274Spec>;
#[doc = "Clock Stop Secure 2 Register 1"]
pub mod scu274;
#[doc = "SCU278 (rw) register accessor: Clock Stop Secure 2 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu278::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu278::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu278`] module"]
#[doc(alias = "SCU278")]
pub type Scu278 = crate::Reg<scu278::Scu278Spec>;
#[doc = "Clock Stop Secure 2 Register 2"]
pub mod scu278;
#[doc = "SCU27C (rw) register accessor: Clock Stop Secure 2 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu27c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu27c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu27c`] module"]
#[doc(alias = "SCU27C")]
pub type Scu27c = crate::Reg<scu27c::Scu27cSpec>;
#[doc = "Clock Stop Secure 2 Register 3"]
pub mod scu27c;
#[doc = "SCU280 (rw) register accessor: Clock Selection 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu280::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu280::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu280`] module"]
#[doc(alias = "SCU280")]
pub type Scu280 = crate::Reg<scu280::Scu280Spec>;
#[doc = "Clock Selection 1 Register"]
pub mod scu280;
#[doc = "SCU284 (rw) register accessor: Clock Selection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu284::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu284::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu284`] module"]
#[doc(alias = "SCU284")]
pub type Scu284 = crate::Reg<scu284::Scu284Spec>;
#[doc = "Clock Selection 2 Register"]
pub mod scu284;
#[doc = "SCU2A0 (rw) register accessor: Clock Selection Lock 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2a0`] module"]
#[doc(alias = "SCU2A0")]
pub type Scu2a0 = crate::Reg<scu2a0::Scu2a0Spec>;
#[doc = "Clock Selection Lock 1 Register"]
pub mod scu2a0;
#[doc = "SCU2A4 (rw) register accessor: Clock Selection Secure 1 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2a4`] module"]
#[doc(alias = "SCU2A4")]
pub type Scu2a4 = crate::Reg<scu2a4::Scu2a4Spec>;
#[doc = "Clock Selection Secure 1 Register 1"]
pub mod scu2a4;
#[doc = "SCU2A8 (rw) register accessor: Clock Selection Secure 1 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2a8`] module"]
#[doc(alias = "SCU2A8")]
pub type Scu2a8 = crate::Reg<scu2a8::Scu2a8Spec>;
#[doc = "Clock Selection Secure 1 Register 2"]
pub mod scu2a8;
#[doc = "SCU2AC (rw) register accessor: Clock Selection Secure 1 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2ac`] module"]
#[doc(alias = "SCU2AC")]
pub type Scu2ac = crate::Reg<scu2ac::Scu2acSpec>;
#[doc = "Clock Selection Secure 1 Register 3"]
pub mod scu2ac;
#[doc = "SCU2B0 (rw) register accessor: Clock Selection Lock 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2b0`] module"]
#[doc(alias = "SCU2B0")]
pub type Scu2b0 = crate::Reg<scu2b0::Scu2b0Spec>;
#[doc = "Clock Selection Lock 2 Register"]
pub mod scu2b0;
#[doc = "SCU2B4 (rw) register accessor: Clock Selection Secure 2 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2b4`] module"]
#[doc(alias = "SCU2B4")]
pub type Scu2b4 = crate::Reg<scu2b4::Scu2b4Spec>;
#[doc = "Clock Selection Secure 2 Register 1"]
pub mod scu2b4;
#[doc = "SCU2B8 (rw) register accessor: Clock Selection Secure 2 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2b8`] module"]
#[doc(alias = "SCU2B8")]
pub type Scu2b8 = crate::Reg<scu2b8::Scu2b8Spec>;
#[doc = "Clock Selection Secure 2 Register 2"]
pub mod scu2b8;
#[doc = "SCU2BC (rw) register accessor: Clock Selection Secure 2 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2bc`] module"]
#[doc(alias = "SCU2BC")]
pub type Scu2bc = crate::Reg<scu2bc::Scu2bcSpec>;
#[doc = "Clock Selection Secure 2 Register 3"]
pub mod scu2bc;
#[doc = "SCU2F0 (rw) register accessor: EXTRST\\# Reset Selection 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2f0`] module"]
#[doc(alias = "SCU2F0")]
pub type Scu2f0 = crate::Reg<scu2f0::Scu2f0Spec>;
#[doc = "EXTRST\\# Reset Selection 1 Register"]
pub mod scu2f0;
#[doc = "SCU2F4 (rw) register accessor: EXTRST\\# Reset Selection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2f4`] module"]
#[doc(alias = "SCU2F4")]
pub type Scu2f4 = crate::Reg<scu2f4::Scu2f4Spec>;
#[doc = "EXTRST\\# Reset Selection 2 Register"]
pub mod scu2f4;
#[doc = "SCU2F8 (rw) register accessor: EXTRST\\# Reset Selection 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu2f8`] module"]
#[doc(alias = "SCU2F8")]
pub type Scu2f8 = crate::Reg<scu2f8::Scu2f8Spec>;
#[doc = "EXTRST\\# Reset Selection 3 Register"]
pub mod scu2f8;
#[doc = "SCU300 (rw) register accessor: HPLL Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu300::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu300::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu300`] module"]
#[doc(alias = "SCU300")]
pub type Scu300 = crate::Reg<scu300::Scu300Spec>;
#[doc = "HPLL Parameter Register"]
pub mod scu300;
#[doc = "SCU304 (rw) register accessor: HPLL Extended Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu304::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu304::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu304`] module"]
#[doc(alias = "SCU304")]
pub type Scu304 = crate::Reg<scu304::Scu304Spec>;
#[doc = "HPLL Extended Parameter Register"]
pub mod scu304;
#[doc = "SCU310 (rw) register accessor: DIPLL Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu310`] module"]
#[doc(alias = "SCU310")]
pub type Scu310 = crate::Reg<scu310::Scu310Spec>;
#[doc = "DIPLL Parameter Register"]
pub mod scu310;
#[doc = "SCU314 (rw) register accessor: DIPLL Parameter Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu314`] module"]
#[doc(alias = "SCU314")]
pub type Scu314 = crate::Reg<scu314::Scu314Spec>;
#[doc = "DIPLL Parameter Register 2"]
pub mod scu314;
#[doc = "SCU318 (rw) register accessor: DIPLL Parameter Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu318::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu318::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu318`] module"]
#[doc(alias = "SCU318")]
pub type Scu318 = crate::Reg<scu318::Scu318Spec>;
#[doc = "DIPLL Parameter Register 3"]
pub mod scu318;
#[doc = "SCU31C (rw) register accessor: DIPLL Parameter Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu31c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu31c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu31c`] module"]
#[doc(alias = "SCU31C")]
pub type Scu31c = crate::Reg<scu31c::Scu31cSpec>;
#[doc = "DIPLL Parameter Register 4"]
pub mod scu31c;
#[doc = "SCU320 (rw) register accessor: DIPLL Parameter Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu320::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu320::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu320`] module"]
#[doc(alias = "SCU320")]
pub type Scu320 = crate::Reg<scu320::Scu320Spec>;
#[doc = "DIPLL Parameter Register 5"]
pub mod scu320;
#[doc = "SCU324 (rw) register accessor: DIPLL Parameter Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu324::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu324::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu324`] module"]
#[doc(alias = "SCU324")]
pub type Scu324 = crate::Reg<scu324::Scu324Spec>;
#[doc = "DIPLL Parameter Register 6"]
pub mod scu324;
#[doc = "SCU32C (rw) register accessor: DIPLL Parameter Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu32c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu32c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu32c`] module"]
#[doc(alias = "SCU32C")]
pub type Scu32c = crate::Reg<scu32c::Scu32cSpec>;
#[doc = "DIPLL Parameter Register 8"]
pub mod scu32c;
#[doc = "SCU330 (rw) register accessor: UARTCLK Generation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu330::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu330::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu330`] module"]
#[doc(alias = "SCU330")]
pub type Scu330 = crate::Reg<scu330::Scu330Spec>;
#[doc = "UARTCLK Generation Register"]
pub mod scu330;
#[doc = "SCU334 (rw) register accessor: HUARTCLK Generation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu334::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu334::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu334`] module"]
#[doc(alias = "SCU334")]
pub type Scu334 = crate::Reg<scu334::Scu334Spec>;
#[doc = "HUARTCLK Generation Register"]
pub mod scu334;
#[doc = "SCU380 (rw) register accessor: Clock Duty Measurement Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu380::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu380::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu380`] module"]
#[doc(alias = "SCU380")]
pub type Scu380 = crate::Reg<scu380::Scu380Spec>;
#[doc = "Clock Duty Measurement Control Register"]
pub mod scu380;
#[doc = "SCU384 (rw) register accessor: Clock Duty Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu384::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu384::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu384`] module"]
#[doc(alias = "SCU384")]
pub type Scu384 = crate::Reg<scu384::Scu384Spec>;
#[doc = "Clock Duty Selection Register"]
pub mod scu384;
#[doc = "SCU388 (rw) register accessor: Clock Duty Measurement Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu388::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu388::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu388`] module"]
#[doc(alias = "SCU388")]
pub type Scu388 = crate::Reg<scu388::Scu388Spec>;
#[doc = "Clock Duty Measurement Result Register"]
pub mod scu388;
#[doc = "SCU390 (rw) register accessor: MAC0/1 Interface Clock Delay Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu390::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu390::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu390`] module"]
#[doc(alias = "SCU390")]
pub type Scu390 = crate::Reg<scu390::Scu390Spec>;
#[doc = "MAC0/1 Interface Clock Delay Setting"]
pub mod scu390;
#[doc = "SCU394 (rw) register accessor: MAC0/1 Interface Clock Delay 100M Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu394::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu394::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu394`] module"]
#[doc(alias = "SCU394")]
pub type Scu394 = crate::Reg<scu394::Scu394Spec>;
#[doc = "MAC0/1 Interface Clock Delay 100M Setting"]
pub mod scu394;
#[doc = "SCU398 (rw) register accessor: MAC0/1 Interface Clock Delay 10M Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu398::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu398::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu398`] module"]
#[doc(alias = "SCU398")]
pub type Scu398 = crate::Reg<scu398::Scu398Spec>;
#[doc = "MAC0/1 Interface Clock Delay 10M Setting"]
pub mod scu398;
#[doc = "SCU3A0 (rw) register accessor: Frequency Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu3a0`] module"]
#[doc(alias = "SCU3A0")]
pub type Scu3a0 = crate::Reg<scu3a0::Scu3a0Spec>;
#[doc = "Frequency Counter Control Register"]
pub mod scu3a0;
#[doc = "SCU3A4 (rw) register accessor: Frequency Counter Comparison\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu3a4`] module"]
#[doc(alias = "SCU3A4")]
pub type Scu3a4 = crate::Reg<scu3a4::Scu3a4Spec>;
#[doc = "Frequency Counter Comparison"]
pub mod scu3a4;
#[doc = "SCU3B0 (rw) register accessor: USB Controler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu3b0`] module"]
#[doc(alias = "SCU3B0")]
pub type Scu3b0 = crate::Reg<scu3b0::Scu3b0Spec>;
#[doc = "USB Controler Register"]
pub mod scu3b0;
#[doc = "SCU3B4 (rw) register accessor: USB Controler Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu3b4`] module"]
#[doc(alias = "SCU3B4")]
pub type Scu3b4 = crate::Reg<scu3b4::Scu3b4Spec>;
#[doc = "USB Controler Lock Register"]
pub mod scu3b4;
#[doc = "SCU3B8 (rw) register accessor: USB Controler Secure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu3b8`] module"]
#[doc(alias = "SCU3B8")]
pub type Scu3b8 = crate::Reg<scu3b8::Scu3b8Spec>;
#[doc = "USB Controler Secure Register 1"]
pub mod scu3b8;
#[doc = "SCU3BC (rw) register accessor: USB Controler Secure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu3bc`] module"]
#[doc(alias = "SCU3BC")]
pub type Scu3bc = crate::Reg<scu3bc::Scu3bcSpec>;
#[doc = "USB Controler Secure Register 2"]
pub mod scu3bc;
#[doc = "SCU3C0 (rw) register accessor: USB Controler Secure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu3c0`] module"]
#[doc(alias = "SCU3C0")]
pub type Scu3c0 = crate::Reg<scu3c0::Scu3c0Spec>;
#[doc = "USB Controler Secure Register 3"]
pub mod scu3c0;
#[doc = "SCU400 (rw) register accessor: Multi-Function Pin Control \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu400::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu400::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu400`] module"]
#[doc(alias = "SCU400")]
pub type Scu400 = crate::Reg<scu400::Scu400Spec>;
#[doc = "Multi-Function Pin Control \\#1"]
pub mod scu400;
#[doc = "SCU404 (rw) register accessor: Multi-Function Pin Control \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu404::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu404::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu404`] module"]
#[doc(alias = "SCU404")]
pub type Scu404 = crate::Reg<scu404::Scu404Spec>;
#[doc = "Multi-Function Pin Control \\#2"]
pub mod scu404;
#[doc = "SCU408 (rw) register accessor: Multi-Function Pin Control \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu408::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu408::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu408`] module"]
#[doc(alias = "SCU408")]
pub type Scu408 = crate::Reg<scu408::Scu408Spec>;
#[doc = "Multi-Function Pin Control \\#3"]
pub mod scu408;
#[doc = "SCU40C (rw) register accessor: Multi-Function Pin Control \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu40c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu40c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu40c`] module"]
#[doc(alias = "SCU40C")]
pub type Scu40c = crate::Reg<scu40c::Scu40cSpec>;
#[doc = "Multi-Function Pin Control \\#4"]
pub mod scu40c;
#[doc = "SCU410 (rw) register accessor: Multi-Function Pin Control \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu410::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu410::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu410`] module"]
#[doc(alias = "SCU410")]
pub type Scu410 = crate::Reg<scu410::Scu410Spec>;
#[doc = "Multi-Function Pin Control \\#5"]
pub mod scu410;
#[doc = "SCU414 (rw) register accessor: Multi-Function Pin Control \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu414::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu414::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu414`] module"]
#[doc(alias = "SCU414")]
pub type Scu414 = crate::Reg<scu414::Scu414Spec>;
#[doc = "Multi-Function Pin Control \\#6"]
pub mod scu414;
#[doc = "SCU418 (rw) register accessor: Multi-Function Pin Control \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu418::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu418::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu418`] module"]
#[doc(alias = "SCU418")]
pub type Scu418 = crate::Reg<scu418::Scu418Spec>;
#[doc = "Multi-Function Pin Control \\#7"]
pub mod scu418;
#[doc = "SCU41C (rw) register accessor: Multi-Function Pin Control \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu41c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu41c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu41c`] module"]
#[doc(alias = "SCU41C")]
pub type Scu41c = crate::Reg<scu41c::Scu41cSpec>;
#[doc = "Multi-Function Pin Control \\#8"]
pub mod scu41c;
#[doc = "SCU420 (rw) register accessor: Multi-Function Pin Control \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu420::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu420::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu420`] module"]
#[doc(alias = "SCU420")]
pub type Scu420 = crate::Reg<scu420::Scu420Spec>;
#[doc = "Multi-Function Pin Control \\#9"]
pub mod scu420;
#[doc = "SCU424 (rw) register accessor: Multi-Function Pin Control \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`scu424::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu424::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu424`] module"]
#[doc(alias = "SCU424")]
pub type Scu424 = crate::Reg<scu424::Scu424Spec>;
#[doc = "Multi-Function Pin Control \\#10"]
pub mod scu424;
#[doc = "SCU428 (rw) register accessor: Multi-Function Pin Control \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu428::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu428::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu428`] module"]
#[doc(alias = "SCU428")]
pub type Scu428 = crate::Reg<scu428::Scu428Spec>;
#[doc = "Multi-Function Pin Control \\#11"]
pub mod scu428;
#[doc = "SCU42C (rw) register accessor: Multi-Function Pin Control \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`scu42c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu42c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu42c`] module"]
#[doc(alias = "SCU42C")]
pub type Scu42c = crate::Reg<scu42c::Scu42cSpec>;
#[doc = "Multi-Function Pin Control \\#12"]
pub mod scu42c;
#[doc = "SCU430 (rw) register accessor: Multi-Function Pin Control \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu430::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu430::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu430`] module"]
#[doc(alias = "SCU430")]
pub type Scu430 = crate::Reg<scu430::Scu430Spec>;
#[doc = "Multi-Function Pin Control \\#13"]
pub mod scu430;
#[doc = "SCU434 (rw) register accessor: Multi-Function Pin Control \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu434::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu434::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu434`] module"]
#[doc(alias = "SCU434")]
pub type Scu434 = crate::Reg<scu434::Scu434Spec>;
#[doc = "Multi-Function Pin Control \\#14"]
pub mod scu434;
#[doc = "SCU438 (rw) register accessor: Multi-Function Pin Control \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`scu438::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu438::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu438`] module"]
#[doc(alias = "SCU438")]
pub type Scu438 = crate::Reg<scu438::Scu438Spec>;
#[doc = "Multi-Function Pin Control \\#15"]
pub mod scu438;
#[doc = "SCU43C (rw) register accessor: Multi-Function Pin Control \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu43c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu43c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu43c`] module"]
#[doc(alias = "SCU43C")]
pub type Scu43c = crate::Reg<scu43c::Scu43cSpec>;
#[doc = "Multi-Function Pin Control \\#16"]
pub mod scu43c;
#[doc = "SCU440 (rw) register accessor: Multi-Function Pin Control \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`scu440::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu440::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu440`] module"]
#[doc(alias = "SCU440")]
pub type Scu440 = crate::Reg<scu440::Scu440Spec>;
#[doc = "Multi-Function Pin Control \\#17"]
pub mod scu440;
#[doc = "SCU444 (rw) register accessor: Multi-Function Pin Control \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`scu444::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu444::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu444`] module"]
#[doc(alias = "SCU444")]
pub type Scu444 = crate::Reg<scu444::Scu444Spec>;
#[doc = "Multi-Function Pin Control \\#18"]
pub mod scu444;
#[doc = "SCU448 (rw) register accessor: Multi-Function Pin Control \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu448::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu448::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu448`] module"]
#[doc(alias = "SCU448")]
pub type Scu448 = crate::Reg<scu448::Scu448Spec>;
#[doc = "Multi-Function Pin Control \\#19"]
pub mod scu448;
#[doc = "SCU44C (rw) register accessor: Multi-Function Pin Control \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`scu44c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu44c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu44c`] module"]
#[doc(alias = "SCU44C")]
pub type Scu44c = crate::Reg<scu44c::Scu44cSpec>;
#[doc = "Multi-Function Pin Control \\#20"]
pub mod scu44c;
#[doc = "SCU450 (rw) register accessor: Multi-Function Pin Control \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scu450::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu450::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu450`] module"]
#[doc(alias = "SCU450")]
pub type Scu450 = crate::Reg<scu450::Scu450Spec>;
#[doc = "Multi-Function Pin Control \\#21"]
pub mod scu450;
#[doc = "SCU454 (rw) register accessor: Multi-Function Pin Control \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`scu454::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu454::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu454`] module"]
#[doc(alias = "SCU454")]
pub type Scu454 = crate::Reg<scu454::Scu454Spec>;
#[doc = "Multi-Function Pin Control \\#22"]
pub mod scu454;
#[doc = "SCU458 (rw) register accessor: Multi-Function Pin Control \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu458::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu458::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu458`] module"]
#[doc(alias = "SCU458")]
pub type Scu458 = crate::Reg<scu458::Scu458Spec>;
#[doc = "Multi-Function Pin Control \\#23"]
pub mod scu458;
#[doc = "SCU45C (rw) register accessor: Multi-Function Pin Control \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu45c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu45c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu45c`] module"]
#[doc(alias = "SCU45C")]
pub type Scu45c = crate::Reg<scu45c::Scu45cSpec>;
#[doc = "Multi-Function Pin Control \\#24"]
pub mod scu45c;
#[doc = "SCU460 (rw) register accessor: Multi-Function Pin Control \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu460::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu460::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu460`] module"]
#[doc(alias = "SCU460")]
pub type Scu460 = crate::Reg<scu460::Scu460Spec>;
#[doc = "Multi-Function Pin Control \\#25"]
pub mod scu460;
#[doc = "SCU480 (rw) register accessor: IO Control \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu480::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu480::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu480`] module"]
#[doc(alias = "SCU480")]
pub type Scu480 = crate::Reg<scu480::Scu480Spec>;
#[doc = "IO Control \\#1"]
pub mod scu480;
#[doc = "SCU484 (rw) register accessor: IO Control \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu484::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu484::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu484`] module"]
#[doc(alias = "SCU484")]
pub type Scu484 = crate::Reg<scu484::Scu484Spec>;
#[doc = "IO Control \\#2"]
pub mod scu484;
#[doc = "SCU488 (rw) register accessor: IO Control \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu488::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu488::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu488`] module"]
#[doc(alias = "SCU488")]
pub type Scu488 = crate::Reg<scu488::Scu488Spec>;
#[doc = "IO Control \\#3"]
pub mod scu488;
#[doc = "SCU48C (rw) register accessor: IO Control \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu48c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu48c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu48c`] module"]
#[doc(alias = "SCU48C")]
pub type Scu48c = crate::Reg<scu48c::Scu48cSpec>;
#[doc = "IO Control \\#4"]
pub mod scu48c;
#[doc = "SCU490 (rw) register accessor: IO Control \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu490::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu490::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu490`] module"]
#[doc(alias = "SCU490")]
pub type Scu490 = crate::Reg<scu490::Scu490Spec>;
#[doc = "IO Control \\#5"]
pub mod scu490;
#[doc = "SCU494 (rw) register accessor: IO Control \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu494::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu494::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu494`] module"]
#[doc(alias = "SCU494")]
pub type Scu494 = crate::Reg<scu494::Scu494Spec>;
#[doc = "IO Control \\#6"]
pub mod scu494;
#[doc = "SCU498 (rw) register accessor: IO Control \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu498::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu498::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu498`] module"]
#[doc(alias = "SCU498")]
pub type Scu498 = crate::Reg<scu498::Scu498Spec>;
#[doc = "IO Control \\#7"]
pub mod scu498;
#[doc = "SCU49C (rw) register accessor: IO Control \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu49c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu49c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu49c`] module"]
#[doc(alias = "SCU49C")]
pub type Scu49c = crate::Reg<scu49c::Scu49cSpec>;
#[doc = "IO Control \\#8"]
pub mod scu49c;
#[doc = "SCU4A0 (rw) register accessor: IO Control \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4a0`] module"]
#[doc(alias = "SCU4A0")]
pub type Scu4a0 = crate::Reg<scu4a0::Scu4a0Spec>;
#[doc = "IO Control \\#9"]
pub mod scu4a0;
#[doc = "SCU4A4 (rw) register accessor: IO Control \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4a4`] module"]
#[doc(alias = "SCU4A4")]
pub type Scu4a4 = crate::Reg<scu4a4::Scu4a4Spec>;
#[doc = "IO Control \\#10"]
pub mod scu4a4;
#[doc = "SCU4A8 (rw) register accessor: IO Control \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4a8`] module"]
#[doc(alias = "SCU4A8")]
pub type Scu4a8 = crate::Reg<scu4a8::Scu4a8Spec>;
#[doc = "IO Control \\#11"]
pub mod scu4a8;
#[doc = "SCU4AC (rw) register accessor: IO Control \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4ac`] module"]
#[doc(alias = "SCU4AC")]
pub type Scu4ac = crate::Reg<scu4ac::Scu4acSpec>;
#[doc = "IO Control \\#12"]
pub mod scu4ac;
#[doc = "SCU4B0 (rw) register accessor: IO Control \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4b0`] module"]
#[doc(alias = "SCU4B0")]
pub type Scu4b0 = crate::Reg<scu4b0::Scu4b0Spec>;
#[doc = "IO Control \\#13"]
pub mod scu4b0;
#[doc = "SCU4B4 (rw) register accessor: IO Control \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4b4`] module"]
#[doc(alias = "SCU4B4")]
pub type Scu4b4 = crate::Reg<scu4b4::Scu4b4Spec>;
#[doc = "IO Control \\#14"]
pub mod scu4b4;
#[doc = "SCU4B8 (rw) register accessor: IO Control \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4b8`] module"]
#[doc(alias = "SCU4B8")]
pub type Scu4b8 = crate::Reg<scu4b8::Scu4b8Spec>;
#[doc = "IO Control \\#15"]
pub mod scu4b8;
#[doc = "SCU4BC (rw) register accessor: IO Control \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4bc`] module"]
#[doc(alias = "SCU4BC")]
pub type Scu4bc = crate::Reg<scu4bc::Scu4bcSpec>;
#[doc = "IO Control \\#16"]
pub mod scu4bc;
#[doc = "SCU4C0 (rw) register accessor: IO Control \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4c0`] module"]
#[doc(alias = "SCU4C0")]
pub type Scu4c0 = crate::Reg<scu4c0::Scu4c0Spec>;
#[doc = "IO Control \\#17"]
pub mod scu4c0;
#[doc = "SCU4C4 (rw) register accessor: IO Control \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4c4`] module"]
#[doc(alias = "SCU4C4")]
pub type Scu4c4 = crate::Reg<scu4c4::Scu4c4Spec>;
#[doc = "IO Control \\#18"]
pub mod scu4c4;
#[doc = "SCU4C8 (rw) register accessor: IO Control \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4c8`] module"]
#[doc(alias = "SCU4C8")]
pub type Scu4c8 = crate::Reg<scu4c8::Scu4c8Spec>;
#[doc = "IO Control \\#19"]
pub mod scu4c8;
#[doc = "SCU4CC (rw) register accessor: IO Control \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4cc`] module"]
#[doc(alias = "SCU4CC")]
pub type Scu4cc = crate::Reg<scu4cc::Scu4ccSpec>;
#[doc = "IO Control \\#20"]
pub mod scu4cc;
#[doc = "SCU4D0 (rw) register accessor: IO Control \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4d0`] module"]
#[doc(alias = "SCU4D0")]
pub type Scu4d0 = crate::Reg<scu4d0::Scu4d0Spec>;
#[doc = "IO Control \\#21"]
pub mod scu4d0;
#[doc = "SCU4D4 (rw) register accessor: IO Control \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4d4`] module"]
#[doc(alias = "SCU4D4")]
pub type Scu4d4 = crate::Reg<scu4d4::Scu4d4Spec>;
#[doc = "IO Control \\#22"]
pub mod scu4d4;
#[doc = "SCU4D8 (rw) register accessor: IO Control \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4d8`] module"]
#[doc(alias = "SCU4D8")]
pub type Scu4d8 = crate::Reg<scu4d8::Scu4d8Spec>;
#[doc = "IO Control \\#23"]
pub mod scu4d8;
#[doc = "SCU4DC (rw) register accessor: IO Control \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4dc`] module"]
#[doc(alias = "SCU4DC")]
pub type Scu4dc = crate::Reg<scu4dc::Scu4dcSpec>;
#[doc = "IO Control \\#24"]
pub mod scu4dc;
#[doc = "SCU4E0 (rw) register accessor: IO Control \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4e0`] module"]
#[doc(alias = "SCU4E0")]
pub type Scu4e0 = crate::Reg<scu4e0::Scu4e0Spec>;
#[doc = "IO Control \\#25"]
pub mod scu4e0;
#[doc = "SCU4E4 (rw) register accessor: IO Control \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4e4`] module"]
#[doc(alias = "SCU4E4")]
pub type Scu4e4 = crate::Reg<scu4e4::Scu4e4Spec>;
#[doc = "IO Control \\#26"]
pub mod scu4e4;
#[doc = "SCU4E8 (rw) register accessor: IO Control \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4e8`] module"]
#[doc(alias = "SCU4E8")]
pub type Scu4e8 = crate::Reg<scu4e8::Scu4e8Spec>;
#[doc = "IO Control \\#27"]
pub mod scu4e8;
#[doc = "SCU4EC (rw) register accessor: IO Control \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4ec`] module"]
#[doc(alias = "SCU4EC")]
pub type Scu4ec = crate::Reg<scu4ec::Scu4ecSpec>;
#[doc = "IO Control \\#28"]
pub mod scu4ec;
#[doc = "SCU4F0 (rw) register accessor: IO Control \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4f0`] module"]
#[doc(alias = "SCU4F0")]
pub type Scu4f0 = crate::Reg<scu4f0::Scu4f0Spec>;
#[doc = "IO Control \\#29"]
pub mod scu4f0;
#[doc = "SCU4F4 (rw) register accessor: IO Control \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4f4`] module"]
#[doc(alias = "SCU4F4")]
pub type Scu4f4 = crate::Reg<scu4f4::Scu4f4Spec>;
#[doc = "IO Control \\#30"]
pub mod scu4f4;
#[doc = "SCU4F8 (rw) register accessor: IO Control \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4f8`] module"]
#[doc(alias = "SCU4F8")]
pub type Scu4f8 = crate::Reg<scu4f8::Scu4f8Spec>;
#[doc = "IO Control \\#31"]
pub mod scu4f8;
#[doc = "SCU4FC (rw) register accessor: IO Control \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4fc`] module"]
#[doc(alias = "SCU4FC")]
pub type Scu4fc = crate::Reg<scu4fc::Scu4fcSpec>;
#[doc = "IO Control \\#32"]
pub mod scu4fc;
#[doc = "SCU500 (rw) register accessor: IO Control \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`scu500::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu500::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu500`] module"]
#[doc(alias = "SCU500")]
pub type Scu500 = crate::Reg<scu500::Scu500Spec>;
#[doc = "IO Control \\#33"]
pub mod scu500;
#[doc = "SCU504 (rw) register accessor: IO Control \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`scu504::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu504::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu504`] module"]
#[doc(alias = "SCU504")]
pub type Scu504 = crate::Reg<scu504::Scu504Spec>;
#[doc = "IO Control \\#34"]
pub mod scu504;
#[doc = "SCU508 (rw) register accessor: IO Control \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`scu508::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu508::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu508`] module"]
#[doc(alias = "SCU508")]
pub type Scu508 = crate::Reg<scu508::Scu508Spec>;
#[doc = "IO Control \\#35"]
pub mod scu508;
#[doc = "SCU50C (rw) register accessor: IO Control \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`scu50c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu50c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu50c`] module"]
#[doc(alias = "SCU50C")]
pub type Scu50c = crate::Reg<scu50c::Scu50cSpec>;
#[doc = "IO Control \\#36"]
pub mod scu50c;
#[doc = "SCU510 (rw) register accessor: IO Control \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`scu510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu510`] module"]
#[doc(alias = "SCU510")]
pub type Scu510 = crate::Reg<scu510::Scu510Spec>;
#[doc = "IO Control \\#37"]
pub mod scu510;
#[doc = "SCU514 (rw) register accessor: IO Control \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`scu514::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu514::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu514`] module"]
#[doc(alias = "SCU514")]
pub type Scu514 = crate::Reg<scu514::Scu514Spec>;
#[doc = "IO Control \\#38"]
pub mod scu514;
#[doc = "SCU518 (rw) register accessor: IO Control \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`scu518::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu518::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu518`] module"]
#[doc(alias = "SCU518")]
pub type Scu518 = crate::Reg<scu518::Scu518Spec>;
#[doc = "IO Control \\#39"]
pub mod scu518;
#[doc = "SCU51C (rw) register accessor: IO Control \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`scu51c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu51c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu51c`] module"]
#[doc(alias = "SCU51C")]
pub type Scu51c = crate::Reg<scu51c::Scu51cSpec>;
#[doc = "IO Control \\#40"]
pub mod scu51c;
#[doc = "SCU520 (rw) register accessor: IO Control \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`scu520::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu520::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu520`] module"]
#[doc(alias = "SCU520")]
pub type Scu520 = crate::Reg<scu520::Scu520Spec>;
#[doc = "IO Control \\#41"]
pub mod scu520;
#[doc = "SCU524 (rw) register accessor: IO Control \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`scu524::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu524::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu524`] module"]
#[doc(alias = "SCU524")]
pub type Scu524 = crate::Reg<scu524::Scu524Spec>;
#[doc = "IO Control \\#42"]
pub mod scu524;
#[doc = "SCU528 (rw) register accessor: IO Control \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`scu528::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu528::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu528`] module"]
#[doc(alias = "SCU528")]
pub type Scu528 = crate::Reg<scu528::Scu528Spec>;
#[doc = "IO Control \\#43"]
pub mod scu528;
#[doc = "SCU52C (rw) register accessor: IO Control \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`scu52c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu52c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu52c`] module"]
#[doc(alias = "SCU52C")]
pub type Scu52c = crate::Reg<scu52c::Scu52cSpec>;
#[doc = "IO Control \\#44"]
pub mod scu52c;
#[doc = "SCU530 (rw) register accessor: IO Control \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`scu530::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu530::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu530`] module"]
#[doc(alias = "SCU530")]
pub type Scu530 = crate::Reg<scu530::Scu530Spec>;
#[doc = "IO Control \\#45"]
pub mod scu530;
#[doc = "SCU534 (rw) register accessor: IO Control \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`scu534::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu534::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu534`] module"]
#[doc(alias = "SCU534")]
pub type Scu534 = crate::Reg<scu534::Scu534Spec>;
#[doc = "IO Control \\#46"]
pub mod scu534;
#[doc = "SCU538 (rw) register accessor: IO Control \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`scu538::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu538::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu538`] module"]
#[doc(alias = "SCU538")]
pub type Scu538 = crate::Reg<scu538::Scu538Spec>;
#[doc = "IO Control \\#47"]
pub mod scu538;
#[doc = "SCU53C (rw) register accessor: IO Control \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`scu53c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu53c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu53c`] module"]
#[doc(alias = "SCU53C")]
pub type Scu53c = crate::Reg<scu53c::Scu53cSpec>;
#[doc = "IO Control \\#48"]
pub mod scu53c;
#[doc = "SCU540 (rw) register accessor: IO Control \\#49\n\nYou can [`read`](crate::Reg::read) this register and get [`scu540::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu540::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu540`] module"]
#[doc(alias = "SCU540")]
pub type Scu540 = crate::Reg<scu540::Scu540Spec>;
#[doc = "IO Control \\#49"]
pub mod scu540;
#[doc = "SCU544 (rw) register accessor: IO Control \\#50\n\nYou can [`read`](crate::Reg::read) this register and get [`scu544::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu544::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu544`] module"]
#[doc(alias = "SCU544")]
pub type Scu544 = crate::Reg<scu544::Scu544Spec>;
#[doc = "IO Control \\#50"]
pub mod scu544;
#[doc = "SCU548 (rw) register accessor: IO Control \\#51\n\nYou can [`read`](crate::Reg::read) this register and get [`scu548::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu548::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu548`] module"]
#[doc(alias = "SCU548")]
pub type Scu548 = crate::Reg<scu548::Scu548Spec>;
#[doc = "IO Control \\#51"]
pub mod scu548;
#[doc = "SCU54C (rw) register accessor: IO Control \\#52\n\nYou can [`read`](crate::Reg::read) this register and get [`scu54c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu54c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu54c`] module"]
#[doc(alias = "SCU54C")]
pub type Scu54c = crate::Reg<scu54c::Scu54cSpec>;
#[doc = "IO Control \\#52"]
pub mod scu54c;
#[doc = "SCU550 (rw) register accessor: IO Control \\#53\n\nYou can [`read`](crate::Reg::read) this register and get [`scu550::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu550::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu550`] module"]
#[doc(alias = "SCU550")]
pub type Scu550 = crate::Reg<scu550::Scu550Spec>;
#[doc = "IO Control \\#53"]
pub mod scu550;
#[doc = "SCU554 (rw) register accessor: IO Control \\#54\n\nYou can [`read`](crate::Reg::read) this register and get [`scu554::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu554::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu554`] module"]
#[doc(alias = "SCU554")]
pub type Scu554 = crate::Reg<scu554::Scu554Spec>;
#[doc = "IO Control \\#54"]
pub mod scu554;
#[doc = "SCU558 (rw) register accessor: IO Control \\#55\n\nYou can [`read`](crate::Reg::read) this register and get [`scu558::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu558::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu558`] module"]
#[doc(alias = "SCU558")]
pub type Scu558 = crate::Reg<scu558::Scu558Spec>;
#[doc = "IO Control \\#55"]
pub mod scu558;
#[doc = "SCU55C (rw) register accessor: IO Control \\#56\n\nYou can [`read`](crate::Reg::read) this register and get [`scu55c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu55c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu55c`] module"]
#[doc(alias = "SCU55C")]
pub type Scu55c = crate::Reg<scu55c::Scu55cSpec>;
#[doc = "IO Control \\#56"]
pub mod scu55c;
#[doc = "SCU560 (rw) register accessor: IO Control \\#57\n\nYou can [`read`](crate::Reg::read) this register and get [`scu560::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu560::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu560`] module"]
#[doc(alias = "SCU560")]
pub type Scu560 = crate::Reg<scu560::Scu560Spec>;
#[doc = "IO Control \\#57"]
pub mod scu560;
#[doc = "SCU564 (rw) register accessor: IO Control \\#58\n\nYou can [`read`](crate::Reg::read) this register and get [`scu564::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu564::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu564`] module"]
#[doc(alias = "SCU564")]
pub type Scu564 = crate::Reg<scu564::Scu564Spec>;
#[doc = "IO Control \\#58"]
pub mod scu564;
#[doc = "SCU568 (rw) register accessor: IO Control \\#59\n\nYou can [`read`](crate::Reg::read) this register and get [`scu568::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu568::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu568`] module"]
#[doc(alias = "SCU568")]
pub type Scu568 = crate::Reg<scu568::Scu568Spec>;
#[doc = "IO Control \\#59"]
pub mod scu568;
#[doc = "SCU56C (rw) register accessor: IO Control \\#60\n\nYou can [`read`](crate::Reg::read) this register and get [`scu56c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu56c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu56c`] module"]
#[doc(alias = "SCU56C")]
pub type Scu56c = crate::Reg<scu56c::Scu56cSpec>;
#[doc = "IO Control \\#60"]
pub mod scu56c;
#[doc = "SCU570 (rw) register accessor: IO Control \\#61\n\nYou can [`read`](crate::Reg::read) this register and get [`scu570::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu570::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu570`] module"]
#[doc(alias = "SCU570")]
pub type Scu570 = crate::Reg<scu570::Scu570Spec>;
#[doc = "IO Control \\#61"]
pub mod scu570;
#[doc = "SCU574 (rw) register accessor: IO Control \\#62\n\nYou can [`read`](crate::Reg::read) this register and get [`scu574::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu574::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu574`] module"]
#[doc(alias = "SCU574")]
pub type Scu574 = crate::Reg<scu574::Scu574Spec>;
#[doc = "IO Control \\#62"]
pub mod scu574;
#[doc = "SCU578 (rw) register accessor: IO Control \\#63\n\nYou can [`read`](crate::Reg::read) this register and get [`scu578::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu578::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu578`] module"]
#[doc(alias = "SCU578")]
pub type Scu578 = crate::Reg<scu578::Scu578Spec>;
#[doc = "IO Control \\#63"]
pub mod scu578;
#[doc = "SCU57C (rw) register accessor: IO Control \\#64\n\nYou can [`read`](crate::Reg::read) this register and get [`scu57c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu57c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu57c`] module"]
#[doc(alias = "SCU57C")]
pub type Scu57c = crate::Reg<scu57c::Scu57cSpec>;
#[doc = "IO Control \\#64"]
pub mod scu57c;
#[doc = "SCU580 (rw) register accessor: IO Control \\#65\n\nYou can [`read`](crate::Reg::read) this register and get [`scu580::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu580::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu580`] module"]
#[doc(alias = "SCU580")]
pub type Scu580 = crate::Reg<scu580::Scu580Spec>;
#[doc = "IO Control \\#65"]
pub mod scu580;
#[doc = "SCU584 (rw) register accessor: IO Control \\#66\n\nYou can [`read`](crate::Reg::read) this register and get [`scu584::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu584::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu584`] module"]
#[doc(alias = "SCU584")]
pub type Scu584 = crate::Reg<scu584::Scu584Spec>;
#[doc = "IO Control \\#66"]
pub mod scu584;
#[doc = "SCU588 (rw) register accessor: IO Control \\#67\n\nYou can [`read`](crate::Reg::read) this register and get [`scu588::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu588::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu588`] module"]
#[doc(alias = "SCU588")]
pub type Scu588 = crate::Reg<scu588::Scu588Spec>;
#[doc = "IO Control \\#67"]
pub mod scu588;
#[doc = "SCU58C (rw) register accessor: IO Control \\#68\n\nYou can [`read`](crate::Reg::read) this register and get [`scu58c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu58c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu58c`] module"]
#[doc(alias = "SCU58C")]
pub type Scu58c = crate::Reg<scu58c::Scu58cSpec>;
#[doc = "IO Control \\#68"]
pub mod scu58c;
#[doc = "SCU590 (rw) register accessor: IO Control \\#69\n\nYou can [`read`](crate::Reg::read) this register and get [`scu590::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu590::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu590`] module"]
#[doc(alias = "SCU590")]
pub type Scu590 = crate::Reg<scu590::Scu590Spec>;
#[doc = "IO Control \\#69"]
pub mod scu590;
#[doc = "SCU594 (rw) register accessor: IO Control \\#70\n\nYou can [`read`](crate::Reg::read) this register and get [`scu594::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu594::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu594`] module"]
#[doc(alias = "SCU594")]
pub type Scu594 = crate::Reg<scu594::Scu594Spec>;
#[doc = "IO Control \\#70"]
pub mod scu594;
#[doc = "SCU598 (rw) register accessor: IO Control \\#71\n\nYou can [`read`](crate::Reg::read) this register and get [`scu598::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu598::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu598`] module"]
#[doc(alias = "SCU598")]
pub type Scu598 = crate::Reg<scu598::Scu598Spec>;
#[doc = "IO Control \\#71"]
pub mod scu598;
#[doc = "SCU59C (rw) register accessor: IO Control \\#72\n\nYou can [`read`](crate::Reg::read) this register and get [`scu59c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu59c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu59c`] module"]
#[doc(alias = "SCU59C")]
pub type Scu59c = crate::Reg<scu59c::Scu59cSpec>;
#[doc = "IO Control \\#72"]
pub mod scu59c;
#[doc = "SCU5A0 (rw) register accessor: IO Control \\#73\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5a0`] module"]
#[doc(alias = "SCU5A0")]
pub type Scu5a0 = crate::Reg<scu5a0::Scu5a0Spec>;
#[doc = "IO Control \\#73"]
pub mod scu5a0;
#[doc = "SCU5A4 (rw) register accessor: IO Control \\#74\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5a4`] module"]
#[doc(alias = "SCU5A4")]
pub type Scu5a4 = crate::Reg<scu5a4::Scu5a4Spec>;
#[doc = "IO Control \\#74"]
pub mod scu5a4;
#[doc = "SCU5A8 (rw) register accessor: IO Control \\#75\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5a8`] module"]
#[doc(alias = "SCU5A8")]
pub type Scu5a8 = crate::Reg<scu5a8::Scu5a8Spec>;
#[doc = "IO Control \\#75"]
pub mod scu5a8;
#[doc = "SCU5AC (rw) register accessor: IO Control \\#76\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5ac`] module"]
#[doc(alias = "SCU5AC")]
pub type Scu5ac = crate::Reg<scu5ac::Scu5acSpec>;
#[doc = "IO Control \\#76"]
pub mod scu5ac;
#[doc = "SCU5B0 (rw) register accessor: IO Control \\#77\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5b0`] module"]
#[doc(alias = "SCU5B0")]
pub type Scu5b0 = crate::Reg<scu5b0::Scu5b0Spec>;
#[doc = "IO Control \\#77"]
pub mod scu5b0;
#[doc = "SCU5B4 (rw) register accessor: IO Control \\#78\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5b4`] module"]
#[doc(alias = "SCU5B4")]
pub type Scu5b4 = crate::Reg<scu5b4::Scu5b4Spec>;
#[doc = "IO Control \\#78"]
pub mod scu5b4;
#[doc = "SCU5B8 (rw) register accessor: IO Control \\#79\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5b8`] module"]
#[doc(alias = "SCU5B8")]
pub type Scu5b8 = crate::Reg<scu5b8::Scu5b8Spec>;
#[doc = "IO Control \\#79"]
pub mod scu5b8;
#[doc = "SCU5BC (rw) register accessor: IO Control \\#80\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5bc`] module"]
#[doc(alias = "SCU5BC")]
pub type Scu5bc = crate::Reg<scu5bc::Scu5bcSpec>;
#[doc = "IO Control \\#80"]
pub mod scu5bc;
#[doc = "SCU5C0 (rw) register accessor: IO Control \\#81\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5c0`] module"]
#[doc(alias = "SCU5C0")]
pub type Scu5c0 = crate::Reg<scu5c0::Scu5c0Spec>;
#[doc = "IO Control \\#81"]
pub mod scu5c0;
#[doc = "SCU5C4 (rw) register accessor: IO Control \\#82\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5c4`] module"]
#[doc(alias = "SCU5C4")]
pub type Scu5c4 = crate::Reg<scu5c4::Scu5c4Spec>;
#[doc = "IO Control \\#82"]
pub mod scu5c4;
#[doc = "SCU5C8 (rw) register accessor: IO Control \\#83\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5c8`] module"]
#[doc(alias = "SCU5C8")]
pub type Scu5c8 = crate::Reg<scu5c8::Scu5c8Spec>;
#[doc = "IO Control \\#83"]
pub mod scu5c8;
#[doc = "SCU5CC (rw) register accessor: IO Control \\#84\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5cc`] module"]
#[doc(alias = "SCU5CC")]
pub type Scu5cc = crate::Reg<scu5cc::Scu5ccSpec>;
#[doc = "IO Control \\#84"]
pub mod scu5cc;
#[doc = "SCU5D0 (rw) register accessor: IO Control \\#85\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5d0`] module"]
#[doc(alias = "SCU5D0")]
pub type Scu5d0 = crate::Reg<scu5d0::Scu5d0Spec>;
#[doc = "IO Control \\#85"]
pub mod scu5d0;
#[doc = "SCU5D4 (rw) register accessor: IO Control \\#86\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5d4`] module"]
#[doc(alias = "SCU5D4")]
pub type Scu5d4 = crate::Reg<scu5d4::Scu5d4Spec>;
#[doc = "IO Control \\#86"]
pub mod scu5d4;
#[doc = "SCU5D8 (rw) register accessor: IO Control \\#87\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5d8`] module"]
#[doc(alias = "SCU5D8")]
pub type Scu5d8 = crate::Reg<scu5d8::Scu5d8Spec>;
#[doc = "IO Control \\#87"]
pub mod scu5d8;
#[doc = "SCU5DC (rw) register accessor: IO Control \\#88\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5dc`] module"]
#[doc(alias = "SCU5DC")]
pub type Scu5dc = crate::Reg<scu5dc::Scu5dcSpec>;
#[doc = "IO Control \\#88"]
pub mod scu5dc;
#[doc = "SCU5E0 (rw) register accessor: IO Control \\#89\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5e0`] module"]
#[doc(alias = "SCU5E0")]
pub type Scu5e0 = crate::Reg<scu5e0::Scu5e0Spec>;
#[doc = "IO Control \\#89"]
pub mod scu5e0;
#[doc = "SCU5E4 (rw) register accessor: IO Control \\#90\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5e4`] module"]
#[doc(alias = "SCU5E4")]
pub type Scu5e4 = crate::Reg<scu5e4::Scu5e4Spec>;
#[doc = "IO Control \\#90"]
pub mod scu5e4;
#[doc = "SCU5E8 (rw) register accessor: IO Control \\#91\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5e8`] module"]
#[doc(alias = "SCU5E8")]
pub type Scu5e8 = crate::Reg<scu5e8::Scu5e8Spec>;
#[doc = "IO Control \\#91"]
pub mod scu5e8;
#[doc = "SCU5EC (rw) register accessor: IO Control \\#92\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5ec`] module"]
#[doc(alias = "SCU5EC")]
pub type Scu5ec = crate::Reg<scu5ec::Scu5ecSpec>;
#[doc = "IO Control \\#92"]
pub mod scu5ec;
#[doc = "SCU5F0 (rw) register accessor: IO Control \\#93\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5f0`] module"]
#[doc(alias = "SCU5F0")]
pub type Scu5f0 = crate::Reg<scu5f0::Scu5f0Spec>;
#[doc = "IO Control \\#93"]
pub mod scu5f0;
#[doc = "SCU5F4 (rw) register accessor: IO Control \\#94\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5f4`] module"]
#[doc(alias = "SCU5F4")]
pub type Scu5f4 = crate::Reg<scu5f4::Scu5f4Spec>;
#[doc = "IO Control \\#94"]
pub mod scu5f4;
#[doc = "SCU5F8 (rw) register accessor: IO Control \\#95\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5f8`] module"]
#[doc(alias = "SCU5F8")]
pub type Scu5f8 = crate::Reg<scu5f8::Scu5f8Spec>;
#[doc = "IO Control \\#95"]
pub mod scu5f8;
#[doc = "SCU5FC (rw) register accessor: IO Control \\#96\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5fc`] module"]
#[doc(alias = "SCU5FC")]
pub type Scu5fc = crate::Reg<scu5fc::Scu5fcSpec>;
#[doc = "IO Control \\#96"]
pub mod scu5fc;
#[doc = "SCU600 (rw) register accessor: IO Control \\#97\n\nYou can [`read`](crate::Reg::read) this register and get [`scu600::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu600::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu600`] module"]
#[doc(alias = "SCU600")]
pub type Scu600 = crate::Reg<scu600::Scu600Spec>;
#[doc = "IO Control \\#97"]
pub mod scu600;
#[doc = "SCU680 (rw) register accessor: IO Control 0 Regiser\n\nYou can [`read`](crate::Reg::read) this register and get [`scu680::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu680::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu680`] module"]
#[doc(alias = "SCU680")]
pub type Scu680 = crate::Reg<scu680::Scu680Spec>;
#[doc = "IO Control 0 Regiser"]
pub mod scu680;
#[doc = "SCU684 (rw) register accessor: HRAM IO Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu684::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu684::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu684`] module"]
#[doc(alias = "SCU684")]
pub type Scu684 = crate::Reg<scu684::Scu684Spec>;
#[doc = "HRAM IO Control 1 Register"]
pub mod scu684;
#[doc = "SCU688 (rw) register accessor: HRAM IO Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu688::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu688::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu688`] module"]
#[doc(alias = "SCU688")]
pub type Scu688 = crate::Reg<scu688::Scu688Spec>;
#[doc = "HRAM IO Control 2 Register"]
pub mod scu688;
#[doc = "SCU68C (rw) register accessor: HRAM IO Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu68c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu68c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu68c`] module"]
#[doc(alias = "SCU68C")]
pub type Scu68c = crate::Reg<scu68c::Scu68cSpec>;
#[doc = "HRAM IO Control 3 Register"]
pub mod scu68c;
#[doc = "SCU6FC (rw) register accessor: GPIO Passthrough Debounce Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu6fc`] module"]
#[doc(alias = "SCU6FC")]
pub type Scu6fc = crate::Reg<scu6fc::Scu6fcSpec>;
#[doc = "GPIO Passthrough Debounce Register"]
pub mod scu6fc;
#[doc = "SCU700 (rw) register accessor: IO Lock 1 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu700::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu700::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu700`] module"]
#[doc(alias = "SCU700")]
pub type Scu700 = crate::Reg<scu700::Scu700Spec>;
#[doc = "IO Lock 1 Register 1"]
pub mod scu700;
#[doc = "SCU704 (rw) register accessor: IO Lock Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu704::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu704::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu704`] module"]
#[doc(alias = "SCU704")]
pub type Scu704 = crate::Reg<scu704::Scu704Spec>;
#[doc = "IO Lock Register 2"]
pub mod scu704;
#[doc = "SCU708 (rw) register accessor: IO Lock Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu708::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu708::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu708`] module"]
#[doc(alias = "SCU708")]
pub type Scu708 = crate::Reg<scu708::Scu708Spec>;
#[doc = "IO Lock Register 3"]
pub mod scu708;
#[doc = "SCU70C (rw) register accessor: IO Lock Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu70c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu70c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu70c`] module"]
#[doc(alias = "SCU70C")]
pub type Scu70c = crate::Reg<scu70c::Scu70cSpec>;
#[doc = "IO Lock Register 4"]
pub mod scu70c;
#[doc = "SCU710 (rw) register accessor: IO Lock Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu710::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu710::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu710`] module"]
#[doc(alias = "SCU710")]
pub type Scu710 = crate::Reg<scu710::Scu710Spec>;
#[doc = "IO Lock Register 5"]
pub mod scu710;
#[doc = "SCU714 (rw) register accessor: IO Lock Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu714::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu714::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu714`] module"]
#[doc(alias = "SCU714")]
pub type Scu714 = crate::Reg<scu714::Scu714Spec>;
#[doc = "IO Lock Register 6"]
pub mod scu714;
#[doc = "SCU718 (rw) register accessor: IO Lock Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu718::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu718::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu718`] module"]
#[doc(alias = "SCU718")]
pub type Scu718 = crate::Reg<scu718::Scu718Spec>;
#[doc = "IO Lock Register 7"]
pub mod scu718;
#[doc = "SCU740 (rw) register accessor: IO Secure 1 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu740::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu740::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu740`] module"]
#[doc(alias = "SCU740")]
pub type Scu740 = crate::Reg<scu740::Scu740Spec>;
#[doc = "IO Secure 1 Register 1"]
pub mod scu740;
#[doc = "SCU744 (rw) register accessor: IO Secure 1 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu744::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu744::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu744`] module"]
#[doc(alias = "SCU744")]
pub type Scu744 = crate::Reg<scu744::Scu744Spec>;
#[doc = "IO Secure 1 Register 2"]
pub mod scu744;
#[doc = "SCU748 (rw) register accessor: IO Secure 1 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu748::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu748::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu748`] module"]
#[doc(alias = "SCU748")]
pub type Scu748 = crate::Reg<scu748::Scu748Spec>;
#[doc = "IO Secure 1 Register 3"]
pub mod scu748;
#[doc = "SCU74C (rw) register accessor: IO Secure 1 Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu74c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu74c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu74c`] module"]
#[doc(alias = "SCU74C")]
pub type Scu74c = crate::Reg<scu74c::Scu74cSpec>;
#[doc = "IO Secure 1 Register 4"]
pub mod scu74c;
#[doc = "SCU750 (rw) register accessor: IO Secure 1 Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu750::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu750::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu750`] module"]
#[doc(alias = "SCU750")]
pub type Scu750 = crate::Reg<scu750::Scu750Spec>;
#[doc = "IO Secure 1 Register 5"]
pub mod scu750;
#[doc = "SCU754 (rw) register accessor: IO Secure 1 Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu754::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu754::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu754`] module"]
#[doc(alias = "SCU754")]
pub type Scu754 = crate::Reg<scu754::Scu754Spec>;
#[doc = "IO Secure 1 Register 6"]
pub mod scu754;
#[doc = "SCU758 (rw) register accessor: IO Secure 1 Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu758::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu758::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu758`] module"]
#[doc(alias = "SCU758")]
pub type Scu758 = crate::Reg<scu758::Scu758Spec>;
#[doc = "IO Secure 1 Register 7"]
pub mod scu758;
#[doc = "SCU780 (rw) register accessor: IO SECURE 2 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu780::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu780::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu780`] module"]
#[doc(alias = "SCU780")]
pub type Scu780 = crate::Reg<scu780::Scu780Spec>;
#[doc = "IO SECURE 2 Register 1"]
pub mod scu780;
#[doc = "SCU784 (rw) register accessor: IO SECURE 2 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu784::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu784::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu784`] module"]
#[doc(alias = "SCU784")]
pub type Scu784 = crate::Reg<scu784::Scu784Spec>;
#[doc = "IO SECURE 2 Register 2"]
pub mod scu784;
#[doc = "SCU788 (rw) register accessor: IO SECURE 2 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu788::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu788::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu788`] module"]
#[doc(alias = "SCU788")]
pub type Scu788 = crate::Reg<scu788::Scu788Spec>;
#[doc = "IO SECURE 2 Register 3"]
pub mod scu788;
#[doc = "SCU78C (rw) register accessor: IO SECURE 2 Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu78c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu78c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu78c`] module"]
#[doc(alias = "SCU78C")]
pub type Scu78c = crate::Reg<scu78c::Scu78cSpec>;
#[doc = "IO SECURE 2 Register 4"]
pub mod scu78c;
#[doc = "SCU790 (rw) register accessor: IO SECURE 2 Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu790::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu790::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu790`] module"]
#[doc(alias = "SCU790")]
pub type Scu790 = crate::Reg<scu790::Scu790Spec>;
#[doc = "IO SECURE 2 Register 5"]
pub mod scu790;
#[doc = "SCU794 (rw) register accessor: IO SECURE 2 Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu794::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu794::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu794`] module"]
#[doc(alias = "SCU794")]
pub type Scu794 = crate::Reg<scu794::Scu794Spec>;
#[doc = "IO SECURE 2 Register 6"]
pub mod scu794;
#[doc = "SCU798 (rw) register accessor: IO SECURE 2 Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu798::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu798::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu798`] module"]
#[doc(alias = "SCU798")]
pub type Scu798 = crate::Reg<scu798::Scu798Spec>;
#[doc = "IO SECURE 2 Register 7"]
pub mod scu798;
#[doc = "SCU7C0 (rw) register accessor: IO Secure 3 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu7c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu7c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu7c0`] module"]
#[doc(alias = "SCU7C0")]
pub type Scu7c0 = crate::Reg<scu7c0::Scu7c0Spec>;
#[doc = "IO Secure 3 Register 1"]
pub mod scu7c0;
#[doc = "SCU7C4 (rw) register accessor: IO Secure 3 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu7c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu7c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu7c4`] module"]
#[doc(alias = "SCU7C4")]
pub type Scu7c4 = crate::Reg<scu7c4::Scu7c4Spec>;
#[doc = "IO Secure 3 Register 2"]
pub mod scu7c4;
#[doc = "SCU7C8 (rw) register accessor: IO Secure 3 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu7c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu7c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu7c8`] module"]
#[doc(alias = "SCU7C8")]
pub type Scu7c8 = crate::Reg<scu7c8::Scu7c8Spec>;
#[doc = "IO Secure 3 Register 3"]
pub mod scu7c8;
#[doc = "SCU7CC (rw) register accessor: IO Secure 3 Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu7cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu7cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu7cc`] module"]
#[doc(alias = "SCU7CC")]
pub type Scu7cc = crate::Reg<scu7cc::Scu7ccSpec>;
#[doc = "IO Secure 3 Register 4"]
pub mod scu7cc;
#[doc = "SCU7D0 (rw) register accessor: IO Secure 3 Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu7d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu7d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu7d0`] module"]
#[doc(alias = "SCU7D0")]
pub type Scu7d0 = crate::Reg<scu7d0::Scu7d0Spec>;
#[doc = "IO Secure 3 Register 5"]
pub mod scu7d0;
#[doc = "SCU7D4 (rw) register accessor: IO Secure 3 Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu7d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu7d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu7d4`] module"]
#[doc(alias = "SCU7D4")]
pub type Scu7d4 = crate::Reg<scu7d4::Scu7d4Spec>;
#[doc = "IO Secure 3 Register 6"]
pub mod scu7d4;
#[doc = "SCU7D8 (rw) register accessor: IO Secure 3 Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu7d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu7d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu7d8`] module"]
#[doc(alias = "SCU7D8")]
pub type Scu7d8 = crate::Reg<scu7d8::Scu7d8Spec>;
#[doc = "IO Secure 3 Register 7"]
pub mod scu7d8;
#[doc = "SCU800 (rw) register accessor: Scratch register for MCU 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu800::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu800::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu800`] module"]
#[doc(alias = "SCU800")]
pub type Scu800 = crate::Reg<scu800::Scu800Spec>;
#[doc = "Scratch register for MCU 0"]
pub mod scu800;
#[doc = "SCU804 (rw) register accessor: Scratch register for MCU 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu804::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu804::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu804`] module"]
#[doc(alias = "SCU804")]
pub type Scu804 = crate::Reg<scu804::Scu804Spec>;
#[doc = "Scratch register for MCU 1"]
pub mod scu804;
#[doc = "SCU808 (rw) register accessor: Scratch register for MCU 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu808::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu808::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu808`] module"]
#[doc(alias = "SCU808")]
pub type Scu808 = crate::Reg<scu808::Scu808Spec>;
#[doc = "Scratch register for MCU 2"]
pub mod scu808;
#[doc = "SCU80C (rw) register accessor: Scratch register for MCU 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu80c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu80c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu80c`] module"]
#[doc(alias = "SCU80C")]
pub type Scu80c = crate::Reg<scu80c::Scu80cSpec>;
#[doc = "Scratch register for MCU 3"]
pub mod scu80c;
#[doc = "SCU810 (rw) register accessor: Scratch register for MCU 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu810::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu810::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu810`] module"]
#[doc(alias = "SCU810")]
pub type Scu810 = crate::Reg<scu810::Scu810Spec>;
#[doc = "Scratch register for MCU 4"]
pub mod scu810;
#[doc = "SCU814 (rw) register accessor: Scratch register for MCU 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu814::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu814::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu814`] module"]
#[doc(alias = "SCU814")]
pub type Scu814 = crate::Reg<scu814::Scu814Spec>;
#[doc = "Scratch register for MCU 5"]
pub mod scu814;
#[doc = "SCU818 (rw) register accessor: Scratch register for MCU 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu818::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu818::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu818`] module"]
#[doc(alias = "SCU818")]
pub type Scu818 = crate::Reg<scu818::Scu818Spec>;
#[doc = "Scratch register for MCU 6"]
pub mod scu818;
#[doc = "SCU81C (rw) register accessor: Scratch register for MCU 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu81c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu81c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu81c`] module"]
#[doc(alias = "SCU81C")]
pub type Scu81c = crate::Reg<scu81c::Scu81cSpec>;
#[doc = "Scratch register for MCU 7"]
pub mod scu81c;
#[doc = "SCU820 (rw) register accessor: Scratch register for MCU 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu820::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu820::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu820`] module"]
#[doc(alias = "SCU820")]
pub type Scu820 = crate::Reg<scu820::Scu820Spec>;
#[doc = "Scratch register for MCU 8"]
pub mod scu820;
#[doc = "SCU824 (rw) register accessor: Scratch register for MCU 9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu824::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu824::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu824`] module"]
#[doc(alias = "SCU824")]
pub type Scu824 = crate::Reg<scu824::Scu824Spec>;
#[doc = "Scratch register for MCU 9"]
pub mod scu824;
#[doc = "SCU828 (rw) register accessor: Scratch register for MCU 10\n\nYou can [`read`](crate::Reg::read) this register and get [`scu828::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu828::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu828`] module"]
#[doc(alias = "SCU828")]
pub type Scu828 = crate::Reg<scu828::Scu828Spec>;
#[doc = "Scratch register for MCU 10"]
pub mod scu828;
#[doc = "SCU82C (rw) register accessor: Scratch register for MCU 11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu82c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu82c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu82c`] module"]
#[doc(alias = "SCU82C")]
pub type Scu82c = crate::Reg<scu82c::Scu82cSpec>;
#[doc = "Scratch register for MCU 11"]
pub mod scu82c;
#[doc = "SCU830 (rw) register accessor: Scratch register for MCU 12\n\nYou can [`read`](crate::Reg::read) this register and get [`scu830::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu830::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu830`] module"]
#[doc(alias = "SCU830")]
pub type Scu830 = crate::Reg<scu830::Scu830Spec>;
#[doc = "Scratch register for MCU 12"]
pub mod scu830;
#[doc = "SCU834 (rw) register accessor: Scratch register for MCU 13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu834::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu834::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu834`] module"]
#[doc(alias = "SCU834")]
pub type Scu834 = crate::Reg<scu834::Scu834Spec>;
#[doc = "Scratch register for MCU 13"]
pub mod scu834;
#[doc = "SCU838 (rw) register accessor: Scratch register for MCU 14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu838::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu838::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu838`] module"]
#[doc(alias = "SCU838")]
pub type Scu838 = crate::Reg<scu838::Scu838Spec>;
#[doc = "Scratch register for MCU 14"]
pub mod scu838;
#[doc = "SCU83C (rw) register accessor: Scratch register for MCU 15\n\nYou can [`read`](crate::Reg::read) this register and get [`scu83c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu83c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu83c`] module"]
#[doc(alias = "SCU83C")]
pub type Scu83c = crate::Reg<scu83c::Scu83cSpec>;
#[doc = "Scratch register for MCU 15"]
pub mod scu83c;
#[doc = "SCU840 (rw) register accessor: Scratch register for MCU 16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu840::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu840::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu840`] module"]
#[doc(alias = "SCU840")]
pub type Scu840 = crate::Reg<scu840::Scu840Spec>;
#[doc = "Scratch register for MCU 16"]
pub mod scu840;
#[doc = "SCU844 (rw) register accessor: Scratch register for MCU 17\n\nYou can [`read`](crate::Reg::read) this register and get [`scu844::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu844::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu844`] module"]
#[doc(alias = "SCU844")]
pub type Scu844 = crate::Reg<scu844::Scu844Spec>;
#[doc = "Scratch register for MCU 17"]
pub mod scu844;
#[doc = "SCU848 (rw) register accessor: Scratch register for MCU 18\n\nYou can [`read`](crate::Reg::read) this register and get [`scu848::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu848::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu848`] module"]
#[doc(alias = "SCU848")]
pub type Scu848 = crate::Reg<scu848::Scu848Spec>;
#[doc = "Scratch register for MCU 18"]
pub mod scu848;
#[doc = "SCU84C (rw) register accessor: Scratch register for MCU 19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu84c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu84c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu84c`] module"]
#[doc(alias = "SCU84C")]
pub type Scu84c = crate::Reg<scu84c::Scu84cSpec>;
#[doc = "Scratch register for MCU 19"]
pub mod scu84c;
#[doc = "SCU850 (rw) register accessor: Scratch register for MCU 20\n\nYou can [`read`](crate::Reg::read) this register and get [`scu850::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu850::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu850`] module"]
#[doc(alias = "SCU850")]
pub type Scu850 = crate::Reg<scu850::Scu850Spec>;
#[doc = "Scratch register for MCU 20"]
pub mod scu850;
#[doc = "SCU854 (rw) register accessor: Scratch register for MCU 21\n\nYou can [`read`](crate::Reg::read) this register and get [`scu854::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu854::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu854`] module"]
#[doc(alias = "SCU854")]
pub type Scu854 = crate::Reg<scu854::Scu854Spec>;
#[doc = "Scratch register for MCU 21"]
pub mod scu854;
#[doc = "SCU858 (rw) register accessor: Scratch register for MCU 22\n\nYou can [`read`](crate::Reg::read) this register and get [`scu858::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu858::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu858`] module"]
#[doc(alias = "SCU858")]
pub type Scu858 = crate::Reg<scu858::Scu858Spec>;
#[doc = "Scratch register for MCU 22"]
pub mod scu858;
#[doc = "SCU85C (rw) register accessor: Scratch register for MCU 23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu85c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu85c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu85c`] module"]
#[doc(alias = "SCU85C")]
pub type Scu85c = crate::Reg<scu85c::Scu85cSpec>;
#[doc = "Scratch register for MCU 23"]
pub mod scu85c;
#[doc = "SCU860 (rw) register accessor: Scratch register for MCU 24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu860::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu860::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu860`] module"]
#[doc(alias = "SCU860")]
pub type Scu860 = crate::Reg<scu860::Scu860Spec>;
#[doc = "Scratch register for MCU 24"]
pub mod scu860;
#[doc = "SCU864 (rw) register accessor: Scratch register for MCU 25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu864::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu864::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu864`] module"]
#[doc(alias = "SCU864")]
pub type Scu864 = crate::Reg<scu864::Scu864Spec>;
#[doc = "Scratch register for MCU 25"]
pub mod scu864;
#[doc = "SCU868 (rw) register accessor: Scratch register for MCU 26\n\nYou can [`read`](crate::Reg::read) this register and get [`scu868::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu868::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu868`] module"]
#[doc(alias = "SCU868")]
pub type Scu868 = crate::Reg<scu868::Scu868Spec>;
#[doc = "Scratch register for MCU 26"]
pub mod scu868;
#[doc = "SCU86C (rw) register accessor: Scratch register for MCU 27\n\nYou can [`read`](crate::Reg::read) this register and get [`scu86c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu86c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu86c`] module"]
#[doc(alias = "SCU86C")]
pub type Scu86c = crate::Reg<scu86c::Scu86cSpec>;
#[doc = "Scratch register for MCU 27"]
pub mod scu86c;
#[doc = "SCU870 (rw) register accessor: Scratch register for MCU 28\n\nYou can [`read`](crate::Reg::read) this register and get [`scu870::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu870::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu870`] module"]
#[doc(alias = "SCU870")]
pub type Scu870 = crate::Reg<scu870::Scu870Spec>;
#[doc = "Scratch register for MCU 28"]
pub mod scu870;
#[doc = "SCU874 (rw) register accessor: Scratch register for MCU 29\n\nYou can [`read`](crate::Reg::read) this register and get [`scu874::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu874::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu874`] module"]
#[doc(alias = "SCU874")]
pub type Scu874 = crate::Reg<scu874::Scu874Spec>;
#[doc = "Scratch register for MCU 29"]
pub mod scu874;
#[doc = "SCU878 (rw) register accessor: Scratch register for MCU 30\n\nYou can [`read`](crate::Reg::read) this register and get [`scu878::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu878::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu878`] module"]
#[doc(alias = "SCU878")]
pub type Scu878 = crate::Reg<scu878::Scu878Spec>;
#[doc = "Scratch register for MCU 30"]
pub mod scu878;
#[doc = "SCU87C (rw) register accessor: Scratch register for MCU 31\n\nYou can [`read`](crate::Reg::read) this register and get [`scu87c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu87c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu87c`] module"]
#[doc(alias = "SCU87C")]
pub type Scu87c = crate::Reg<scu87c::Scu87cSpec>;
#[doc = "Scratch register for MCU 31"]
pub mod scu87c;
#[doc = "SCU880 (rw) register accessor: Scratch register for MCU 32\n\nYou can [`read`](crate::Reg::read) this register and get [`scu880::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu880::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu880`] module"]
#[doc(alias = "SCU880")]
pub type Scu880 = crate::Reg<scu880::Scu880Spec>;
#[doc = "Scratch register for MCU 32"]
pub mod scu880;
#[doc = "SCU884 (rw) register accessor: Scratch register for MCU 33\n\nYou can [`read`](crate::Reg::read) this register and get [`scu884::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu884::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu884`] module"]
#[doc(alias = "SCU884")]
pub type Scu884 = crate::Reg<scu884::Scu884Spec>;
#[doc = "Scratch register for MCU 33"]
pub mod scu884;
#[doc = "SCU888 (rw) register accessor: Scratch register for MCU 34\n\nYou can [`read`](crate::Reg::read) this register and get [`scu888::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu888::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu888`] module"]
#[doc(alias = "SCU888")]
pub type Scu888 = crate::Reg<scu888::Scu888Spec>;
#[doc = "Scratch register for MCU 34"]
pub mod scu888;
#[doc = "SCU88C (rw) register accessor: Scratch register for MCU 35\n\nYou can [`read`](crate::Reg::read) this register and get [`scu88c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu88c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu88c`] module"]
#[doc(alias = "SCU88C")]
pub type Scu88c = crate::Reg<scu88c::Scu88cSpec>;
#[doc = "Scratch register for MCU 35"]
pub mod scu88c;
#[doc = "SCU890 (rw) register accessor: Scratch register for MCU 36\n\nYou can [`read`](crate::Reg::read) this register and get [`scu890::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu890::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu890`] module"]
#[doc(alias = "SCU890")]
pub type Scu890 = crate::Reg<scu890::Scu890Spec>;
#[doc = "Scratch register for MCU 36"]
pub mod scu890;
#[doc = "SCU894 (rw) register accessor: Scratch register for MCU 37\n\nYou can [`read`](crate::Reg::read) this register and get [`scu894::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu894::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu894`] module"]
#[doc(alias = "SCU894")]
pub type Scu894 = crate::Reg<scu894::Scu894Spec>;
#[doc = "Scratch register for MCU 37"]
pub mod scu894;
#[doc = "SCU898 (rw) register accessor: Scratch register for MCU 38\n\nYou can [`read`](crate::Reg::read) this register and get [`scu898::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu898::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu898`] module"]
#[doc(alias = "SCU898")]
pub type Scu898 = crate::Reg<scu898::Scu898Spec>;
#[doc = "Scratch register for MCU 38"]
pub mod scu898;
#[doc = "SCU89C (rw) register accessor: Scratch register for MCU 39\n\nYou can [`read`](crate::Reg::read) this register and get [`scu89c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu89c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu89c`] module"]
#[doc(alias = "SCU89C")]
pub type Scu89c = crate::Reg<scu89c::Scu89cSpec>;
#[doc = "Scratch register for MCU 39"]
pub mod scu89c;
#[doc = "SCU8A0 (rw) register accessor: Scratch register for MCU 40\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8a0`] module"]
#[doc(alias = "SCU8A0")]
pub type Scu8a0 = crate::Reg<scu8a0::Scu8a0Spec>;
#[doc = "Scratch register for MCU 40"]
pub mod scu8a0;
#[doc = "SCU8A4 (rw) register accessor: Scratch register for MCU 41\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8a4`] module"]
#[doc(alias = "SCU8A4")]
pub type Scu8a4 = crate::Reg<scu8a4::Scu8a4Spec>;
#[doc = "Scratch register for MCU 41"]
pub mod scu8a4;
#[doc = "SCU8A8 (rw) register accessor: Scratch register for MCU 42\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8a8`] module"]
#[doc(alias = "SCU8A8")]
pub type Scu8a8 = crate::Reg<scu8a8::Scu8a8Spec>;
#[doc = "Scratch register for MCU 42"]
pub mod scu8a8;
#[doc = "SCU8AC (rw) register accessor: Scratch register for MCU 43\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8ac`] module"]
#[doc(alias = "SCU8AC")]
pub type Scu8ac = crate::Reg<scu8ac::Scu8acSpec>;
#[doc = "Scratch register for MCU 43"]
pub mod scu8ac;
#[doc = "SCU8B0 (rw) register accessor: Scratch register for MCU 44\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8b0`] module"]
#[doc(alias = "SCU8B0")]
pub type Scu8b0 = crate::Reg<scu8b0::Scu8b0Spec>;
#[doc = "Scratch register for MCU 44"]
pub mod scu8b0;
#[doc = "SCU8B4 (rw) register accessor: Scratch register for MCU 45\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8b4`] module"]
#[doc(alias = "SCU8B4")]
pub type Scu8b4 = crate::Reg<scu8b4::Scu8b4Spec>;
#[doc = "Scratch register for MCU 45"]
pub mod scu8b4;
#[doc = "SCU8B8 (rw) register accessor: Scratch register for MCU 46\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8b8`] module"]
#[doc(alias = "SCU8B8")]
pub type Scu8b8 = crate::Reg<scu8b8::Scu8b8Spec>;
#[doc = "Scratch register for MCU 46"]
pub mod scu8b8;
#[doc = "SCU8BC (rw) register accessor: Scratch register for MCU 47\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8bc`] module"]
#[doc(alias = "SCU8BC")]
pub type Scu8bc = crate::Reg<scu8bc::Scu8bcSpec>;
#[doc = "Scratch register for MCU 47"]
pub mod scu8bc;
#[doc = "SCU8C0 (rw) register accessor: Scratch register for MCU 48\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8c0`] module"]
#[doc(alias = "SCU8C0")]
pub type Scu8c0 = crate::Reg<scu8c0::Scu8c0Spec>;
#[doc = "Scratch register for MCU 48"]
pub mod scu8c0;
#[doc = "SCU8C4 (rw) register accessor: Scratch register for MCU 49\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8c4`] module"]
#[doc(alias = "SCU8C4")]
pub type Scu8c4 = crate::Reg<scu8c4::Scu8c4Spec>;
#[doc = "Scratch register for MCU 49"]
pub mod scu8c4;
#[doc = "SCU8C8 (rw) register accessor: Scratch register for MCU 50\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8c8`] module"]
#[doc(alias = "SCU8C8")]
pub type Scu8c8 = crate::Reg<scu8c8::Scu8c8Spec>;
#[doc = "Scratch register for MCU 50"]
pub mod scu8c8;
#[doc = "SCU8CC (rw) register accessor: Scratch register for MCU 51\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8cc`] module"]
#[doc(alias = "SCU8CC")]
pub type Scu8cc = crate::Reg<scu8cc::Scu8ccSpec>;
#[doc = "Scratch register for MCU 51"]
pub mod scu8cc;
#[doc = "SCU8D0 (rw) register accessor: Scratch register for MCU 52\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8d0`] module"]
#[doc(alias = "SCU8D0")]
pub type Scu8d0 = crate::Reg<scu8d0::Scu8d0Spec>;
#[doc = "Scratch register for MCU 52"]
pub mod scu8d0;
#[doc = "SCU8D4 (rw) register accessor: Scratch register for MCU 53\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8d4`] module"]
#[doc(alias = "SCU8D4")]
pub type Scu8d4 = crate::Reg<scu8d4::Scu8d4Spec>;
#[doc = "Scratch register for MCU 53"]
pub mod scu8d4;
#[doc = "SCU8D8 (rw) register accessor: Scratch register for MCU 54\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8d8`] module"]
#[doc(alias = "SCU8D8")]
pub type Scu8d8 = crate::Reg<scu8d8::Scu8d8Spec>;
#[doc = "Scratch register for MCU 54"]
pub mod scu8d8;
#[doc = "SCU8DC (rw) register accessor: Scratch register for MCU 55\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8dc`] module"]
#[doc(alias = "SCU8DC")]
pub type Scu8dc = crate::Reg<scu8dc::Scu8dcSpec>;
#[doc = "Scratch register for MCU 55"]
pub mod scu8dc;
#[doc = "SCU8E0 (rw) register accessor: Scratch register for MCU 56\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8e0`] module"]
#[doc(alias = "SCU8E0")]
pub type Scu8e0 = crate::Reg<scu8e0::Scu8e0Spec>;
#[doc = "Scratch register for MCU 56"]
pub mod scu8e0;
#[doc = "SCU8E4 (rw) register accessor: Scratch register for MCU 57\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8e4`] module"]
#[doc(alias = "SCU8E4")]
pub type Scu8e4 = crate::Reg<scu8e4::Scu8e4Spec>;
#[doc = "Scratch register for MCU 57"]
pub mod scu8e4;
#[doc = "SCU8E8 (rw) register accessor: Scratch register for MCU 58\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8e8`] module"]
#[doc(alias = "SCU8E8")]
pub type Scu8e8 = crate::Reg<scu8e8::Scu8e8Spec>;
#[doc = "Scratch register for MCU 58"]
pub mod scu8e8;
#[doc = "SCU8EC (rw) register accessor: Scratch register for MCU 59\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8ec`] module"]
#[doc(alias = "SCU8EC")]
pub type Scu8ec = crate::Reg<scu8ec::Scu8ecSpec>;
#[doc = "Scratch register for MCU 59"]
pub mod scu8ec;
#[doc = "SCU8F0 (rw) register accessor: Scratch register for MCU 60\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8f0`] module"]
#[doc(alias = "SCU8F0")]
pub type Scu8f0 = crate::Reg<scu8f0::Scu8f0Spec>;
#[doc = "Scratch register for MCU 60"]
pub mod scu8f0;
#[doc = "SCU8F4 (rw) register accessor: Scratch register for MCU 61\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8f4`] module"]
#[doc(alias = "SCU8F4")]
pub type Scu8f4 = crate::Reg<scu8f4::Scu8f4Spec>;
#[doc = "Scratch register for MCU 61"]
pub mod scu8f4;
#[doc = "SCU8F8 (rw) register accessor: Scratch register for MCU 62\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8f8`] module"]
#[doc(alias = "SCU8F8")]
pub type Scu8f8 = crate::Reg<scu8f8::Scu8f8Spec>;
#[doc = "Scratch register for MCU 62"]
pub mod scu8f8;
#[doc = "SCU8FC (rw) register accessor: Scratch register for MCU 63\n\nYou can [`read`](crate::Reg::read) this register and get [`scu8fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu8fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu8fc`] module"]
#[doc(alias = "SCU8FC")]
pub type Scu8fc = crate::Reg<scu8fc::Scu8fcSpec>;
#[doc = "Scratch register for MCU 63"]
pub mod scu8fc;
#[doc = "SCU900 (rw) register accessor: \\PSP\\ Service Processor Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu900::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu900::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu900`] module"]
#[doc(alias = "SCU900")]
pub type Scu900 = crate::Reg<scu900::Scu900Spec>;
#[doc = "\\PSP\\ Service Processor Control Register 1"]
pub mod scu900;
#[doc = "SCU904 (rw) register accessor: \\PSP\\ Service Processor Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu904::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu904::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu904`] module"]
#[doc(alias = "SCU904")]
pub type Scu904 = crate::Reg<scu904::Scu904Spec>;
#[doc = "\\PSP\\ Service Processor Control Register 2"]
pub mod scu904;
#[doc = "SCU908 (rw) register accessor: \\PSP\\ Service Processor Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu908::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu908::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu908`] module"]
#[doc(alias = "SCU908")]
pub type Scu908 = crate::Reg<scu908::Scu908Spec>;
#[doc = "\\PSP\\ Service Processor Control Register 3"]
pub mod scu908;
#[doc = "SCU90C (rw) register accessor: \\PSP\\ Service Processor Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu90c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu90c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu90c`] module"]
#[doc(alias = "SCU90C")]
pub type Scu90c = crate::Reg<scu90c::Scu90cSpec>;
#[doc = "\\PSP\\ Service Processor Control Register 4"]
pub mod scu90c;
#[doc = "SCU910 (rw) register accessor: \\PSP\\ Service Processor Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu910::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu910::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu910`] module"]
#[doc(alias = "SCU910")]
pub type Scu910 = crate::Reg<scu910::Scu910Spec>;
#[doc = "\\PSP\\ Service Processor Control Register 5"]
pub mod scu910;
#[doc = "SCU914 (rw) register accessor: \\PSP\\ Service Processor Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu914::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu914::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu914`] module"]
#[doc(alias = "SCU914")]
pub type Scu914 = crate::Reg<scu914::Scu914Spec>;
#[doc = "\\PSP\\ Service Processor Control Register 6"]
pub mod scu914;
#[doc = "SCU918 (rw) register accessor: \\PSP\\ Service Processor Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu918::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu918::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu918`] module"]
#[doc(alias = "SCU918")]
pub type Scu918 = crate::Reg<scu918::Scu918Spec>;
#[doc = "\\PSP\\ Service Processor Control Register 7"]
pub mod scu918;
#[doc = "SCU920 (rw) register accessor: \\PSP\\ Service Processor REMAP Base Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu920::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu920::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu920`] module"]
#[doc(alias = "SCU920")]
pub type Scu920 = crate::Reg<scu920::Scu920Spec>;
#[doc = "\\PSP\\ Service Processor REMAP Base Register 0"]
pub mod scu920;
#[doc = "SCU924 (rw) register accessor: \\PSP\\ Service Processor REMAP Size Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu924::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu924::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu924`] module"]
#[doc(alias = "SCU924")]
pub type Scu924 = crate::Reg<scu924::Scu924Spec>;
#[doc = "\\PSP\\ Service Processor REMAP Size Register 0"]
pub mod scu924;
#[doc = "SCU928 (rw) register accessor: \\PSP\\ Service Processor REMAP Base Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu928::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu928::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu928`] module"]
#[doc(alias = "SCU928")]
pub type Scu928 = crate::Reg<scu928::Scu928Spec>;
#[doc = "\\PSP\\ Service Processor REMAP Base Register 1"]
pub mod scu928;
#[doc = "SCU92C (rw) register accessor: \\PSP\\ Service Processor REMAP Size Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu92c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu92c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu92c`] module"]
#[doc(alias = "SCU92C")]
pub type Scu92c = crate::Reg<scu92c::Scu92cSpec>;
#[doc = "\\PSP\\ Service Processor REMAP Size Register 1"]
pub mod scu92c;
#[doc = "SCU930 (rw) register accessor: \\PSP\\ Service Processor REMAP Base Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu930::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu930::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu930`] module"]
#[doc(alias = "SCU930")]
pub type Scu930 = crate::Reg<scu930::Scu930Spec>;
#[doc = "\\PSP\\ Service Processor REMAP Base Register 2"]
pub mod scu930;
#[doc = "SCU934 (rw) register accessor: \\PSP\\ Service Processor REMAP Size Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu934::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu934::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu934`] module"]
#[doc(alias = "SCU934")]
pub type Scu934 = crate::Reg<scu934::Scu934Spec>;
#[doc = "\\PSP\\ Service Processor REMAP Size Register 2"]
pub mod scu934;
#[doc = "SCU940 (rw) register accessor: \\SSP\\ Service Processor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu940::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu940::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu940`] module"]
#[doc(alias = "SCU940")]
pub type Scu940 = crate::Reg<scu940::Scu940Spec>;
#[doc = "\\SSP\\ Service Processor Control Register"]
pub mod scu940;
#[doc = "SCU948 (rw) register accessor: \\SSP\\ Service Processor Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu948::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu948::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu948`] module"]
#[doc(alias = "SCU948")]
pub type Scu948 = crate::Reg<scu948::Scu948Spec>;
#[doc = "\\SSP\\ Service Processor Control Register 3"]
pub mod scu948;
#[doc = "SCU94C (rw) register accessor: \\SSP\\ Service Processor Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu94c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu94c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu94c`] module"]
#[doc(alias = "SCU94C")]
pub type Scu94c = crate::Reg<scu94c::Scu94cSpec>;
#[doc = "\\SSP\\ Service Processor Control Register 4"]
pub mod scu94c;
#[doc = "SCU950 (rw) register accessor: \\SSP\\ Service Processor Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu950::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu950::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu950`] module"]
#[doc(alias = "SCU950")]
pub type Scu950 = crate::Reg<scu950::Scu950Spec>;
#[doc = "\\SSP\\ Service Processor Control Register 5"]
pub mod scu950;
#[doc = "SCU954 (rw) register accessor: \\SSP\\ Service Processor Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu954::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu954::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu954`] module"]
#[doc(alias = "SCU954")]
pub type Scu954 = crate::Reg<scu954::Scu954Spec>;
#[doc = "\\SSP\\ Service Processor Control Register 6"]
pub mod scu954;
#[doc = "SCU958 (rw) register accessor: \\SSP\\ Service Processor Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu958::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu958::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu958`] module"]
#[doc(alias = "SCU958")]
pub type Scu958 = crate::Reg<scu958::Scu958Spec>;
#[doc = "\\SSP\\ Service Processor Control Register 7"]
pub mod scu958;
#[doc = "SCU974 (rw) register accessor: \\SSP\\ Service Processor REMAP Size Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu974::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu974::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu974`] module"]
#[doc(alias = "SCU974")]
pub type Scu974 = crate::Reg<scu974::Scu974Spec>;
#[doc = "\\SSP\\ Service Processor REMAP Size Register 2"]
pub mod scu974;
#[doc = "SCU980 (rw) register accessor: EFUSE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu980::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu980::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu980`] module"]
#[doc(alias = "SCU980")]
pub type Scu980 = crate::Reg<scu980::Scu980Spec>;
#[doc = "EFUSE Control Register"]
pub mod scu980;
#[doc = "SCU984 (rw) register accessor: EFUSE Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu984::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu984::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu984`] module"]
#[doc(alias = "SCU984")]
pub type Scu984 = crate::Reg<scu984::Scu984Spec>;
#[doc = "EFUSE Data Register"]
pub mod scu984;
#[doc = "SCU988 (rw) register accessor: EFUSE Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu988::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu988::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu988`] module"]
#[doc(alias = "SCU988")]
pub type Scu988 = crate::Reg<scu988::Scu988Spec>;
#[doc = "EFUSE Command Register"]
pub mod scu988;
#[doc = "SCU98C (rw) register accessor: EFUSE Program Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu98c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu98c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu98c`] module"]
#[doc(alias = "SCU98C")]
pub type Scu98c = crate::Reg<scu98c::Scu98cSpec>;
#[doc = "EFUSE Program Pattern Register"]
pub mod scu98c;
#[doc = "SCU990 (rw) register accessor: Chip Unique ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu990::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu990::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu990`] module"]
#[doc(alias = "SCU990")]
pub type Scu990 = crate::Reg<scu990::Scu990Spec>;
#[doc = "Chip Unique ID 0"]
pub mod scu990;
#[doc = "SCU994 (rw) register accessor: Chip Unique ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu994::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu994::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu994`] module"]
#[doc(alias = "SCU994")]
pub type Scu994 = crate::Reg<scu994::Scu994Spec>;
#[doc = "Chip Unique ID 1"]
pub mod scu994;
#[doc = "SCU998 (rw) register accessor: Reserved Read Only ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu998::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu998::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu998`] module"]
#[doc(alias = "SCU998")]
pub type Scu998 = crate::Reg<scu998::Scu998Spec>;
#[doc = "Reserved Read Only ID 0"]
pub mod scu998;
#[doc = "SCU99C (rw) register accessor: Reserved Read Only ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu99c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu99c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu99c`] module"]
#[doc(alias = "SCU99C")]
pub type Scu99c = crate::Reg<scu99c::Scu99cSpec>;
#[doc = "Reserved Read Only ID 1"]
pub mod scu99c;
#[doc = "SCU9A0 (rw) register accessor: Reserved Read Only ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu9a0`] module"]
#[doc(alias = "SCU9A0")]
pub type Scu9a0 = crate::Reg<scu9a0::Scu9a0Spec>;
#[doc = "Reserved Read Only ID 2"]
pub mod scu9a0;
#[doc = "SCU9A4 (rw) register accessor: Reserved Read Only ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu9a4`] module"]
#[doc(alias = "SCU9A4")]
pub type Scu9a4 = crate::Reg<scu9a4::Scu9a4Spec>;
#[doc = "Reserved Read Only ID 3"]
pub mod scu9a4;
#[doc = "SCU9A8 (rw) register accessor: Reserved Read Only ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu9a8`] module"]
#[doc(alias = "SCU9A8")]
pub type Scu9a8 = crate::Reg<scu9a8::Scu9a8Spec>;
#[doc = "Reserved Read Only ID 4"]
pub mod scu9a8;
#[doc = "SCU9AC (rw) register accessor: Reserved Read Only ID 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu9ac`] module"]
#[doc(alias = "SCU9AC")]
pub type Scu9ac = crate::Reg<scu9ac::Scu9acSpec>;
#[doc = "Reserved Read Only ID 5"]
pub mod scu9ac;
#[doc = "SCU9B0 (rw) register accessor: EFUSE PGM Timing Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu9b0`] module"]
#[doc(alias = "SCU9B0")]
pub type Scu9b0 = crate::Reg<scu9b0::Scu9b0Spec>;
#[doc = "EFUSE PGM Timing Register 0"]
pub mod scu9b0;
#[doc = "SCU9B4 (rw) register accessor: EFUSE PGM Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu9b4`] module"]
#[doc(alias = "SCU9B4")]
pub type Scu9b4 = crate::Reg<scu9b4::Scu9b4Spec>;
#[doc = "EFUSE PGM Timing Register 1"]
pub mod scu9b4;
#[doc = "SCU9B8 (rw) register accessor: EFUSE Read Timing Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu9b8`] module"]
#[doc(alias = "SCU9B8")]
pub type Scu9b8 = crate::Reg<scu9b8::Scu9b8Spec>;
#[doc = "EFUSE Read Timing Register 0"]
pub mod scu9b8;
#[doc = "SCU9BC (rw) register accessor: EFUSE Read Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu9bc`] module"]
#[doc(alias = "SCU9BC")]
pub type Scu9bc = crate::Reg<scu9bc::Scu9bcSpec>;
#[doc = "EFUSE Read Timing Register 1"]
pub mod scu9bc;
#[doc = "SCUB00 (rw) register accessor: SW PUF Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scub00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub00`] module"]
#[doc(alias = "SCUB00")]
pub type Scub00 = crate::Reg<scub00::Scub00Spec>;
#[doc = "SW PUF Register 0"]
pub mod scub00;
#[doc = "SCUB04 (rw) register accessor: SW PUF Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scub04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub04`] module"]
#[doc(alias = "SCUB04")]
pub type Scub04 = crate::Reg<scub04::Scub04Spec>;
#[doc = "SW PUF Register 1"]
pub mod scub04;
#[doc = "SCUB08 (rw) register accessor: SW PUF Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scub08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub08`] module"]
#[doc(alias = "SCUB08")]
pub type Scub08 = crate::Reg<scub08::Scub08Spec>;
#[doc = "SW PUF Register 2"]
pub mod scub08;
#[doc = "SCUB0C (rw) register accessor: SW PUF Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scub0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub0c`] module"]
#[doc(alias = "SCUB0C")]
pub type Scub0c = crate::Reg<scub0c::Scub0cSpec>;
#[doc = "SW PUF Register 3"]
pub mod scub0c;
#[doc = "SCUB10 (rw) register accessor: SW PUF Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scub10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub10`] module"]
#[doc(alias = "SCUB10")]
pub type Scub10 = crate::Reg<scub10::Scub10Spec>;
#[doc = "SW PUF Register 4"]
pub mod scub10;
#[doc = "SCUB14 (rw) register accessor: SW PUF Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scub14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub14`] module"]
#[doc(alias = "SCUB14")]
pub type Scub14 = crate::Reg<scub14::Scub14Spec>;
#[doc = "SW PUF Register 5"]
pub mod scub14;
#[doc = "SCUB18 (rw) register accessor: SW PUF Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scub18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub18`] module"]
#[doc(alias = "SCUB18")]
pub type Scub18 = crate::Reg<scub18::Scub18Spec>;
#[doc = "SW PUF Register 6"]
pub mod scub18;
#[doc = "SCUB1C (rw) register accessor: SW PUF Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scub1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub1c`] module"]
#[doc(alias = "SCUB1C")]
pub type Scub1c = crate::Reg<scub1c::Scub1cSpec>;
#[doc = "SW PUF Register 7"]
pub mod scub1c;
#[doc = "SCUB20 (rw) register accessor: SW PUF Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scub20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub20`] module"]
#[doc(alias = "SCUB20")]
pub type Scub20 = crate::Reg<scub20::Scub20Spec>;
#[doc = "SW PUF Register 8"]
pub mod scub20;
#[doc = "SCUB24 (rw) register accessor: SW PUF Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`scub24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub24`] module"]
#[doc(alias = "SCUB24")]
pub type Scub24 = crate::Reg<scub24::Scub24Spec>;
#[doc = "SW PUF Register 9"]
pub mod scub24;
#[doc = "SCUB28 (rw) register accessor: SW PUF Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`scub28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub28`] module"]
#[doc(alias = "SCUB28")]
pub type Scub28 = crate::Reg<scub28::Scub28Spec>;
#[doc = "SW PUF Register 10"]
pub mod scub28;
#[doc = "SCUB2C (rw) register accessor: SW PUF Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`scub2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub2c`] module"]
#[doc(alias = "SCUB2C")]
pub type Scub2c = crate::Reg<scub2c::Scub2cSpec>;
#[doc = "SW PUF Register 11"]
pub mod scub2c;
#[doc = "SCUB30 (rw) register accessor: SW PUF Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`scub30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub30`] module"]
#[doc(alias = "SCUB30")]
pub type Scub30 = crate::Reg<scub30::Scub30Spec>;
#[doc = "SW PUF Register 12"]
pub mod scub30;
#[doc = "SCUB34 (rw) register accessor: SW PUF Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`scub34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub34`] module"]
#[doc(alias = "SCUB34")]
pub type Scub34 = crate::Reg<scub34::Scub34Spec>;
#[doc = "SW PUF Register 13"]
pub mod scub34;
#[doc = "SCUB38 (rw) register accessor: SW PUF Register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`scub38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub38`] module"]
#[doc(alias = "SCUB38")]
pub type Scub38 = crate::Reg<scub38::Scub38Spec>;
#[doc = "SW PUF Register 14"]
pub mod scub38;
#[doc = "SCUB3C (rw) register accessor: SW PUF Register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`scub3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub3c`] module"]
#[doc(alias = "SCUB3C")]
pub type Scub3c = crate::Reg<scub3c::Scub3cSpec>;
#[doc = "SW PUF Register 15"]
pub mod scub3c;
#[doc = "SCUB40 (rw) register accessor: SW PUF Register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`scub40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub40`] module"]
#[doc(alias = "SCUB40")]
pub type Scub40 = crate::Reg<scub40::Scub40Spec>;
#[doc = "SW PUF Register 16"]
pub mod scub40;
#[doc = "SCUB44 (rw) register accessor: SW PUF Register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`scub44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub44`] module"]
#[doc(alias = "SCUB44")]
pub type Scub44 = crate::Reg<scub44::Scub44Spec>;
#[doc = "SW PUF Register 17"]
pub mod scub44;
#[doc = "SCUB48 (rw) register accessor: SW PUF Register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`scub48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub48`] module"]
#[doc(alias = "SCUB48")]
pub type Scub48 = crate::Reg<scub48::Scub48Spec>;
#[doc = "SW PUF Register 18"]
pub mod scub48;
#[doc = "SCUB4C (rw) register accessor: SW PUF Register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`scub4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub4c`] module"]
#[doc(alias = "SCUB4C")]
pub type Scub4c = crate::Reg<scub4c::Scub4cSpec>;
#[doc = "SW PUF Register 19"]
pub mod scub4c;
#[doc = "SCUB50 (rw) register accessor: SW PUF Register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`scub50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub50`] module"]
#[doc(alias = "SCUB50")]
pub type Scub50 = crate::Reg<scub50::Scub50Spec>;
#[doc = "SW PUF Register 20"]
pub mod scub50;
#[doc = "SCUB54 (rw) register accessor: SW PUF Register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`scub54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub54`] module"]
#[doc(alias = "SCUB54")]
pub type Scub54 = crate::Reg<scub54::Scub54Spec>;
#[doc = "SW PUF Register 21"]
pub mod scub54;
#[doc = "SCUB58 (rw) register accessor: SW PUF Register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`scub58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub58`] module"]
#[doc(alias = "SCUB58")]
pub type Scub58 = crate::Reg<scub58::Scub58Spec>;
#[doc = "SW PUF Register 22"]
pub mod scub58;
#[doc = "SCUB5C (rw) register accessor: SW PUF Register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`scub5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub5c`] module"]
#[doc(alias = "SCUB5C")]
pub type Scub5c = crate::Reg<scub5c::Scub5cSpec>;
#[doc = "SW PUF Register 23"]
pub mod scub5c;
#[doc = "SCUB60 (rw) register accessor: SW PUF Register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`scub60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub60`] module"]
#[doc(alias = "SCUB60")]
pub type Scub60 = crate::Reg<scub60::Scub60Spec>;
#[doc = "SW PUF Register 24"]
pub mod scub60;
#[doc = "SCUB64 (rw) register accessor: SW PUF Register 25\n\nYou can [`read`](crate::Reg::read) this register and get [`scub64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub64`] module"]
#[doc(alias = "SCUB64")]
pub type Scub64 = crate::Reg<scub64::Scub64Spec>;
#[doc = "SW PUF Register 25"]
pub mod scub64;
#[doc = "SCUB68 (rw) register accessor: SW PUF Register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`scub68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub68`] module"]
#[doc(alias = "SCUB68")]
pub type Scub68 = crate::Reg<scub68::Scub68Spec>;
#[doc = "SW PUF Register 26"]
pub mod scub68;
#[doc = "SCUB6C (rw) register accessor: SW PUF Register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`scub6c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub6c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub6c`] module"]
#[doc(alias = "SCUB6C")]
pub type Scub6c = crate::Reg<scub6c::Scub6cSpec>;
#[doc = "SW PUF Register 27"]
pub mod scub6c;
#[doc = "SCUB70 (rw) register accessor: SW PUF Register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`scub70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub70`] module"]
#[doc(alias = "SCUB70")]
pub type Scub70 = crate::Reg<scub70::Scub70Spec>;
#[doc = "SW PUF Register 28"]
pub mod scub70;
#[doc = "SCUB74 (rw) register accessor: SW PUF Register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`scub74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub74`] module"]
#[doc(alias = "SCUB74")]
pub type Scub74 = crate::Reg<scub74::Scub74Spec>;
#[doc = "SW PUF Register 29"]
pub mod scub74;
#[doc = "SCUB78 (rw) register accessor: SW PUF Register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`scub78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub78`] module"]
#[doc(alias = "SCUB78")]
pub type Scub78 = crate::Reg<scub78::Scub78Spec>;
#[doc = "SW PUF Register 30"]
pub mod scub78;
#[doc = "SCUB7C (rw) register accessor: SW PUF Register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`scub7c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub7c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub7c`] module"]
#[doc(alias = "SCUB7C")]
pub type Scub7c = crate::Reg<scub7c::Scub7cSpec>;
#[doc = "SW PUF Register 31"]
pub mod scub7c;
#[doc = "SCUB80 (rw) register accessor: PUF Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scub80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scub80`] module"]
#[doc(alias = "SCUB80")]
pub type Scub80 = crate::Reg<scub80::Scub80Spec>;
#[doc = "PUF Control Register"]
pub mod scub80;
#[doc = "SCUBA0 (rw) register accessor: HW PUF Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scuba0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuba0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuba0`] module"]
#[doc(alias = "SCUBA0")]
pub type Scuba0 = crate::Reg<scuba0::Scuba0Spec>;
#[doc = "HW PUF Register 8"]
pub mod scuba0;
#[doc = "SCUBA4 (rw) register accessor: HW PUF Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`scuba4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuba4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuba4`] module"]
#[doc(alias = "SCUBA4")]
pub type Scuba4 = crate::Reg<scuba4::Scuba4Spec>;
#[doc = "HW PUF Register 9"]
pub mod scuba4;
#[doc = "SCUBA8 (rw) register accessor: HW PUF Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`scuba8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuba8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuba8`] module"]
#[doc(alias = "SCUBA8")]
pub type Scuba8 = crate::Reg<scuba8::Scuba8Spec>;
#[doc = "HW PUF Register 10"]
pub mod scuba8;
#[doc = "SCUBAC (rw) register accessor: HW PUF Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`scubac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubac`] module"]
#[doc(alias = "SCUBAC")]
pub type Scubac = crate::Reg<scubac::ScubacSpec>;
#[doc = "HW PUF Register 11"]
pub mod scubac;
#[doc = "SCUBB0 (rw) register accessor: HW PUF Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`scubb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubb0`] module"]
#[doc(alias = "SCUBB0")]
pub type Scubb0 = crate::Reg<scubb0::Scubb0Spec>;
#[doc = "HW PUF Register 12"]
pub mod scubb0;
#[doc = "SCUBB4 (rw) register accessor: HW PUF Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`scubb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubb4`] module"]
#[doc(alias = "SCUBB4")]
pub type Scubb4 = crate::Reg<scubb4::Scubb4Spec>;
#[doc = "HW PUF Register 13"]
pub mod scubb4;
#[doc = "SCUBB8 (rw) register accessor: HW PUF Register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`scubb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubb8`] module"]
#[doc(alias = "SCUBB8")]
pub type Scubb8 = crate::Reg<scubb8::Scubb8Spec>;
#[doc = "HW PUF Register 14"]
pub mod scubb8;
#[doc = "SCUBBC (rw) register accessor: HW PUF Register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`scubbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubbc`] module"]
#[doc(alias = "SCUBBC")]
pub type Scubbc = crate::Reg<scubbc::ScubbcSpec>;
#[doc = "HW PUF Register 15"]
pub mod scubbc;
#[doc = "SCUBC0 (rw) register accessor: HW PUF Register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`scubc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubc0`] module"]
#[doc(alias = "SCUBC0")]
pub type Scubc0 = crate::Reg<scubc0::Scubc0Spec>;
#[doc = "HW PUF Register 16"]
pub mod scubc0;
#[doc = "SCUBC4 (rw) register accessor: HW PUF Register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`scubc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubc4`] module"]
#[doc(alias = "SCUBC4")]
pub type Scubc4 = crate::Reg<scubc4::Scubc4Spec>;
#[doc = "HW PUF Register 17"]
pub mod scubc4;
#[doc = "SCUBC8 (rw) register accessor: HW PUF Register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`scubc8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubc8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubc8`] module"]
#[doc(alias = "SCUBC8")]
pub type Scubc8 = crate::Reg<scubc8::Scubc8Spec>;
#[doc = "HW PUF Register 18"]
pub mod scubc8;
#[doc = "SCUBCC (rw) register accessor: HW PUF Register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`scubcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubcc`] module"]
#[doc(alias = "SCUBCC")]
pub type Scubcc = crate::Reg<scubcc::ScubccSpec>;
#[doc = "HW PUF Register 19"]
pub mod scubcc;
#[doc = "SCUBD0 (rw) register accessor: HW PUF Register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`scubd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubd0`] module"]
#[doc(alias = "SCUBD0")]
pub type Scubd0 = crate::Reg<scubd0::Scubd0Spec>;
#[doc = "HW PUF Register 20"]
pub mod scubd0;
#[doc = "SCUBD4 (rw) register accessor: HW PUF Register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`scubd4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubd4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubd4`] module"]
#[doc(alias = "SCUBD4")]
pub type Scubd4 = crate::Reg<scubd4::Scubd4Spec>;
#[doc = "HW PUF Register 21"]
pub mod scubd4;
#[doc = "SCUBD8 (rw) register accessor: HW PUF Register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`scubd8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubd8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubd8`] module"]
#[doc(alias = "SCUBD8")]
pub type Scubd8 = crate::Reg<scubd8::Scubd8Spec>;
#[doc = "HW PUF Register 22"]
pub mod scubd8;
#[doc = "SCUBDC (rw) register accessor: HW PUF Register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`scubdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubdc`] module"]
#[doc(alias = "SCUBDC")]
pub type Scubdc = crate::Reg<scubdc::ScubdcSpec>;
#[doc = "HW PUF Register 23"]
pub mod scubdc;
#[doc = "SCUBE0 (rw) register accessor: HW PUF Register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`scube0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scube0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scube0`] module"]
#[doc(alias = "SCUBE0")]
pub type Scube0 = crate::Reg<scube0::Scube0Spec>;
#[doc = "HW PUF Register 24"]
pub mod scube0;
#[doc = "SCUBE4 (rw) register accessor: HW PUF Register 25\n\nYou can [`read`](crate::Reg::read) this register and get [`scube4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scube4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scube4`] module"]
#[doc(alias = "SCUBE4")]
pub type Scube4 = crate::Reg<scube4::Scube4Spec>;
#[doc = "HW PUF Register 25"]
pub mod scube4;
#[doc = "SCUBE8 (rw) register accessor: HW PUF Register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`scube8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scube8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scube8`] module"]
#[doc(alias = "SCUBE8")]
pub type Scube8 = crate::Reg<scube8::Scube8Spec>;
#[doc = "HW PUF Register 26"]
pub mod scube8;
#[doc = "SCUBEC (rw) register accessor: HW PUF Register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`scubec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubec`] module"]
#[doc(alias = "SCUBEC")]
pub type Scubec = crate::Reg<scubec::ScubecSpec>;
#[doc = "HW PUF Register 27"]
pub mod scubec;
#[doc = "SCUBF0 (rw) register accessor: HW PUF Register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`scubf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubf0`] module"]
#[doc(alias = "SCUBF0")]
pub type Scubf0 = crate::Reg<scubf0::Scubf0Spec>;
#[doc = "HW PUF Register 28"]
pub mod scubf0;
#[doc = "SCUBF4 (rw) register accessor: HW PUF Register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`scubf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubf4`] module"]
#[doc(alias = "SCUBF4")]
pub type Scubf4 = crate::Reg<scubf4::Scubf4Spec>;
#[doc = "HW PUF Register 29"]
pub mod scubf4;
#[doc = "SCUBF8 (rw) register accessor: HW PUF Register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`scubf8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubf8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubf8`] module"]
#[doc(alias = "SCUBF8")]
pub type Scubf8 = crate::Reg<scubf8::Scubf8Spec>;
#[doc = "HW PUF Register 30"]
pub mod scubf8;
#[doc = "SCUBFC (rw) register accessor: HW PUF Register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`scubfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scubfc`] module"]
#[doc(alias = "SCUBFC")]
pub type Scubfc = crate::Reg<scubfc::ScubfcSpec>;
#[doc = "HW PUF Register 31"]
pub mod scubfc;
#[doc = "SCUC04 (rw) register accessor: Secure1 Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc04`] module"]
#[doc(alias = "SCUC04")]
pub type Scuc04 = crate::Reg<scuc04::Scuc04Spec>;
#[doc = "Secure1 Control 2 Register"]
pub mod scuc04;
#[doc = "SCUC08 (rw) register accessor: Secure1 Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc08`] module"]
#[doc(alias = "SCUC08")]
pub type Scuc08 = crate::Reg<scuc08::Scuc08Spec>;
#[doc = "Secure1 Control 3 Register"]
pub mod scuc08;
#[doc = "SCUC0C (rw) register accessor: Secure1 Control 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc0c`] module"]
#[doc(alias = "SCUC0C")]
pub type Scuc0c = crate::Reg<scuc0c::Scuc0cSpec>;
#[doc = "Secure1 Control 4 Register"]
pub mod scuc0c;
#[doc = "SCUC14 (rw) register accessor: Secure1 Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc14`] module"]
#[doc(alias = "SCUC14")]
pub type Scuc14 = crate::Reg<scuc14::Scuc14Spec>;
#[doc = "Secure1 Control 6 Register"]
pub mod scuc14;
#[doc = "SCUC18 (rw) register accessor: Secure1 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc18`] module"]
#[doc(alias = "SCUC18")]
pub type Scuc18 = crate::Reg<scuc18::Scuc18Spec>;
#[doc = "Secure1 Control 7 Register"]
pub mod scuc18;
#[doc = "SCUC1C (rw) register accessor: Secure1 Control 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc1c`] module"]
#[doc(alias = "SCUC1C")]
pub type Scuc1c = crate::Reg<scuc1c::Scuc1cSpec>;
#[doc = "Secure1 Control 8 Register"]
pub mod scuc1c;
#[doc = "SCUC28 (rw) register accessor: Secure1 Control 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc28`] module"]
#[doc(alias = "SCUC28")]
pub type Scuc28 = crate::Reg<scuc28::Scuc28Spec>;
#[doc = "Secure1 Control 11 Register"]
pub mod scuc28;
#[doc = "SCUC34 (rw) register accessor: Secure1 Control 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc34`] module"]
#[doc(alias = "SCUC34")]
pub type Scuc34 = crate::Reg<scuc34::Scuc34Spec>;
#[doc = "Secure1 Control 14 Register"]
pub mod scuc34;
#[doc = "SCUC48 (rw) register accessor: Secure1 Control 19 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc48`] module"]
#[doc(alias = "SCUC48")]
pub type Scuc48 = crate::Reg<scuc48::Scuc48Spec>;
#[doc = "Secure1 Control 19 Register"]
pub mod scuc48;
#[doc = "SCUC4C (rw) register accessor: Secure1 Control 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc4c`] module"]
#[doc(alias = "SCUC4C")]
pub type Scuc4c = crate::Reg<scuc4c::Scuc4cSpec>;
#[doc = "Secure1 Control 20 Register"]
pub mod scuc4c;
#[doc = "SCUC50 (rw) register accessor: Secure1 Control 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc50`] module"]
#[doc(alias = "SCUC50")]
pub type Scuc50 = crate::Reg<scuc50::Scuc50Spec>;
#[doc = "Secure1 Control 21 Register"]
pub mod scuc50;
#[doc = "SCUC5C (rw) register accessor: Secure1 Control 24 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc5c`] module"]
#[doc(alias = "SCUC5C")]
pub type Scuc5c = crate::Reg<scuc5c::Scuc5cSpec>;
#[doc = "Secure1 Control 24 Register"]
pub mod scuc5c;
#[doc = "SCUC84 (rw) register accessor: Secure2 Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc84`] module"]
#[doc(alias = "SCUC84")]
pub type Scuc84 = crate::Reg<scuc84::Scuc84Spec>;
#[doc = "Secure2 Control 2 Register"]
pub mod scuc84;
#[doc = "SCUC88 (rw) register accessor: Secure2 Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc88`] module"]
#[doc(alias = "SCUC88")]
pub type Scuc88 = crate::Reg<scuc88::Scuc88Spec>;
#[doc = "Secure2 Control 3 Register"]
pub mod scuc88;
#[doc = "SCUC8C (rw) register accessor: Secure2 Control 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc8c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc8c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc8c`] module"]
#[doc(alias = "SCUC8C")]
pub type Scuc8c = crate::Reg<scuc8c::Scuc8cSpec>;
#[doc = "Secure2 Control 4 Register"]
pub mod scuc8c;
#[doc = "SCUC94 (rw) register accessor: Secure2 Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc94`] module"]
#[doc(alias = "SCUC94")]
pub type Scuc94 = crate::Reg<scuc94::Scuc94Spec>;
#[doc = "Secure2 Control 6 Register"]
pub mod scuc94;
#[doc = "SCUC98 (rw) register accessor: Secure2 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc98`] module"]
#[doc(alias = "SCUC98")]
pub type Scuc98 = crate::Reg<scuc98::Scuc98Spec>;
#[doc = "Secure2 Control 7 Register"]
pub mod scuc98;
#[doc = "SCUC9C (rw) register accessor: Secure2 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc9c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc9c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuc9c`] module"]
#[doc(alias = "SCUC9C")]
pub type Scuc9c = crate::Reg<scuc9c::Scuc9cSpec>;
#[doc = "Secure2 Control 7 Register"]
pub mod scuc9c;
#[doc = "SCUCA8 (rw) register accessor: Secure2 Control 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuca8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuca8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuca8`] module"]
#[doc(alias = "SCUCA8")]
pub type Scuca8 = crate::Reg<scuca8::Scuca8Spec>;
#[doc = "Secure2 Control 11 Register"]
pub mod scuca8;
#[doc = "SCUCB4 (rw) register accessor: Secure2 Control 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scucb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scucb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scucb4`] module"]
#[doc(alias = "SCUCB4")]
pub type Scucb4 = crate::Reg<scucb4::Scucb4Spec>;
#[doc = "Secure2 Control 14 Register"]
pub mod scucb4;
#[doc = "SCUCC8 (rw) register accessor: Secure2 Control 19 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scucc8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scucc8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scucc8`] module"]
#[doc(alias = "SCUCC8")]
pub type Scucc8 = crate::Reg<scucc8::Scucc8Spec>;
#[doc = "Secure2 Control 19 Register"]
pub mod scucc8;
#[doc = "SCUCCC (rw) register accessor: Secure2 Control 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuccc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuccc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuccc`] module"]
#[doc(alias = "SCUCCC")]
pub type Scuccc = crate::Reg<scuccc::ScucccSpec>;
#[doc = "Secure2 Control 20 Register"]
pub mod scuccc;
#[doc = "SCUCD0 (rw) register accessor: Secure2 Control 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scucd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scucd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scucd0`] module"]
#[doc(alias = "SCUCD0")]
pub type Scucd0 = crate::Reg<scucd0::Scucd0Spec>;
#[doc = "Secure2 Control 21 Register"]
pub mod scucd0;
#[doc = "SCUCDC (rw) register accessor: Secure2 Control 24 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scucdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scucdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scucdc`] module"]
#[doc(alias = "SCUCDC")]
pub type Scucdc = crate::Reg<scucdc::ScucdcSpec>;
#[doc = "Secure2 Control 24 Register"]
pub mod scucdc;
#[doc = "SCUD04 (rw) register accessor: Secure3 Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud04`] module"]
#[doc(alias = "SCUD04")]
pub type Scud04 = crate::Reg<scud04::Scud04Spec>;
#[doc = "Secure3 Control 2 Register"]
pub mod scud04;
#[doc = "SCUD08 (rw) register accessor: Secure3 Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud08`] module"]
#[doc(alias = "SCUD08")]
pub type Scud08 = crate::Reg<scud08::Scud08Spec>;
#[doc = "Secure3 Control 3 Register"]
pub mod scud08;
#[doc = "SCUD0C (rw) register accessor: Secure3 Control 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud0c`] module"]
#[doc(alias = "SCUD0C")]
pub type Scud0c = crate::Reg<scud0c::Scud0cSpec>;
#[doc = "Secure3 Control 4 Register"]
pub mod scud0c;
#[doc = "SCUD14 (rw) register accessor: Secure3 Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud14`] module"]
#[doc(alias = "SCUD14")]
pub type Scud14 = crate::Reg<scud14::Scud14Spec>;
#[doc = "Secure3 Control 6 Register"]
pub mod scud14;
#[doc = "SCUD18 (rw) register accessor: Secure3 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud18`] module"]
#[doc(alias = "SCUD18")]
pub type Scud18 = crate::Reg<scud18::Scud18Spec>;
#[doc = "Secure3 Control 7 Register"]
pub mod scud18;
#[doc = "SCUD1C (rw) register accessor: Secure3 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud1c`] module"]
#[doc(alias = "SCUD1C")]
pub type Scud1c = crate::Reg<scud1c::Scud1cSpec>;
#[doc = "Secure3 Control 7 Register"]
pub mod scud1c;
#[doc = "SCUD28 (rw) register accessor: Secure3 Control 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud28`] module"]
#[doc(alias = "SCUD28")]
pub type Scud28 = crate::Reg<scud28::Scud28Spec>;
#[doc = "Secure3 Control 11 Register"]
pub mod scud28;
#[doc = "SCUD34 (rw) register accessor: Secure3 Control 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud34`] module"]
#[doc(alias = "SCUD34")]
pub type Scud34 = crate::Reg<scud34::Scud34Spec>;
#[doc = "Secure3 Control 14 Register"]
pub mod scud34;
#[doc = "SCUD48 (rw) register accessor: Secure3 Control 19 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud48`] module"]
#[doc(alias = "SCUD48")]
pub type Scud48 = crate::Reg<scud48::Scud48Spec>;
#[doc = "Secure3 Control 19 Register"]
pub mod scud48;
#[doc = "SCUD4C (rw) register accessor: Secure3 Control 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud4c`] module"]
#[doc(alias = "SCUD4C")]
pub type Scud4c = crate::Reg<scud4c::Scud4cSpec>;
#[doc = "Secure3 Control 20 Register"]
pub mod scud4c;
#[doc = "SCUD50 (rw) register accessor: Secure3 Control 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud50`] module"]
#[doc(alias = "SCUD50")]
pub type Scud50 = crate::Reg<scud50::Scud50Spec>;
#[doc = "Secure3 Control 21 Register"]
pub mod scud50;
#[doc = "SCUD5C (rw) register accessor: Secure3 Control 24 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scud5c`] module"]
#[doc(alias = "SCUD5C")]
pub type Scud5c = crate::Reg<scud5c::Scud5cSpec>;
#[doc = "Secure3 Control 24 Register"]
pub mod scud5c;
#[doc = "SCUE00 (rw) register accessor: Write Protection 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue00`] module"]
#[doc(alias = "SCUE00")]
pub type Scue00 = crate::Reg<scue00::Scue00Spec>;
#[doc = "Write Protection 1 Register"]
pub mod scue00;
#[doc = "SCUE04 (rw) register accessor: Write Protection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue04`] module"]
#[doc(alias = "SCUE04")]
pub type Scue04 = crate::Reg<scue04::Scue04Spec>;
#[doc = "Write Protection 2 Register"]
pub mod scue04;
#[doc = "SCUE08 (rw) register accessor: Write Protection 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue08`] module"]
#[doc(alias = "SCUE08")]
pub type Scue08 = crate::Reg<scue08::Scue08Spec>;
#[doc = "Write Protection 3 Register"]
pub mod scue08;
#[doc = "SCUE0C (rw) register accessor: Write Protection 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue0c`] module"]
#[doc(alias = "SCUE0C")]
pub type Scue0c = crate::Reg<scue0c::Scue0cSpec>;
#[doc = "Write Protection 4 Register"]
pub mod scue0c;
#[doc = "SCUE10 (rw) register accessor: Write Protection 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue10`] module"]
#[doc(alias = "SCUE10")]
pub type Scue10 = crate::Reg<scue10::Scue10Spec>;
#[doc = "Write Protection 5 Register"]
pub mod scue10;
#[doc = "SCUE14 (rw) register accessor: Write Protection 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue14`] module"]
#[doc(alias = "SCUE14")]
pub type Scue14 = crate::Reg<scue14::Scue14Spec>;
#[doc = "Write Protection 6 Register"]
pub mod scue14;
#[doc = "SCUE18 (rw) register accessor: Write Protection 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue18`] module"]
#[doc(alias = "SCUE18")]
pub type Scue18 = crate::Reg<scue18::Scue18Spec>;
#[doc = "Write Protection 7 Register"]
pub mod scue18;
#[doc = "SCUE1C (rw) register accessor: Write Protection 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue1c`] module"]
#[doc(alias = "SCUE1C")]
pub type Scue1c = crate::Reg<scue1c::Scue1cSpec>;
#[doc = "Write Protection 8 Register"]
pub mod scue1c;
#[doc = "SCUE24 (rw) register accessor: Write Protection 10 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue24`] module"]
#[doc(alias = "SCUE24")]
pub type Scue24 = crate::Reg<scue24::Scue24Spec>;
#[doc = "Write Protection 10 Register"]
pub mod scue24;
#[doc = "SCUE28 (rw) register accessor: Write Protection 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue28`] module"]
#[doc(alias = "SCUE28")]
pub type Scue28 = crate::Reg<scue28::Scue28Spec>;
#[doc = "Write Protection 11 Register"]
pub mod scue28;
#[doc = "SCUE2C (rw) register accessor: Write Protection 12 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue2c`] module"]
#[doc(alias = "SCUE2C")]
pub type Scue2c = crate::Reg<scue2c::Scue2cSpec>;
#[doc = "Write Protection 12 Register"]
pub mod scue2c;
#[doc = "SCUE30 (rw) register accessor: Write Protection 13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue30`] module"]
#[doc(alias = "SCUE30")]
pub type Scue30 = crate::Reg<scue30::Scue30Spec>;
#[doc = "Write Protection 13 Register"]
pub mod scue30;
#[doc = "SCUE34 (rw) register accessor: Write Protection 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue34`] module"]
#[doc(alias = "SCUE34")]
pub type Scue34 = crate::Reg<scue34::Scue34Spec>;
#[doc = "Write Protection 14 Register"]
pub mod scue34;
#[doc = "SCUE38 (rw) register accessor: Write Protection 15 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue38`] module"]
#[doc(alias = "SCUE38")]
pub type Scue38 = crate::Reg<scue38::Scue38Spec>;
#[doc = "Write Protection 15 Register"]
pub mod scue38;
#[doc = "SCUE3C (rw) register accessor: Write Protection 16 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue3c`] module"]
#[doc(alias = "SCUE3C")]
pub type Scue3c = crate::Reg<scue3c::Scue3cSpec>;
#[doc = "Write Protection 16 Register"]
pub mod scue3c;
#[doc = "SCUE40 (rw) register accessor: Write Protection 17 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue40`] module"]
#[doc(alias = "SCUE40")]
pub type Scue40 = crate::Reg<scue40::Scue40Spec>;
#[doc = "Write Protection 17 Register"]
pub mod scue40;
#[doc = "SCUE44 (rw) register accessor: Write Protection 18 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue44`] module"]
#[doc(alias = "SCUE44")]
pub type Scue44 = crate::Reg<scue44::Scue44Spec>;
#[doc = "Write Protection 18 Register"]
pub mod scue44;
#[doc = "SCUE48 (rw) register accessor: Write Protection 19 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue48`] module"]
#[doc(alias = "SCUE48")]
pub type Scue48 = crate::Reg<scue48::Scue48Spec>;
#[doc = "Write Protection 19 Register"]
pub mod scue48;
#[doc = "SCUE4C (rw) register accessor: Write Protection 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue4c`] module"]
#[doc(alias = "SCUE4C")]
pub type Scue4c = crate::Reg<scue4c::Scue4cSpec>;
#[doc = "Write Protection 20 Register"]
pub mod scue4c;
#[doc = "SCUE50 (rw) register accessor: Write Protection 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue50`] module"]
#[doc(alias = "SCUE50")]
pub type Scue50 = crate::Reg<scue50::Scue50Spec>;
#[doc = "Write Protection 21 Register"]
pub mod scue50;
#[doc = "SCUE54 (rw) register accessor: Write Protection 22 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue54`] module"]
#[doc(alias = "SCUE54")]
pub type Scue54 = crate::Reg<scue54::Scue54Spec>;
#[doc = "Write Protection 22 Register"]
pub mod scue54;
#[doc = "SCUE5C (rw) register accessor: Write Protection 23 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue5c`] module"]
#[doc(alias = "SCUE5C")]
pub type Scue5c = crate::Reg<scue5c::Scue5cSpec>;
#[doc = "Write Protection 23 Register"]
pub mod scue5c;
#[doc = "SCUE60 (rw) register accessor: Write Protection 24 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue60`] module"]
#[doc(alias = "SCUE60")]
pub type Scue60 = crate::Reg<scue60::Scue60Spec>;
#[doc = "Write Protection 24 Register"]
pub mod scue60;
#[doc = "SCUE64 (rw) register accessor: Write Protection 26 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue64`] module"]
#[doc(alias = "SCUE64")]
pub type Scue64 = crate::Reg<scue64::Scue64Spec>;
#[doc = "Write Protection 26 Register"]
pub mod scue64;
#[doc = "SCUE68 (rw) register accessor: Write Protection 27 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue68`] module"]
#[doc(alias = "SCUE68")]
pub type Scue68 = crate::Reg<scue68::Scue68Spec>;
#[doc = "Write Protection 27 Register"]
pub mod scue68;
#[doc = "SCUE70 (rw) register accessor: Write Protection 29 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue70`] module"]
#[doc(alias = "SCUE70")]
pub type Scue70 = crate::Reg<scue70::Scue70Spec>;
#[doc = "Write Protection 29 Register"]
pub mod scue70;
#[doc = "SCUE78 (rw) register accessor: Write Protection 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scue78`] module"]
#[doc(alias = "SCUE78")]
pub type Scue78 = crate::Reg<scue78::Scue78Spec>;
#[doc = "Write Protection 31 Register"]
pub mod scue78;
#[doc = "SCUF00 (rw) register accessor: Reset Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf00`] module"]
#[doc(alias = "SCUF00")]
pub type Scuf00 = crate::Reg<scuf00::Scuf00Spec>;
#[doc = "Reset Control 1 Register"]
pub mod scuf00;
#[doc = "SCUF04 (rw) register accessor: Reset Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf04`] module"]
#[doc(alias = "SCUF04")]
pub type Scuf04 = crate::Reg<scuf04::Scuf04Spec>;
#[doc = "Reset Control 2 Register"]
pub mod scuf04;
#[doc = "SCUF08 (rw) register accessor: Reset Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf08`] module"]
#[doc(alias = "SCUF08")]
pub type Scuf08 = crate::Reg<scuf08::Scuf08Spec>;
#[doc = "Reset Control 3 Register"]
pub mod scuf08;
#[doc = "SCUF0C (rw) register accessor: Reset Control 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf0c`] module"]
#[doc(alias = "SCUF0C")]
pub type Scuf0c = crate::Reg<scuf0c::Scuf0cSpec>;
#[doc = "Reset Control 4 Register"]
pub mod scuf0c;
#[doc = "SCUF10 (rw) register accessor: Reset Control 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf10`] module"]
#[doc(alias = "SCUF10")]
pub type Scuf10 = crate::Reg<scuf10::Scuf10Spec>;
#[doc = "Reset Control 5 Register"]
pub mod scuf10;
#[doc = "SCUF14 (rw) register accessor: Reset Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf14`] module"]
#[doc(alias = "SCUF14")]
pub type Scuf14 = crate::Reg<scuf14::Scuf14Spec>;
#[doc = "Reset Control 6 Register"]
pub mod scuf14;
#[doc = "SCUF18 (rw) register accessor: Reset Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf18`] module"]
#[doc(alias = "SCUF18")]
pub type Scuf18 = crate::Reg<scuf18::Scuf18Spec>;
#[doc = "Reset Control 7 Register"]
pub mod scuf18;
#[doc = "SCUF1C (rw) register accessor: Reset Control 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf1c`] module"]
#[doc(alias = "SCUF1C")]
pub type Scuf1c = crate::Reg<scuf1c::Scuf1cSpec>;
#[doc = "Reset Control 8 Register"]
pub mod scuf1c;
#[doc = "SCUF20 (rw) register accessor: Reset Control 9 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf20`] module"]
#[doc(alias = "SCUF20")]
pub type Scuf20 = crate::Reg<scuf20::Scuf20Spec>;
#[doc = "Reset Control 9 Register"]
pub mod scuf20;
#[doc = "SCUF24 (rw) register accessor: Reset Control 10 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf24`] module"]
#[doc(alias = "SCUF24")]
pub type Scuf24 = crate::Reg<scuf24::Scuf24Spec>;
#[doc = "Reset Control 10 Register"]
pub mod scuf24;
#[doc = "SCUF28 (rw) register accessor: Reset Control 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf28`] module"]
#[doc(alias = "SCUF28")]
pub type Scuf28 = crate::Reg<scuf28::Scuf28Spec>;
#[doc = "Reset Control 11 Register"]
pub mod scuf28;
#[doc = "SCUF2C (rw) register accessor: Reset Control 12 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf2c`] module"]
#[doc(alias = "SCUF2C")]
pub type Scuf2c = crate::Reg<scuf2c::Scuf2cSpec>;
#[doc = "Reset Control 12 Register"]
pub mod scuf2c;
#[doc = "SCUF30 (rw) register accessor: Reset Control 13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf30`] module"]
#[doc(alias = "SCUF30")]
pub type Scuf30 = crate::Reg<scuf30::Scuf30Spec>;
#[doc = "Reset Control 13 Register"]
pub mod scuf30;
#[doc = "SCUF34 (rw) register accessor: Reset Control 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf34`] module"]
#[doc(alias = "SCUF34")]
pub type Scuf34 = crate::Reg<scuf34::Scuf34Spec>;
#[doc = "Reset Control 14 Register"]
pub mod scuf34;
#[doc = "SCUF38 (rw) register accessor: Reset Control 15 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf38`] module"]
#[doc(alias = "SCUF38")]
pub type Scuf38 = crate::Reg<scuf38::Scuf38Spec>;
#[doc = "Reset Control 15 Register"]
pub mod scuf38;
#[doc = "SCUF3C (rw) register accessor: Reset Control 16 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf3c`] module"]
#[doc(alias = "SCUF3C")]
pub type Scuf3c = crate::Reg<scuf3c::Scuf3cSpec>;
#[doc = "Reset Control 16 Register"]
pub mod scuf3c;
#[doc = "SCUF48 (rw) register accessor: Reset Control 19 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf48`] module"]
#[doc(alias = "SCUF48")]
pub type Scuf48 = crate::Reg<scuf48::Scuf48Spec>;
#[doc = "Reset Control 19 Register"]
pub mod scuf48;
#[doc = "SCUF4C (rw) register accessor: Reset Control 20 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf4c`] module"]
#[doc(alias = "SCUF4C")]
pub type Scuf4c = crate::Reg<scuf4c::Scuf4cSpec>;
#[doc = "Reset Control 20 Register"]
pub mod scuf4c;
#[doc = "SCUF50 (rw) register accessor: Reset Control 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf50`] module"]
#[doc(alias = "SCUF50")]
pub type Scuf50 = crate::Reg<scuf50::Scuf50Spec>;
#[doc = "Reset Control 21 Register"]
pub mod scuf50;
#[doc = "SCUF5C (rw) register accessor: Reset Control 24 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf5c`] module"]
#[doc(alias = "SCUF5C")]
pub type Scuf5c = crate::Reg<scuf5c::Scuf5cSpec>;
#[doc = "Reset Control 24 Register"]
pub mod scuf5c;
#[doc = "SCUF60 (rw) register accessor: Reset Control 25 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf60`] module"]
#[doc(alias = "SCUF60")]
pub type Scuf60 = crate::Reg<scuf60::Scuf60Spec>;
#[doc = "Reset Control 25 Register"]
pub mod scuf60;
#[doc = "SCUF64 (rw) register accessor: Reset Control 26 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf64`] module"]
#[doc(alias = "SCUF64")]
pub type Scuf64 = crate::Reg<scuf64::Scuf64Spec>;
#[doc = "Reset Control 26 Register"]
pub mod scuf64;
#[doc = "SCUF68 (rw) register accessor: Reset Control 27 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf68`] module"]
#[doc(alias = "SCUF68")]
pub type Scuf68 = crate::Reg<scuf68::Scuf68Spec>;
#[doc = "Reset Control 27 Register"]
pub mod scuf68;
#[doc = "SCUF6C (rw) register accessor: Reset Control 28 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf6c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf6c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf6c`] module"]
#[doc(alias = "SCUF6C")]
pub type Scuf6c = crate::Reg<scuf6c::Scuf6cSpec>;
#[doc = "Reset Control 28 Register"]
pub mod scuf6c;
#[doc = "SCUF70 (rw) register accessor: Reset Control 29 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf70`] module"]
#[doc(alias = "SCUF70")]
pub type Scuf70 = crate::Reg<scuf70::Scuf70Spec>;
#[doc = "Reset Control 29 Register"]
pub mod scuf70;
#[doc = "SCUF78 (rw) register accessor: Reset Control 31 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf78`] module"]
#[doc(alias = "SCUF78")]
pub type Scuf78 = crate::Reg<scuf78::Scuf78Spec>;
#[doc = "Reset Control 31 Register"]
pub mod scuf78;
