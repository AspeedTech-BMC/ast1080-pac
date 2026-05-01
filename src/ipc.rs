#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipc000: Ipc000,
    ipc004: Ipc004,
    ipc008: Ipc008,
    ipc00c: Ipc00c,
    ipc010: Ipc010,
    ipc014: Ipc014,
    ipc018: Ipc018,
    ipc01c: Ipc01c,
    ipc020: Ipc020,
    ipc024: Ipc024,
    ipc028: Ipc028,
    ipc02c: Ipc02c,
    ipc030: Ipc030,
    ipc034: Ipc034,
    ipc038: Ipc038,
    ipc03c: Ipc03c,
    ipc040: Ipc040,
    ipc044: Ipc044,
    ipc048: Ipc048,
    ipc04c: Ipc04c,
    ipc050: Ipc050,
    ipc054: Ipc054,
    ipc058: Ipc058,
    ipc05c: Ipc05c,
    ipc060: Ipc060,
    ipc064: Ipc064,
    ipc068: Ipc068,
    ipc06c: Ipc06c,
    ipc070: Ipc070,
    ipc074: Ipc074,
    ipc078: Ipc078,
    ipc07c: Ipc07c,
    ipc080: Ipc080,
    ipc084: Ipc084,
    ipc088: Ipc088,
    ipc08c: Ipc08c,
}
impl RegisterBlock {
    #[doc = "0x00 - IPI trig , w1 trigger"]
    #[inline(always)]
    pub const fn ipc000(&self) -> &Ipc000 {
        &self.ipc000
    }
    #[doc = "0x04 - IPI enable"]
    #[inline(always)]
    pub const fn ipc004(&self) -> &Ipc004 {
        &self.ipc004
    }
    #[doc = "0x08 - ipi status"]
    #[inline(always)]
    pub const fn ipc008(&self) -> &Ipc008 {
        &self.ipc008
    }
    #[doc = "0x0c - reserved"]
    #[inline(always)]
    pub const fn ipc00c(&self) -> &Ipc00c {
        &self.ipc00c
    }
    #[doc = "0x10 - tx ipi0 reg0"]
    #[inline(always)]
    pub const fn ipc010(&self) -> &Ipc010 {
        &self.ipc010
    }
    #[doc = "0x14 - tx ipi0 reg1"]
    #[inline(always)]
    pub const fn ipc014(&self) -> &Ipc014 {
        &self.ipc014
    }
    #[doc = "0x18 - tx ipi0 reg2"]
    #[inline(always)]
    pub const fn ipc018(&self) -> &Ipc018 {
        &self.ipc018
    }
    #[doc = "0x1c - tx ipi0 reg3"]
    #[inline(always)]
    pub const fn ipc01c(&self) -> &Ipc01c {
        &self.ipc01c
    }
    #[doc = "0x20 - tx ipi0 reg4"]
    #[inline(always)]
    pub const fn ipc020(&self) -> &Ipc020 {
        &self.ipc020
    }
    #[doc = "0x24 - tx ipi0 reg5"]
    #[inline(always)]
    pub const fn ipc024(&self) -> &Ipc024 {
        &self.ipc024
    }
    #[doc = "0x28 - tx ipi0 reg6"]
    #[inline(always)]
    pub const fn ipc028(&self) -> &Ipc028 {
        &self.ipc028
    }
    #[doc = "0x2c - tx ipi0 reg7"]
    #[inline(always)]
    pub const fn ipc02c(&self) -> &Ipc02c {
        &self.ipc02c
    }
    #[doc = "0x30 - tx ipi1 reg0"]
    #[inline(always)]
    pub const fn ipc030(&self) -> &Ipc030 {
        &self.ipc030
    }
    #[doc = "0x34 - tx ipi1 reg1"]
    #[inline(always)]
    pub const fn ipc034(&self) -> &Ipc034 {
        &self.ipc034
    }
    #[doc = "0x38 - tx ipi1 reg2"]
    #[inline(always)]
    pub const fn ipc038(&self) -> &Ipc038 {
        &self.ipc038
    }
    #[doc = "0x3c - tx ipi1 reg3"]
    #[inline(always)]
    pub const fn ipc03c(&self) -> &Ipc03c {
        &self.ipc03c
    }
    #[doc = "0x40 - tx ipi1 reg4"]
    #[inline(always)]
    pub const fn ipc040(&self) -> &Ipc040 {
        &self.ipc040
    }
    #[doc = "0x44 - tx ipi1 reg5"]
    #[inline(always)]
    pub const fn ipc044(&self) -> &Ipc044 {
        &self.ipc044
    }
    #[doc = "0x48 - tx ipi1 reg6"]
    #[inline(always)]
    pub const fn ipc048(&self) -> &Ipc048 {
        &self.ipc048
    }
    #[doc = "0x4c - tx ipi1 reg7"]
    #[inline(always)]
    pub const fn ipc04c(&self) -> &Ipc04c {
        &self.ipc04c
    }
    #[doc = "0x50 - tx ipi2 reg0"]
    #[inline(always)]
    pub const fn ipc050(&self) -> &Ipc050 {
        &self.ipc050
    }
    #[doc = "0x54 - tx ipi2 reg1"]
    #[inline(always)]
    pub const fn ipc054(&self) -> &Ipc054 {
        &self.ipc054
    }
    #[doc = "0x58 - tx ipi2 reg2"]
    #[inline(always)]
    pub const fn ipc058(&self) -> &Ipc058 {
        &self.ipc058
    }
    #[doc = "0x5c - tx ipi2 reg3"]
    #[inline(always)]
    pub const fn ipc05c(&self) -> &Ipc05c {
        &self.ipc05c
    }
    #[doc = "0x60 - tx ipi2 reg4"]
    #[inline(always)]
    pub const fn ipc060(&self) -> &Ipc060 {
        &self.ipc060
    }
    #[doc = "0x64 - tx ipi2 reg5"]
    #[inline(always)]
    pub const fn ipc064(&self) -> &Ipc064 {
        &self.ipc064
    }
    #[doc = "0x68 - tx ipi2 reg6"]
    #[inline(always)]
    pub const fn ipc068(&self) -> &Ipc068 {
        &self.ipc068
    }
    #[doc = "0x6c - tx ipi2 reg7"]
    #[inline(always)]
    pub const fn ipc06c(&self) -> &Ipc06c {
        &self.ipc06c
    }
    #[doc = "0x70 - tx ipi3 reg0"]
    #[inline(always)]
    pub const fn ipc070(&self) -> &Ipc070 {
        &self.ipc070
    }
    #[doc = "0x74 - tx ipi3 reg1"]
    #[inline(always)]
    pub const fn ipc074(&self) -> &Ipc074 {
        &self.ipc074
    }
    #[doc = "0x78 - tx ipi3 reg2"]
    #[inline(always)]
    pub const fn ipc078(&self) -> &Ipc078 {
        &self.ipc078
    }
    #[doc = "0x7c - tx ipi3 reg3"]
    #[inline(always)]
    pub const fn ipc07c(&self) -> &Ipc07c {
        &self.ipc07c
    }
    #[doc = "0x80 - tx ipi3 reg4"]
    #[inline(always)]
    pub const fn ipc080(&self) -> &Ipc080 {
        &self.ipc080
    }
    #[doc = "0x84 - tx ipi3 reg5"]
    #[inline(always)]
    pub const fn ipc084(&self) -> &Ipc084 {
        &self.ipc084
    }
    #[doc = "0x88 - tx ipi3 reg6"]
    #[inline(always)]
    pub const fn ipc088(&self) -> &Ipc088 {
        &self.ipc088
    }
    #[doc = "0x8c - tx ipi3 reg7"]
    #[inline(always)]
    pub const fn ipc08c(&self) -> &Ipc08c {
        &self.ipc08c
    }
}
#[doc = "IPC000 (rw) register accessor: IPI trig , w1 trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc000`] module"]
#[doc(alias = "IPC000")]
pub type Ipc000 = crate::Reg<ipc000::Ipc000Spec>;
#[doc = "IPI trig , w1 trigger"]
pub mod ipc000;
#[doc = "IPC004 (rw) register accessor: IPI enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc004`] module"]
#[doc(alias = "IPC004")]
pub type Ipc004 = crate::Reg<ipc004::Ipc004Spec>;
#[doc = "IPI enable"]
pub mod ipc004;
#[doc = "IPC008 (rw) register accessor: ipi status\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc008`] module"]
#[doc(alias = "IPC008")]
pub type Ipc008 = crate::Reg<ipc008::Ipc008Spec>;
#[doc = "ipi status"]
pub mod ipc008;
#[doc = "IPC00C (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc00c`] module"]
#[doc(alias = "IPC00C")]
pub type Ipc00c = crate::Reg<ipc00c::Ipc00cSpec>;
#[doc = "reserved"]
pub mod ipc00c;
#[doc = "IPC010 (rw) register accessor: tx ipi0 reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc010`] module"]
#[doc(alias = "IPC010")]
pub type Ipc010 = crate::Reg<ipc010::Ipc010Spec>;
#[doc = "tx ipi0 reg0"]
pub mod ipc010;
#[doc = "IPC014 (rw) register accessor: tx ipi0 reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc014`] module"]
#[doc(alias = "IPC014")]
pub type Ipc014 = crate::Reg<ipc014::Ipc014Spec>;
#[doc = "tx ipi0 reg1"]
pub mod ipc014;
#[doc = "IPC018 (rw) register accessor: tx ipi0 reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc018`] module"]
#[doc(alias = "IPC018")]
pub type Ipc018 = crate::Reg<ipc018::Ipc018Spec>;
#[doc = "tx ipi0 reg2"]
pub mod ipc018;
#[doc = "IPC01C (rw) register accessor: tx ipi0 reg3\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc01c`] module"]
#[doc(alias = "IPC01C")]
pub type Ipc01c = crate::Reg<ipc01c::Ipc01cSpec>;
#[doc = "tx ipi0 reg3"]
pub mod ipc01c;
#[doc = "IPC020 (rw) register accessor: tx ipi0 reg4\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc020`] module"]
#[doc(alias = "IPC020")]
pub type Ipc020 = crate::Reg<ipc020::Ipc020Spec>;
#[doc = "tx ipi0 reg4"]
pub mod ipc020;
#[doc = "IPC024 (rw) register accessor: tx ipi0 reg5\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc024`] module"]
#[doc(alias = "IPC024")]
pub type Ipc024 = crate::Reg<ipc024::Ipc024Spec>;
#[doc = "tx ipi0 reg5"]
pub mod ipc024;
#[doc = "IPC028 (rw) register accessor: tx ipi0 reg6\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc028`] module"]
#[doc(alias = "IPC028")]
pub type Ipc028 = crate::Reg<ipc028::Ipc028Spec>;
#[doc = "tx ipi0 reg6"]
pub mod ipc028;
#[doc = "IPC02C (rw) register accessor: tx ipi0 reg7\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc02c`] module"]
#[doc(alias = "IPC02C")]
pub type Ipc02c = crate::Reg<ipc02c::Ipc02cSpec>;
#[doc = "tx ipi0 reg7"]
pub mod ipc02c;
#[doc = "IPC030 (rw) register accessor: tx ipi1 reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc030`] module"]
#[doc(alias = "IPC030")]
pub type Ipc030 = crate::Reg<ipc030::Ipc030Spec>;
#[doc = "tx ipi1 reg0"]
pub mod ipc030;
#[doc = "IPC034 (rw) register accessor: tx ipi1 reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc034`] module"]
#[doc(alias = "IPC034")]
pub type Ipc034 = crate::Reg<ipc034::Ipc034Spec>;
#[doc = "tx ipi1 reg1"]
pub mod ipc034;
#[doc = "IPC038 (rw) register accessor: tx ipi1 reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc038`] module"]
#[doc(alias = "IPC038")]
pub type Ipc038 = crate::Reg<ipc038::Ipc038Spec>;
#[doc = "tx ipi1 reg2"]
pub mod ipc038;
#[doc = "IPC03C (rw) register accessor: tx ipi1 reg3\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc03c`] module"]
#[doc(alias = "IPC03C")]
pub type Ipc03c = crate::Reg<ipc03c::Ipc03cSpec>;
#[doc = "tx ipi1 reg3"]
pub mod ipc03c;
#[doc = "IPC040 (rw) register accessor: tx ipi1 reg4\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc040`] module"]
#[doc(alias = "IPC040")]
pub type Ipc040 = crate::Reg<ipc040::Ipc040Spec>;
#[doc = "tx ipi1 reg4"]
pub mod ipc040;
#[doc = "IPC044 (rw) register accessor: tx ipi1 reg5\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc044`] module"]
#[doc(alias = "IPC044")]
pub type Ipc044 = crate::Reg<ipc044::Ipc044Spec>;
#[doc = "tx ipi1 reg5"]
pub mod ipc044;
#[doc = "IPC048 (rw) register accessor: tx ipi1 reg6\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc048`] module"]
#[doc(alias = "IPC048")]
pub type Ipc048 = crate::Reg<ipc048::Ipc048Spec>;
#[doc = "tx ipi1 reg6"]
pub mod ipc048;
#[doc = "IPC04C (rw) register accessor: tx ipi1 reg7\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc04c`] module"]
#[doc(alias = "IPC04C")]
pub type Ipc04c = crate::Reg<ipc04c::Ipc04cSpec>;
#[doc = "tx ipi1 reg7"]
pub mod ipc04c;
#[doc = "IPC050 (rw) register accessor: tx ipi2 reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc050`] module"]
#[doc(alias = "IPC050")]
pub type Ipc050 = crate::Reg<ipc050::Ipc050Spec>;
#[doc = "tx ipi2 reg0"]
pub mod ipc050;
#[doc = "IPC054 (rw) register accessor: tx ipi2 reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc054`] module"]
#[doc(alias = "IPC054")]
pub type Ipc054 = crate::Reg<ipc054::Ipc054Spec>;
#[doc = "tx ipi2 reg1"]
pub mod ipc054;
#[doc = "IPC058 (rw) register accessor: tx ipi2 reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc058`] module"]
#[doc(alias = "IPC058")]
pub type Ipc058 = crate::Reg<ipc058::Ipc058Spec>;
#[doc = "tx ipi2 reg2"]
pub mod ipc058;
#[doc = "IPC05C (rw) register accessor: tx ipi2 reg3\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc05c`] module"]
#[doc(alias = "IPC05C")]
pub type Ipc05c = crate::Reg<ipc05c::Ipc05cSpec>;
#[doc = "tx ipi2 reg3"]
pub mod ipc05c;
#[doc = "IPC060 (rw) register accessor: tx ipi2 reg4\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc060`] module"]
#[doc(alias = "IPC060")]
pub type Ipc060 = crate::Reg<ipc060::Ipc060Spec>;
#[doc = "tx ipi2 reg4"]
pub mod ipc060;
#[doc = "IPC064 (rw) register accessor: tx ipi2 reg5\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc064`] module"]
#[doc(alias = "IPC064")]
pub type Ipc064 = crate::Reg<ipc064::Ipc064Spec>;
#[doc = "tx ipi2 reg5"]
pub mod ipc064;
#[doc = "IPC068 (rw) register accessor: tx ipi2 reg6\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc068`] module"]
#[doc(alias = "IPC068")]
pub type Ipc068 = crate::Reg<ipc068::Ipc068Spec>;
#[doc = "tx ipi2 reg6"]
pub mod ipc068;
#[doc = "IPC06C (rw) register accessor: tx ipi2 reg7\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc06c`] module"]
#[doc(alias = "IPC06C")]
pub type Ipc06c = crate::Reg<ipc06c::Ipc06cSpec>;
#[doc = "tx ipi2 reg7"]
pub mod ipc06c;
#[doc = "IPC070 (rw) register accessor: tx ipi3 reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc070`] module"]
#[doc(alias = "IPC070")]
pub type Ipc070 = crate::Reg<ipc070::Ipc070Spec>;
#[doc = "tx ipi3 reg0"]
pub mod ipc070;
#[doc = "IPC074 (rw) register accessor: tx ipi3 reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc074`] module"]
#[doc(alias = "IPC074")]
pub type Ipc074 = crate::Reg<ipc074::Ipc074Spec>;
#[doc = "tx ipi3 reg1"]
pub mod ipc074;
#[doc = "IPC078 (rw) register accessor: tx ipi3 reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc078`] module"]
#[doc(alias = "IPC078")]
pub type Ipc078 = crate::Reg<ipc078::Ipc078Spec>;
#[doc = "tx ipi3 reg2"]
pub mod ipc078;
#[doc = "IPC07C (rw) register accessor: tx ipi3 reg3\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc07c`] module"]
#[doc(alias = "IPC07C")]
pub type Ipc07c = crate::Reg<ipc07c::Ipc07cSpec>;
#[doc = "tx ipi3 reg3"]
pub mod ipc07c;
#[doc = "IPC080 (rw) register accessor: tx ipi3 reg4\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc080`] module"]
#[doc(alias = "IPC080")]
pub type Ipc080 = crate::Reg<ipc080::Ipc080Spec>;
#[doc = "tx ipi3 reg4"]
pub mod ipc080;
#[doc = "IPC084 (rw) register accessor: tx ipi3 reg5\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc084`] module"]
#[doc(alias = "IPC084")]
pub type Ipc084 = crate::Reg<ipc084::Ipc084Spec>;
#[doc = "tx ipi3 reg5"]
pub mod ipc084;
#[doc = "IPC088 (rw) register accessor: tx ipi3 reg6\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc088`] module"]
#[doc(alias = "IPC088")]
pub type Ipc088 = crate::Reg<ipc088::Ipc088Spec>;
#[doc = "tx ipi3 reg6"]
pub mod ipc088;
#[doc = "IPC08C (rw) register accessor: tx ipi3 reg7\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipc08c`] module"]
#[doc(alias = "IPC08C")]
pub type Ipc08c = crate::Reg<ipc08c::Ipc08cSpec>;
#[doc = "tx ipi3 reg7"]
pub mod ipc08c;
