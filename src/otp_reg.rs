#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    otp_reg000: OtpReg000,
    otp_reg004: OtpReg004,
    otp_reg008: OtpReg008,
    otp_reg00c: OtpReg00c,
    otp_reg010: OtpReg010,
    otp_reg014: OtpReg014,
    otp_reg018: OtpReg018,
    otp_reg01c: OtpReg01c,
    otp_reg020: OtpReg020,
    otp_reg024: OtpReg024,
    otp_reg028: OtpReg028,
    otp_reg02c: OtpReg02c,
    otp_reg030: OtpReg030,
    otp_reg034: OtpReg034,
    otp_reg038: OtpReg038,
    otp_reg03c: OtpReg03c,
    otp_reg040: OtpReg040,
    otp_reg044: OtpReg044,
    otp_reg048: OtpReg048,
    otp_reg04c: OtpReg04c,
    otp_reg050: OtpReg050,
    otp_reg054: OtpReg054,
    otp_reg058: OtpReg058,
    otp_reg05c: OtpReg05c,
    otp_reg060: OtpReg060,
    otp_reg064: OtpReg064,
    otp_reg068: OtpReg068,
    otp_reg06c: OtpReg06c,
    otp_reg070: OtpReg070,
    otp_reg074: OtpReg074,
    otp_reg078: OtpReg078,
    otp_reg07c: OtpReg07c,
    otp_reg080: OtpReg080,
    otp_reg084: OtpReg084,
    otp_reg088: OtpReg088,
    otp_reg08c: OtpReg08c,
    otp_reg090: OtpReg090,
    otp_reg094: OtpReg094,
    otp_reg098: OtpReg098,
    otp_reg09c: OtpReg09c,
    otp_reg0a0: OtpReg0a0,
    otp_reg0a4: OtpReg0a4,
    otp_reg0a8: OtpReg0a8,
    otp_reg0ac: OtpReg0ac,
    otp_reg0b0: OtpReg0b0,
    otp_reg0b4: OtpReg0b4,
    otp_reg0b8: OtpReg0b8,
    otp_reg0bc: OtpReg0bc,
    otp_reg0c0: OtpReg0c0,
    otp_reg0c4: OtpReg0c4,
    otp_reg0c8: OtpReg0c8,
    otp_reg0cc: OtpReg0cc,
    otp_reg0d0: OtpReg0d0,
    otp_reg0d4: OtpReg0d4,
    otp_reg0d8: OtpReg0d8,
    otp_reg0dc: OtpReg0dc,
    otp_reg0e0: OtpReg0e0,
    _reserved57: [u8; 0x0c],
    otp_reg0f0: OtpReg0f0,
    otp_reg0f4: OtpReg0f4,
    otp_reg0f8: OtpReg0f8,
    _reserved60: [u8; 0x04],
    otp_reg100: OtpReg100,
    otp_reg104: OtpReg104,
    otp_reg108: OtpReg108,
    otp_reg10c: OtpReg10c,
    _reserved64: [u8; 0x10],
    otp_reg120: OtpReg120,
    otp_reg124: OtpReg124,
    otp_reg128: OtpReg128,
    otp_reg12c: OtpReg12c,
    otp_reg130: OtpReg130,
    otp_reg134: OtpReg134,
    otp_reg138: OtpReg138,
    otp_reg13c: OtpReg13c,
    otp_reg140: OtpReg140,
    otp_reg144: OtpReg144,
    otp_reg148: OtpReg148,
    otp_reg14c: OtpReg14c,
    otp_reg150: OtpReg150,
    otp_reg154: OtpReg154,
    otp_reg158: OtpReg158,
    otp_reg15c: OtpReg15c,
    otp_reg160: OtpReg160,
    otp_reg164: OtpReg164,
    otp_reg168: OtpReg168,
    otp_reg16c: OtpReg16c,
    otp_reg170: OtpReg170,
    otp_reg174: OtpReg174,
    otp_reg178: OtpReg178,
    otp_reg17c: OtpReg17c,
    otp_reg180: OtpReg180,
    otp_reg184: OtpReg184,
    otp_reg188: OtpReg188,
    otp_reg18c: OtpReg18c,
    _reserved92: [u8; 0x10],
    otp_reg1a0: OtpReg1a0,
    _reserved93: [u8; 0x0c],
    otp_reg1b0: OtpReg1b0,
    otp_reg1b4: OtpReg1b4,
    otp_reg1b8: OtpReg1b8,
    otp_reg1bc: OtpReg1bc,
    otp_reg1c0: OtpReg1c0,
    otp_reg1c4: OtpReg1c4,
    otp_reg1c8: OtpReg1c8,
    otp_reg1cc: OtpReg1cc,
    otp_reg1d0: OtpReg1d0,
    otp_reg1d4: OtpReg1d4,
    otp_reg1d8: OtpReg1d8,
    otp_reg1dc: OtpReg1dc,
    otp_reg1e0: OtpReg1e0,
    otp_reg1e4: OtpReg1e4,
    otp_reg1e8: OtpReg1e8,
    otp_reg1ec: OtpReg1ec,
    otp_reg1f0: OtpReg1f0,
    otp_reg1f4: OtpReg1f4,
    _reserved111: [u8; 0x08],
    otp_reg200: OtpReg200,
    otp_reg204: OtpReg204,
    otp_reg208: OtpReg208,
    otp_reg20c: OtpReg20c,
    otp_reg210: OtpReg210,
    otp_reg214: OtpReg214,
    _reserved117: [u8; 0xe8],
    otp_reg300: OtpReg300,
    otp_reg304: OtpReg304,
    otp_reg308: OtpReg308,
    otp_reg30c: OtpReg30c,
    otp_reg310: OtpReg310,
    otp_reg314: OtpReg314,
    otp_reg318: OtpReg318,
    otp_reg31c: OtpReg31c,
    otp_reg320: OtpReg320,
    otp_reg324: OtpReg324,
    otp_reg328: OtpReg328,
    otp_reg32c: OtpReg32c,
}
impl RegisterBlock {
    #[doc = "0x00 - otp key"]
    #[inline(always)]
    pub const fn otp_reg000(&self) -> &OtpReg000 {
        &self.otp_reg000
    }
    #[doc = "0x04 - otp\\_cmd\\_m0"]
    #[inline(always)]
    pub const fn otp_reg004(&self) -> &OtpReg004 {
        &self.otp_reg004
    }
    #[doc = "0x08 - otp\\_wdata\\_m0\\_0"]
    #[inline(always)]
    pub const fn otp_reg008(&self) -> &OtpReg008 {
        &self.otp_reg008
    }
    #[doc = "0x0c - otp\\_wdata\\_m0\\_1"]
    #[inline(always)]
    pub const fn otp_reg00c(&self) -> &OtpReg00c {
        &self.otp_reg00c
    }
    #[doc = "0x10 - otp\\_wdata\\_m0\\_2"]
    #[inline(always)]
    pub const fn otp_reg010(&self) -> &OtpReg010 {
        &self.otp_reg010
    }
    #[doc = "0x14 - otp\\_wdata\\_m0\\_3"]
    #[inline(always)]
    pub const fn otp_reg014(&self) -> &OtpReg014 {
        &self.otp_reg014
    }
    #[doc = "0x18 - otp\\_states\\_m0"]
    #[inline(always)]
    pub const fn otp_reg018(&self) -> &OtpReg018 {
        &self.otp_reg018
    }
    #[doc = "0x1c - otp\\_addr\\_m0"]
    #[inline(always)]
    pub const fn otp_reg01c(&self) -> &OtpReg01c {
        &self.otp_reg01c
    }
    #[doc = "0x20 - otp\\_rdata\\_m0"]
    #[inline(always)]
    pub const fn otp_reg020(&self) -> &OtpReg020 {
        &self.otp_reg020
    }
    #[doc = "0x24 - otp\\_cmd\\_m1"]
    #[inline(always)]
    pub const fn otp_reg024(&self) -> &OtpReg024 {
        &self.otp_reg024
    }
    #[doc = "0x28 - otp\\_wdata\\_m1\\_0"]
    #[inline(always)]
    pub const fn otp_reg028(&self) -> &OtpReg028 {
        &self.otp_reg028
    }
    #[doc = "0x2c - otp\\_wdata\\_m1\\_1"]
    #[inline(always)]
    pub const fn otp_reg02c(&self) -> &OtpReg02c {
        &self.otp_reg02c
    }
    #[doc = "0x30 - otp\\_wdata\\_m1\\_2"]
    #[inline(always)]
    pub const fn otp_reg030(&self) -> &OtpReg030 {
        &self.otp_reg030
    }
    #[doc = "0x34 - otp\\_wdata\\_m1\\_3"]
    #[inline(always)]
    pub const fn otp_reg034(&self) -> &OtpReg034 {
        &self.otp_reg034
    }
    #[doc = "0x38 - otp\\_states\\_m1"]
    #[inline(always)]
    pub const fn otp_reg038(&self) -> &OtpReg038 {
        &self.otp_reg038
    }
    #[doc = "0x3c - otp\\_addr\\_m1"]
    #[inline(always)]
    pub const fn otp_reg03c(&self) -> &OtpReg03c {
        &self.otp_reg03c
    }
    #[doc = "0x40 - otp\\_rdata\\_m1"]
    #[inline(always)]
    pub const fn otp_reg040(&self) -> &OtpReg040 {
        &self.otp_reg040
    }
    #[doc = "0x44 - otp\\_cmd\\_m2"]
    #[inline(always)]
    pub const fn otp_reg044(&self) -> &OtpReg044 {
        &self.otp_reg044
    }
    #[doc = "0x48 - otp\\_wdata\\_m2\\_0"]
    #[inline(always)]
    pub const fn otp_reg048(&self) -> &OtpReg048 {
        &self.otp_reg048
    }
    #[doc = "0x4c - otp\\_wdata\\_m2\\_1"]
    #[inline(always)]
    pub const fn otp_reg04c(&self) -> &OtpReg04c {
        &self.otp_reg04c
    }
    #[doc = "0x50 - otp\\_wdata\\_m2\\_2"]
    #[inline(always)]
    pub const fn otp_reg050(&self) -> &OtpReg050 {
        &self.otp_reg050
    }
    #[doc = "0x54 - otp\\_wdata\\_m2\\_3"]
    #[inline(always)]
    pub const fn otp_reg054(&self) -> &OtpReg054 {
        &self.otp_reg054
    }
    #[doc = "0x58 - otp\\_states\\_m2"]
    #[inline(always)]
    pub const fn otp_reg058(&self) -> &OtpReg058 {
        &self.otp_reg058
    }
    #[doc = "0x5c - otp\\_addr\\_m2"]
    #[inline(always)]
    pub const fn otp_reg05c(&self) -> &OtpReg05c {
        &self.otp_reg05c
    }
    #[doc = "0x60 - otp\\_rdata\\_m2"]
    #[inline(always)]
    pub const fn otp_reg060(&self) -> &OtpReg060 {
        &self.otp_reg060
    }
    #[doc = "0x64 - otp\\_cmd\\_m3"]
    #[inline(always)]
    pub const fn otp_reg064(&self) -> &OtpReg064 {
        &self.otp_reg064
    }
    #[doc = "0x68 - otp\\_wdata\\_m3\\_0"]
    #[inline(always)]
    pub const fn otp_reg068(&self) -> &OtpReg068 {
        &self.otp_reg068
    }
    #[doc = "0x6c - otp\\_wdata\\_m3\\_1"]
    #[inline(always)]
    pub const fn otp_reg06c(&self) -> &OtpReg06c {
        &self.otp_reg06c
    }
    #[doc = "0x70 - otp\\_wdata\\_m3\\_2"]
    #[inline(always)]
    pub const fn otp_reg070(&self) -> &OtpReg070 {
        &self.otp_reg070
    }
    #[doc = "0x74 - otp\\_wdata\\_m3\\_3"]
    #[inline(always)]
    pub const fn otp_reg074(&self) -> &OtpReg074 {
        &self.otp_reg074
    }
    #[doc = "0x78 - otp\\_states\\_m3"]
    #[inline(always)]
    pub const fn otp_reg078(&self) -> &OtpReg078 {
        &self.otp_reg078
    }
    #[doc = "0x7c - otp\\_addr\\_m3"]
    #[inline(always)]
    pub const fn otp_reg07c(&self) -> &OtpReg07c {
        &self.otp_reg07c
    }
    #[doc = "0x80 - otp\\_rdata\\_m3"]
    #[inline(always)]
    pub const fn otp_reg080(&self) -> &OtpReg080 {
        &self.otp_reg080
    }
    #[doc = "0x84 - otp\\_cmd\\_m4"]
    #[inline(always)]
    pub const fn otp_reg084(&self) -> &OtpReg084 {
        &self.otp_reg084
    }
    #[doc = "0x88 - otp\\_wdata\\_m4\\_0"]
    #[inline(always)]
    pub const fn otp_reg088(&self) -> &OtpReg088 {
        &self.otp_reg088
    }
    #[doc = "0x8c - otp\\_wdata\\_m4\\_1"]
    #[inline(always)]
    pub const fn otp_reg08c(&self) -> &OtpReg08c {
        &self.otp_reg08c
    }
    #[doc = "0x90 - otp\\_wdata\\_m4\\_2"]
    #[inline(always)]
    pub const fn otp_reg090(&self) -> &OtpReg090 {
        &self.otp_reg090
    }
    #[doc = "0x94 - otp\\_wdata\\_m4\\_3"]
    #[inline(always)]
    pub const fn otp_reg094(&self) -> &OtpReg094 {
        &self.otp_reg094
    }
    #[doc = "0x98 - otp\\_states\\_m4"]
    #[inline(always)]
    pub const fn otp_reg098(&self) -> &OtpReg098 {
        &self.otp_reg098
    }
    #[doc = "0x9c - otp\\_addr\\_m4"]
    #[inline(always)]
    pub const fn otp_reg09c(&self) -> &OtpReg09c {
        &self.otp_reg09c
    }
    #[doc = "0xa0 - otp\\_rdata\\_m4"]
    #[inline(always)]
    pub const fn otp_reg0a0(&self) -> &OtpReg0a0 {
        &self.otp_reg0a0
    }
    #[doc = "0xa4 - otp\\_cmd\\_m5"]
    #[inline(always)]
    pub const fn otp_reg0a4(&self) -> &OtpReg0a4 {
        &self.otp_reg0a4
    }
    #[doc = "0xa8 - otp\\_wdata\\_m5\\_0"]
    #[inline(always)]
    pub const fn otp_reg0a8(&self) -> &OtpReg0a8 {
        &self.otp_reg0a8
    }
    #[doc = "0xac - otp\\_wdata\\_m5\\_1"]
    #[inline(always)]
    pub const fn otp_reg0ac(&self) -> &OtpReg0ac {
        &self.otp_reg0ac
    }
    #[doc = "0xb0 - otp\\_wdata\\_m5\\_2"]
    #[inline(always)]
    pub const fn otp_reg0b0(&self) -> &OtpReg0b0 {
        &self.otp_reg0b0
    }
    #[doc = "0xb4 - otp\\_wdata\\_m5\\_3"]
    #[inline(always)]
    pub const fn otp_reg0b4(&self) -> &OtpReg0b4 {
        &self.otp_reg0b4
    }
    #[doc = "0xb8 - otp\\_states\\_m5"]
    #[inline(always)]
    pub const fn otp_reg0b8(&self) -> &OtpReg0b8 {
        &self.otp_reg0b8
    }
    #[doc = "0xbc - otp\\_addr\\_m5"]
    #[inline(always)]
    pub const fn otp_reg0bc(&self) -> &OtpReg0bc {
        &self.otp_reg0bc
    }
    #[doc = "0xc0 - otp\\_rdata\\_m5"]
    #[inline(always)]
    pub const fn otp_reg0c0(&self) -> &OtpReg0c0 {
        &self.otp_reg0c0
    }
    #[doc = "0xc4 - otp\\_dbg\\_00"]
    #[inline(always)]
    pub const fn otp_reg0c4(&self) -> &OtpReg0c4 {
        &self.otp_reg0c4
    }
    #[doc = "0xc8 - otp\\_dbg\\_01"]
    #[inline(always)]
    pub const fn otp_reg0c8(&self) -> &OtpReg0c8 {
        &self.otp_reg0c8
    }
    #[doc = "0xcc - ecc\\_rlock"]
    #[inline(always)]
    pub const fn otp_reg0cc(&self) -> &OtpReg0cc {
        &self.otp_reg0cc
    }
    #[doc = "0xd0 - pid"]
    #[inline(always)]
    pub const fn otp_reg0d0(&self) -> &OtpReg0d0 {
        &self.otp_reg0d0
    }
    #[doc = "0xd4 - otp\\_eccbrp\\_en"]
    #[inline(always)]
    pub const fn otp_reg0d4(&self) -> &OtpReg0d4 {
        &self.otp_reg0d4
    }
    #[doc = "0xd8 - OTP\\_CMD\\_LOCK"]
    #[inline(always)]
    pub const fn otp_reg0d8(&self) -> &OtpReg0d8 {
        &self.otp_reg0d8
    }
    #[doc = "0xdc - otp\\_sw\\_reset"]
    #[inline(always)]
    pub const fn otp_reg0dc(&self) -> &OtpReg0dc {
        &self.otp_reg0dc
    }
    #[doc = "0xe0 - otp\\_trng\\_dbg"]
    #[inline(always)]
    pub const fn otp_reg0e0(&self) -> &OtpReg0e0 {
        &self.otp_reg0e0
    }
    #[doc = "0xf0 - otp\\_rom\\_clr"]
    #[inline(always)]
    pub const fn otp_reg0f0(&self) -> &OtpReg0f0 {
        &self.otp_reg0f0
    }
    #[doc = "0xf4 - otp\\_puf\\_ctrl"]
    #[inline(always)]
    pub const fn otp_reg0f4(&self) -> &OtpReg0f4 {
        &self.otp_reg0f4
    }
    #[doc = "0xf8 - otp\\_puf\\_sts"]
    #[inline(always)]
    pub const fn otp_reg0f8(&self) -> &OtpReg0f8 {
        &self.otp_reg0f8
    }
    #[doc = "0x100 - OTP\\_REGION\\_ROM\\_PATCH"]
    #[inline(always)]
    pub const fn otp_reg100(&self) -> &OtpReg100 {
        &self.otp_reg100
    }
    #[doc = "0x104 - OTP\\_REGION\\_OTPCFG"]
    #[inline(always)]
    pub const fn otp_reg104(&self) -> &OtpReg104 {
        &self.otp_reg104
    }
    #[doc = "0x108 - OTP\\_REGION\\_OTPSTRAP"]
    #[inline(always)]
    pub const fn otp_reg108(&self) -> &OtpReg108 {
        &self.otp_reg108
    }
    #[doc = "0x10c - OTP\\_REGION\\_OTP\\_CTRL"]
    #[inline(always)]
    pub const fn otp_reg10c(&self) -> &OtpReg10c {
        &self.otp_reg10c
    }
    #[doc = "0x120 - OTP\\_REGION\\_SECURE0"]
    #[inline(always)]
    pub const fn otp_reg120(&self) -> &OtpReg120 {
        &self.otp_reg120
    }
    #[doc = "0x124 - OTP\\_REGION\\_SECURE0\\_RANGE"]
    #[inline(always)]
    pub const fn otp_reg124(&self) -> &OtpReg124 {
        &self.otp_reg124
    }
    #[doc = "0x128 - OTP\\_REGION\\_SECURE1"]
    #[inline(always)]
    pub const fn otp_reg128(&self) -> &OtpReg128 {
        &self.otp_reg128
    }
    #[doc = "0x12c - OTP\\_REGION\\_SECURE\\_RANGE1"]
    #[inline(always)]
    pub const fn otp_reg12c(&self) -> &OtpReg12c {
        &self.otp_reg12c
    }
    #[doc = "0x130 - OTP\\_REGION\\_SECURE2"]
    #[inline(always)]
    pub const fn otp_reg130(&self) -> &OtpReg130 {
        &self.otp_reg130
    }
    #[doc = "0x134 - OTP\\_REGION\\_SECURE\\_RANGE2"]
    #[inline(always)]
    pub const fn otp_reg134(&self) -> &OtpReg134 {
        &self.otp_reg134
    }
    #[doc = "0x138 - OTP\\_REGION\\_SECURE3"]
    #[inline(always)]
    pub const fn otp_reg138(&self) -> &OtpReg138 {
        &self.otp_reg138
    }
    #[doc = "0x13c - OTP\\_REGION\\_SECURE\\_RANGE3"]
    #[inline(always)]
    pub const fn otp_reg13c(&self) -> &OtpReg13c {
        &self.otp_reg13c
    }
    #[doc = "0x140 - OTP\\_REGION\\_USR\\_0"]
    #[inline(always)]
    pub const fn otp_reg140(&self) -> &OtpReg140 {
        &self.otp_reg140
    }
    #[doc = "0x144 - OTP\\_REGION\\_USR\\_0\\_RANGE"]
    #[inline(always)]
    pub const fn otp_reg144(&self) -> &OtpReg144 {
        &self.otp_reg144
    }
    #[doc = "0x148 - OTP\\_REGION\\_USR\\_1"]
    #[inline(always)]
    pub const fn otp_reg148(&self) -> &OtpReg148 {
        &self.otp_reg148
    }
    #[doc = "0x14c - OTP\\_REGION\\_USR\\_1\\_RANGE"]
    #[inline(always)]
    pub const fn otp_reg14c(&self) -> &OtpReg14c {
        &self.otp_reg14c
    }
    #[doc = "0x150 - OTP\\_REGION\\_USR\\_2"]
    #[inline(always)]
    pub const fn otp_reg150(&self) -> &OtpReg150 {
        &self.otp_reg150
    }
    #[doc = "0x154 - OTP\\_REGION\\_USR\\_2\\_RANGE"]
    #[inline(always)]
    pub const fn otp_reg154(&self) -> &OtpReg154 {
        &self.otp_reg154
    }
    #[doc = "0x158 - OTP\\_REGION\\_USR\\_3"]
    #[inline(always)]
    pub const fn otp_reg158(&self) -> &OtpReg158 {
        &self.otp_reg158
    }
    #[doc = "0x15c - OTP\\_REGION\\_USR\\_3\\_RANGE"]
    #[inline(always)]
    pub const fn otp_reg15c(&self) -> &OtpReg15c {
        &self.otp_reg15c
    }
    #[doc = "0x160 - OTP\\_REGION\\_CALIPTRA\\_0"]
    #[inline(always)]
    pub const fn otp_reg160(&self) -> &OtpReg160 {
        &self.otp_reg160
    }
    #[doc = "0x164 - OTP\\_REGION\\_CALIPTRA\\_0\\_RANGE"]
    #[inline(always)]
    pub const fn otp_reg164(&self) -> &OtpReg164 {
        &self.otp_reg164
    }
    #[doc = "0x168 - OTP\\_REGION\\_CALIPTRA\\_1"]
    #[inline(always)]
    pub const fn otp_reg168(&self) -> &OtpReg168 {
        &self.otp_reg168
    }
    #[doc = "0x16c - OTP\\_REGION\\_CALIPTRA\\_1\\_RANGE"]
    #[inline(always)]
    pub const fn otp_reg16c(&self) -> &OtpReg16c {
        &self.otp_reg16c
    }
    #[doc = "0x170 - OTP\\_REGION\\_CALIPTRA\\_2"]
    #[inline(always)]
    pub const fn otp_reg170(&self) -> &OtpReg170 {
        &self.otp_reg170
    }
    #[doc = "0x174 - OTP\\_REGION\\_CALIPTRA\\_2\\_RANGE"]
    #[inline(always)]
    pub const fn otp_reg174(&self) -> &OtpReg174 {
        &self.otp_reg174
    }
    #[doc = "0x178 - OTP\\_REGION\\_CALIPTRA\\_3"]
    #[inline(always)]
    pub const fn otp_reg178(&self) -> &OtpReg178 {
        &self.otp_reg178
    }
    #[doc = "0x17c - OTP\\_REGION\\_CALIPTRA\\_3\\_RANGE"]
    #[inline(always)]
    pub const fn otp_reg17c(&self) -> &OtpReg17c {
        &self.otp_reg17c
    }
    #[doc = "0x180 - OTP\\_RBP\\_SOC\\_SVN"]
    #[inline(always)]
    pub const fn otp_reg180(&self) -> &OtpReg180 {
        &self.otp_reg180
    }
    #[doc = "0x184 - OTP\\_RBP\\_SOC\\_KEYRETIRE"]
    #[inline(always)]
    pub const fn otp_reg184(&self) -> &OtpReg184 {
        &self.otp_reg184
    }
    #[doc = "0x188 - OTP\\_RBP\\_CALIP\\_SVN"]
    #[inline(always)]
    pub const fn otp_reg188(&self) -> &OtpReg188 {
        &self.otp_reg188
    }
    #[doc = "0x18c - OTP\\_RBP\\_CALIP\\_KEYRETIRE"]
    #[inline(always)]
    pub const fn otp_reg18c(&self) -> &OtpReg18c {
        &self.otp_reg18c
    }
    #[doc = "0x1a0 - PUF"]
    #[inline(always)]
    pub const fn otp_reg1a0(&self) -> &OtpReg1a0 {
        &self.otp_reg1a0
    }
    #[doc = "0x1b0 - OTP\\_MASTER\\_ID"]
    #[inline(always)]
    pub const fn otp_reg1b0(&self) -> &OtpReg1b0 {
        &self.otp_reg1b0
    }
    #[doc = "0x1b4 - OTP\\_MASTER\\_ID\\_EXT"]
    #[inline(always)]
    pub const fn otp_reg1b4(&self) -> &OtpReg1b4 {
        &self.otp_reg1b4
    }
    #[doc = "0x1b8 - OTP\\_R\\_MASTER\\_ID"]
    #[inline(always)]
    pub const fn otp_reg1b8(&self) -> &OtpReg1b8 {
        &self.otp_reg1b8
    }
    #[doc = "0x1bc - OTP\\_R\\_MASTER\\_ID\\_EXT"]
    #[inline(always)]
    pub const fn otp_reg1bc(&self) -> &OtpReg1bc {
        &self.otp_reg1bc
    }
    #[doc = "0x1c0 - OTP\\_SOC\\_ECCKEY"]
    #[inline(always)]
    pub const fn otp_reg1c0(&self) -> &OtpReg1c0 {
        &self.otp_reg1c0
    }
    #[doc = "0x1c4 - OTP\\_SEC\\_BOOT\\_EN"]
    #[inline(always)]
    pub const fn otp_reg1c4(&self) -> &OtpReg1c4 {
        &self.otp_reg1c4
    }
    #[doc = "0x1c8 - OTP\\_SOC\\_KEY"]
    #[inline(always)]
    pub const fn otp_reg1c8(&self) -> &OtpReg1c8 {
        &self.otp_reg1c8
    }
    #[doc = "0x1cc - OTP\\_CALPITRA\\_MANU\\_KEY"]
    #[inline(always)]
    pub const fn otp_reg1cc(&self) -> &OtpReg1cc {
        &self.otp_reg1cc
    }
    #[doc = "0x1d0 - OTP\\_CALPITRA\\_OWNER\\_KEY"]
    #[inline(always)]
    pub const fn otp_reg1d0(&self) -> &OtpReg1d0 {
        &self.otp_reg1d0
    }
    #[doc = "0x1d4 - OTP\\_FW\\_ID\\_LSB"]
    #[inline(always)]
    pub const fn otp_reg1d4(&self) -> &OtpReg1d4 {
        &self.otp_reg1d4
    }
    #[doc = "0x1d8 - OTP\\_FW\\_ID\\_MSB"]
    #[inline(always)]
    pub const fn otp_reg1d8(&self) -> &OtpReg1d8 {
        &self.otp_reg1d8
    }
    #[doc = "0x1dc - OTP\\_CALIP\\_FMC\\_SVN"]
    #[inline(always)]
    pub const fn otp_reg1dc(&self) -> &OtpReg1dc {
        &self.otp_reg1dc
    }
    #[doc = "0x1e0 - OTP\\_CALIP\\_RUNTIME\\_SVN0"]
    #[inline(always)]
    pub const fn otp_reg1e0(&self) -> &OtpReg1e0 {
        &self.otp_reg1e0
    }
    #[doc = "0x1e4 - OTP\\_CALIP\\_RUNTIME\\_SVN1"]
    #[inline(always)]
    pub const fn otp_reg1e4(&self) -> &OtpReg1e4 {
        &self.otp_reg1e4
    }
    #[doc = "0x1e8 - OTP\\_CALIP\\_RUNTIME\\_SVN2"]
    #[inline(always)]
    pub const fn otp_reg1e8(&self) -> &OtpReg1e8 {
        &self.otp_reg1e8
    }
    #[doc = "0x1ec - OTP\\_CALIP\\_RUNTIME\\_SVN3"]
    #[inline(always)]
    pub const fn otp_reg1ec(&self) -> &OtpReg1ec {
        &self.otp_reg1ec
    }
    #[doc = "0x1f0 - OTP\\_SVN\\_WLOCK"]
    #[inline(always)]
    pub const fn otp_reg1f0(&self) -> &OtpReg1f0 {
        &self.otp_reg1f0
    }
    #[doc = "0x1f4 - OTP\\_CMD\\_SOC2CALIP"]
    #[inline(always)]
    pub const fn otp_reg1f4(&self) -> &OtpReg1f4 {
        &self.otp_reg1f4
    }
    #[doc = "0x200 - otp\\_interrupt\\_en"]
    #[inline(always)]
    pub const fn otp_reg200(&self) -> &OtpReg200 {
        &self.otp_reg200
    }
    #[doc = "0x204 - otp\\_interrupt\\_status"]
    #[inline(always)]
    pub const fn otp_reg204(&self) -> &OtpReg204 {
        &self.otp_reg204
    }
    #[doc = "0x208 - OTP\\_INTR\\_MID"]
    #[inline(always)]
    pub const fn otp_reg208(&self) -> &OtpReg208 {
        &self.otp_reg208
    }
    #[doc = "0x20c - OTP\\_INTR\\_FUNC\\_INFO"]
    #[inline(always)]
    pub const fn otp_reg20c(&self) -> &OtpReg20c {
        &self.otp_reg20c
    }
    #[doc = "0x210 - OTP\\_INTR\\_M\\_INFO"]
    #[inline(always)]
    pub const fn otp_reg210(&self) -> &OtpReg210 {
        &self.otp_reg210
    }
    #[doc = "0x214 - OTP\\_INTR\\_R\\_INFO"]
    #[inline(always)]
    pub const fn otp_reg214(&self) -> &OtpReg214 {
        &self.otp_reg214
    }
    #[doc = "0x300 - OTP\\_SW\\_USAGE0"]
    #[inline(always)]
    pub const fn otp_reg300(&self) -> &OtpReg300 {
        &self.otp_reg300
    }
    #[doc = "0x304 - OTP\\_SW\\_USAGE1"]
    #[inline(always)]
    pub const fn otp_reg304(&self) -> &OtpReg304 {
        &self.otp_reg304
    }
    #[doc = "0x308 - OTP\\_SW\\_USAGE2"]
    #[inline(always)]
    pub const fn otp_reg308(&self) -> &OtpReg308 {
        &self.otp_reg308
    }
    #[doc = "0x30c - OTP\\_SW\\_USAGE3"]
    #[inline(always)]
    pub const fn otp_reg30c(&self) -> &OtpReg30c {
        &self.otp_reg30c
    }
    #[doc = "0x310 - OTP\\_SW\\_USAGE4"]
    #[inline(always)]
    pub const fn otp_reg310(&self) -> &OtpReg310 {
        &self.otp_reg310
    }
    #[doc = "0x314 - OTP\\_SW\\_USAGE5"]
    #[inline(always)]
    pub const fn otp_reg314(&self) -> &OtpReg314 {
        &self.otp_reg314
    }
    #[doc = "0x318 - OTP\\_SW\\_USAGE6"]
    #[inline(always)]
    pub const fn otp_reg318(&self) -> &OtpReg318 {
        &self.otp_reg318
    }
    #[doc = "0x31c - OTP\\_SW\\_USAGE7"]
    #[inline(always)]
    pub const fn otp_reg31c(&self) -> &OtpReg31c {
        &self.otp_reg31c
    }
    #[doc = "0x320 - OTP\\_SW\\_USAGE8"]
    #[inline(always)]
    pub const fn otp_reg320(&self) -> &OtpReg320 {
        &self.otp_reg320
    }
    #[doc = "0x324 - OTP\\_SW\\_USAGE9"]
    #[inline(always)]
    pub const fn otp_reg324(&self) -> &OtpReg324 {
        &self.otp_reg324
    }
    #[doc = "0x328 - OTP\\_SW\\_USAGE10"]
    #[inline(always)]
    pub const fn otp_reg328(&self) -> &OtpReg328 {
        &self.otp_reg328
    }
    #[doc = "0x32c - OTP\\_SW\\_USAGE11"]
    #[inline(always)]
    pub const fn otp_reg32c(&self) -> &OtpReg32c {
        &self.otp_reg32c
    }
}
#[doc = "OTP_REG000 (rw) register accessor: otp key\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg000`] module"]
#[doc(alias = "OTP_REG000")]
pub type OtpReg000 = crate::Reg<otp_reg000::OtpReg000Spec>;
#[doc = "otp key"]
pub mod otp_reg000;
#[doc = "OTP_REG004 (rw) register accessor: otp\\_cmd\\_m0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg004`] module"]
#[doc(alias = "OTP_REG004")]
pub type OtpReg004 = crate::Reg<otp_reg004::OtpReg004Spec>;
#[doc = "otp\\_cmd\\_m0"]
pub mod otp_reg004;
#[doc = "OTP_REG008 (rw) register accessor: otp\\_wdata\\_m0\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg008`] module"]
#[doc(alias = "OTP_REG008")]
pub type OtpReg008 = crate::Reg<otp_reg008::OtpReg008Spec>;
#[doc = "otp\\_wdata\\_m0\\_0"]
pub mod otp_reg008;
#[doc = "OTP_REG00C (rw) register accessor: otp\\_wdata\\_m0\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg00c`] module"]
#[doc(alias = "OTP_REG00C")]
pub type OtpReg00c = crate::Reg<otp_reg00c::OtpReg00cSpec>;
#[doc = "otp\\_wdata\\_m0\\_1"]
pub mod otp_reg00c;
#[doc = "OTP_REG010 (rw) register accessor: otp\\_wdata\\_m0\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg010`] module"]
#[doc(alias = "OTP_REG010")]
pub type OtpReg010 = crate::Reg<otp_reg010::OtpReg010Spec>;
#[doc = "otp\\_wdata\\_m0\\_2"]
pub mod otp_reg010;
#[doc = "OTP_REG014 (rw) register accessor: otp\\_wdata\\_m0\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg014`] module"]
#[doc(alias = "OTP_REG014")]
pub type OtpReg014 = crate::Reg<otp_reg014::OtpReg014Spec>;
#[doc = "otp\\_wdata\\_m0\\_3"]
pub mod otp_reg014;
#[doc = "OTP_REG018 (rw) register accessor: otp\\_states\\_m0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg018`] module"]
#[doc(alias = "OTP_REG018")]
pub type OtpReg018 = crate::Reg<otp_reg018::OtpReg018Spec>;
#[doc = "otp\\_states\\_m0"]
pub mod otp_reg018;
#[doc = "OTP_REG01C (rw) register accessor: otp\\_addr\\_m0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg01c`] module"]
#[doc(alias = "OTP_REG01C")]
pub type OtpReg01c = crate::Reg<otp_reg01c::OtpReg01cSpec>;
#[doc = "otp\\_addr\\_m0"]
pub mod otp_reg01c;
#[doc = "OTP_REG020 (rw) register accessor: otp\\_rdata\\_m0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg020`] module"]
#[doc(alias = "OTP_REG020")]
pub type OtpReg020 = crate::Reg<otp_reg020::OtpReg020Spec>;
#[doc = "otp\\_rdata\\_m0"]
pub mod otp_reg020;
#[doc = "OTP_REG024 (rw) register accessor: otp\\_cmd\\_m1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg024`] module"]
#[doc(alias = "OTP_REG024")]
pub type OtpReg024 = crate::Reg<otp_reg024::OtpReg024Spec>;
#[doc = "otp\\_cmd\\_m1"]
pub mod otp_reg024;
#[doc = "OTP_REG028 (rw) register accessor: otp\\_wdata\\_m1\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg028`] module"]
#[doc(alias = "OTP_REG028")]
pub type OtpReg028 = crate::Reg<otp_reg028::OtpReg028Spec>;
#[doc = "otp\\_wdata\\_m1\\_0"]
pub mod otp_reg028;
#[doc = "OTP_REG02C (rw) register accessor: otp\\_wdata\\_m1\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg02c`] module"]
#[doc(alias = "OTP_REG02C")]
pub type OtpReg02c = crate::Reg<otp_reg02c::OtpReg02cSpec>;
#[doc = "otp\\_wdata\\_m1\\_1"]
pub mod otp_reg02c;
#[doc = "OTP_REG030 (rw) register accessor: otp\\_wdata\\_m1\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg030`] module"]
#[doc(alias = "OTP_REG030")]
pub type OtpReg030 = crate::Reg<otp_reg030::OtpReg030Spec>;
#[doc = "otp\\_wdata\\_m1\\_2"]
pub mod otp_reg030;
#[doc = "OTP_REG034 (rw) register accessor: otp\\_wdata\\_m1\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg034`] module"]
#[doc(alias = "OTP_REG034")]
pub type OtpReg034 = crate::Reg<otp_reg034::OtpReg034Spec>;
#[doc = "otp\\_wdata\\_m1\\_3"]
pub mod otp_reg034;
#[doc = "OTP_REG038 (rw) register accessor: otp\\_states\\_m1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg038`] module"]
#[doc(alias = "OTP_REG038")]
pub type OtpReg038 = crate::Reg<otp_reg038::OtpReg038Spec>;
#[doc = "otp\\_states\\_m1"]
pub mod otp_reg038;
#[doc = "OTP_REG03C (rw) register accessor: otp\\_addr\\_m1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg03c`] module"]
#[doc(alias = "OTP_REG03C")]
pub type OtpReg03c = crate::Reg<otp_reg03c::OtpReg03cSpec>;
#[doc = "otp\\_addr\\_m1"]
pub mod otp_reg03c;
#[doc = "OTP_REG040 (rw) register accessor: otp\\_rdata\\_m1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg040`] module"]
#[doc(alias = "OTP_REG040")]
pub type OtpReg040 = crate::Reg<otp_reg040::OtpReg040Spec>;
#[doc = "otp\\_rdata\\_m1"]
pub mod otp_reg040;
#[doc = "OTP_REG044 (rw) register accessor: otp\\_cmd\\_m2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg044`] module"]
#[doc(alias = "OTP_REG044")]
pub type OtpReg044 = crate::Reg<otp_reg044::OtpReg044Spec>;
#[doc = "otp\\_cmd\\_m2"]
pub mod otp_reg044;
#[doc = "OTP_REG048 (rw) register accessor: otp\\_wdata\\_m2\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg048`] module"]
#[doc(alias = "OTP_REG048")]
pub type OtpReg048 = crate::Reg<otp_reg048::OtpReg048Spec>;
#[doc = "otp\\_wdata\\_m2\\_0"]
pub mod otp_reg048;
#[doc = "OTP_REG04C (rw) register accessor: otp\\_wdata\\_m2\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg04c`] module"]
#[doc(alias = "OTP_REG04C")]
pub type OtpReg04c = crate::Reg<otp_reg04c::OtpReg04cSpec>;
#[doc = "otp\\_wdata\\_m2\\_1"]
pub mod otp_reg04c;
#[doc = "OTP_REG050 (rw) register accessor: otp\\_wdata\\_m2\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg050`] module"]
#[doc(alias = "OTP_REG050")]
pub type OtpReg050 = crate::Reg<otp_reg050::OtpReg050Spec>;
#[doc = "otp\\_wdata\\_m2\\_2"]
pub mod otp_reg050;
#[doc = "OTP_REG054 (rw) register accessor: otp\\_wdata\\_m2\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg054`] module"]
#[doc(alias = "OTP_REG054")]
pub type OtpReg054 = crate::Reg<otp_reg054::OtpReg054Spec>;
#[doc = "otp\\_wdata\\_m2\\_3"]
pub mod otp_reg054;
#[doc = "OTP_REG058 (rw) register accessor: otp\\_states\\_m2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg058`] module"]
#[doc(alias = "OTP_REG058")]
pub type OtpReg058 = crate::Reg<otp_reg058::OtpReg058Spec>;
#[doc = "otp\\_states\\_m2"]
pub mod otp_reg058;
#[doc = "OTP_REG05C (rw) register accessor: otp\\_addr\\_m2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg05c`] module"]
#[doc(alias = "OTP_REG05C")]
pub type OtpReg05c = crate::Reg<otp_reg05c::OtpReg05cSpec>;
#[doc = "otp\\_addr\\_m2"]
pub mod otp_reg05c;
#[doc = "OTP_REG060 (rw) register accessor: otp\\_rdata\\_m2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg060`] module"]
#[doc(alias = "OTP_REG060")]
pub type OtpReg060 = crate::Reg<otp_reg060::OtpReg060Spec>;
#[doc = "otp\\_rdata\\_m2"]
pub mod otp_reg060;
#[doc = "OTP_REG064 (rw) register accessor: otp\\_cmd\\_m3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg064`] module"]
#[doc(alias = "OTP_REG064")]
pub type OtpReg064 = crate::Reg<otp_reg064::OtpReg064Spec>;
#[doc = "otp\\_cmd\\_m3"]
pub mod otp_reg064;
#[doc = "OTP_REG068 (rw) register accessor: otp\\_wdata\\_m3\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg068`] module"]
#[doc(alias = "OTP_REG068")]
pub type OtpReg068 = crate::Reg<otp_reg068::OtpReg068Spec>;
#[doc = "otp\\_wdata\\_m3\\_0"]
pub mod otp_reg068;
#[doc = "OTP_REG06C (rw) register accessor: otp\\_wdata\\_m3\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg06c`] module"]
#[doc(alias = "OTP_REG06C")]
pub type OtpReg06c = crate::Reg<otp_reg06c::OtpReg06cSpec>;
#[doc = "otp\\_wdata\\_m3\\_1"]
pub mod otp_reg06c;
#[doc = "OTP_REG070 (rw) register accessor: otp\\_wdata\\_m3\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg070`] module"]
#[doc(alias = "OTP_REG070")]
pub type OtpReg070 = crate::Reg<otp_reg070::OtpReg070Spec>;
#[doc = "otp\\_wdata\\_m3\\_2"]
pub mod otp_reg070;
#[doc = "OTP_REG074 (rw) register accessor: otp\\_wdata\\_m3\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg074`] module"]
#[doc(alias = "OTP_REG074")]
pub type OtpReg074 = crate::Reg<otp_reg074::OtpReg074Spec>;
#[doc = "otp\\_wdata\\_m3\\_3"]
pub mod otp_reg074;
#[doc = "OTP_REG078 (rw) register accessor: otp\\_states\\_m3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg078`] module"]
#[doc(alias = "OTP_REG078")]
pub type OtpReg078 = crate::Reg<otp_reg078::OtpReg078Spec>;
#[doc = "otp\\_states\\_m3"]
pub mod otp_reg078;
#[doc = "OTP_REG07C (rw) register accessor: otp\\_addr\\_m3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg07c`] module"]
#[doc(alias = "OTP_REG07C")]
pub type OtpReg07c = crate::Reg<otp_reg07c::OtpReg07cSpec>;
#[doc = "otp\\_addr\\_m3"]
pub mod otp_reg07c;
#[doc = "OTP_REG080 (rw) register accessor: otp\\_rdata\\_m3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg080`] module"]
#[doc(alias = "OTP_REG080")]
pub type OtpReg080 = crate::Reg<otp_reg080::OtpReg080Spec>;
#[doc = "otp\\_rdata\\_m3"]
pub mod otp_reg080;
#[doc = "OTP_REG084 (rw) register accessor: otp\\_cmd\\_m4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg084`] module"]
#[doc(alias = "OTP_REG084")]
pub type OtpReg084 = crate::Reg<otp_reg084::OtpReg084Spec>;
#[doc = "otp\\_cmd\\_m4"]
pub mod otp_reg084;
#[doc = "OTP_REG088 (rw) register accessor: otp\\_wdata\\_m4\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg088`] module"]
#[doc(alias = "OTP_REG088")]
pub type OtpReg088 = crate::Reg<otp_reg088::OtpReg088Spec>;
#[doc = "otp\\_wdata\\_m4\\_0"]
pub mod otp_reg088;
#[doc = "OTP_REG08C (rw) register accessor: otp\\_wdata\\_m4\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg08c`] module"]
#[doc(alias = "OTP_REG08C")]
pub type OtpReg08c = crate::Reg<otp_reg08c::OtpReg08cSpec>;
#[doc = "otp\\_wdata\\_m4\\_1"]
pub mod otp_reg08c;
#[doc = "OTP_REG090 (rw) register accessor: otp\\_wdata\\_m4\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg090`] module"]
#[doc(alias = "OTP_REG090")]
pub type OtpReg090 = crate::Reg<otp_reg090::OtpReg090Spec>;
#[doc = "otp\\_wdata\\_m4\\_2"]
pub mod otp_reg090;
#[doc = "OTP_REG094 (rw) register accessor: otp\\_wdata\\_m4\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg094`] module"]
#[doc(alias = "OTP_REG094")]
pub type OtpReg094 = crate::Reg<otp_reg094::OtpReg094Spec>;
#[doc = "otp\\_wdata\\_m4\\_3"]
pub mod otp_reg094;
#[doc = "OTP_REG098 (rw) register accessor: otp\\_states\\_m4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg098`] module"]
#[doc(alias = "OTP_REG098")]
pub type OtpReg098 = crate::Reg<otp_reg098::OtpReg098Spec>;
#[doc = "otp\\_states\\_m4"]
pub mod otp_reg098;
#[doc = "OTP_REG09C (rw) register accessor: otp\\_addr\\_m4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg09c`] module"]
#[doc(alias = "OTP_REG09C")]
pub type OtpReg09c = crate::Reg<otp_reg09c::OtpReg09cSpec>;
#[doc = "otp\\_addr\\_m4"]
pub mod otp_reg09c;
#[doc = "OTP_REG0A0 (rw) register accessor: otp\\_rdata\\_m4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0a0`] module"]
#[doc(alias = "OTP_REG0A0")]
pub type OtpReg0a0 = crate::Reg<otp_reg0a0::OtpReg0a0Spec>;
#[doc = "otp\\_rdata\\_m4"]
pub mod otp_reg0a0;
#[doc = "OTP_REG0A4 (rw) register accessor: otp\\_cmd\\_m5\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0a4`] module"]
#[doc(alias = "OTP_REG0A4")]
pub type OtpReg0a4 = crate::Reg<otp_reg0a4::OtpReg0a4Spec>;
#[doc = "otp\\_cmd\\_m5"]
pub mod otp_reg0a4;
#[doc = "OTP_REG0A8 (rw) register accessor: otp\\_wdata\\_m5\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0a8`] module"]
#[doc(alias = "OTP_REG0A8")]
pub type OtpReg0a8 = crate::Reg<otp_reg0a8::OtpReg0a8Spec>;
#[doc = "otp\\_wdata\\_m5\\_0"]
pub mod otp_reg0a8;
#[doc = "OTP_REG0AC (rw) register accessor: otp\\_wdata\\_m5\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0ac`] module"]
#[doc(alias = "OTP_REG0AC")]
pub type OtpReg0ac = crate::Reg<otp_reg0ac::OtpReg0acSpec>;
#[doc = "otp\\_wdata\\_m5\\_1"]
pub mod otp_reg0ac;
#[doc = "OTP_REG0B0 (rw) register accessor: otp\\_wdata\\_m5\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0b0`] module"]
#[doc(alias = "OTP_REG0B0")]
pub type OtpReg0b0 = crate::Reg<otp_reg0b0::OtpReg0b0Spec>;
#[doc = "otp\\_wdata\\_m5\\_2"]
pub mod otp_reg0b0;
#[doc = "OTP_REG0B4 (rw) register accessor: otp\\_wdata\\_m5\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0b4`] module"]
#[doc(alias = "OTP_REG0B4")]
pub type OtpReg0b4 = crate::Reg<otp_reg0b4::OtpReg0b4Spec>;
#[doc = "otp\\_wdata\\_m5\\_3"]
pub mod otp_reg0b4;
#[doc = "OTP_REG0B8 (rw) register accessor: otp\\_states\\_m5\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0b8`] module"]
#[doc(alias = "OTP_REG0B8")]
pub type OtpReg0b8 = crate::Reg<otp_reg0b8::OtpReg0b8Spec>;
#[doc = "otp\\_states\\_m5"]
pub mod otp_reg0b8;
#[doc = "OTP_REG0BC (rw) register accessor: otp\\_addr\\_m5\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0bc`] module"]
#[doc(alias = "OTP_REG0BC")]
pub type OtpReg0bc = crate::Reg<otp_reg0bc::OtpReg0bcSpec>;
#[doc = "otp\\_addr\\_m5"]
pub mod otp_reg0bc;
#[doc = "OTP_REG0C0 (rw) register accessor: otp\\_rdata\\_m5\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0c0`] module"]
#[doc(alias = "OTP_REG0C0")]
pub type OtpReg0c0 = crate::Reg<otp_reg0c0::OtpReg0c0Spec>;
#[doc = "otp\\_rdata\\_m5"]
pub mod otp_reg0c0;
#[doc = "OTP_REG0C4 (rw) register accessor: otp\\_dbg\\_00\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0c4`] module"]
#[doc(alias = "OTP_REG0C4")]
pub type OtpReg0c4 = crate::Reg<otp_reg0c4::OtpReg0c4Spec>;
#[doc = "otp\\_dbg\\_00"]
pub mod otp_reg0c4;
#[doc = "OTP_REG0C8 (rw) register accessor: otp\\_dbg\\_01\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0c8`] module"]
#[doc(alias = "OTP_REG0C8")]
pub type OtpReg0c8 = crate::Reg<otp_reg0c8::OtpReg0c8Spec>;
#[doc = "otp\\_dbg\\_01"]
pub mod otp_reg0c8;
#[doc = "OTP_REG0CC (rw) register accessor: ecc\\_rlock\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0cc`] module"]
#[doc(alias = "OTP_REG0CC")]
pub type OtpReg0cc = crate::Reg<otp_reg0cc::OtpReg0ccSpec>;
#[doc = "ecc\\_rlock"]
pub mod otp_reg0cc;
#[doc = "OTP_REG0D0 (rw) register accessor: pid\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0d0`] module"]
#[doc(alias = "OTP_REG0D0")]
pub type OtpReg0d0 = crate::Reg<otp_reg0d0::OtpReg0d0Spec>;
#[doc = "pid"]
pub mod otp_reg0d0;
#[doc = "OTP_REG0D4 (rw) register accessor: otp\\_eccbrp\\_en\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0d4`] module"]
#[doc(alias = "OTP_REG0D4")]
pub type OtpReg0d4 = crate::Reg<otp_reg0d4::OtpReg0d4Spec>;
#[doc = "otp\\_eccbrp\\_en"]
pub mod otp_reg0d4;
#[doc = "OTP_REG0D8 (rw) register accessor: OTP\\_CMD\\_LOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0d8`] module"]
#[doc(alias = "OTP_REG0D8")]
pub type OtpReg0d8 = crate::Reg<otp_reg0d8::OtpReg0d8Spec>;
#[doc = "OTP\\_CMD\\_LOCK"]
pub mod otp_reg0d8;
#[doc = "OTP_REG0DC (rw) register accessor: otp\\_sw\\_reset\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0dc`] module"]
#[doc(alias = "OTP_REG0DC")]
pub type OtpReg0dc = crate::Reg<otp_reg0dc::OtpReg0dcSpec>;
#[doc = "otp\\_sw\\_reset"]
pub mod otp_reg0dc;
#[doc = "OTP_REG0E0 (rw) register accessor: otp\\_trng\\_dbg\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0e0`] module"]
#[doc(alias = "OTP_REG0E0")]
pub type OtpReg0e0 = crate::Reg<otp_reg0e0::OtpReg0e0Spec>;
#[doc = "otp\\_trng\\_dbg"]
pub mod otp_reg0e0;
#[doc = "OTP_REG0F0 (rw) register accessor: otp\\_rom\\_clr\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0f0`] module"]
#[doc(alias = "OTP_REG0F0")]
pub type OtpReg0f0 = crate::Reg<otp_reg0f0::OtpReg0f0Spec>;
#[doc = "otp\\_rom\\_clr"]
pub mod otp_reg0f0;
#[doc = "OTP_REG0F4 (rw) register accessor: otp\\_puf\\_ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0f4`] module"]
#[doc(alias = "OTP_REG0F4")]
pub type OtpReg0f4 = crate::Reg<otp_reg0f4::OtpReg0f4Spec>;
#[doc = "otp\\_puf\\_ctrl"]
pub mod otp_reg0f4;
#[doc = "OTP_REG0F8 (rw) register accessor: otp\\_puf\\_sts\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg0f8`] module"]
#[doc(alias = "OTP_REG0F8")]
pub type OtpReg0f8 = crate::Reg<otp_reg0f8::OtpReg0f8Spec>;
#[doc = "otp\\_puf\\_sts"]
pub mod otp_reg0f8;
#[doc = "OTP_REG100 (rw) register accessor: OTP\\_REGION\\_ROM\\_PATCH\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg100`] module"]
#[doc(alias = "OTP_REG100")]
pub type OtpReg100 = crate::Reg<otp_reg100::OtpReg100Spec>;
#[doc = "OTP\\_REGION\\_ROM\\_PATCH"]
pub mod otp_reg100;
#[doc = "OTP_REG104 (rw) register accessor: OTP\\_REGION\\_OTPCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg104`] module"]
#[doc(alias = "OTP_REG104")]
pub type OtpReg104 = crate::Reg<otp_reg104::OtpReg104Spec>;
#[doc = "OTP\\_REGION\\_OTPCFG"]
pub mod otp_reg104;
#[doc = "OTP_REG108 (rw) register accessor: OTP\\_REGION\\_OTPSTRAP\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg108`] module"]
#[doc(alias = "OTP_REG108")]
pub type OtpReg108 = crate::Reg<otp_reg108::OtpReg108Spec>;
#[doc = "OTP\\_REGION\\_OTPSTRAP"]
pub mod otp_reg108;
#[doc = "OTP_REG10C (rw) register accessor: OTP\\_REGION\\_OTP\\_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg10c`] module"]
#[doc(alias = "OTP_REG10C")]
pub type OtpReg10c = crate::Reg<otp_reg10c::OtpReg10cSpec>;
#[doc = "OTP\\_REGION\\_OTP\\_CTRL"]
pub mod otp_reg10c;
#[doc = "OTP_REG120 (rw) register accessor: OTP\\_REGION\\_SECURE0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg120`] module"]
#[doc(alias = "OTP_REG120")]
pub type OtpReg120 = crate::Reg<otp_reg120::OtpReg120Spec>;
#[doc = "OTP\\_REGION\\_SECURE0"]
pub mod otp_reg120;
#[doc = "OTP_REG124 (rw) register accessor: OTP\\_REGION\\_SECURE0\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg124`] module"]
#[doc(alias = "OTP_REG124")]
pub type OtpReg124 = crate::Reg<otp_reg124::OtpReg124Spec>;
#[doc = "OTP\\_REGION\\_SECURE0\\_RANGE"]
pub mod otp_reg124;
#[doc = "OTP_REG128 (rw) register accessor: OTP\\_REGION\\_SECURE1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg128`] module"]
#[doc(alias = "OTP_REG128")]
pub type OtpReg128 = crate::Reg<otp_reg128::OtpReg128Spec>;
#[doc = "OTP\\_REGION\\_SECURE1"]
pub mod otp_reg128;
#[doc = "OTP_REG12C (rw) register accessor: OTP\\_REGION\\_SECURE\\_RANGE1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg12c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg12c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg12c`] module"]
#[doc(alias = "OTP_REG12C")]
pub type OtpReg12c = crate::Reg<otp_reg12c::OtpReg12cSpec>;
#[doc = "OTP\\_REGION\\_SECURE\\_RANGE1"]
pub mod otp_reg12c;
#[doc = "OTP_REG130 (rw) register accessor: OTP\\_REGION\\_SECURE2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg130`] module"]
#[doc(alias = "OTP_REG130")]
pub type OtpReg130 = crate::Reg<otp_reg130::OtpReg130Spec>;
#[doc = "OTP\\_REGION\\_SECURE2"]
pub mod otp_reg130;
#[doc = "OTP_REG134 (rw) register accessor: OTP\\_REGION\\_SECURE\\_RANGE2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg134`] module"]
#[doc(alias = "OTP_REG134")]
pub type OtpReg134 = crate::Reg<otp_reg134::OtpReg134Spec>;
#[doc = "OTP\\_REGION\\_SECURE\\_RANGE2"]
pub mod otp_reg134;
#[doc = "OTP_REG138 (rw) register accessor: OTP\\_REGION\\_SECURE3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg138`] module"]
#[doc(alias = "OTP_REG138")]
pub type OtpReg138 = crate::Reg<otp_reg138::OtpReg138Spec>;
#[doc = "OTP\\_REGION\\_SECURE3"]
pub mod otp_reg138;
#[doc = "OTP_REG13C (rw) register accessor: OTP\\_REGION\\_SECURE\\_RANGE3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg13c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg13c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg13c`] module"]
#[doc(alias = "OTP_REG13C")]
pub type OtpReg13c = crate::Reg<otp_reg13c::OtpReg13cSpec>;
#[doc = "OTP\\_REGION\\_SECURE\\_RANGE3"]
pub mod otp_reg13c;
#[doc = "OTP_REG140 (rw) register accessor: OTP\\_REGION\\_USR\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg140`] module"]
#[doc(alias = "OTP_REG140")]
pub type OtpReg140 = crate::Reg<otp_reg140::OtpReg140Spec>;
#[doc = "OTP\\_REGION\\_USR\\_0"]
pub mod otp_reg140;
#[doc = "OTP_REG144 (rw) register accessor: OTP\\_REGION\\_USR\\_0\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg144`] module"]
#[doc(alias = "OTP_REG144")]
pub type OtpReg144 = crate::Reg<otp_reg144::OtpReg144Spec>;
#[doc = "OTP\\_REGION\\_USR\\_0\\_RANGE"]
pub mod otp_reg144;
#[doc = "OTP_REG148 (rw) register accessor: OTP\\_REGION\\_USR\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg148`] module"]
#[doc(alias = "OTP_REG148")]
pub type OtpReg148 = crate::Reg<otp_reg148::OtpReg148Spec>;
#[doc = "OTP\\_REGION\\_USR\\_1"]
pub mod otp_reg148;
#[doc = "OTP_REG14C (rw) register accessor: OTP\\_REGION\\_USR\\_1\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg14c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg14c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg14c`] module"]
#[doc(alias = "OTP_REG14C")]
pub type OtpReg14c = crate::Reg<otp_reg14c::OtpReg14cSpec>;
#[doc = "OTP\\_REGION\\_USR\\_1\\_RANGE"]
pub mod otp_reg14c;
#[doc = "OTP_REG150 (rw) register accessor: OTP\\_REGION\\_USR\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg150`] module"]
#[doc(alias = "OTP_REG150")]
pub type OtpReg150 = crate::Reg<otp_reg150::OtpReg150Spec>;
#[doc = "OTP\\_REGION\\_USR\\_2"]
pub mod otp_reg150;
#[doc = "OTP_REG154 (rw) register accessor: OTP\\_REGION\\_USR\\_2\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg154`] module"]
#[doc(alias = "OTP_REG154")]
pub type OtpReg154 = crate::Reg<otp_reg154::OtpReg154Spec>;
#[doc = "OTP\\_REGION\\_USR\\_2\\_RANGE"]
pub mod otp_reg154;
#[doc = "OTP_REG158 (rw) register accessor: OTP\\_REGION\\_USR\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg158`] module"]
#[doc(alias = "OTP_REG158")]
pub type OtpReg158 = crate::Reg<otp_reg158::OtpReg158Spec>;
#[doc = "OTP\\_REGION\\_USR\\_3"]
pub mod otp_reg158;
#[doc = "OTP_REG15C (rw) register accessor: OTP\\_REGION\\_USR\\_3\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg15c`] module"]
#[doc(alias = "OTP_REG15C")]
pub type OtpReg15c = crate::Reg<otp_reg15c::OtpReg15cSpec>;
#[doc = "OTP\\_REGION\\_USR\\_3\\_RANGE"]
pub mod otp_reg15c;
#[doc = "OTP_REG160 (rw) register accessor: OTP\\_REGION\\_CALIPTRA\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg160`] module"]
#[doc(alias = "OTP_REG160")]
pub type OtpReg160 = crate::Reg<otp_reg160::OtpReg160Spec>;
#[doc = "OTP\\_REGION\\_CALIPTRA\\_0"]
pub mod otp_reg160;
#[doc = "OTP_REG164 (rw) register accessor: OTP\\_REGION\\_CALIPTRA\\_0\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg164`] module"]
#[doc(alias = "OTP_REG164")]
pub type OtpReg164 = crate::Reg<otp_reg164::OtpReg164Spec>;
#[doc = "OTP\\_REGION\\_CALIPTRA\\_0\\_RANGE"]
pub mod otp_reg164;
#[doc = "OTP_REG168 (rw) register accessor: OTP\\_REGION\\_CALIPTRA\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg168`] module"]
#[doc(alias = "OTP_REG168")]
pub type OtpReg168 = crate::Reg<otp_reg168::OtpReg168Spec>;
#[doc = "OTP\\_REGION\\_CALIPTRA\\_1"]
pub mod otp_reg168;
#[doc = "OTP_REG16C (rw) register accessor: OTP\\_REGION\\_CALIPTRA\\_1\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg16c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg16c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg16c`] module"]
#[doc(alias = "OTP_REG16C")]
pub type OtpReg16c = crate::Reg<otp_reg16c::OtpReg16cSpec>;
#[doc = "OTP\\_REGION\\_CALIPTRA\\_1\\_RANGE"]
pub mod otp_reg16c;
#[doc = "OTP_REG170 (rw) register accessor: OTP\\_REGION\\_CALIPTRA\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg170`] module"]
#[doc(alias = "OTP_REG170")]
pub type OtpReg170 = crate::Reg<otp_reg170::OtpReg170Spec>;
#[doc = "OTP\\_REGION\\_CALIPTRA\\_2"]
pub mod otp_reg170;
#[doc = "OTP_REG174 (rw) register accessor: OTP\\_REGION\\_CALIPTRA\\_2\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg174`] module"]
#[doc(alias = "OTP_REG174")]
pub type OtpReg174 = crate::Reg<otp_reg174::OtpReg174Spec>;
#[doc = "OTP\\_REGION\\_CALIPTRA\\_2\\_RANGE"]
pub mod otp_reg174;
#[doc = "OTP_REG178 (rw) register accessor: OTP\\_REGION\\_CALIPTRA\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg178`] module"]
#[doc(alias = "OTP_REG178")]
pub type OtpReg178 = crate::Reg<otp_reg178::OtpReg178Spec>;
#[doc = "OTP\\_REGION\\_CALIPTRA\\_3"]
pub mod otp_reg178;
#[doc = "OTP_REG17C (rw) register accessor: OTP\\_REGION\\_CALIPTRA\\_3\\_RANGE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg17c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg17c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg17c`] module"]
#[doc(alias = "OTP_REG17C")]
pub type OtpReg17c = crate::Reg<otp_reg17c::OtpReg17cSpec>;
#[doc = "OTP\\_REGION\\_CALIPTRA\\_3\\_RANGE"]
pub mod otp_reg17c;
#[doc = "OTP_REG180 (rw) register accessor: OTP\\_RBP\\_SOC\\_SVN\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg180`] module"]
#[doc(alias = "OTP_REG180")]
pub type OtpReg180 = crate::Reg<otp_reg180::OtpReg180Spec>;
#[doc = "OTP\\_RBP\\_SOC\\_SVN"]
pub mod otp_reg180;
#[doc = "OTP_REG184 (rw) register accessor: OTP\\_RBP\\_SOC\\_KEYRETIRE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg184`] module"]
#[doc(alias = "OTP_REG184")]
pub type OtpReg184 = crate::Reg<otp_reg184::OtpReg184Spec>;
#[doc = "OTP\\_RBP\\_SOC\\_KEYRETIRE"]
pub mod otp_reg184;
#[doc = "OTP_REG188 (rw) register accessor: OTP\\_RBP\\_CALIP\\_SVN\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg188`] module"]
#[doc(alias = "OTP_REG188")]
pub type OtpReg188 = crate::Reg<otp_reg188::OtpReg188Spec>;
#[doc = "OTP\\_RBP\\_CALIP\\_SVN"]
pub mod otp_reg188;
#[doc = "OTP_REG18C (rw) register accessor: OTP\\_RBP\\_CALIP\\_KEYRETIRE\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg18c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg18c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg18c`] module"]
#[doc(alias = "OTP_REG18C")]
pub type OtpReg18c = crate::Reg<otp_reg18c::OtpReg18cSpec>;
#[doc = "OTP\\_RBP\\_CALIP\\_KEYRETIRE"]
pub mod otp_reg18c;
#[doc = "OTP_REG1A0 (rw) register accessor: PUF\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1a0`] module"]
#[doc(alias = "OTP_REG1A0")]
pub type OtpReg1a0 = crate::Reg<otp_reg1a0::OtpReg1a0Spec>;
#[doc = "PUF"]
pub mod otp_reg1a0;
#[doc = "OTP_REG1B0 (rw) register accessor: OTP\\_MASTER\\_ID\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1b0`] module"]
#[doc(alias = "OTP_REG1B0")]
pub type OtpReg1b0 = crate::Reg<otp_reg1b0::OtpReg1b0Spec>;
#[doc = "OTP\\_MASTER\\_ID"]
pub mod otp_reg1b0;
#[doc = "OTP_REG1B4 (rw) register accessor: OTP\\_MASTER\\_ID\\_EXT\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1b4`] module"]
#[doc(alias = "OTP_REG1B4")]
pub type OtpReg1b4 = crate::Reg<otp_reg1b4::OtpReg1b4Spec>;
#[doc = "OTP\\_MASTER\\_ID\\_EXT"]
pub mod otp_reg1b4;
#[doc = "OTP_REG1B8 (rw) register accessor: OTP\\_R\\_MASTER\\_ID\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1b8`] module"]
#[doc(alias = "OTP_REG1B8")]
pub type OtpReg1b8 = crate::Reg<otp_reg1b8::OtpReg1b8Spec>;
#[doc = "OTP\\_R\\_MASTER\\_ID"]
pub mod otp_reg1b8;
#[doc = "OTP_REG1BC (rw) register accessor: OTP\\_R\\_MASTER\\_ID\\_EXT\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1bc`] module"]
#[doc(alias = "OTP_REG1BC")]
pub type OtpReg1bc = crate::Reg<otp_reg1bc::OtpReg1bcSpec>;
#[doc = "OTP\\_R\\_MASTER\\_ID\\_EXT"]
pub mod otp_reg1bc;
#[doc = "OTP_REG1C0 (rw) register accessor: OTP\\_SOC\\_ECCKEY\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1c0`] module"]
#[doc(alias = "OTP_REG1C0")]
pub type OtpReg1c0 = crate::Reg<otp_reg1c0::OtpReg1c0Spec>;
#[doc = "OTP\\_SOC\\_ECCKEY"]
pub mod otp_reg1c0;
#[doc = "OTP_REG1C4 (rw) register accessor: OTP\\_SEC\\_BOOT\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1c4`] module"]
#[doc(alias = "OTP_REG1C4")]
pub type OtpReg1c4 = crate::Reg<otp_reg1c4::OtpReg1c4Spec>;
#[doc = "OTP\\_SEC\\_BOOT\\_EN"]
pub mod otp_reg1c4;
#[doc = "OTP_REG1C8 (rw) register accessor: OTP\\_SOC\\_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1c8`] module"]
#[doc(alias = "OTP_REG1C8")]
pub type OtpReg1c8 = crate::Reg<otp_reg1c8::OtpReg1c8Spec>;
#[doc = "OTP\\_SOC\\_KEY"]
pub mod otp_reg1c8;
#[doc = "OTP_REG1CC (rw) register accessor: OTP\\_CALPITRA\\_MANU\\_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1cc`] module"]
#[doc(alias = "OTP_REG1CC")]
pub type OtpReg1cc = crate::Reg<otp_reg1cc::OtpReg1ccSpec>;
#[doc = "OTP\\_CALPITRA\\_MANU\\_KEY"]
pub mod otp_reg1cc;
#[doc = "OTP_REG1D0 (rw) register accessor: OTP\\_CALPITRA\\_OWNER\\_KEY\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1d0`] module"]
#[doc(alias = "OTP_REG1D0")]
pub type OtpReg1d0 = crate::Reg<otp_reg1d0::OtpReg1d0Spec>;
#[doc = "OTP\\_CALPITRA\\_OWNER\\_KEY"]
pub mod otp_reg1d0;
#[doc = "OTP_REG1D4 (rw) register accessor: OTP\\_FW\\_ID\\_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1d4`] module"]
#[doc(alias = "OTP_REG1D4")]
pub type OtpReg1d4 = crate::Reg<otp_reg1d4::OtpReg1d4Spec>;
#[doc = "OTP\\_FW\\_ID\\_LSB"]
pub mod otp_reg1d4;
#[doc = "OTP_REG1D8 (rw) register accessor: OTP\\_FW\\_ID\\_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1d8`] module"]
#[doc(alias = "OTP_REG1D8")]
pub type OtpReg1d8 = crate::Reg<otp_reg1d8::OtpReg1d8Spec>;
#[doc = "OTP\\_FW\\_ID\\_MSB"]
pub mod otp_reg1d8;
#[doc = "OTP_REG1DC (rw) register accessor: OTP\\_CALIP\\_FMC\\_SVN\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1dc`] module"]
#[doc(alias = "OTP_REG1DC")]
pub type OtpReg1dc = crate::Reg<otp_reg1dc::OtpReg1dcSpec>;
#[doc = "OTP\\_CALIP\\_FMC\\_SVN"]
pub mod otp_reg1dc;
#[doc = "OTP_REG1E0 (rw) register accessor: OTP\\_CALIP\\_RUNTIME\\_SVN0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1e0`] module"]
#[doc(alias = "OTP_REG1E0")]
pub type OtpReg1e0 = crate::Reg<otp_reg1e0::OtpReg1e0Spec>;
#[doc = "OTP\\_CALIP\\_RUNTIME\\_SVN0"]
pub mod otp_reg1e0;
#[doc = "OTP_REG1E4 (rw) register accessor: OTP\\_CALIP\\_RUNTIME\\_SVN1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1e4`] module"]
#[doc(alias = "OTP_REG1E4")]
pub type OtpReg1e4 = crate::Reg<otp_reg1e4::OtpReg1e4Spec>;
#[doc = "OTP\\_CALIP\\_RUNTIME\\_SVN1"]
pub mod otp_reg1e4;
#[doc = "OTP_REG1E8 (rw) register accessor: OTP\\_CALIP\\_RUNTIME\\_SVN2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1e8`] module"]
#[doc(alias = "OTP_REG1E8")]
pub type OtpReg1e8 = crate::Reg<otp_reg1e8::OtpReg1e8Spec>;
#[doc = "OTP\\_CALIP\\_RUNTIME\\_SVN2"]
pub mod otp_reg1e8;
#[doc = "OTP_REG1EC (rw) register accessor: OTP\\_CALIP\\_RUNTIME\\_SVN3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1ec`] module"]
#[doc(alias = "OTP_REG1EC")]
pub type OtpReg1ec = crate::Reg<otp_reg1ec::OtpReg1ecSpec>;
#[doc = "OTP\\_CALIP\\_RUNTIME\\_SVN3"]
pub mod otp_reg1ec;
#[doc = "OTP_REG1F0 (rw) register accessor: OTP\\_SVN\\_WLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1f0`] module"]
#[doc(alias = "OTP_REG1F0")]
pub type OtpReg1f0 = crate::Reg<otp_reg1f0::OtpReg1f0Spec>;
#[doc = "OTP\\_SVN\\_WLOCK"]
pub mod otp_reg1f0;
#[doc = "OTP_REG1F4 (rw) register accessor: OTP\\_CMD\\_SOC2CALIP\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg1f4`] module"]
#[doc(alias = "OTP_REG1F4")]
pub type OtpReg1f4 = crate::Reg<otp_reg1f4::OtpReg1f4Spec>;
#[doc = "OTP\\_CMD\\_SOC2CALIP"]
pub mod otp_reg1f4;
#[doc = "OTP_REG200 (rw) register accessor: otp\\_interrupt\\_en\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg200`] module"]
#[doc(alias = "OTP_REG200")]
pub type OtpReg200 = crate::Reg<otp_reg200::OtpReg200Spec>;
#[doc = "otp\\_interrupt\\_en"]
pub mod otp_reg200;
#[doc = "OTP_REG204 (rw) register accessor: otp\\_interrupt\\_status\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg204`] module"]
#[doc(alias = "OTP_REG204")]
pub type OtpReg204 = crate::Reg<otp_reg204::OtpReg204Spec>;
#[doc = "otp\\_interrupt\\_status"]
pub mod otp_reg204;
#[doc = "OTP_REG208 (rw) register accessor: OTP\\_INTR\\_MID\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg208`] module"]
#[doc(alias = "OTP_REG208")]
pub type OtpReg208 = crate::Reg<otp_reg208::OtpReg208Spec>;
#[doc = "OTP\\_INTR\\_MID"]
pub mod otp_reg208;
#[doc = "OTP_REG20C (rw) register accessor: OTP\\_INTR\\_FUNC\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg20c`] module"]
#[doc(alias = "OTP_REG20C")]
pub type OtpReg20c = crate::Reg<otp_reg20c::OtpReg20cSpec>;
#[doc = "OTP\\_INTR\\_FUNC\\_INFO"]
pub mod otp_reg20c;
#[doc = "OTP_REG210 (rw) register accessor: OTP\\_INTR\\_M\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg210`] module"]
#[doc(alias = "OTP_REG210")]
pub type OtpReg210 = crate::Reg<otp_reg210::OtpReg210Spec>;
#[doc = "OTP\\_INTR\\_M\\_INFO"]
pub mod otp_reg210;
#[doc = "OTP_REG214 (rw) register accessor: OTP\\_INTR\\_R\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg214`] module"]
#[doc(alias = "OTP_REG214")]
pub type OtpReg214 = crate::Reg<otp_reg214::OtpReg214Spec>;
#[doc = "OTP\\_INTR\\_R\\_INFO"]
pub mod otp_reg214;
#[doc = "OTP_REG300 (rw) register accessor: OTP\\_SW\\_USAGE0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg300::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg300::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg300`] module"]
#[doc(alias = "OTP_REG300")]
pub type OtpReg300 = crate::Reg<otp_reg300::OtpReg300Spec>;
#[doc = "OTP\\_SW\\_USAGE0"]
pub mod otp_reg300;
#[doc = "OTP_REG304 (rw) register accessor: OTP\\_SW\\_USAGE1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg304::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg304::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg304`] module"]
#[doc(alias = "OTP_REG304")]
pub type OtpReg304 = crate::Reg<otp_reg304::OtpReg304Spec>;
#[doc = "OTP\\_SW\\_USAGE1"]
pub mod otp_reg304;
#[doc = "OTP_REG308 (rw) register accessor: OTP\\_SW\\_USAGE2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg308::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg308::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg308`] module"]
#[doc(alias = "OTP_REG308")]
pub type OtpReg308 = crate::Reg<otp_reg308::OtpReg308Spec>;
#[doc = "OTP\\_SW\\_USAGE2"]
pub mod otp_reg308;
#[doc = "OTP_REG30C (rw) register accessor: OTP\\_SW\\_USAGE3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg30c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg30c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg30c`] module"]
#[doc(alias = "OTP_REG30C")]
pub type OtpReg30c = crate::Reg<otp_reg30c::OtpReg30cSpec>;
#[doc = "OTP\\_SW\\_USAGE3"]
pub mod otp_reg30c;
#[doc = "OTP_REG310 (rw) register accessor: OTP\\_SW\\_USAGE4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg310`] module"]
#[doc(alias = "OTP_REG310")]
pub type OtpReg310 = crate::Reg<otp_reg310::OtpReg310Spec>;
#[doc = "OTP\\_SW\\_USAGE4"]
pub mod otp_reg310;
#[doc = "OTP_REG314 (rw) register accessor: OTP\\_SW\\_USAGE5\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg314`] module"]
#[doc(alias = "OTP_REG314")]
pub type OtpReg314 = crate::Reg<otp_reg314::OtpReg314Spec>;
#[doc = "OTP\\_SW\\_USAGE5"]
pub mod otp_reg314;
#[doc = "OTP_REG318 (rw) register accessor: OTP\\_SW\\_USAGE6\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg318::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg318::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg318`] module"]
#[doc(alias = "OTP_REG318")]
pub type OtpReg318 = crate::Reg<otp_reg318::OtpReg318Spec>;
#[doc = "OTP\\_SW\\_USAGE6"]
pub mod otp_reg318;
#[doc = "OTP_REG31C (rw) register accessor: OTP\\_SW\\_USAGE7\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg31c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg31c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg31c`] module"]
#[doc(alias = "OTP_REG31C")]
pub type OtpReg31c = crate::Reg<otp_reg31c::OtpReg31cSpec>;
#[doc = "OTP\\_SW\\_USAGE7"]
pub mod otp_reg31c;
#[doc = "OTP_REG320 (rw) register accessor: OTP\\_SW\\_USAGE8\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg320::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg320::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg320`] module"]
#[doc(alias = "OTP_REG320")]
pub type OtpReg320 = crate::Reg<otp_reg320::OtpReg320Spec>;
#[doc = "OTP\\_SW\\_USAGE8"]
pub mod otp_reg320;
#[doc = "OTP_REG324 (rw) register accessor: OTP\\_SW\\_USAGE9\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg324::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg324::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg324`] module"]
#[doc(alias = "OTP_REG324")]
pub type OtpReg324 = crate::Reg<otp_reg324::OtpReg324Spec>;
#[doc = "OTP\\_SW\\_USAGE9"]
pub mod otp_reg324;
#[doc = "OTP_REG328 (rw) register accessor: OTP\\_SW\\_USAGE10\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg328::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg328::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg328`] module"]
#[doc(alias = "OTP_REG328")]
pub type OtpReg328 = crate::Reg<otp_reg328::OtpReg328Spec>;
#[doc = "OTP\\_SW\\_USAGE10"]
pub mod otp_reg328;
#[doc = "OTP_REG32C (rw) register accessor: OTP\\_SW\\_USAGE11\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg32c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg32c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp_reg32c`] module"]
#[doc(alias = "OTP_REG32C")]
pub type OtpReg32c = crate::Reg<otp_reg32c::OtpReg32cSpec>;
#[doc = "OTP\\_SW\\_USAGE11"]
pub mod otp_reg32c;
