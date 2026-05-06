#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uartdma000: Uartdma000,
    uartdma004: Uartdma004,
    uartdma008: Uartdma008,
    uartdma00c: Uartdma00c,
    _reserved4: [u8; 0x10],
    uartdma020: Uartdma020,
    uartdma024: Uartdma024,
    _reserved6: [u8; 0x08],
    uartdma030: Uartdma030,
    uartdma034: Uartdma034,
    uartdma038: Uartdma038,
    uartdma03c: Uartdma03c,
    uartdma040: Uartdma040,
    uartdma044: Uartdma044,
    uartdma048: Uartdma048,
    uartdma04c: Uartdma04c,
    uartdma050: Uartdma050,
    uartdma054: Uartdma054,
    uartdma058: Uartdma058,
    uartdma05c: Uartdma05c,
    uartdma060: Uartdma060,
    uartdma064: Uartdma064,
    uartdma068: Uartdma068,
    uartdma06c: Uartdma06c,
    uartdma070: Uartdma070,
    uartdma074: Uartdma074,
    uartdma078: Uartdma078,
    uartdma07c: Uartdma07c,
    uartdma080: Uartdma080,
    uartdma084: Uartdma084,
    uartdma088: Uartdma088,
    uartdma08c: Uartdma08c,
    uartdma090: Uartdma090,
    uartdma094: Uartdma094,
    uartdma098: Uartdma098,
    uartdma09c: Uartdma09c,
    uartdma0a0: Uartdma0a0,
    uartdma0a4: Uartdma0a4,
    uartdma0a8: Uartdma0a8,
    uartdma0ac: Uartdma0ac,
    uartdma0b0: Uartdma0b0,
    uartdma0b4: Uartdma0b4,
    uartdma0b8: Uartdma0b8,
    uartdma0bc: Uartdma0bc,
    uartdma0c0: Uartdma0c0,
    uartdma0c4: Uartdma0c4,
    uartdma0c8: Uartdma0c8,
    uartdma0cc: Uartdma0cc,
    uartdma0d0: Uartdma0d0,
    uartdma0d4: Uartdma0d4,
    uartdma0d8: Uartdma0d8,
    uartdma0dc: Uartdma0dc,
    uartdma0e0: Uartdma0e0,
    uartdma0e4: Uartdma0e4,
    uartdma0e8: Uartdma0e8,
    uartdma0ec: Uartdma0ec,
    uartdma0f0: Uartdma0f0,
    uartdma0f4: Uartdma0f4,
    uartdma0f8: Uartdma0f8,
    uartdma0fc: Uartdma0fc,
    uartdma100: Uartdma100,
    uartdma104: Uartdma104,
    uartdma108: Uartdma108,
    uartdma10c: Uartdma10c,
    uartdma110: Uartdma110,
    uartdma114: Uartdma114,
    uartdma118: Uartdma118,
    uartdma11c: Uartdma11c,
    uartdma120: Uartdma120,
    uartdma124: Uartdma124,
    uartdma128: Uartdma128,
    uartdma12c: Uartdma12c,
    uartdma130: Uartdma130,
    uartdma134: Uartdma134,
    uartdma138: Uartdma138,
    uartdma13c: Uartdma13c,
    uartdma140: Uartdma140,
    uartdma144: Uartdma144,
    uartdma148: Uartdma148,
    uartdma14c: Uartdma14c,
    uartdma150: Uartdma150,
    uartdma154: Uartdma154,
    uartdma158: Uartdma158,
    uartdma15c: Uartdma15c,
    uartdma160: Uartdma160,
    uartdma164: Uartdma164,
    uartdma168: Uartdma168,
    uartdma16c: Uartdma16c,
    uartdma170: Uartdma170,
    uartdma174: Uartdma174,
    uartdma178: Uartdma178,
    uartdma17c: Uartdma17c,
    uartdma180: Uartdma180,
    uartdma184: Uartdma184,
    uartdma188: Uartdma188,
    uartdma18c: Uartdma18c,
    uartdma190: Uartdma190,
    uartdma194: Uartdma194,
    uartdma198: Uartdma198,
    uartdma19c: Uartdma19c,
    uartdma1a0: Uartdma1a0,
    uartdma1a4: Uartdma1a4,
    uartdma1a8: Uartdma1a8,
    uartdma1ac: Uartdma1ac,
    uartdma1b0: Uartdma1b0,
    uartdma1b4: Uartdma1b4,
    uartdma1b8: Uartdma1b8,
    uartdma1bc: Uartdma1bc,
    uartdma1c0: Uartdma1c0,
    uartdma1c4: Uartdma1c4,
    uartdma1c8: Uartdma1c8,
    uartdma1cc: Uartdma1cc,
    uartdma1d0: Uartdma1d0,
    uartdma1d4: Uartdma1d4,
    uartdma1d8: Uartdma1d8,
    uartdma1dc: Uartdma1dc,
    uartdma1e0: Uartdma1e0,
    uartdma1e4: Uartdma1e4,
    uartdma1e8: Uartdma1e8,
    uartdma1ec: Uartdma1ec,
    uartdma1f0: Uartdma1f0,
    uartdma1f4: Uartdma1f4,
    uartdma1f8: Uartdma1f8,
    uartdma1fc: Uartdma1fc,
    uartdma200: Uartdma200,
    uartdma204: Uartdma204,
    uartdma208: Uartdma208,
    uartdma20c: Uartdma20c,
    uartdma210: Uartdma210,
    uartdma214: Uartdma214,
    uartdma218: Uartdma218,
    uartdma21c: Uartdma21c,
    uartdma220: Uartdma220,
    uartdma224: Uartdma224,
    uartdma228: Uartdma228,
    uartdma22c: Uartdma22c,
    uartdma230: Uartdma230,
    uartdma234: Uartdma234,
    uartdma238: Uartdma238,
    uartdma23c: Uartdma23c,
}
impl RegisterBlock {
    #[doc = "0x00 - UART TX DMA enable"]
    #[inline(always)]
    pub const fn uartdma000(&self) -> &Uartdma000 {
        &self.uartdma000
    }
    #[doc = "0x04 - UART RX DMA enable"]
    #[inline(always)]
    pub const fn uartdma004(&self) -> &Uartdma004 {
        &self.uartdma004
    }
    #[doc = "0x08 - Misc control"]
    #[inline(always)]
    pub const fn uartdma008(&self) -> &Uartdma008 {
        &self.uartdma008
    }
    #[doc = "0x0c - UART DMA time out timer"]
    #[inline(always)]
    pub const fn uartdma00c(&self) -> &Uartdma00c {
        &self.uartdma00c
    }
    #[doc = "0x20 - UART TX DMA reset"]
    #[inline(always)]
    pub const fn uartdma020(&self) -> &Uartdma020 {
        &self.uartdma020
    }
    #[doc = "0x24 - UART RX DMA reset"]
    #[inline(always)]
    pub const fn uartdma024(&self) -> &Uartdma024 {
        &self.uartdma024
    }
    #[doc = "0x30 - UART TX DMA interrrupt enable"]
    #[inline(always)]
    pub const fn uartdma030(&self) -> &Uartdma030 {
        &self.uartdma030
    }
    #[doc = "0x34 - UART TX DMA interrrupt status"]
    #[inline(always)]
    pub const fn uartdma034(&self) -> &Uartdma034 {
        &self.uartdma034
    }
    #[doc = "0x38 - UART RX DMA interrrupt enable"]
    #[inline(always)]
    pub const fn uartdma038(&self) -> &Uartdma038 {
        &self.uartdma038
    }
    #[doc = "0x3c - UART RX DMA interrrupt status"]
    #[inline(always)]
    pub const fn uartdma03c(&self) -> &Uartdma03c {
        &self.uartdma03c
    }
    #[doc = "0x40 - UART0 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma040(&self) -> &Uartdma040 {
        &self.uartdma040
    }
    #[doc = "0x44 - UART0 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma044(&self) -> &Uartdma044 {
        &self.uartdma044
    }
    #[doc = "0x48 - UART0 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma048(&self) -> &Uartdma048 {
        &self.uartdma048
    }
    #[doc = "0x4c - UART0 TX control register"]
    #[inline(always)]
    pub const fn uartdma04c(&self) -> &Uartdma04c {
        &self.uartdma04c
    }
    #[doc = "0x50 - UART0 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma050(&self) -> &Uartdma050 {
        &self.uartdma050
    }
    #[doc = "0x54 - UART0 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma054(&self) -> &Uartdma054 {
        &self.uartdma054
    }
    #[doc = "0x58 - UART0 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma058(&self) -> &Uartdma058 {
        &self.uartdma058
    }
    #[doc = "0x5c - UART0 RX control register"]
    #[inline(always)]
    pub const fn uartdma05c(&self) -> &Uartdma05c {
        &self.uartdma05c
    }
    #[doc = "0x60 - UART1 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma060(&self) -> &Uartdma060 {
        &self.uartdma060
    }
    #[doc = "0x64 - UART1 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma064(&self) -> &Uartdma064 {
        &self.uartdma064
    }
    #[doc = "0x68 - UART1 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma068(&self) -> &Uartdma068 {
        &self.uartdma068
    }
    #[doc = "0x6c - UART1 TX control register"]
    #[inline(always)]
    pub const fn uartdma06c(&self) -> &Uartdma06c {
        &self.uartdma06c
    }
    #[doc = "0x70 - UART1 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma070(&self) -> &Uartdma070 {
        &self.uartdma070
    }
    #[doc = "0x74 - UART1 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma074(&self) -> &Uartdma074 {
        &self.uartdma074
    }
    #[doc = "0x78 - UART1 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma078(&self) -> &Uartdma078 {
        &self.uartdma078
    }
    #[doc = "0x7c - UART1 RX control register"]
    #[inline(always)]
    pub const fn uartdma07c(&self) -> &Uartdma07c {
        &self.uartdma07c
    }
    #[doc = "0x80 - UART2 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma080(&self) -> &Uartdma080 {
        &self.uartdma080
    }
    #[doc = "0x84 - UART2 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma084(&self) -> &Uartdma084 {
        &self.uartdma084
    }
    #[doc = "0x88 - UART2 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma088(&self) -> &Uartdma088 {
        &self.uartdma088
    }
    #[doc = "0x8c - UART2 TX control register"]
    #[inline(always)]
    pub const fn uartdma08c(&self) -> &Uartdma08c {
        &self.uartdma08c
    }
    #[doc = "0x90 - UART2 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma090(&self) -> &Uartdma090 {
        &self.uartdma090
    }
    #[doc = "0x94 - UART2 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma094(&self) -> &Uartdma094 {
        &self.uartdma094
    }
    #[doc = "0x98 - UART2 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma098(&self) -> &Uartdma098 {
        &self.uartdma098
    }
    #[doc = "0x9c - UART2 RX control register"]
    #[inline(always)]
    pub const fn uartdma09c(&self) -> &Uartdma09c {
        &self.uartdma09c
    }
    #[doc = "0xa0 - UART3 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma0a0(&self) -> &Uartdma0a0 {
        &self.uartdma0a0
    }
    #[doc = "0xa4 - UART3 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma0a4(&self) -> &Uartdma0a4 {
        &self.uartdma0a4
    }
    #[doc = "0xa8 - UART3 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma0a8(&self) -> &Uartdma0a8 {
        &self.uartdma0a8
    }
    #[doc = "0xac - UART3 TX control register"]
    #[inline(always)]
    pub const fn uartdma0ac(&self) -> &Uartdma0ac {
        &self.uartdma0ac
    }
    #[doc = "0xb0 - UART3 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma0b0(&self) -> &Uartdma0b0 {
        &self.uartdma0b0
    }
    #[doc = "0xb4 - UART3 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma0b4(&self) -> &Uartdma0b4 {
        &self.uartdma0b4
    }
    #[doc = "0xb8 - UART3 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma0b8(&self) -> &Uartdma0b8 {
        &self.uartdma0b8
    }
    #[doc = "0xbc - UART3 RX control register"]
    #[inline(always)]
    pub const fn uartdma0bc(&self) -> &Uartdma0bc {
        &self.uartdma0bc
    }
    #[doc = "0xc0 - UART5 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma0c0(&self) -> &Uartdma0c0 {
        &self.uartdma0c0
    }
    #[doc = "0xc4 - UART5 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma0c4(&self) -> &Uartdma0c4 {
        &self.uartdma0c4
    }
    #[doc = "0xc8 - UART5 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma0c8(&self) -> &Uartdma0c8 {
        &self.uartdma0c8
    }
    #[doc = "0xcc - UART5 TX control register"]
    #[inline(always)]
    pub const fn uartdma0cc(&self) -> &Uartdma0cc {
        &self.uartdma0cc
    }
    #[doc = "0xd0 - UART5 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma0d0(&self) -> &Uartdma0d0 {
        &self.uartdma0d0
    }
    #[doc = "0xd4 - UART5 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma0d4(&self) -> &Uartdma0d4 {
        &self.uartdma0d4
    }
    #[doc = "0xd8 - UART5 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma0d8(&self) -> &Uartdma0d8 {
        &self.uartdma0d8
    }
    #[doc = "0xdc - UART5 RX control register"]
    #[inline(always)]
    pub const fn uartdma0dc(&self) -> &Uartdma0dc {
        &self.uartdma0dc
    }
    #[doc = "0xe0 - UART6 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma0e0(&self) -> &Uartdma0e0 {
        &self.uartdma0e0
    }
    #[doc = "0xe4 - UART6 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma0e4(&self) -> &Uartdma0e4 {
        &self.uartdma0e4
    }
    #[doc = "0xe8 - UART6 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma0e8(&self) -> &Uartdma0e8 {
        &self.uartdma0e8
    }
    #[doc = "0xec - UART6 TX control register"]
    #[inline(always)]
    pub const fn uartdma0ec(&self) -> &Uartdma0ec {
        &self.uartdma0ec
    }
    #[doc = "0xf0 - UART6 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma0f0(&self) -> &Uartdma0f0 {
        &self.uartdma0f0
    }
    #[doc = "0xf4 - UART6 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma0f4(&self) -> &Uartdma0f4 {
        &self.uartdma0f4
    }
    #[doc = "0xf8 - UART6 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma0f8(&self) -> &Uartdma0f8 {
        &self.uartdma0f8
    }
    #[doc = "0xfc - UART6 RX control register"]
    #[inline(always)]
    pub const fn uartdma0fc(&self) -> &Uartdma0fc {
        &self.uartdma0fc
    }
    #[doc = "0x100 - UART7 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma100(&self) -> &Uartdma100 {
        &self.uartdma100
    }
    #[doc = "0x104 - UART7 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma104(&self) -> &Uartdma104 {
        &self.uartdma104
    }
    #[doc = "0x108 - UART7 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma108(&self) -> &Uartdma108 {
        &self.uartdma108
    }
    #[doc = "0x10c - UART7 TX control register"]
    #[inline(always)]
    pub const fn uartdma10c(&self) -> &Uartdma10c {
        &self.uartdma10c
    }
    #[doc = "0x110 - UART7 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma110(&self) -> &Uartdma110 {
        &self.uartdma110
    }
    #[doc = "0x114 - UART7 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma114(&self) -> &Uartdma114 {
        &self.uartdma114
    }
    #[doc = "0x118 - UART7 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma118(&self) -> &Uartdma118 {
        &self.uartdma118
    }
    #[doc = "0x11c - UART7 RX control register"]
    #[inline(always)]
    pub const fn uartdma11c(&self) -> &Uartdma11c {
        &self.uartdma11c
    }
    #[doc = "0x120 - UART8 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma120(&self) -> &Uartdma120 {
        &self.uartdma120
    }
    #[doc = "0x124 - UART8 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma124(&self) -> &Uartdma124 {
        &self.uartdma124
    }
    #[doc = "0x128 - UART8 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma128(&self) -> &Uartdma128 {
        &self.uartdma128
    }
    #[doc = "0x12c - UART8 TX control register"]
    #[inline(always)]
    pub const fn uartdma12c(&self) -> &Uartdma12c {
        &self.uartdma12c
    }
    #[doc = "0x130 - UART8 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma130(&self) -> &Uartdma130 {
        &self.uartdma130
    }
    #[doc = "0x134 - UART8 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma134(&self) -> &Uartdma134 {
        &self.uartdma134
    }
    #[doc = "0x138 - UART8 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma138(&self) -> &Uartdma138 {
        &self.uartdma138
    }
    #[doc = "0x13c - UART8 RX control register"]
    #[inline(always)]
    pub const fn uartdma13c(&self) -> &Uartdma13c {
        &self.uartdma13c
    }
    #[doc = "0x140 - UART9 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma140(&self) -> &Uartdma140 {
        &self.uartdma140
    }
    #[doc = "0x144 - UART9 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma144(&self) -> &Uartdma144 {
        &self.uartdma144
    }
    #[doc = "0x148 - UART9 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma148(&self) -> &Uartdma148 {
        &self.uartdma148
    }
    #[doc = "0x14c - UART9 TX control register"]
    #[inline(always)]
    pub const fn uartdma14c(&self) -> &Uartdma14c {
        &self.uartdma14c
    }
    #[doc = "0x150 - UART9 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma150(&self) -> &Uartdma150 {
        &self.uartdma150
    }
    #[doc = "0x154 - UART9 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma154(&self) -> &Uartdma154 {
        &self.uartdma154
    }
    #[doc = "0x158 - UART9 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma158(&self) -> &Uartdma158 {
        &self.uartdma158
    }
    #[doc = "0x15c - UART9 RX control register"]
    #[inline(always)]
    pub const fn uartdma15c(&self) -> &Uartdma15c {
        &self.uartdma15c
    }
    #[doc = "0x160 - UART10 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma160(&self) -> &Uartdma160 {
        &self.uartdma160
    }
    #[doc = "0x164 - UART10 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma164(&self) -> &Uartdma164 {
        &self.uartdma164
    }
    #[doc = "0x168 - UART10 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma168(&self) -> &Uartdma168 {
        &self.uartdma168
    }
    #[doc = "0x16c - UART10 TX control register"]
    #[inline(always)]
    pub const fn uartdma16c(&self) -> &Uartdma16c {
        &self.uartdma16c
    }
    #[doc = "0x170 - UART10 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma170(&self) -> &Uartdma170 {
        &self.uartdma170
    }
    #[doc = "0x174 - UART10 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma174(&self) -> &Uartdma174 {
        &self.uartdma174
    }
    #[doc = "0x178 - UART10 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma178(&self) -> &Uartdma178 {
        &self.uartdma178
    }
    #[doc = "0x17c - UART10 RX control register"]
    #[inline(always)]
    pub const fn uartdma17c(&self) -> &Uartdma17c {
        &self.uartdma17c
    }
    #[doc = "0x180 - UART11 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma180(&self) -> &Uartdma180 {
        &self.uartdma180
    }
    #[doc = "0x184 - UART11 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma184(&self) -> &Uartdma184 {
        &self.uartdma184
    }
    #[doc = "0x188 - UART11 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma188(&self) -> &Uartdma188 {
        &self.uartdma188
    }
    #[doc = "0x18c - UART11 TX control register"]
    #[inline(always)]
    pub const fn uartdma18c(&self) -> &Uartdma18c {
        &self.uartdma18c
    }
    #[doc = "0x190 - UART11 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma190(&self) -> &Uartdma190 {
        &self.uartdma190
    }
    #[doc = "0x194 - UART11 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma194(&self) -> &Uartdma194 {
        &self.uartdma194
    }
    #[doc = "0x198 - UART11 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma198(&self) -> &Uartdma198 {
        &self.uartdma198
    }
    #[doc = "0x19c - UART11 RX control register"]
    #[inline(always)]
    pub const fn uartdma19c(&self) -> &Uartdma19c {
        &self.uartdma19c
    }
    #[doc = "0x1a0 - UART-BMC TX read pointer"]
    #[inline(always)]
    pub const fn uartdma1a0(&self) -> &Uartdma1a0 {
        &self.uartdma1a0
    }
    #[doc = "0x1a4 - UART-BMC TX write pointer"]
    #[inline(always)]
    pub const fn uartdma1a4(&self) -> &Uartdma1a4 {
        &self.uartdma1a4
    }
    #[doc = "0x1a8 - UART-BMC TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma1a8(&self) -> &Uartdma1a8 {
        &self.uartdma1a8
    }
    #[doc = "0x1ac - UART-BMC TX control register"]
    #[inline(always)]
    pub const fn uartdma1ac(&self) -> &Uartdma1ac {
        &self.uartdma1ac
    }
    #[doc = "0x1b0 - UART-BMC RX read pointer"]
    #[inline(always)]
    pub const fn uartdma1b0(&self) -> &Uartdma1b0 {
        &self.uartdma1b0
    }
    #[doc = "0x1b4 - UART-BMC RX write pointer"]
    #[inline(always)]
    pub const fn uartdma1b4(&self) -> &Uartdma1b4 {
        &self.uartdma1b4
    }
    #[doc = "0x1b8 - UART-BMC RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma1b8(&self) -> &Uartdma1b8 {
        &self.uartdma1b8
    }
    #[doc = "0x1bc - UART-BMC RX control register"]
    #[inline(always)]
    pub const fn uartdma1bc(&self) -> &Uartdma1bc {
        &self.uartdma1bc
    }
    #[doc = "0x1c0 - VUART0 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma1c0(&self) -> &Uartdma1c0 {
        &self.uartdma1c0
    }
    #[doc = "0x1c4 - VUART0 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma1c4(&self) -> &Uartdma1c4 {
        &self.uartdma1c4
    }
    #[doc = "0x1c8 - VUART0 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma1c8(&self) -> &Uartdma1c8 {
        &self.uartdma1c8
    }
    #[doc = "0x1cc - VUART0 TX control register"]
    #[inline(always)]
    pub const fn uartdma1cc(&self) -> &Uartdma1cc {
        &self.uartdma1cc
    }
    #[doc = "0x1d0 - VUART0 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma1d0(&self) -> &Uartdma1d0 {
        &self.uartdma1d0
    }
    #[doc = "0x1d4 - VUART0 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma1d4(&self) -> &Uartdma1d4 {
        &self.uartdma1d4
    }
    #[doc = "0x1d8 - VUART0 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma1d8(&self) -> &Uartdma1d8 {
        &self.uartdma1d8
    }
    #[doc = "0x1dc - VUART0 RX control register"]
    #[inline(always)]
    pub const fn uartdma1dc(&self) -> &Uartdma1dc {
        &self.uartdma1dc
    }
    #[doc = "0x1e0 - VUART1 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma1e0(&self) -> &Uartdma1e0 {
        &self.uartdma1e0
    }
    #[doc = "0x1e4 - VUART1 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma1e4(&self) -> &Uartdma1e4 {
        &self.uartdma1e4
    }
    #[doc = "0x1e8 - VUART1 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma1e8(&self) -> &Uartdma1e8 {
        &self.uartdma1e8
    }
    #[doc = "0x1ec - VUART1 TX control register"]
    #[inline(always)]
    pub const fn uartdma1ec(&self) -> &Uartdma1ec {
        &self.uartdma1ec
    }
    #[doc = "0x1f0 - VUART1 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma1f0(&self) -> &Uartdma1f0 {
        &self.uartdma1f0
    }
    #[doc = "0x1f4 - VUART1 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma1f4(&self) -> &Uartdma1f4 {
        &self.uartdma1f4
    }
    #[doc = "0x1f8 - VUART1 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma1f8(&self) -> &Uartdma1f8 {
        &self.uartdma1f8
    }
    #[doc = "0x1fc - VUART1 RX control register"]
    #[inline(always)]
    pub const fn uartdma1fc(&self) -> &Uartdma1fc {
        &self.uartdma1fc
    }
    #[doc = "0x200 - VUART2 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma200(&self) -> &Uartdma200 {
        &self.uartdma200
    }
    #[doc = "0x204 - VUART2 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma204(&self) -> &Uartdma204 {
        &self.uartdma204
    }
    #[doc = "0x208 - VUART2 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma208(&self) -> &Uartdma208 {
        &self.uartdma208
    }
    #[doc = "0x20c - VUART2 TX control register"]
    #[inline(always)]
    pub const fn uartdma20c(&self) -> &Uartdma20c {
        &self.uartdma20c
    }
    #[doc = "0x210 - VUART2 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma210(&self) -> &Uartdma210 {
        &self.uartdma210
    }
    #[doc = "0x214 - VUART2 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma214(&self) -> &Uartdma214 {
        &self.uartdma214
    }
    #[doc = "0x218 - VUART2 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma218(&self) -> &Uartdma218 {
        &self.uartdma218
    }
    #[doc = "0x21c - VUART2 RX control register"]
    #[inline(always)]
    pub const fn uartdma21c(&self) -> &Uartdma21c {
        &self.uartdma21c
    }
    #[doc = "0x220 - VUART3 TX read pointer"]
    #[inline(always)]
    pub const fn uartdma220(&self) -> &Uartdma220 {
        &self.uartdma220
    }
    #[doc = "0x224 - VUART3 TX write pointer"]
    #[inline(always)]
    pub const fn uartdma224(&self) -> &Uartdma224 {
        &self.uartdma224
    }
    #[doc = "0x228 - VUART3 TX buffer base address"]
    #[inline(always)]
    pub const fn uartdma228(&self) -> &Uartdma228 {
        &self.uartdma228
    }
    #[doc = "0x22c - VUART3 TX control register"]
    #[inline(always)]
    pub const fn uartdma22c(&self) -> &Uartdma22c {
        &self.uartdma22c
    }
    #[doc = "0x230 - VUART3 RX read pointer"]
    #[inline(always)]
    pub const fn uartdma230(&self) -> &Uartdma230 {
        &self.uartdma230
    }
    #[doc = "0x234 - VUART3 RX write pointer"]
    #[inline(always)]
    pub const fn uartdma234(&self) -> &Uartdma234 {
        &self.uartdma234
    }
    #[doc = "0x238 - VUART3 RX buffer base address"]
    #[inline(always)]
    pub const fn uartdma238(&self) -> &Uartdma238 {
        &self.uartdma238
    }
    #[doc = "0x23c - VUART3 RX control register"]
    #[inline(always)]
    pub const fn uartdma23c(&self) -> &Uartdma23c {
        &self.uartdma23c
    }
}
#[doc = "UARTDMA000 (rw) register accessor: UART TX DMA enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma000`] module"]
#[doc(alias = "UARTDMA000")]
pub type Uartdma000 = crate::Reg<uartdma000::Uartdma000Spec>;
#[doc = "UART TX DMA enable"]
pub mod uartdma000;
#[doc = "UARTDMA004 (rw) register accessor: UART RX DMA enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma004`] module"]
#[doc(alias = "UARTDMA004")]
pub type Uartdma004 = crate::Reg<uartdma004::Uartdma004Spec>;
#[doc = "UART RX DMA enable"]
pub mod uartdma004;
#[doc = "UARTDMA008 (rw) register accessor: Misc control\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma008`] module"]
#[doc(alias = "UARTDMA008")]
pub type Uartdma008 = crate::Reg<uartdma008::Uartdma008Spec>;
#[doc = "Misc control"]
pub mod uartdma008;
#[doc = "UARTDMA00C (rw) register accessor: UART DMA time out timer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma00c`] module"]
#[doc(alias = "UARTDMA00C")]
pub type Uartdma00c = crate::Reg<uartdma00c::Uartdma00cSpec>;
#[doc = "UART DMA time out timer"]
pub mod uartdma00c;
#[doc = "UARTDMA020 (rw) register accessor: UART TX DMA reset\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma020`] module"]
#[doc(alias = "UARTDMA020")]
pub type Uartdma020 = crate::Reg<uartdma020::Uartdma020Spec>;
#[doc = "UART TX DMA reset"]
pub mod uartdma020;
#[doc = "UARTDMA024 (rw) register accessor: UART RX DMA reset\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma024`] module"]
#[doc(alias = "UARTDMA024")]
pub type Uartdma024 = crate::Reg<uartdma024::Uartdma024Spec>;
#[doc = "UART RX DMA reset"]
pub mod uartdma024;
#[doc = "UARTDMA030 (rw) register accessor: UART TX DMA interrrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma030`] module"]
#[doc(alias = "UARTDMA030")]
pub type Uartdma030 = crate::Reg<uartdma030::Uartdma030Spec>;
#[doc = "UART TX DMA interrrupt enable"]
pub mod uartdma030;
#[doc = "UARTDMA034 (rw) register accessor: UART TX DMA interrrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma034`] module"]
#[doc(alias = "UARTDMA034")]
pub type Uartdma034 = crate::Reg<uartdma034::Uartdma034Spec>;
#[doc = "UART TX DMA interrrupt status"]
pub mod uartdma034;
#[doc = "UARTDMA038 (rw) register accessor: UART RX DMA interrrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma038`] module"]
#[doc(alias = "UARTDMA038")]
pub type Uartdma038 = crate::Reg<uartdma038::Uartdma038Spec>;
#[doc = "UART RX DMA interrrupt enable"]
pub mod uartdma038;
#[doc = "UARTDMA03C (rw) register accessor: UART RX DMA interrrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma03c`] module"]
#[doc(alias = "UARTDMA03C")]
pub type Uartdma03c = crate::Reg<uartdma03c::Uartdma03cSpec>;
#[doc = "UART RX DMA interrrupt status"]
pub mod uartdma03c;
#[doc = "UARTDMA040 (rw) register accessor: UART0 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma040`] module"]
#[doc(alias = "UARTDMA040")]
pub type Uartdma040 = crate::Reg<uartdma040::Uartdma040Spec>;
#[doc = "UART0 TX read pointer"]
pub mod uartdma040;
#[doc = "UARTDMA044 (rw) register accessor: UART0 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma044`] module"]
#[doc(alias = "UARTDMA044")]
pub type Uartdma044 = crate::Reg<uartdma044::Uartdma044Spec>;
#[doc = "UART0 TX write pointer"]
pub mod uartdma044;
#[doc = "UARTDMA048 (rw) register accessor: UART0 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma048`] module"]
#[doc(alias = "UARTDMA048")]
pub type Uartdma048 = crate::Reg<uartdma048::Uartdma048Spec>;
#[doc = "UART0 TX buffer base address"]
pub mod uartdma048;
#[doc = "UARTDMA04C (rw) register accessor: UART0 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma04c`] module"]
#[doc(alias = "UARTDMA04C")]
pub type Uartdma04c = crate::Reg<uartdma04c::Uartdma04cSpec>;
#[doc = "UART0 TX control register"]
pub mod uartdma04c;
#[doc = "UARTDMA050 (rw) register accessor: UART0 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma050`] module"]
#[doc(alias = "UARTDMA050")]
pub type Uartdma050 = crate::Reg<uartdma050::Uartdma050Spec>;
#[doc = "UART0 RX read pointer"]
pub mod uartdma050;
#[doc = "UARTDMA054 (rw) register accessor: UART0 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma054`] module"]
#[doc(alias = "UARTDMA054")]
pub type Uartdma054 = crate::Reg<uartdma054::Uartdma054Spec>;
#[doc = "UART0 RX write pointer"]
pub mod uartdma054;
#[doc = "UARTDMA058 (rw) register accessor: UART0 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma058`] module"]
#[doc(alias = "UARTDMA058")]
pub type Uartdma058 = crate::Reg<uartdma058::Uartdma058Spec>;
#[doc = "UART0 RX buffer base address"]
pub mod uartdma058;
#[doc = "UARTDMA05C (rw) register accessor: UART0 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma05c`] module"]
#[doc(alias = "UARTDMA05C")]
pub type Uartdma05c = crate::Reg<uartdma05c::Uartdma05cSpec>;
#[doc = "UART0 RX control register"]
pub mod uartdma05c;
#[doc = "UARTDMA060 (rw) register accessor: UART1 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma060`] module"]
#[doc(alias = "UARTDMA060")]
pub type Uartdma060 = crate::Reg<uartdma060::Uartdma060Spec>;
#[doc = "UART1 TX read pointer"]
pub mod uartdma060;
#[doc = "UARTDMA064 (rw) register accessor: UART1 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma064`] module"]
#[doc(alias = "UARTDMA064")]
pub type Uartdma064 = crate::Reg<uartdma064::Uartdma064Spec>;
#[doc = "UART1 TX write pointer"]
pub mod uartdma064;
#[doc = "UARTDMA068 (rw) register accessor: UART1 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma068`] module"]
#[doc(alias = "UARTDMA068")]
pub type Uartdma068 = crate::Reg<uartdma068::Uartdma068Spec>;
#[doc = "UART1 TX buffer base address"]
pub mod uartdma068;
#[doc = "UARTDMA06C (rw) register accessor: UART1 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma06c`] module"]
#[doc(alias = "UARTDMA06C")]
pub type Uartdma06c = crate::Reg<uartdma06c::Uartdma06cSpec>;
#[doc = "UART1 TX control register"]
pub mod uartdma06c;
#[doc = "UARTDMA070 (rw) register accessor: UART1 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma070`] module"]
#[doc(alias = "UARTDMA070")]
pub type Uartdma070 = crate::Reg<uartdma070::Uartdma070Spec>;
#[doc = "UART1 RX read pointer"]
pub mod uartdma070;
#[doc = "UARTDMA074 (rw) register accessor: UART1 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma074`] module"]
#[doc(alias = "UARTDMA074")]
pub type Uartdma074 = crate::Reg<uartdma074::Uartdma074Spec>;
#[doc = "UART1 RX write pointer"]
pub mod uartdma074;
#[doc = "UARTDMA078 (rw) register accessor: UART1 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma078`] module"]
#[doc(alias = "UARTDMA078")]
pub type Uartdma078 = crate::Reg<uartdma078::Uartdma078Spec>;
#[doc = "UART1 RX buffer base address"]
pub mod uartdma078;
#[doc = "UARTDMA07C (rw) register accessor: UART1 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma07c`] module"]
#[doc(alias = "UARTDMA07C")]
pub type Uartdma07c = crate::Reg<uartdma07c::Uartdma07cSpec>;
#[doc = "UART1 RX control register"]
pub mod uartdma07c;
#[doc = "UARTDMA080 (rw) register accessor: UART2 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma080`] module"]
#[doc(alias = "UARTDMA080")]
pub type Uartdma080 = crate::Reg<uartdma080::Uartdma080Spec>;
#[doc = "UART2 TX read pointer"]
pub mod uartdma080;
#[doc = "UARTDMA084 (rw) register accessor: UART2 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma084`] module"]
#[doc(alias = "UARTDMA084")]
pub type Uartdma084 = crate::Reg<uartdma084::Uartdma084Spec>;
#[doc = "UART2 TX write pointer"]
pub mod uartdma084;
#[doc = "UARTDMA088 (rw) register accessor: UART2 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma088`] module"]
#[doc(alias = "UARTDMA088")]
pub type Uartdma088 = crate::Reg<uartdma088::Uartdma088Spec>;
#[doc = "UART2 TX buffer base address"]
pub mod uartdma088;
#[doc = "UARTDMA08C (rw) register accessor: UART2 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma08c`] module"]
#[doc(alias = "UARTDMA08C")]
pub type Uartdma08c = crate::Reg<uartdma08c::Uartdma08cSpec>;
#[doc = "UART2 TX control register"]
pub mod uartdma08c;
#[doc = "UARTDMA090 (rw) register accessor: UART2 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma090`] module"]
#[doc(alias = "UARTDMA090")]
pub type Uartdma090 = crate::Reg<uartdma090::Uartdma090Spec>;
#[doc = "UART2 RX read pointer"]
pub mod uartdma090;
#[doc = "UARTDMA094 (rw) register accessor: UART2 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma094`] module"]
#[doc(alias = "UARTDMA094")]
pub type Uartdma094 = crate::Reg<uartdma094::Uartdma094Spec>;
#[doc = "UART2 RX write pointer"]
pub mod uartdma094;
#[doc = "UARTDMA098 (rw) register accessor: UART2 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma098`] module"]
#[doc(alias = "UARTDMA098")]
pub type Uartdma098 = crate::Reg<uartdma098::Uartdma098Spec>;
#[doc = "UART2 RX buffer base address"]
pub mod uartdma098;
#[doc = "UARTDMA09C (rw) register accessor: UART2 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma09c`] module"]
#[doc(alias = "UARTDMA09C")]
pub type Uartdma09c = crate::Reg<uartdma09c::Uartdma09cSpec>;
#[doc = "UART2 RX control register"]
pub mod uartdma09c;
#[doc = "UARTDMA0A0 (rw) register accessor: UART3 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0a0`] module"]
#[doc(alias = "UARTDMA0A0")]
pub type Uartdma0a0 = crate::Reg<uartdma0a0::Uartdma0a0Spec>;
#[doc = "UART3 TX read pointer"]
pub mod uartdma0a0;
#[doc = "UARTDMA0A4 (rw) register accessor: UART3 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0a4`] module"]
#[doc(alias = "UARTDMA0A4")]
pub type Uartdma0a4 = crate::Reg<uartdma0a4::Uartdma0a4Spec>;
#[doc = "UART3 TX write pointer"]
pub mod uartdma0a4;
#[doc = "UARTDMA0A8 (rw) register accessor: UART3 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0a8`] module"]
#[doc(alias = "UARTDMA0A8")]
pub type Uartdma0a8 = crate::Reg<uartdma0a8::Uartdma0a8Spec>;
#[doc = "UART3 TX buffer base address"]
pub mod uartdma0a8;
#[doc = "UARTDMA0AC (rw) register accessor: UART3 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0ac`] module"]
#[doc(alias = "UARTDMA0AC")]
pub type Uartdma0ac = crate::Reg<uartdma0ac::Uartdma0acSpec>;
#[doc = "UART3 TX control register"]
pub mod uartdma0ac;
#[doc = "UARTDMA0B0 (rw) register accessor: UART3 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0b0`] module"]
#[doc(alias = "UARTDMA0B0")]
pub type Uartdma0b0 = crate::Reg<uartdma0b0::Uartdma0b0Spec>;
#[doc = "UART3 RX read pointer"]
pub mod uartdma0b0;
#[doc = "UARTDMA0B4 (rw) register accessor: UART3 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0b4`] module"]
#[doc(alias = "UARTDMA0B4")]
pub type Uartdma0b4 = crate::Reg<uartdma0b4::Uartdma0b4Spec>;
#[doc = "UART3 RX write pointer"]
pub mod uartdma0b4;
#[doc = "UARTDMA0B8 (rw) register accessor: UART3 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0b8`] module"]
#[doc(alias = "UARTDMA0B8")]
pub type Uartdma0b8 = crate::Reg<uartdma0b8::Uartdma0b8Spec>;
#[doc = "UART3 RX buffer base address"]
pub mod uartdma0b8;
#[doc = "UARTDMA0BC (rw) register accessor: UART3 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0bc`] module"]
#[doc(alias = "UARTDMA0BC")]
pub type Uartdma0bc = crate::Reg<uartdma0bc::Uartdma0bcSpec>;
#[doc = "UART3 RX control register"]
pub mod uartdma0bc;
#[doc = "UARTDMA0C0 (rw) register accessor: UART5 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0c0`] module"]
#[doc(alias = "UARTDMA0C0")]
pub type Uartdma0c0 = crate::Reg<uartdma0c0::Uartdma0c0Spec>;
#[doc = "UART5 TX read pointer"]
pub mod uartdma0c0;
#[doc = "UARTDMA0C4 (rw) register accessor: UART5 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0c4`] module"]
#[doc(alias = "UARTDMA0C4")]
pub type Uartdma0c4 = crate::Reg<uartdma0c4::Uartdma0c4Spec>;
#[doc = "UART5 TX write pointer"]
pub mod uartdma0c4;
#[doc = "UARTDMA0C8 (rw) register accessor: UART5 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0c8`] module"]
#[doc(alias = "UARTDMA0C8")]
pub type Uartdma0c8 = crate::Reg<uartdma0c8::Uartdma0c8Spec>;
#[doc = "UART5 TX buffer base address"]
pub mod uartdma0c8;
#[doc = "UARTDMA0CC (rw) register accessor: UART5 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0cc`] module"]
#[doc(alias = "UARTDMA0CC")]
pub type Uartdma0cc = crate::Reg<uartdma0cc::Uartdma0ccSpec>;
#[doc = "UART5 TX control register"]
pub mod uartdma0cc;
#[doc = "UARTDMA0D0 (rw) register accessor: UART5 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0d0`] module"]
#[doc(alias = "UARTDMA0D0")]
pub type Uartdma0d0 = crate::Reg<uartdma0d0::Uartdma0d0Spec>;
#[doc = "UART5 RX read pointer"]
pub mod uartdma0d0;
#[doc = "UARTDMA0D4 (rw) register accessor: UART5 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0d4`] module"]
#[doc(alias = "UARTDMA0D4")]
pub type Uartdma0d4 = crate::Reg<uartdma0d4::Uartdma0d4Spec>;
#[doc = "UART5 RX write pointer"]
pub mod uartdma0d4;
#[doc = "UARTDMA0D8 (rw) register accessor: UART5 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0d8`] module"]
#[doc(alias = "UARTDMA0D8")]
pub type Uartdma0d8 = crate::Reg<uartdma0d8::Uartdma0d8Spec>;
#[doc = "UART5 RX buffer base address"]
pub mod uartdma0d8;
#[doc = "UARTDMA0DC (rw) register accessor: UART5 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0dc`] module"]
#[doc(alias = "UARTDMA0DC")]
pub type Uartdma0dc = crate::Reg<uartdma0dc::Uartdma0dcSpec>;
#[doc = "UART5 RX control register"]
pub mod uartdma0dc;
#[doc = "UARTDMA0E0 (rw) register accessor: UART6 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0e0`] module"]
#[doc(alias = "UARTDMA0E0")]
pub type Uartdma0e0 = crate::Reg<uartdma0e0::Uartdma0e0Spec>;
#[doc = "UART6 TX read pointer"]
pub mod uartdma0e0;
#[doc = "UARTDMA0E4 (rw) register accessor: UART6 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0e4`] module"]
#[doc(alias = "UARTDMA0E4")]
pub type Uartdma0e4 = crate::Reg<uartdma0e4::Uartdma0e4Spec>;
#[doc = "UART6 TX write pointer"]
pub mod uartdma0e4;
#[doc = "UARTDMA0E8 (rw) register accessor: UART6 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0e8`] module"]
#[doc(alias = "UARTDMA0E8")]
pub type Uartdma0e8 = crate::Reg<uartdma0e8::Uartdma0e8Spec>;
#[doc = "UART6 TX buffer base address"]
pub mod uartdma0e8;
#[doc = "UARTDMA0EC (rw) register accessor: UART6 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0ec`] module"]
#[doc(alias = "UARTDMA0EC")]
pub type Uartdma0ec = crate::Reg<uartdma0ec::Uartdma0ecSpec>;
#[doc = "UART6 TX control register"]
pub mod uartdma0ec;
#[doc = "UARTDMA0F0 (rw) register accessor: UART6 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0f0`] module"]
#[doc(alias = "UARTDMA0F0")]
pub type Uartdma0f0 = crate::Reg<uartdma0f0::Uartdma0f0Spec>;
#[doc = "UART6 RX read pointer"]
pub mod uartdma0f0;
#[doc = "UARTDMA0F4 (rw) register accessor: UART6 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0f4`] module"]
#[doc(alias = "UARTDMA0F4")]
pub type Uartdma0f4 = crate::Reg<uartdma0f4::Uartdma0f4Spec>;
#[doc = "UART6 RX write pointer"]
pub mod uartdma0f4;
#[doc = "UARTDMA0F8 (rw) register accessor: UART6 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0f8`] module"]
#[doc(alias = "UARTDMA0F8")]
pub type Uartdma0f8 = crate::Reg<uartdma0f8::Uartdma0f8Spec>;
#[doc = "UART6 RX buffer base address"]
pub mod uartdma0f8;
#[doc = "UARTDMA0FC (rw) register accessor: UART6 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma0fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma0fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma0fc`] module"]
#[doc(alias = "UARTDMA0FC")]
pub type Uartdma0fc = crate::Reg<uartdma0fc::Uartdma0fcSpec>;
#[doc = "UART6 RX control register"]
pub mod uartdma0fc;
#[doc = "UARTDMA100 (rw) register accessor: UART7 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma100`] module"]
#[doc(alias = "UARTDMA100")]
pub type Uartdma100 = crate::Reg<uartdma100::Uartdma100Spec>;
#[doc = "UART7 TX read pointer"]
pub mod uartdma100;
#[doc = "UARTDMA104 (rw) register accessor: UART7 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma104`] module"]
#[doc(alias = "UARTDMA104")]
pub type Uartdma104 = crate::Reg<uartdma104::Uartdma104Spec>;
#[doc = "UART7 TX write pointer"]
pub mod uartdma104;
#[doc = "UARTDMA108 (rw) register accessor: UART7 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma108`] module"]
#[doc(alias = "UARTDMA108")]
pub type Uartdma108 = crate::Reg<uartdma108::Uartdma108Spec>;
#[doc = "UART7 TX buffer base address"]
pub mod uartdma108;
#[doc = "UARTDMA10C (rw) register accessor: UART7 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma10c`] module"]
#[doc(alias = "UARTDMA10C")]
pub type Uartdma10c = crate::Reg<uartdma10c::Uartdma10cSpec>;
#[doc = "UART7 TX control register"]
pub mod uartdma10c;
#[doc = "UARTDMA110 (rw) register accessor: UART7 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma110`] module"]
#[doc(alias = "UARTDMA110")]
pub type Uartdma110 = crate::Reg<uartdma110::Uartdma110Spec>;
#[doc = "UART7 RX read pointer"]
pub mod uartdma110;
#[doc = "UARTDMA114 (rw) register accessor: UART7 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma114`] module"]
#[doc(alias = "UARTDMA114")]
pub type Uartdma114 = crate::Reg<uartdma114::Uartdma114Spec>;
#[doc = "UART7 RX write pointer"]
pub mod uartdma114;
#[doc = "UARTDMA118 (rw) register accessor: UART7 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma118`] module"]
#[doc(alias = "UARTDMA118")]
pub type Uartdma118 = crate::Reg<uartdma118::Uartdma118Spec>;
#[doc = "UART7 RX buffer base address"]
pub mod uartdma118;
#[doc = "UARTDMA11C (rw) register accessor: UART7 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma11c`] module"]
#[doc(alias = "UARTDMA11C")]
pub type Uartdma11c = crate::Reg<uartdma11c::Uartdma11cSpec>;
#[doc = "UART7 RX control register"]
pub mod uartdma11c;
#[doc = "UARTDMA120 (rw) register accessor: UART8 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma120`] module"]
#[doc(alias = "UARTDMA120")]
pub type Uartdma120 = crate::Reg<uartdma120::Uartdma120Spec>;
#[doc = "UART8 TX read pointer"]
pub mod uartdma120;
#[doc = "UARTDMA124 (rw) register accessor: UART8 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma124`] module"]
#[doc(alias = "UARTDMA124")]
pub type Uartdma124 = crate::Reg<uartdma124::Uartdma124Spec>;
#[doc = "UART8 TX write pointer"]
pub mod uartdma124;
#[doc = "UARTDMA128 (rw) register accessor: UART8 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma128`] module"]
#[doc(alias = "UARTDMA128")]
pub type Uartdma128 = crate::Reg<uartdma128::Uartdma128Spec>;
#[doc = "UART8 TX buffer base address"]
pub mod uartdma128;
#[doc = "UARTDMA12C (rw) register accessor: UART8 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma12c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma12c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma12c`] module"]
#[doc(alias = "UARTDMA12C")]
pub type Uartdma12c = crate::Reg<uartdma12c::Uartdma12cSpec>;
#[doc = "UART8 TX control register"]
pub mod uartdma12c;
#[doc = "UARTDMA130 (rw) register accessor: UART8 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma130`] module"]
#[doc(alias = "UARTDMA130")]
pub type Uartdma130 = crate::Reg<uartdma130::Uartdma130Spec>;
#[doc = "UART8 RX read pointer"]
pub mod uartdma130;
#[doc = "UARTDMA134 (rw) register accessor: UART8 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma134`] module"]
#[doc(alias = "UARTDMA134")]
pub type Uartdma134 = crate::Reg<uartdma134::Uartdma134Spec>;
#[doc = "UART8 RX write pointer"]
pub mod uartdma134;
#[doc = "UARTDMA138 (rw) register accessor: UART8 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma138`] module"]
#[doc(alias = "UARTDMA138")]
pub type Uartdma138 = crate::Reg<uartdma138::Uartdma138Spec>;
#[doc = "UART8 RX buffer base address"]
pub mod uartdma138;
#[doc = "UARTDMA13C (rw) register accessor: UART8 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma13c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma13c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma13c`] module"]
#[doc(alias = "UARTDMA13C")]
pub type Uartdma13c = crate::Reg<uartdma13c::Uartdma13cSpec>;
#[doc = "UART8 RX control register"]
pub mod uartdma13c;
#[doc = "UARTDMA140 (rw) register accessor: UART9 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma140`] module"]
#[doc(alias = "UARTDMA140")]
pub type Uartdma140 = crate::Reg<uartdma140::Uartdma140Spec>;
#[doc = "UART9 TX read pointer"]
pub mod uartdma140;
#[doc = "UARTDMA144 (rw) register accessor: UART9 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma144`] module"]
#[doc(alias = "UARTDMA144")]
pub type Uartdma144 = crate::Reg<uartdma144::Uartdma144Spec>;
#[doc = "UART9 TX write pointer"]
pub mod uartdma144;
#[doc = "UARTDMA148 (rw) register accessor: UART9 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma148`] module"]
#[doc(alias = "UARTDMA148")]
pub type Uartdma148 = crate::Reg<uartdma148::Uartdma148Spec>;
#[doc = "UART9 TX buffer base address"]
pub mod uartdma148;
#[doc = "UARTDMA14C (rw) register accessor: UART9 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma14c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma14c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma14c`] module"]
#[doc(alias = "UARTDMA14C")]
pub type Uartdma14c = crate::Reg<uartdma14c::Uartdma14cSpec>;
#[doc = "UART9 TX control register"]
pub mod uartdma14c;
#[doc = "UARTDMA150 (rw) register accessor: UART9 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma150`] module"]
#[doc(alias = "UARTDMA150")]
pub type Uartdma150 = crate::Reg<uartdma150::Uartdma150Spec>;
#[doc = "UART9 RX read pointer"]
pub mod uartdma150;
#[doc = "UARTDMA154 (rw) register accessor: UART9 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma154`] module"]
#[doc(alias = "UARTDMA154")]
pub type Uartdma154 = crate::Reg<uartdma154::Uartdma154Spec>;
#[doc = "UART9 RX write pointer"]
pub mod uartdma154;
#[doc = "UARTDMA158 (rw) register accessor: UART9 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma158`] module"]
#[doc(alias = "UARTDMA158")]
pub type Uartdma158 = crate::Reg<uartdma158::Uartdma158Spec>;
#[doc = "UART9 RX buffer base address"]
pub mod uartdma158;
#[doc = "UARTDMA15C (rw) register accessor: UART9 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma15c`] module"]
#[doc(alias = "UARTDMA15C")]
pub type Uartdma15c = crate::Reg<uartdma15c::Uartdma15cSpec>;
#[doc = "UART9 RX control register"]
pub mod uartdma15c;
#[doc = "UARTDMA160 (rw) register accessor: UART10 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma160`] module"]
#[doc(alias = "UARTDMA160")]
pub type Uartdma160 = crate::Reg<uartdma160::Uartdma160Spec>;
#[doc = "UART10 TX read pointer"]
pub mod uartdma160;
#[doc = "UARTDMA164 (rw) register accessor: UART10 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma164`] module"]
#[doc(alias = "UARTDMA164")]
pub type Uartdma164 = crate::Reg<uartdma164::Uartdma164Spec>;
#[doc = "UART10 TX write pointer"]
pub mod uartdma164;
#[doc = "UARTDMA168 (rw) register accessor: UART10 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma168`] module"]
#[doc(alias = "UARTDMA168")]
pub type Uartdma168 = crate::Reg<uartdma168::Uartdma168Spec>;
#[doc = "UART10 TX buffer base address"]
pub mod uartdma168;
#[doc = "UARTDMA16C (rw) register accessor: UART10 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma16c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma16c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma16c`] module"]
#[doc(alias = "UARTDMA16C")]
pub type Uartdma16c = crate::Reg<uartdma16c::Uartdma16cSpec>;
#[doc = "UART10 TX control register"]
pub mod uartdma16c;
#[doc = "UARTDMA170 (rw) register accessor: UART10 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma170`] module"]
#[doc(alias = "UARTDMA170")]
pub type Uartdma170 = crate::Reg<uartdma170::Uartdma170Spec>;
#[doc = "UART10 RX read pointer"]
pub mod uartdma170;
#[doc = "UARTDMA174 (rw) register accessor: UART10 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma174`] module"]
#[doc(alias = "UARTDMA174")]
pub type Uartdma174 = crate::Reg<uartdma174::Uartdma174Spec>;
#[doc = "UART10 RX write pointer"]
pub mod uartdma174;
#[doc = "UARTDMA178 (rw) register accessor: UART10 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma178`] module"]
#[doc(alias = "UARTDMA178")]
pub type Uartdma178 = crate::Reg<uartdma178::Uartdma178Spec>;
#[doc = "UART10 RX buffer base address"]
pub mod uartdma178;
#[doc = "UARTDMA17C (rw) register accessor: UART10 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma17c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma17c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma17c`] module"]
#[doc(alias = "UARTDMA17C")]
pub type Uartdma17c = crate::Reg<uartdma17c::Uartdma17cSpec>;
#[doc = "UART10 RX control register"]
pub mod uartdma17c;
#[doc = "UARTDMA180 (rw) register accessor: UART11 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma180`] module"]
#[doc(alias = "UARTDMA180")]
pub type Uartdma180 = crate::Reg<uartdma180::Uartdma180Spec>;
#[doc = "UART11 TX read pointer"]
pub mod uartdma180;
#[doc = "UARTDMA184 (rw) register accessor: UART11 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma184`] module"]
#[doc(alias = "UARTDMA184")]
pub type Uartdma184 = crate::Reg<uartdma184::Uartdma184Spec>;
#[doc = "UART11 TX write pointer"]
pub mod uartdma184;
#[doc = "UARTDMA188 (rw) register accessor: UART11 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma188`] module"]
#[doc(alias = "UARTDMA188")]
pub type Uartdma188 = crate::Reg<uartdma188::Uartdma188Spec>;
#[doc = "UART11 TX buffer base address"]
pub mod uartdma188;
#[doc = "UARTDMA18C (rw) register accessor: UART11 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma18c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma18c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma18c`] module"]
#[doc(alias = "UARTDMA18C")]
pub type Uartdma18c = crate::Reg<uartdma18c::Uartdma18cSpec>;
#[doc = "UART11 TX control register"]
pub mod uartdma18c;
#[doc = "UARTDMA190 (rw) register accessor: UART11 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma190`] module"]
#[doc(alias = "UARTDMA190")]
pub type Uartdma190 = crate::Reg<uartdma190::Uartdma190Spec>;
#[doc = "UART11 RX read pointer"]
pub mod uartdma190;
#[doc = "UARTDMA194 (rw) register accessor: UART11 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma194`] module"]
#[doc(alias = "UARTDMA194")]
pub type Uartdma194 = crate::Reg<uartdma194::Uartdma194Spec>;
#[doc = "UART11 RX write pointer"]
pub mod uartdma194;
#[doc = "UARTDMA198 (rw) register accessor: UART11 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma198`] module"]
#[doc(alias = "UARTDMA198")]
pub type Uartdma198 = crate::Reg<uartdma198::Uartdma198Spec>;
#[doc = "UART11 RX buffer base address"]
pub mod uartdma198;
#[doc = "UARTDMA19C (rw) register accessor: UART11 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma19c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma19c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma19c`] module"]
#[doc(alias = "UARTDMA19C")]
pub type Uartdma19c = crate::Reg<uartdma19c::Uartdma19cSpec>;
#[doc = "UART11 RX control register"]
pub mod uartdma19c;
#[doc = "UARTDMA1A0 (rw) register accessor: UART-BMC TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1a0`] module"]
#[doc(alias = "UARTDMA1A0")]
pub type Uartdma1a0 = crate::Reg<uartdma1a0::Uartdma1a0Spec>;
#[doc = "UART-BMC TX read pointer"]
pub mod uartdma1a0;
#[doc = "UARTDMA1A4 (rw) register accessor: UART-BMC TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1a4`] module"]
#[doc(alias = "UARTDMA1A4")]
pub type Uartdma1a4 = crate::Reg<uartdma1a4::Uartdma1a4Spec>;
#[doc = "UART-BMC TX write pointer"]
pub mod uartdma1a4;
#[doc = "UARTDMA1A8 (rw) register accessor: UART-BMC TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1a8`] module"]
#[doc(alias = "UARTDMA1A8")]
pub type Uartdma1a8 = crate::Reg<uartdma1a8::Uartdma1a8Spec>;
#[doc = "UART-BMC TX buffer base address"]
pub mod uartdma1a8;
#[doc = "UARTDMA1AC (rw) register accessor: UART-BMC TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1ac`] module"]
#[doc(alias = "UARTDMA1AC")]
pub type Uartdma1ac = crate::Reg<uartdma1ac::Uartdma1acSpec>;
#[doc = "UART-BMC TX control register"]
pub mod uartdma1ac;
#[doc = "UARTDMA1B0 (rw) register accessor: UART-BMC RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1b0`] module"]
#[doc(alias = "UARTDMA1B0")]
pub type Uartdma1b0 = crate::Reg<uartdma1b0::Uartdma1b0Spec>;
#[doc = "UART-BMC RX read pointer"]
pub mod uartdma1b0;
#[doc = "UARTDMA1B4 (rw) register accessor: UART-BMC RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1b4`] module"]
#[doc(alias = "UARTDMA1B4")]
pub type Uartdma1b4 = crate::Reg<uartdma1b4::Uartdma1b4Spec>;
#[doc = "UART-BMC RX write pointer"]
pub mod uartdma1b4;
#[doc = "UARTDMA1B8 (rw) register accessor: UART-BMC RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1b8`] module"]
#[doc(alias = "UARTDMA1B8")]
pub type Uartdma1b8 = crate::Reg<uartdma1b8::Uartdma1b8Spec>;
#[doc = "UART-BMC RX buffer base address"]
pub mod uartdma1b8;
#[doc = "UARTDMA1BC (rw) register accessor: UART-BMC RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1bc`] module"]
#[doc(alias = "UARTDMA1BC")]
pub type Uartdma1bc = crate::Reg<uartdma1bc::Uartdma1bcSpec>;
#[doc = "UART-BMC RX control register"]
pub mod uartdma1bc;
#[doc = "UARTDMA1C0 (rw) register accessor: VUART0 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1c0`] module"]
#[doc(alias = "UARTDMA1C0")]
pub type Uartdma1c0 = crate::Reg<uartdma1c0::Uartdma1c0Spec>;
#[doc = "VUART0 TX read pointer"]
pub mod uartdma1c0;
#[doc = "UARTDMA1C4 (rw) register accessor: VUART0 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1c4`] module"]
#[doc(alias = "UARTDMA1C4")]
pub type Uartdma1c4 = crate::Reg<uartdma1c4::Uartdma1c4Spec>;
#[doc = "VUART0 TX write pointer"]
pub mod uartdma1c4;
#[doc = "UARTDMA1C8 (rw) register accessor: VUART0 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1c8`] module"]
#[doc(alias = "UARTDMA1C8")]
pub type Uartdma1c8 = crate::Reg<uartdma1c8::Uartdma1c8Spec>;
#[doc = "VUART0 TX buffer base address"]
pub mod uartdma1c8;
#[doc = "UARTDMA1CC (rw) register accessor: VUART0 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1cc`] module"]
#[doc(alias = "UARTDMA1CC")]
pub type Uartdma1cc = crate::Reg<uartdma1cc::Uartdma1ccSpec>;
#[doc = "VUART0 TX control register"]
pub mod uartdma1cc;
#[doc = "UARTDMA1D0 (rw) register accessor: VUART0 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1d0`] module"]
#[doc(alias = "UARTDMA1D0")]
pub type Uartdma1d0 = crate::Reg<uartdma1d0::Uartdma1d0Spec>;
#[doc = "VUART0 RX read pointer"]
pub mod uartdma1d0;
#[doc = "UARTDMA1D4 (rw) register accessor: VUART0 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1d4`] module"]
#[doc(alias = "UARTDMA1D4")]
pub type Uartdma1d4 = crate::Reg<uartdma1d4::Uartdma1d4Spec>;
#[doc = "VUART0 RX write pointer"]
pub mod uartdma1d4;
#[doc = "UARTDMA1D8 (rw) register accessor: VUART0 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1d8`] module"]
#[doc(alias = "UARTDMA1D8")]
pub type Uartdma1d8 = crate::Reg<uartdma1d8::Uartdma1d8Spec>;
#[doc = "VUART0 RX buffer base address"]
pub mod uartdma1d8;
#[doc = "UARTDMA1DC (rw) register accessor: VUART0 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1dc`] module"]
#[doc(alias = "UARTDMA1DC")]
pub type Uartdma1dc = crate::Reg<uartdma1dc::Uartdma1dcSpec>;
#[doc = "VUART0 RX control register"]
pub mod uartdma1dc;
#[doc = "UARTDMA1E0 (rw) register accessor: VUART1 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1e0`] module"]
#[doc(alias = "UARTDMA1E0")]
pub type Uartdma1e0 = crate::Reg<uartdma1e0::Uartdma1e0Spec>;
#[doc = "VUART1 TX read pointer"]
pub mod uartdma1e0;
#[doc = "UARTDMA1E4 (rw) register accessor: VUART1 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1e4`] module"]
#[doc(alias = "UARTDMA1E4")]
pub type Uartdma1e4 = crate::Reg<uartdma1e4::Uartdma1e4Spec>;
#[doc = "VUART1 TX write pointer"]
pub mod uartdma1e4;
#[doc = "UARTDMA1E8 (rw) register accessor: VUART1 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1e8`] module"]
#[doc(alias = "UARTDMA1E8")]
pub type Uartdma1e8 = crate::Reg<uartdma1e8::Uartdma1e8Spec>;
#[doc = "VUART1 TX buffer base address"]
pub mod uartdma1e8;
#[doc = "UARTDMA1EC (rw) register accessor: VUART1 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1ec`] module"]
#[doc(alias = "UARTDMA1EC")]
pub type Uartdma1ec = crate::Reg<uartdma1ec::Uartdma1ecSpec>;
#[doc = "VUART1 TX control register"]
pub mod uartdma1ec;
#[doc = "UARTDMA1F0 (rw) register accessor: VUART1 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1f0`] module"]
#[doc(alias = "UARTDMA1F0")]
pub type Uartdma1f0 = crate::Reg<uartdma1f0::Uartdma1f0Spec>;
#[doc = "VUART1 RX read pointer"]
pub mod uartdma1f0;
#[doc = "UARTDMA1F4 (rw) register accessor: VUART1 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1f4`] module"]
#[doc(alias = "UARTDMA1F4")]
pub type Uartdma1f4 = crate::Reg<uartdma1f4::Uartdma1f4Spec>;
#[doc = "VUART1 RX write pointer"]
pub mod uartdma1f4;
#[doc = "UARTDMA1F8 (rw) register accessor: VUART1 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1f8`] module"]
#[doc(alias = "UARTDMA1F8")]
pub type Uartdma1f8 = crate::Reg<uartdma1f8::Uartdma1f8Spec>;
#[doc = "VUART1 RX buffer base address"]
pub mod uartdma1f8;
#[doc = "UARTDMA1FC (rw) register accessor: VUART1 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma1fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma1fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma1fc`] module"]
#[doc(alias = "UARTDMA1FC")]
pub type Uartdma1fc = crate::Reg<uartdma1fc::Uartdma1fcSpec>;
#[doc = "VUART1 RX control register"]
pub mod uartdma1fc;
#[doc = "UARTDMA200 (rw) register accessor: VUART2 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma200`] module"]
#[doc(alias = "UARTDMA200")]
pub type Uartdma200 = crate::Reg<uartdma200::Uartdma200Spec>;
#[doc = "VUART2 TX read pointer"]
pub mod uartdma200;
#[doc = "UARTDMA204 (rw) register accessor: VUART2 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma204`] module"]
#[doc(alias = "UARTDMA204")]
pub type Uartdma204 = crate::Reg<uartdma204::Uartdma204Spec>;
#[doc = "VUART2 TX write pointer"]
pub mod uartdma204;
#[doc = "UARTDMA208 (rw) register accessor: VUART2 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma208`] module"]
#[doc(alias = "UARTDMA208")]
pub type Uartdma208 = crate::Reg<uartdma208::Uartdma208Spec>;
#[doc = "VUART2 TX buffer base address"]
pub mod uartdma208;
#[doc = "UARTDMA20C (rw) register accessor: VUART2 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma20c`] module"]
#[doc(alias = "UARTDMA20C")]
pub type Uartdma20c = crate::Reg<uartdma20c::Uartdma20cSpec>;
#[doc = "VUART2 TX control register"]
pub mod uartdma20c;
#[doc = "UARTDMA210 (rw) register accessor: VUART2 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma210`] module"]
#[doc(alias = "UARTDMA210")]
pub type Uartdma210 = crate::Reg<uartdma210::Uartdma210Spec>;
#[doc = "VUART2 RX read pointer"]
pub mod uartdma210;
#[doc = "UARTDMA214 (rw) register accessor: VUART2 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma214`] module"]
#[doc(alias = "UARTDMA214")]
pub type Uartdma214 = crate::Reg<uartdma214::Uartdma214Spec>;
#[doc = "VUART2 RX write pointer"]
pub mod uartdma214;
#[doc = "UARTDMA218 (rw) register accessor: VUART2 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma218`] module"]
#[doc(alias = "UARTDMA218")]
pub type Uartdma218 = crate::Reg<uartdma218::Uartdma218Spec>;
#[doc = "VUART2 RX buffer base address"]
pub mod uartdma218;
#[doc = "UARTDMA21C (rw) register accessor: VUART2 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma21c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma21c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma21c`] module"]
#[doc(alias = "UARTDMA21C")]
pub type Uartdma21c = crate::Reg<uartdma21c::Uartdma21cSpec>;
#[doc = "VUART2 RX control register"]
pub mod uartdma21c;
#[doc = "UARTDMA220 (rw) register accessor: VUART3 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma220`] module"]
#[doc(alias = "UARTDMA220")]
pub type Uartdma220 = crate::Reg<uartdma220::Uartdma220Spec>;
#[doc = "VUART3 TX read pointer"]
pub mod uartdma220;
#[doc = "UARTDMA224 (rw) register accessor: VUART3 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma224`] module"]
#[doc(alias = "UARTDMA224")]
pub type Uartdma224 = crate::Reg<uartdma224::Uartdma224Spec>;
#[doc = "VUART3 TX write pointer"]
pub mod uartdma224;
#[doc = "UARTDMA228 (rw) register accessor: VUART3 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma228`] module"]
#[doc(alias = "UARTDMA228")]
pub type Uartdma228 = crate::Reg<uartdma228::Uartdma228Spec>;
#[doc = "VUART3 TX buffer base address"]
pub mod uartdma228;
#[doc = "UARTDMA22C (rw) register accessor: VUART3 TX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma22c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma22c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma22c`] module"]
#[doc(alias = "UARTDMA22C")]
pub type Uartdma22c = crate::Reg<uartdma22c::Uartdma22cSpec>;
#[doc = "VUART3 TX control register"]
pub mod uartdma22c;
#[doc = "UARTDMA230 (rw) register accessor: VUART3 RX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma230`] module"]
#[doc(alias = "UARTDMA230")]
pub type Uartdma230 = crate::Reg<uartdma230::Uartdma230Spec>;
#[doc = "VUART3 RX read pointer"]
pub mod uartdma230;
#[doc = "UARTDMA234 (rw) register accessor: VUART3 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma234`] module"]
#[doc(alias = "UARTDMA234")]
pub type Uartdma234 = crate::Reg<uartdma234::Uartdma234Spec>;
#[doc = "VUART3 RX write pointer"]
pub mod uartdma234;
#[doc = "UARTDMA238 (rw) register accessor: VUART3 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma238`] module"]
#[doc(alias = "UARTDMA238")]
pub type Uartdma238 = crate::Reg<uartdma238::Uartdma238Spec>;
#[doc = "VUART3 RX buffer base address"]
pub mod uartdma238;
#[doc = "UARTDMA23C (rw) register accessor: VUART3 RX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma23c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma23c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdma23c`] module"]
#[doc(alias = "UARTDMA23C")]
pub type Uartdma23c = crate::Reg<uartdma23c::Uartdma23cSpec>;
#[doc = "VUART3 RX control register"]
pub mod uartdma23c;
