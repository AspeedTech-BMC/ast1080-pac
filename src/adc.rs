#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc000: Adc000,
    adc004: Adc004,
    adc008: Adc008,
    adc00c: Adc00c,
    adc010: Adc010,
    adc014: Adc014,
    adc018: Adc018,
    adc01c: Adc01c,
    _reserved8: [u8; 0x10],
    adc030: Adc030,
    adc034: Adc034,
    adc038: Adc038,
    adc03c: Adc03c,
    adc040: Adc040,
    adc044: Adc044,
    adc048: Adc048,
    adc04c: Adc04c,
    _reserved16: [u8; 0x20],
    adc070: Adc070,
    adc074: Adc074,
    adc078: Adc078,
    adc07c: Adc07c,
    adc080: Adc080,
    adc084: Adc084,
    adc088: Adc088,
    adc08c: Adc08c,
    _reserved24: [u8; 0x30],
    adc0c0: Adc0c0,
    adc0c4: Adc0c4,
    _reserved26: [u8; 0x38],
    adc100: Adc100,
    adc104: Adc104,
    adc108: Adc108,
    adc10c: Adc10c,
    adc110: Adc110,
    adc114: Adc114,
    adc118: Adc118,
    adc11c: Adc11c,
    _reserved34: [u8; 0x10],
    adc130: Adc130,
    adc134: Adc134,
    adc138: Adc138,
    adc13c: Adc13c,
    adc140: Adc140,
    adc144: Adc144,
    adc148: Adc148,
    adc14c: Adc14c,
    _reserved42: [u8; 0x20],
    adc170: Adc170,
    adc174: Adc174,
    adc178: Adc178,
    adc17c: Adc17c,
    adc180: Adc180,
    adc184: Adc184,
    adc188: Adc188,
    adc18c: Adc18c,
    _reserved50: [u8; 0x30],
    adc1c0: Adc1c0,
    adc1c4: Adc1c4,
}
impl RegisterBlock {
    #[doc = "0x00 - Engine Control"]
    #[inline(always)]
    pub const fn adc000(&self) -> &Adc000 {
        &self.adc000
    }
    #[doc = "0x04 - Interrupt Enable and Interrupt Status"]
    #[inline(always)]
    pub const fn adc004(&self) -> &Adc004 {
        &self.adc004
    }
    #[doc = "0x08 - ADC VGA Detect Control"]
    #[inline(always)]
    pub const fn adc008(&self) -> &Adc008 {
        &self.adc008
    }
    #[doc = "0x0c - ADC Clock Control"]
    #[inline(always)]
    pub const fn adc00c(&self) -> &Adc00c {
        &self.adc00c
    }
    #[doc = "0x10 - Data of Channel 1 and 0"]
    #[inline(always)]
    pub const fn adc010(&self) -> &Adc010 {
        &self.adc010
    }
    #[doc = "0x14 - Data of Channel 3 and 2"]
    #[inline(always)]
    pub const fn adc014(&self) -> &Adc014 {
        &self.adc014
    }
    #[doc = "0x18 - Data of Channel 5 and 4"]
    #[inline(always)]
    pub const fn adc018(&self) -> &Adc018 {
        &self.adc018
    }
    #[doc = "0x1c - Data of Channel 7 and 6"]
    #[inline(always)]
    pub const fn adc01c(&self) -> &Adc01c {
        &self.adc01c
    }
    #[doc = "0x30 - Upper and Lower bound of Channel 0"]
    #[inline(always)]
    pub const fn adc030(&self) -> &Adc030 {
        &self.adc030
    }
    #[doc = "0x34 - Upper and Lower bound of Channel 1"]
    #[inline(always)]
    pub const fn adc034(&self) -> &Adc034 {
        &self.adc034
    }
    #[doc = "0x38 - Upper and Lower bound of Channel 2"]
    #[inline(always)]
    pub const fn adc038(&self) -> &Adc038 {
        &self.adc038
    }
    #[doc = "0x3c - Upper and Lower bound of Channel 3"]
    #[inline(always)]
    pub const fn adc03c(&self) -> &Adc03c {
        &self.adc03c
    }
    #[doc = "0x40 - Upper and Lower bound of Channel 4"]
    #[inline(always)]
    pub const fn adc040(&self) -> &Adc040 {
        &self.adc040
    }
    #[doc = "0x44 - Upper and Lower bound of Channel 5"]
    #[inline(always)]
    pub const fn adc044(&self) -> &Adc044 {
        &self.adc044
    }
    #[doc = "0x48 - Upper and Lower bound of Channel 6"]
    #[inline(always)]
    pub const fn adc048(&self) -> &Adc048 {
        &self.adc048
    }
    #[doc = "0x4c - Upper and Lower bound of Channel 7"]
    #[inline(always)]
    pub const fn adc04c(&self) -> &Adc04c {
        &self.adc04c
    }
    #[doc = "0x70 - Hysteresis Control and bound of Channel 0"]
    #[inline(always)]
    pub const fn adc070(&self) -> &Adc070 {
        &self.adc070
    }
    #[doc = "0x74 - Hysteresis Control and bound of Channel 1"]
    #[inline(always)]
    pub const fn adc074(&self) -> &Adc074 {
        &self.adc074
    }
    #[doc = "0x78 - Hysteresis Control and bound of Channel 2"]
    #[inline(always)]
    pub const fn adc078(&self) -> &Adc078 {
        &self.adc078
    }
    #[doc = "0x7c - Hysteresis Control and bound of Channel 3"]
    #[inline(always)]
    pub const fn adc07c(&self) -> &Adc07c {
        &self.adc07c
    }
    #[doc = "0x80 - Hysteresis Control and bound of Channel 4"]
    #[inline(always)]
    pub const fn adc080(&self) -> &Adc080 {
        &self.adc080
    }
    #[doc = "0x84 - Hysteresis Control and bound of Channel 5"]
    #[inline(always)]
    pub const fn adc084(&self) -> &Adc084 {
        &self.adc084
    }
    #[doc = "0x88 - Hysteresis Control and bound of Channel 6"]
    #[inline(always)]
    pub const fn adc088(&self) -> &Adc088 {
        &self.adc088
    }
    #[doc = "0x8c - Hysteresis Control and bound of Channel 7"]
    #[inline(always)]
    pub const fn adc08c(&self) -> &Adc08c {
        &self.adc08c
    }
    #[doc = "0xc0 - Interrupt Source"]
    #[inline(always)]
    pub const fn adc0c0(&self) -> &Adc0c0 {
        &self.adc0c0
    }
    #[doc = "0xc4 - Compensating and Trimming"]
    #[inline(always)]
    pub const fn adc0c4(&self) -> &Adc0c4 {
        &self.adc0c4
    }
    #[doc = "0x100 - Engine Control"]
    #[inline(always)]
    pub const fn adc100(&self) -> &Adc100 {
        &self.adc100
    }
    #[doc = "0x104 - Interrupt Enable and Interrupt Status"]
    #[inline(always)]
    pub const fn adc104(&self) -> &Adc104 {
        &self.adc104
    }
    #[doc = "0x108 - ADC VGA Detect Control"]
    #[inline(always)]
    pub const fn adc108(&self) -> &Adc108 {
        &self.adc108
    }
    #[doc = "0x10c - ADC Clock Control"]
    #[inline(always)]
    pub const fn adc10c(&self) -> &Adc10c {
        &self.adc10c
    }
    #[doc = "0x110 - Data of Channel 9 and 8"]
    #[inline(always)]
    pub const fn adc110(&self) -> &Adc110 {
        &self.adc110
    }
    #[doc = "0x114 - Data of Channel 11 and 10"]
    #[inline(always)]
    pub const fn adc114(&self) -> &Adc114 {
        &self.adc114
    }
    #[doc = "0x118 - Data of Channel 13 and 12"]
    #[inline(always)]
    pub const fn adc118(&self) -> &Adc118 {
        &self.adc118
    }
    #[doc = "0x11c - Data of Channel 15 and 14"]
    #[inline(always)]
    pub const fn adc11c(&self) -> &Adc11c {
        &self.adc11c
    }
    #[doc = "0x130 - Upper and Lower bound of Channel 8"]
    #[inline(always)]
    pub const fn adc130(&self) -> &Adc130 {
        &self.adc130
    }
    #[doc = "0x134 - Upper and Lower bound of Channel 9"]
    #[inline(always)]
    pub const fn adc134(&self) -> &Adc134 {
        &self.adc134
    }
    #[doc = "0x138 - Upper and Lower bound of Channel 10"]
    #[inline(always)]
    pub const fn adc138(&self) -> &Adc138 {
        &self.adc138
    }
    #[doc = "0x13c - Upper and Lower bound of Channel 11"]
    #[inline(always)]
    pub const fn adc13c(&self) -> &Adc13c {
        &self.adc13c
    }
    #[doc = "0x140 - Upper and Lower bound of Channel 12"]
    #[inline(always)]
    pub const fn adc140(&self) -> &Adc140 {
        &self.adc140
    }
    #[doc = "0x144 - Upper and Lower bound of Channel 13"]
    #[inline(always)]
    pub const fn adc144(&self) -> &Adc144 {
        &self.adc144
    }
    #[doc = "0x148 - Upper and Lower bound of Channel 14"]
    #[inline(always)]
    pub const fn adc148(&self) -> &Adc148 {
        &self.adc148
    }
    #[doc = "0x14c - Upper and Lower bound of Channel 15"]
    #[inline(always)]
    pub const fn adc14c(&self) -> &Adc14c {
        &self.adc14c
    }
    #[doc = "0x170 - Hysteresis Control and bound of Channel 8"]
    #[inline(always)]
    pub const fn adc170(&self) -> &Adc170 {
        &self.adc170
    }
    #[doc = "0x174 - Hysteresis Control and bound of Channel 9"]
    #[inline(always)]
    pub const fn adc174(&self) -> &Adc174 {
        &self.adc174
    }
    #[doc = "0x178 - Hysteresis Control and bound of Channel 10"]
    #[inline(always)]
    pub const fn adc178(&self) -> &Adc178 {
        &self.adc178
    }
    #[doc = "0x17c - Hysteresis Control and bound of Channel 11"]
    #[inline(always)]
    pub const fn adc17c(&self) -> &Adc17c {
        &self.adc17c
    }
    #[doc = "0x180 - Hysteresis Control and bound of Channel 12"]
    #[inline(always)]
    pub const fn adc180(&self) -> &Adc180 {
        &self.adc180
    }
    #[doc = "0x184 - Hysteresis Control and bound of Channel 13"]
    #[inline(always)]
    pub const fn adc184(&self) -> &Adc184 {
        &self.adc184
    }
    #[doc = "0x188 - Hysteresis Control and bound of Channel 14"]
    #[inline(always)]
    pub const fn adc188(&self) -> &Adc188 {
        &self.adc188
    }
    #[doc = "0x18c - Hysteresis Control and bound of Channel 15"]
    #[inline(always)]
    pub const fn adc18c(&self) -> &Adc18c {
        &self.adc18c
    }
    #[doc = "0x1c0 - Interrupt Source"]
    #[inline(always)]
    pub const fn adc1c0(&self) -> &Adc1c0 {
        &self.adc1c0
    }
    #[doc = "0x1c4 - Compensating and Trimming"]
    #[inline(always)]
    pub const fn adc1c4(&self) -> &Adc1c4 {
        &self.adc1c4
    }
}
#[doc = "ADC000 (rw) register accessor: Engine Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc000`] module"]
#[doc(alias = "ADC000")]
pub type Adc000 = crate::Reg<adc000::Adc000Spec>;
#[doc = "Engine Control"]
pub mod adc000;
#[doc = "ADC004 (rw) register accessor: Interrupt Enable and Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`adc004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc004`] module"]
#[doc(alias = "ADC004")]
pub type Adc004 = crate::Reg<adc004::Adc004Spec>;
#[doc = "Interrupt Enable and Interrupt Status"]
pub mod adc004;
#[doc = "ADC008 (rw) register accessor: ADC VGA Detect Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc008`] module"]
#[doc(alias = "ADC008")]
pub type Adc008 = crate::Reg<adc008::Adc008Spec>;
#[doc = "ADC VGA Detect Control"]
pub mod adc008;
#[doc = "ADC00C (rw) register accessor: ADC Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc00c`] module"]
#[doc(alias = "ADC00C")]
pub type Adc00c = crate::Reg<adc00c::Adc00cSpec>;
#[doc = "ADC Clock Control"]
pub mod adc00c;
#[doc = "ADC010 (rw) register accessor: Data of Channel 1 and 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc010`] module"]
#[doc(alias = "ADC010")]
pub type Adc010 = crate::Reg<adc010::Adc010Spec>;
#[doc = "Data of Channel 1 and 0"]
pub mod adc010;
#[doc = "ADC014 (rw) register accessor: Data of Channel 3 and 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc014`] module"]
#[doc(alias = "ADC014")]
pub type Adc014 = crate::Reg<adc014::Adc014Spec>;
#[doc = "Data of Channel 3 and 2"]
pub mod adc014;
#[doc = "ADC018 (rw) register accessor: Data of Channel 5 and 4\n\nYou can [`read`](crate::Reg::read) this register and get [`adc018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc018`] module"]
#[doc(alias = "ADC018")]
pub type Adc018 = crate::Reg<adc018::Adc018Spec>;
#[doc = "Data of Channel 5 and 4"]
pub mod adc018;
#[doc = "ADC01C (rw) register accessor: Data of Channel 7 and 6\n\nYou can [`read`](crate::Reg::read) this register and get [`adc01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc01c`] module"]
#[doc(alias = "ADC01C")]
pub type Adc01c = crate::Reg<adc01c::Adc01cSpec>;
#[doc = "Data of Channel 7 and 6"]
pub mod adc01c;
#[doc = "ADC030 (rw) register accessor: Upper and Lower bound of Channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc030`] module"]
#[doc(alias = "ADC030")]
pub type Adc030 = crate::Reg<adc030::Adc030Spec>;
#[doc = "Upper and Lower bound of Channel 0"]
pub mod adc030;
#[doc = "ADC034 (rw) register accessor: Upper and Lower bound of Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc034`] module"]
#[doc(alias = "ADC034")]
pub type Adc034 = crate::Reg<adc034::Adc034Spec>;
#[doc = "Upper and Lower bound of Channel 1"]
pub mod adc034;
#[doc = "ADC038 (rw) register accessor: Upper and Lower bound of Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc038`] module"]
#[doc(alias = "ADC038")]
pub type Adc038 = crate::Reg<adc038::Adc038Spec>;
#[doc = "Upper and Lower bound of Channel 2"]
pub mod adc038;
#[doc = "ADC03C (rw) register accessor: Upper and Lower bound of Channel 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adc03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc03c`] module"]
#[doc(alias = "ADC03C")]
pub type Adc03c = crate::Reg<adc03c::Adc03cSpec>;
#[doc = "Upper and Lower bound of Channel 3"]
pub mod adc03c;
#[doc = "ADC040 (rw) register accessor: Upper and Lower bound of Channel 4\n\nYou can [`read`](crate::Reg::read) this register and get [`adc040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc040`] module"]
#[doc(alias = "ADC040")]
pub type Adc040 = crate::Reg<adc040::Adc040Spec>;
#[doc = "Upper and Lower bound of Channel 4"]
pub mod adc040;
#[doc = "ADC044 (rw) register accessor: Upper and Lower bound of Channel 5\n\nYou can [`read`](crate::Reg::read) this register and get [`adc044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc044`] module"]
#[doc(alias = "ADC044")]
pub type Adc044 = crate::Reg<adc044::Adc044Spec>;
#[doc = "Upper and Lower bound of Channel 5"]
pub mod adc044;
#[doc = "ADC048 (rw) register accessor: Upper and Lower bound of Channel 6\n\nYou can [`read`](crate::Reg::read) this register and get [`adc048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc048`] module"]
#[doc(alias = "ADC048")]
pub type Adc048 = crate::Reg<adc048::Adc048Spec>;
#[doc = "Upper and Lower bound of Channel 6"]
pub mod adc048;
#[doc = "ADC04C (rw) register accessor: Upper and Lower bound of Channel 7\n\nYou can [`read`](crate::Reg::read) this register and get [`adc04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc04c`] module"]
#[doc(alias = "ADC04C")]
pub type Adc04c = crate::Reg<adc04c::Adc04cSpec>;
#[doc = "Upper and Lower bound of Channel 7"]
pub mod adc04c;
#[doc = "ADC070 (rw) register accessor: Hysteresis Control and bound of Channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc070`] module"]
#[doc(alias = "ADC070")]
pub type Adc070 = crate::Reg<adc070::Adc070Spec>;
#[doc = "Hysteresis Control and bound of Channel 0"]
pub mod adc070;
#[doc = "ADC074 (rw) register accessor: Hysteresis Control and bound of Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc074`] module"]
#[doc(alias = "ADC074")]
pub type Adc074 = crate::Reg<adc074::Adc074Spec>;
#[doc = "Hysteresis Control and bound of Channel 1"]
pub mod adc074;
#[doc = "ADC078 (rw) register accessor: Hysteresis Control and bound of Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc078`] module"]
#[doc(alias = "ADC078")]
pub type Adc078 = crate::Reg<adc078::Adc078Spec>;
#[doc = "Hysteresis Control and bound of Channel 2"]
pub mod adc078;
#[doc = "ADC07C (rw) register accessor: Hysteresis Control and bound of Channel 3\n\nYou can [`read`](crate::Reg::read) this register and get [`adc07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc07c`] module"]
#[doc(alias = "ADC07C")]
pub type Adc07c = crate::Reg<adc07c::Adc07cSpec>;
#[doc = "Hysteresis Control and bound of Channel 3"]
pub mod adc07c;
#[doc = "ADC080 (rw) register accessor: Hysteresis Control and bound of Channel 4\n\nYou can [`read`](crate::Reg::read) this register and get [`adc080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc080`] module"]
#[doc(alias = "ADC080")]
pub type Adc080 = crate::Reg<adc080::Adc080Spec>;
#[doc = "Hysteresis Control and bound of Channel 4"]
pub mod adc080;
#[doc = "ADC084 (rw) register accessor: Hysteresis Control and bound of Channel 5\n\nYou can [`read`](crate::Reg::read) this register and get [`adc084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc084`] module"]
#[doc(alias = "ADC084")]
pub type Adc084 = crate::Reg<adc084::Adc084Spec>;
#[doc = "Hysteresis Control and bound of Channel 5"]
pub mod adc084;
#[doc = "ADC088 (rw) register accessor: Hysteresis Control and bound of Channel 6\n\nYou can [`read`](crate::Reg::read) this register and get [`adc088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc088`] module"]
#[doc(alias = "ADC088")]
pub type Adc088 = crate::Reg<adc088::Adc088Spec>;
#[doc = "Hysteresis Control and bound of Channel 6"]
pub mod adc088;
#[doc = "ADC08C (rw) register accessor: Hysteresis Control and bound of Channel 7\n\nYou can [`read`](crate::Reg::read) this register and get [`adc08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc08c`] module"]
#[doc(alias = "ADC08C")]
pub type Adc08c = crate::Reg<adc08c::Adc08cSpec>;
#[doc = "Hysteresis Control and bound of Channel 7"]
pub mod adc08c;
#[doc = "ADC0C0 (rw) register accessor: Interrupt Source\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0c0`] module"]
#[doc(alias = "ADC0C0")]
pub type Adc0c0 = crate::Reg<adc0c0::Adc0c0Spec>;
#[doc = "Interrupt Source"]
pub mod adc0c0;
#[doc = "ADC0C4 (rw) register accessor: Compensating and Trimming\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0c4`] module"]
#[doc(alias = "ADC0C4")]
pub type Adc0c4 = crate::Reg<adc0c4::Adc0c4Spec>;
#[doc = "Compensating and Trimming"]
pub mod adc0c4;
#[doc = "ADC100 (rw) register accessor: Engine Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc100`] module"]
#[doc(alias = "ADC100")]
pub type Adc100 = crate::Reg<adc100::Adc100Spec>;
#[doc = "Engine Control"]
pub mod adc100;
#[doc = "ADC104 (rw) register accessor: Interrupt Enable and Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`adc104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc104`] module"]
#[doc(alias = "ADC104")]
pub type Adc104 = crate::Reg<adc104::Adc104Spec>;
#[doc = "Interrupt Enable and Interrupt Status"]
pub mod adc104;
#[doc = "ADC108 (rw) register accessor: ADC VGA Detect Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc108`] module"]
#[doc(alias = "ADC108")]
pub type Adc108 = crate::Reg<adc108::Adc108Spec>;
#[doc = "ADC VGA Detect Control"]
pub mod adc108;
#[doc = "ADC10C (rw) register accessor: ADC Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc10c`] module"]
#[doc(alias = "ADC10C")]
pub type Adc10c = crate::Reg<adc10c::Adc10cSpec>;
#[doc = "ADC Clock Control"]
pub mod adc10c;
#[doc = "ADC110 (rw) register accessor: Data of Channel 9 and 8\n\nYou can [`read`](crate::Reg::read) this register and get [`adc110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc110`] module"]
#[doc(alias = "ADC110")]
pub type Adc110 = crate::Reg<adc110::Adc110Spec>;
#[doc = "Data of Channel 9 and 8"]
pub mod adc110;
#[doc = "ADC114 (rw) register accessor: Data of Channel 11 and 10\n\nYou can [`read`](crate::Reg::read) this register and get [`adc114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc114`] module"]
#[doc(alias = "ADC114")]
pub type Adc114 = crate::Reg<adc114::Adc114Spec>;
#[doc = "Data of Channel 11 and 10"]
pub mod adc114;
#[doc = "ADC118 (rw) register accessor: Data of Channel 13 and 12\n\nYou can [`read`](crate::Reg::read) this register and get [`adc118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc118`] module"]
#[doc(alias = "ADC118")]
pub type Adc118 = crate::Reg<adc118::Adc118Spec>;
#[doc = "Data of Channel 13 and 12"]
pub mod adc118;
#[doc = "ADC11C (rw) register accessor: Data of Channel 15 and 14\n\nYou can [`read`](crate::Reg::read) this register and get [`adc11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc11c`] module"]
#[doc(alias = "ADC11C")]
pub type Adc11c = crate::Reg<adc11c::Adc11cSpec>;
#[doc = "Data of Channel 15 and 14"]
pub mod adc11c;
#[doc = "ADC130 (rw) register accessor: Upper and Lower bound of Channel 8\n\nYou can [`read`](crate::Reg::read) this register and get [`adc130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc130`] module"]
#[doc(alias = "ADC130")]
pub type Adc130 = crate::Reg<adc130::Adc130Spec>;
#[doc = "Upper and Lower bound of Channel 8"]
pub mod adc130;
#[doc = "ADC134 (rw) register accessor: Upper and Lower bound of Channel 9\n\nYou can [`read`](crate::Reg::read) this register and get [`adc134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc134`] module"]
#[doc(alias = "ADC134")]
pub type Adc134 = crate::Reg<adc134::Adc134Spec>;
#[doc = "Upper and Lower bound of Channel 9"]
pub mod adc134;
#[doc = "ADC138 (rw) register accessor: Upper and Lower bound of Channel 10\n\nYou can [`read`](crate::Reg::read) this register and get [`adc138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc138`] module"]
#[doc(alias = "ADC138")]
pub type Adc138 = crate::Reg<adc138::Adc138Spec>;
#[doc = "Upper and Lower bound of Channel 10"]
pub mod adc138;
#[doc = "ADC13C (rw) register accessor: Upper and Lower bound of Channel 11\n\nYou can [`read`](crate::Reg::read) this register and get [`adc13c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc13c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc13c`] module"]
#[doc(alias = "ADC13C")]
pub type Adc13c = crate::Reg<adc13c::Adc13cSpec>;
#[doc = "Upper and Lower bound of Channel 11"]
pub mod adc13c;
#[doc = "ADC140 (rw) register accessor: Upper and Lower bound of Channel 12\n\nYou can [`read`](crate::Reg::read) this register and get [`adc140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc140`] module"]
#[doc(alias = "ADC140")]
pub type Adc140 = crate::Reg<adc140::Adc140Spec>;
#[doc = "Upper and Lower bound of Channel 12"]
pub mod adc140;
#[doc = "ADC144 (rw) register accessor: Upper and Lower bound of Channel 13\n\nYou can [`read`](crate::Reg::read) this register and get [`adc144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc144`] module"]
#[doc(alias = "ADC144")]
pub type Adc144 = crate::Reg<adc144::Adc144Spec>;
#[doc = "Upper and Lower bound of Channel 13"]
pub mod adc144;
#[doc = "ADC148 (rw) register accessor: Upper and Lower bound of Channel 14\n\nYou can [`read`](crate::Reg::read) this register and get [`adc148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc148`] module"]
#[doc(alias = "ADC148")]
pub type Adc148 = crate::Reg<adc148::Adc148Spec>;
#[doc = "Upper and Lower bound of Channel 14"]
pub mod adc148;
#[doc = "ADC14C (rw) register accessor: Upper and Lower bound of Channel 15\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc14c`] module"]
#[doc(alias = "ADC14C")]
pub type Adc14c = crate::Reg<adc14c::Adc14cSpec>;
#[doc = "Upper and Lower bound of Channel 15"]
pub mod adc14c;
#[doc = "ADC170 (rw) register accessor: Hysteresis Control and bound of Channel 8\n\nYou can [`read`](crate::Reg::read) this register and get [`adc170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc170`] module"]
#[doc(alias = "ADC170")]
pub type Adc170 = crate::Reg<adc170::Adc170Spec>;
#[doc = "Hysteresis Control and bound of Channel 8"]
pub mod adc170;
#[doc = "ADC174 (rw) register accessor: Hysteresis Control and bound of Channel 9\n\nYou can [`read`](crate::Reg::read) this register and get [`adc174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc174`] module"]
#[doc(alias = "ADC174")]
pub type Adc174 = crate::Reg<adc174::Adc174Spec>;
#[doc = "Hysteresis Control and bound of Channel 9"]
pub mod adc174;
#[doc = "ADC178 (rw) register accessor: Hysteresis Control and bound of Channel 10\n\nYou can [`read`](crate::Reg::read) this register and get [`adc178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc178`] module"]
#[doc(alias = "ADC178")]
pub type Adc178 = crate::Reg<adc178::Adc178Spec>;
#[doc = "Hysteresis Control and bound of Channel 10"]
pub mod adc178;
#[doc = "ADC17C (rw) register accessor: Hysteresis Control and bound of Channel 11\n\nYou can [`read`](crate::Reg::read) this register and get [`adc17c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc17c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc17c`] module"]
#[doc(alias = "ADC17C")]
pub type Adc17c = crate::Reg<adc17c::Adc17cSpec>;
#[doc = "Hysteresis Control and bound of Channel 11"]
pub mod adc17c;
#[doc = "ADC180 (rw) register accessor: Hysteresis Control and bound of Channel 12\n\nYou can [`read`](crate::Reg::read) this register and get [`adc180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc180`] module"]
#[doc(alias = "ADC180")]
pub type Adc180 = crate::Reg<adc180::Adc180Spec>;
#[doc = "Hysteresis Control and bound of Channel 12"]
pub mod adc180;
#[doc = "ADC184 (rw) register accessor: Hysteresis Control and bound of Channel 13\n\nYou can [`read`](crate::Reg::read) this register and get [`adc184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc184`] module"]
#[doc(alias = "ADC184")]
pub type Adc184 = crate::Reg<adc184::Adc184Spec>;
#[doc = "Hysteresis Control and bound of Channel 13"]
pub mod adc184;
#[doc = "ADC188 (rw) register accessor: Hysteresis Control and bound of Channel 14\n\nYou can [`read`](crate::Reg::read) this register and get [`adc188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc188`] module"]
#[doc(alias = "ADC188")]
pub type Adc188 = crate::Reg<adc188::Adc188Spec>;
#[doc = "Hysteresis Control and bound of Channel 14"]
pub mod adc188;
#[doc = "ADC18C (rw) register accessor: Hysteresis Control and bound of Channel 15\n\nYou can [`read`](crate::Reg::read) this register and get [`adc18c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc18c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc18c`] module"]
#[doc(alias = "ADC18C")]
pub type Adc18c = crate::Reg<adc18c::Adc18cSpec>;
#[doc = "Hysteresis Control and bound of Channel 15"]
pub mod adc18c;
#[doc = "ADC1C0 (rw) register accessor: Interrupt Source\n\nYou can [`read`](crate::Reg::read) this register and get [`adc1c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc1c0`] module"]
#[doc(alias = "ADC1C0")]
pub type Adc1c0 = crate::Reg<adc1c0::Adc1c0Spec>;
#[doc = "Interrupt Source"]
pub mod adc1c0;
#[doc = "ADC1C4 (rw) register accessor: Compensating and Trimming\n\nYou can [`read`](crate::Reg::read) this register and get [`adc1c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc1c4`] module"]
#[doc(alias = "ADC1C4")]
pub type Adc1c4 = crate::Reg<adc1c4::Adc1c4Spec>;
#[doc = "Compensating and Trimming"]
pub mod adc1c4;
