#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sgpio000: Sgpio000,
    sgpio004: Sgpio004,
    sgpio008: Sgpio008,
    sgpio00c: Sgpio00c,
    sgpio010: Sgpio010,
    sgpio014: Sgpio014,
    _reserved6: [u8; 0x08],
    sgpio020: Sgpio020,
    sgpio024: Sgpio024,
    sgpio028: Sgpio028,
    sgpio02c: Sgpio02c,
    sgpio030: Sgpio030,
    sgpio034: Sgpio034,
    _reserved12: [u8; 0x04],
    sgpio03c: Sgpio03c,
    sgpio040: Sgpio040,
    sgpio044: Sgpio044,
    sgpio048: Sgpio048,
    sgpio04c: Sgpio04c,
    sgpio050: Sgpio050,
    sgpio054: Sgpio054,
    sgpio058: Sgpio058,
    sgpio05c: Sgpio05c,
    _reserved21: [u8; 0x20],
    sgpio__control_register: [Sgpio_ControlRegister; 256],
}
impl RegisterBlock {
    #[doc = "0x00 - Serial GPIO Configuration Register"]
    #[inline(always)]
    pub const fn sgpio000(&self) -> &Sgpio000 {
        &self.sgpio000
    }
    #[doc = "0x04 - Debug Serial In Register \\#0"]
    #[inline(always)]
    pub const fn sgpio004(&self) -> &Sgpio004 {
        &self.sgpio004
    }
    #[doc = "0x08 - Debug Serial In Register \\#1"]
    #[inline(always)]
    pub const fn sgpio008(&self) -> &Sgpio008 {
        &self.sgpio008
    }
    #[doc = "0x0c - Debug Serial In Register \\#2"]
    #[inline(always)]
    pub const fn sgpio00c(&self) -> &Sgpio00c {
        &self.sgpio00c
    }
    #[doc = "0x10 - Debug Serial In Register \\#3"]
    #[inline(always)]
    pub const fn sgpio010(&self) -> &Sgpio010 {
        &self.sgpio010
    }
    #[doc = "0x14 - Debug Serial In Register \\#4"]
    #[inline(always)]
    pub const fn sgpio014(&self) -> &Sgpio014 {
        &self.sgpio014
    }
    #[doc = "0x20 - 80h Enable Register"]
    #[inline(always)]
    pub const fn sgpio020(&self) -> &Sgpio020 {
        &self.sgpio020
    }
    #[doc = "0x24 - Debug Serial Out Register \\#0"]
    #[inline(always)]
    pub const fn sgpio024(&self) -> &Sgpio024 {
        &self.sgpio024
    }
    #[doc = "0x28 - Debug Serial Out Register \\#1"]
    #[inline(always)]
    pub const fn sgpio028(&self) -> &Sgpio028 {
        &self.sgpio028
    }
    #[doc = "0x2c - Debug Serial In Register \\#2"]
    #[inline(always)]
    pub const fn sgpio02c(&self) -> &Sgpio02c {
        &self.sgpio02c
    }
    #[doc = "0x30 - Debug Serial In Register \\#3"]
    #[inline(always)]
    pub const fn sgpio030(&self) -> &Sgpio030 {
        &self.sgpio030
    }
    #[doc = "0x34 - Debug Serial Out Register \\#4"]
    #[inline(always)]
    pub const fn sgpio034(&self) -> &Sgpio034 {
        &self.sgpio034
    }
    #[doc = "0x3c - Write Protection Register"]
    #[inline(always)]
    pub const fn sgpio03c(&self) -> &Sgpio03c {
        &self.sgpio03c
    }
    #[doc = "0x40 - Interrupt Status Register \\#0"]
    #[inline(always)]
    pub const fn sgpio040(&self) -> &Sgpio040 {
        &self.sgpio040
    }
    #[doc = "0x44 - Interrupt Status Register \\#1"]
    #[inline(always)]
    pub const fn sgpio044(&self) -> &Sgpio044 {
        &self.sgpio044
    }
    #[doc = "0x48 - Interrupt Status Register \\#2"]
    #[inline(always)]
    pub const fn sgpio048(&self) -> &Sgpio048 {
        &self.sgpio048
    }
    #[doc = "0x4c - Interrupt Status Register \\#3"]
    #[inline(always)]
    pub const fn sgpio04c(&self) -> &Sgpio04c {
        &self.sgpio04c
    }
    #[doc = "0x50 - Interrupt Status Register \\#4"]
    #[inline(always)]
    pub const fn sgpio050(&self) -> &Sgpio050 {
        &self.sgpio050
    }
    #[doc = "0x54 - Interrupt Status Register \\#5"]
    #[inline(always)]
    pub const fn sgpio054(&self) -> &Sgpio054 {
        &self.sgpio054
    }
    #[doc = "0x58 - Interrupt Status Register \\#6"]
    #[inline(always)]
    pub const fn sgpio058(&self) -> &Sgpio058 {
        &self.sgpio058
    }
    #[doc = "0x5c - Interrupt Status Register \\#7"]
    #[inline(always)]
    pub const fn sgpio05c(&self) -> &Sgpio05c {
        &self.sgpio05c
    }
    #[doc = "0x80..0x480 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio__control_register(&self, n: usize) -> &Sgpio_ControlRegister {
        &self.sgpio__control_register[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x480 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub fn sgpio__control_register_iter(&self) -> impl Iterator<Item = &Sgpio_ControlRegister> {
        self.sgpio__control_register.iter()
    }
    #[doc = "0x80 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_0_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(0)
    }
    #[doc = "0x84 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_1_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(1)
    }
    #[doc = "0x88 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_2_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(2)
    }
    #[doc = "0x8c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_3_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(3)
    }
    #[doc = "0x90 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_4_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(4)
    }
    #[doc = "0x94 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_5_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(5)
    }
    #[doc = "0x98 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_6_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(6)
    }
    #[doc = "0x9c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_7_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(7)
    }
    #[doc = "0xa0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_8_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(8)
    }
    #[doc = "0xa4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_9_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(9)
    }
    #[doc = "0xa8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_10_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(10)
    }
    #[doc = "0xac - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_11_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(11)
    }
    #[doc = "0xb0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_12_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(12)
    }
    #[doc = "0xb4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_13_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(13)
    }
    #[doc = "0xb8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_14_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(14)
    }
    #[doc = "0xbc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_15_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(15)
    }
    #[doc = "0xc0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_16_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(16)
    }
    #[doc = "0xc4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_17_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(17)
    }
    #[doc = "0xc8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_18_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(18)
    }
    #[doc = "0xcc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_19_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(19)
    }
    #[doc = "0xd0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_20_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(20)
    }
    #[doc = "0xd4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_21_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(21)
    }
    #[doc = "0xd8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_22_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(22)
    }
    #[doc = "0xdc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_23_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(23)
    }
    #[doc = "0xe0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_24_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(24)
    }
    #[doc = "0xe4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_25_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(25)
    }
    #[doc = "0xe8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_26_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(26)
    }
    #[doc = "0xec - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_27_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(27)
    }
    #[doc = "0xf0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_28_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(28)
    }
    #[doc = "0xf4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_29_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(29)
    }
    #[doc = "0xf8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_30_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(30)
    }
    #[doc = "0xfc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_31_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(31)
    }
    #[doc = "0x100 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_32_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(32)
    }
    #[doc = "0x104 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_33_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(33)
    }
    #[doc = "0x108 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_34_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(34)
    }
    #[doc = "0x10c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_35_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(35)
    }
    #[doc = "0x110 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_36_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(36)
    }
    #[doc = "0x114 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_37_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(37)
    }
    #[doc = "0x118 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_38_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(38)
    }
    #[doc = "0x11c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_39_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(39)
    }
    #[doc = "0x120 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_40_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(40)
    }
    #[doc = "0x124 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_41_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(41)
    }
    #[doc = "0x128 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_42_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(42)
    }
    #[doc = "0x12c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_43_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(43)
    }
    #[doc = "0x130 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_44_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(44)
    }
    #[doc = "0x134 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_45_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(45)
    }
    #[doc = "0x138 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_46_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(46)
    }
    #[doc = "0x13c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_47_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(47)
    }
    #[doc = "0x140 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_48_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(48)
    }
    #[doc = "0x144 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_49_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(49)
    }
    #[doc = "0x148 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_50_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(50)
    }
    #[doc = "0x14c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_51_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(51)
    }
    #[doc = "0x150 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_52_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(52)
    }
    #[doc = "0x154 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_53_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(53)
    }
    #[doc = "0x158 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_54_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(54)
    }
    #[doc = "0x15c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_55_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(55)
    }
    #[doc = "0x160 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_56_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(56)
    }
    #[doc = "0x164 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_57_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(57)
    }
    #[doc = "0x168 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_58_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(58)
    }
    #[doc = "0x16c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_59_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(59)
    }
    #[doc = "0x170 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_60_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(60)
    }
    #[doc = "0x174 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_61_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(61)
    }
    #[doc = "0x178 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_62_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(62)
    }
    #[doc = "0x17c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_63_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(63)
    }
    #[doc = "0x180 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_64_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(64)
    }
    #[doc = "0x184 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_65_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(65)
    }
    #[doc = "0x188 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_66_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(66)
    }
    #[doc = "0x18c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_67_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(67)
    }
    #[doc = "0x190 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_68_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(68)
    }
    #[doc = "0x194 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_69_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(69)
    }
    #[doc = "0x198 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_70_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(70)
    }
    #[doc = "0x19c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_71_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(71)
    }
    #[doc = "0x1a0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_72_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(72)
    }
    #[doc = "0x1a4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_73_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(73)
    }
    #[doc = "0x1a8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_74_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(74)
    }
    #[doc = "0x1ac - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_75_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(75)
    }
    #[doc = "0x1b0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_76_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(76)
    }
    #[doc = "0x1b4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_77_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(77)
    }
    #[doc = "0x1b8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_78_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(78)
    }
    #[doc = "0x1bc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_79_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(79)
    }
    #[doc = "0x1c0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_80_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(80)
    }
    #[doc = "0x1c4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_81_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(81)
    }
    #[doc = "0x1c8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_82_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(82)
    }
    #[doc = "0x1cc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_83_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(83)
    }
    #[doc = "0x1d0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_84_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(84)
    }
    #[doc = "0x1d4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_85_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(85)
    }
    #[doc = "0x1d8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_86_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(86)
    }
    #[doc = "0x1dc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_87_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(87)
    }
    #[doc = "0x1e0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_88_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(88)
    }
    #[doc = "0x1e4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_89_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(89)
    }
    #[doc = "0x1e8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_90_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(90)
    }
    #[doc = "0x1ec - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_91_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(91)
    }
    #[doc = "0x1f0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_92_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(92)
    }
    #[doc = "0x1f4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_93_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(93)
    }
    #[doc = "0x1f8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_94_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(94)
    }
    #[doc = "0x1fc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_95_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(95)
    }
    #[doc = "0x200 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_96_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(96)
    }
    #[doc = "0x204 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_97_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(97)
    }
    #[doc = "0x208 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_98_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(98)
    }
    #[doc = "0x20c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_99_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(99)
    }
    #[doc = "0x210 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_100_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(100)
    }
    #[doc = "0x214 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_101_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(101)
    }
    #[doc = "0x218 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_102_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(102)
    }
    #[doc = "0x21c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_103_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(103)
    }
    #[doc = "0x220 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_104_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(104)
    }
    #[doc = "0x224 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_105_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(105)
    }
    #[doc = "0x228 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_106_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(106)
    }
    #[doc = "0x22c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_107_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(107)
    }
    #[doc = "0x230 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_108_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(108)
    }
    #[doc = "0x234 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_109_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(109)
    }
    #[doc = "0x238 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_110_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(110)
    }
    #[doc = "0x23c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_111_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(111)
    }
    #[doc = "0x240 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_112_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(112)
    }
    #[doc = "0x244 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_113_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(113)
    }
    #[doc = "0x248 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_114_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(114)
    }
    #[doc = "0x24c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_115_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(115)
    }
    #[doc = "0x250 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_116_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(116)
    }
    #[doc = "0x254 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_117_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(117)
    }
    #[doc = "0x258 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_118_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(118)
    }
    #[doc = "0x25c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_119_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(119)
    }
    #[doc = "0x260 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_120_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(120)
    }
    #[doc = "0x264 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_121_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(121)
    }
    #[doc = "0x268 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_122_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(122)
    }
    #[doc = "0x26c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_123_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(123)
    }
    #[doc = "0x270 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_124_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(124)
    }
    #[doc = "0x274 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_125_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(125)
    }
    #[doc = "0x278 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_126_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(126)
    }
    #[doc = "0x27c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_127_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(127)
    }
    #[doc = "0x280 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_128_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(128)
    }
    #[doc = "0x284 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_129_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(129)
    }
    #[doc = "0x288 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_130_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(130)
    }
    #[doc = "0x28c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_131_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(131)
    }
    #[doc = "0x290 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_132_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(132)
    }
    #[doc = "0x294 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_133_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(133)
    }
    #[doc = "0x298 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_134_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(134)
    }
    #[doc = "0x29c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_135_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(135)
    }
    #[doc = "0x2a0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_136_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(136)
    }
    #[doc = "0x2a4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_137_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(137)
    }
    #[doc = "0x2a8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_138_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(138)
    }
    #[doc = "0x2ac - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_139_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(139)
    }
    #[doc = "0x2b0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_140_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(140)
    }
    #[doc = "0x2b4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_141_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(141)
    }
    #[doc = "0x2b8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_142_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(142)
    }
    #[doc = "0x2bc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_143_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(143)
    }
    #[doc = "0x2c0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_144_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(144)
    }
    #[doc = "0x2c4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_145_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(145)
    }
    #[doc = "0x2c8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_146_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(146)
    }
    #[doc = "0x2cc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_147_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(147)
    }
    #[doc = "0x2d0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_148_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(148)
    }
    #[doc = "0x2d4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_149_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(149)
    }
    #[doc = "0x2d8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_150_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(150)
    }
    #[doc = "0x2dc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_151_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(151)
    }
    #[doc = "0x2e0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_152_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(152)
    }
    #[doc = "0x2e4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_153_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(153)
    }
    #[doc = "0x2e8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_154_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(154)
    }
    #[doc = "0x2ec - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_155_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(155)
    }
    #[doc = "0x2f0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_156_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(156)
    }
    #[doc = "0x2f4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_157_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(157)
    }
    #[doc = "0x2f8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_158_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(158)
    }
    #[doc = "0x2fc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_159_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(159)
    }
    #[doc = "0x300 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_160_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(160)
    }
    #[doc = "0x304 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_161_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(161)
    }
    #[doc = "0x308 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_162_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(162)
    }
    #[doc = "0x30c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_163_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(163)
    }
    #[doc = "0x310 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_164_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(164)
    }
    #[doc = "0x314 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_165_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(165)
    }
    #[doc = "0x318 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_166_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(166)
    }
    #[doc = "0x31c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_167_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(167)
    }
    #[doc = "0x320 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_168_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(168)
    }
    #[doc = "0x324 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_169_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(169)
    }
    #[doc = "0x328 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_170_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(170)
    }
    #[doc = "0x32c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_171_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(171)
    }
    #[doc = "0x330 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_172_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(172)
    }
    #[doc = "0x334 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_173_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(173)
    }
    #[doc = "0x338 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_174_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(174)
    }
    #[doc = "0x33c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_175_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(175)
    }
    #[doc = "0x340 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_176_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(176)
    }
    #[doc = "0x344 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_177_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(177)
    }
    #[doc = "0x348 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_178_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(178)
    }
    #[doc = "0x34c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_179_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(179)
    }
    #[doc = "0x350 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_180_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(180)
    }
    #[doc = "0x354 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_181_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(181)
    }
    #[doc = "0x358 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_182_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(182)
    }
    #[doc = "0x35c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_183_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(183)
    }
    #[doc = "0x360 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_184_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(184)
    }
    #[doc = "0x364 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_185_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(185)
    }
    #[doc = "0x368 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_186_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(186)
    }
    #[doc = "0x36c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_187_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(187)
    }
    #[doc = "0x370 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_188_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(188)
    }
    #[doc = "0x374 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_189_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(189)
    }
    #[doc = "0x378 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_190_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(190)
    }
    #[doc = "0x37c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_191_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(191)
    }
    #[doc = "0x380 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_192_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(192)
    }
    #[doc = "0x384 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_193_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(193)
    }
    #[doc = "0x388 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_194_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(194)
    }
    #[doc = "0x38c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_195_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(195)
    }
    #[doc = "0x390 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_196_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(196)
    }
    #[doc = "0x394 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_197_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(197)
    }
    #[doc = "0x398 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_198_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(198)
    }
    #[doc = "0x39c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_199_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(199)
    }
    #[doc = "0x3a0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_200_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(200)
    }
    #[doc = "0x3a4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_201_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(201)
    }
    #[doc = "0x3a8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_202_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(202)
    }
    #[doc = "0x3ac - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_203_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(203)
    }
    #[doc = "0x3b0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_204_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(204)
    }
    #[doc = "0x3b4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_205_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(205)
    }
    #[doc = "0x3b8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_206_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(206)
    }
    #[doc = "0x3bc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_207_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(207)
    }
    #[doc = "0x3c0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_208_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(208)
    }
    #[doc = "0x3c4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_209_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(209)
    }
    #[doc = "0x3c8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_210_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(210)
    }
    #[doc = "0x3cc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_211_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(211)
    }
    #[doc = "0x3d0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_212_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(212)
    }
    #[doc = "0x3d4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_213_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(213)
    }
    #[doc = "0x3d8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_214_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(214)
    }
    #[doc = "0x3dc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_215_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(215)
    }
    #[doc = "0x3e0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_216_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(216)
    }
    #[doc = "0x3e4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_217_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(217)
    }
    #[doc = "0x3e8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_218_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(218)
    }
    #[doc = "0x3ec - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_219_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(219)
    }
    #[doc = "0x3f0 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_220_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(220)
    }
    #[doc = "0x3f4 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_221_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(221)
    }
    #[doc = "0x3f8 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_222_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(222)
    }
    #[doc = "0x3fc - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_223_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(223)
    }
    #[doc = "0x400 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_224_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(224)
    }
    #[doc = "0x404 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_225_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(225)
    }
    #[doc = "0x408 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_226_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(226)
    }
    #[doc = "0x40c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_227_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(227)
    }
    #[doc = "0x410 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_228_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(228)
    }
    #[doc = "0x414 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_229_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(229)
    }
    #[doc = "0x418 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_230_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(230)
    }
    #[doc = "0x41c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_231_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(231)
    }
    #[doc = "0x420 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_232_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(232)
    }
    #[doc = "0x424 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_233_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(233)
    }
    #[doc = "0x428 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_234_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(234)
    }
    #[doc = "0x42c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_235_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(235)
    }
    #[doc = "0x430 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_236_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(236)
    }
    #[doc = "0x434 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_237_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(237)
    }
    #[doc = "0x438 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_238_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(238)
    }
    #[doc = "0x43c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_239_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(239)
    }
    #[doc = "0x440 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_240_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(240)
    }
    #[doc = "0x444 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_241_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(241)
    }
    #[doc = "0x448 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_242_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(242)
    }
    #[doc = "0x44c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_243_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(243)
    }
    #[doc = "0x450 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_244_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(244)
    }
    #[doc = "0x454 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_245_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(245)
    }
    #[doc = "0x458 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_246_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(246)
    }
    #[doc = "0x45c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_247_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(247)
    }
    #[doc = "0x460 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_248_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(248)
    }
    #[doc = "0x464 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_249_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(249)
    }
    #[doc = "0x468 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_250_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(250)
    }
    #[doc = "0x46c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_251_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(251)
    }
    #[doc = "0x470 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_252_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(252)
    }
    #[doc = "0x474 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_253_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(253)
    }
    #[doc = "0x478 - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_254_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(254)
    }
    #[doc = "0x47c - SGPIO\\_N Control Register N=0~255"]
    #[inline(always)]
    pub const fn sgpio_255_control_register(&self) -> &Sgpio_ControlRegister {
        self.sgpio__control_register(255)
    }
}
#[doc = "SGPIO000 (rw) register accessor: Serial GPIO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio000`] module"]
#[doc(alias = "SGPIO000")]
pub type Sgpio000 = crate::Reg<sgpio000::Sgpio000Spec>;
#[doc = "Serial GPIO Configuration Register"]
pub mod sgpio000;
#[doc = "SGPIO004 (rw) register accessor: Debug Serial In Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio004`] module"]
#[doc(alias = "SGPIO004")]
pub type Sgpio004 = crate::Reg<sgpio004::Sgpio004Spec>;
#[doc = "Debug Serial In Register \\#0"]
pub mod sgpio004;
#[doc = "SGPIO008 (rw) register accessor: Debug Serial In Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio008`] module"]
#[doc(alias = "SGPIO008")]
pub type Sgpio008 = crate::Reg<sgpio008::Sgpio008Spec>;
#[doc = "Debug Serial In Register \\#1"]
pub mod sgpio008;
#[doc = "SGPIO00C (rw) register accessor: Debug Serial In Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio00c`] module"]
#[doc(alias = "SGPIO00C")]
pub type Sgpio00c = crate::Reg<sgpio00c::Sgpio00cSpec>;
#[doc = "Debug Serial In Register \\#2"]
pub mod sgpio00c;
#[doc = "SGPIO010 (rw) register accessor: Debug Serial In Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio010`] module"]
#[doc(alias = "SGPIO010")]
pub type Sgpio010 = crate::Reg<sgpio010::Sgpio010Spec>;
#[doc = "Debug Serial In Register \\#3"]
pub mod sgpio010;
#[doc = "SGPIO014 (rw) register accessor: Debug Serial In Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio014`] module"]
#[doc(alias = "SGPIO014")]
pub type Sgpio014 = crate::Reg<sgpio014::Sgpio014Spec>;
#[doc = "Debug Serial In Register \\#4"]
pub mod sgpio014;
#[doc = "SGPIO020 (rw) register accessor: 80h Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio020`] module"]
#[doc(alias = "SGPIO020")]
pub type Sgpio020 = crate::Reg<sgpio020::Sgpio020Spec>;
#[doc = "80h Enable Register"]
pub mod sgpio020;
#[doc = "SGPIO024 (rw) register accessor: Debug Serial Out Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio024`] module"]
#[doc(alias = "SGPIO024")]
pub type Sgpio024 = crate::Reg<sgpio024::Sgpio024Spec>;
#[doc = "Debug Serial Out Register \\#0"]
pub mod sgpio024;
#[doc = "SGPIO028 (rw) register accessor: Debug Serial Out Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio028`] module"]
#[doc(alias = "SGPIO028")]
pub type Sgpio028 = crate::Reg<sgpio028::Sgpio028Spec>;
#[doc = "Debug Serial Out Register \\#1"]
pub mod sgpio028;
#[doc = "SGPIO02C (rw) register accessor: Debug Serial In Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio02c`] module"]
#[doc(alias = "SGPIO02C")]
pub type Sgpio02c = crate::Reg<sgpio02c::Sgpio02cSpec>;
#[doc = "Debug Serial In Register \\#2"]
pub mod sgpio02c;
#[doc = "SGPIO030 (rw) register accessor: Debug Serial In Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio030`] module"]
#[doc(alias = "SGPIO030")]
pub type Sgpio030 = crate::Reg<sgpio030::Sgpio030Spec>;
#[doc = "Debug Serial In Register \\#3"]
pub mod sgpio030;
#[doc = "SGPIO034 (rw) register accessor: Debug Serial Out Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio034`] module"]
#[doc(alias = "SGPIO034")]
pub type Sgpio034 = crate::Reg<sgpio034::Sgpio034Spec>;
#[doc = "Debug Serial Out Register \\#4"]
pub mod sgpio034;
#[doc = "SGPIO03C (rw) register accessor: Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio03c`] module"]
#[doc(alias = "SGPIO03C")]
pub type Sgpio03c = crate::Reg<sgpio03c::Sgpio03cSpec>;
#[doc = "Write Protection Register"]
pub mod sgpio03c;
#[doc = "SGPIO040 (rw) register accessor: Interrupt Status Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio040`] module"]
#[doc(alias = "SGPIO040")]
pub type Sgpio040 = crate::Reg<sgpio040::Sgpio040Spec>;
#[doc = "Interrupt Status Register \\#0"]
pub mod sgpio040;
#[doc = "SGPIO044 (rw) register accessor: Interrupt Status Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio044`] module"]
#[doc(alias = "SGPIO044")]
pub type Sgpio044 = crate::Reg<sgpio044::Sgpio044Spec>;
#[doc = "Interrupt Status Register \\#1"]
pub mod sgpio044;
#[doc = "SGPIO048 (rw) register accessor: Interrupt Status Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio048`] module"]
#[doc(alias = "SGPIO048")]
pub type Sgpio048 = crate::Reg<sgpio048::Sgpio048Spec>;
#[doc = "Interrupt Status Register \\#2"]
pub mod sgpio048;
#[doc = "SGPIO04C (rw) register accessor: Interrupt Status Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio04c`] module"]
#[doc(alias = "SGPIO04C")]
pub type Sgpio04c = crate::Reg<sgpio04c::Sgpio04cSpec>;
#[doc = "Interrupt Status Register \\#3"]
pub mod sgpio04c;
#[doc = "SGPIO050 (rw) register accessor: Interrupt Status Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio050`] module"]
#[doc(alias = "SGPIO050")]
pub type Sgpio050 = crate::Reg<sgpio050::Sgpio050Spec>;
#[doc = "Interrupt Status Register \\#4"]
pub mod sgpio050;
#[doc = "SGPIO054 (rw) register accessor: Interrupt Status Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio054`] module"]
#[doc(alias = "SGPIO054")]
pub type Sgpio054 = crate::Reg<sgpio054::Sgpio054Spec>;
#[doc = "Interrupt Status Register \\#5"]
pub mod sgpio054;
#[doc = "SGPIO058 (rw) register accessor: Interrupt Status Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio058`] module"]
#[doc(alias = "SGPIO058")]
pub type Sgpio058 = crate::Reg<sgpio058::Sgpio058Spec>;
#[doc = "Interrupt Status Register \\#6"]
pub mod sgpio058;
#[doc = "SGPIO05C (rw) register accessor: Interrupt Status Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio05c`] module"]
#[doc(alias = "SGPIO05C")]
pub type Sgpio05c = crate::Reg<sgpio05c::Sgpio05cSpec>;
#[doc = "Interrupt Status Register \\#7"]
pub mod sgpio05c;
#[doc = "SGPIO__ControlRegister (rw) register accessor: SGPIO\\_N Control Register N=0~255\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio__control_register::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio__control_register::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sgpio__control_register`] module"]
#[doc(alias = "SGPIO__ControlRegister")]
pub type Sgpio_ControlRegister = crate::Reg<sgpio__control_register::Sgpio_ControlRegisterSpec>;
#[doc = "SGPIO\\_N Control Register N=0~255"]
pub mod sgpio__control_register;
