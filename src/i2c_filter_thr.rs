#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_filter_thr000: I2cFilterThr000,
    i2c_filter_thr004: I2cFilterThr004,
    i2c_filter_thr008: I2cFilterThr008,
    i2c_filter_thr00c: I2cFilterThr00c,
    i2c_filter_thr010: I2cFilterThr010,
    i2c_filter_thr014: I2cFilterThr014,
    i2c_filter_thr018: I2cFilterThr018,
    _reserved7: [u8; 0x04],
    i2c_filter_thr020: I2cFilterThr020,
    i2c_filter_thr024: I2cFilterThr024,
    _reserved9: [u8; 0x18],
    i2c_filter_thr040: I2cFilterThr040,
    i2c_filter_thr044: I2cFilterThr044,
    i2c_filter_thr048: I2cFilterThr048,
    i2c_filter_thr04c: I2cFilterThr04c,
    _reserved13: [u8; 0x10],
    i2c_filter_thr060: I2cFilterThr060,
    _reserved14: [u8; 0x0c],
    i2c_filter_thr070: I2cFilterThr070,
    _reserved15: [u8; 0x08],
    i2c_filter_thr07c: I2cFilterThr07c,
    i2c_filter_thr080: I2cFilterThr080,
    i2c_filter_thr084: I2cFilterThr084,
    i2c_filter_thr088: I2cFilterThr088,
    i2c_filter_thr08c: I2cFilterThr08c,
    i2c_filter_thr090: I2cFilterThr090,
    i2c_filter_thr094: I2cFilterThr094,
    i2c_filter_thr098: I2cFilterThr098,
    i2c_filter_thr09c: I2cFilterThr09c,
}
impl RegisterBlock {
    #[doc = "0x00 - I2CFLT\\_THR0\\_RST"]
    #[inline(always)]
    pub const fn i2c_filter_thr000(&self) -> &I2cFilterThr000 {
        &self.i2c_filter_thr000
    }
    #[doc = "0x04 - I2CFLT\\_THR0\\_EN"]
    #[inline(always)]
    pub const fn i2c_filter_thr004(&self) -> &I2cFilterThr004 {
        &self.i2c_filter_thr004
    }
    #[doc = "0x08 - I2CFLT\\_THR0\\_ADR"]
    #[inline(always)]
    pub const fn i2c_filter_thr008(&self) -> &I2cFilterThr008 {
        &self.i2c_filter_thr008
    }
    #[doc = "0x0c - I2CFLT\\_THR0\\_CFG"]
    #[inline(always)]
    pub const fn i2c_filter_thr00c(&self) -> &I2cFilterThr00c {
        &self.i2c_filter_thr00c
    }
    #[doc = "0x10 - I2CFLT\\_THR0\\_TMR"]
    #[inline(always)]
    pub const fn i2c_filter_thr010(&self) -> &I2cFilterThr010 {
        &self.i2c_filter_thr010
    }
    #[doc = "0x14 - I2CFLT\\_THR0\\_INTEN"]
    #[inline(always)]
    pub const fn i2c_filter_thr014(&self) -> &I2cFilterThr014 {
        &self.i2c_filter_thr014
    }
    #[doc = "0x18 - I2CFLT\\_THR0\\_INTS"]
    #[inline(always)]
    pub const fn i2c_filter_thr018(&self) -> &I2cFilterThr018 {
        &self.i2c_filter_thr018
    }
    #[doc = "0x20 - I2CFLT\\_THR0\\_STATUS"]
    #[inline(always)]
    pub const fn i2c_filter_thr020(&self) -> &I2cFilterThr020 {
        &self.i2c_filter_thr020
    }
    #[doc = "0x24 - I2CFLT\\_THR0\\_SEQ"]
    #[inline(always)]
    pub const fn i2c_filter_thr024(&self) -> &I2cFilterThr024 {
        &self.i2c_filter_thr024
    }
    #[doc = "0x40 - I2CFLT\\_THR0\\_MAP0"]
    #[inline(always)]
    pub const fn i2c_filter_thr040(&self) -> &I2cFilterThr040 {
        &self.i2c_filter_thr040
    }
    #[doc = "0x44 - I2CFLT\\_THR0\\_MAP1"]
    #[inline(always)]
    pub const fn i2c_filter_thr044(&self) -> &I2cFilterThr044 {
        &self.i2c_filter_thr044
    }
    #[doc = "0x48 - I2CFLT\\_THR0\\_MAP2"]
    #[inline(always)]
    pub const fn i2c_filter_thr048(&self) -> &I2cFilterThr048 {
        &self.i2c_filter_thr048
    }
    #[doc = "0x4c - I2CFLT\\_THR0\\_MAP3"]
    #[inline(always)]
    pub const fn i2c_filter_thr04c(&self) -> &I2cFilterThr04c {
        &self.i2c_filter_thr04c
    }
    #[doc = "0x60 - I2CFLT\\_THR0\\_INFO"]
    #[inline(always)]
    pub const fn i2c_filter_thr060(&self) -> &I2cFilterThr060 {
        &self.i2c_filter_thr060
    }
    #[doc = "0x70 - I2CFLT\\_THR0\\_ADR\\_HI"]
    #[inline(always)]
    pub const fn i2c_filter_thr070(&self) -> &I2cFilterThr070 {
        &self.i2c_filter_thr070
    }
    #[doc = "0x7c - I2CF\\_ELOG\\_CNT"]
    #[inline(always)]
    pub const fn i2c_filter_thr07c(&self) -> &I2cFilterThr07c {
        &self.i2c_filter_thr07c
    }
    #[doc = "0x80 - I2CF\\_ELOG00"]
    #[inline(always)]
    pub const fn i2c_filter_thr080(&self) -> &I2cFilterThr080 {
        &self.i2c_filter_thr080
    }
    #[doc = "0x84 - I2CF\\_ELOG01"]
    #[inline(always)]
    pub const fn i2c_filter_thr084(&self) -> &I2cFilterThr084 {
        &self.i2c_filter_thr084
    }
    #[doc = "0x88 - I2CF\\_ELOG02"]
    #[inline(always)]
    pub const fn i2c_filter_thr088(&self) -> &I2cFilterThr088 {
        &self.i2c_filter_thr088
    }
    #[doc = "0x8c - I2CF\\_ELOG03"]
    #[inline(always)]
    pub const fn i2c_filter_thr08c(&self) -> &I2cFilterThr08c {
        &self.i2c_filter_thr08c
    }
    #[doc = "0x90 - I2CF\\_ELOG04"]
    #[inline(always)]
    pub const fn i2c_filter_thr090(&self) -> &I2cFilterThr090 {
        &self.i2c_filter_thr090
    }
    #[doc = "0x94 - I2CF\\_ELOG05"]
    #[inline(always)]
    pub const fn i2c_filter_thr094(&self) -> &I2cFilterThr094 {
        &self.i2c_filter_thr094
    }
    #[doc = "0x98 - I2CF\\_ELOG06"]
    #[inline(always)]
    pub const fn i2c_filter_thr098(&self) -> &I2cFilterThr098 {
        &self.i2c_filter_thr098
    }
    #[doc = "0x9c - I2CF\\_ELOG07"]
    #[inline(always)]
    pub const fn i2c_filter_thr09c(&self) -> &I2cFilterThr09c {
        &self.i2c_filter_thr09c
    }
}
#[doc = "I2C_FILTER_THR000 (rw) register accessor: I2CFLT\\_THR0\\_RST\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr000`] module"]
#[doc(alias = "I2C_FILTER_THR000")]
pub type I2cFilterThr000 = crate::Reg<i2c_filter_thr000::I2cFilterThr000Spec>;
#[doc = "I2CFLT\\_THR0\\_RST"]
pub mod i2c_filter_thr000;
#[doc = "I2C_FILTER_THR004 (rw) register accessor: I2CFLT\\_THR0\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr004`] module"]
#[doc(alias = "I2C_FILTER_THR004")]
pub type I2cFilterThr004 = crate::Reg<i2c_filter_thr004::I2cFilterThr004Spec>;
#[doc = "I2CFLT\\_THR0\\_EN"]
pub mod i2c_filter_thr004;
#[doc = "I2C_FILTER_THR008 (rw) register accessor: I2CFLT\\_THR0\\_ADR\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr008`] module"]
#[doc(alias = "I2C_FILTER_THR008")]
pub type I2cFilterThr008 = crate::Reg<i2c_filter_thr008::I2cFilterThr008Spec>;
#[doc = "I2CFLT\\_THR0\\_ADR"]
pub mod i2c_filter_thr008;
#[doc = "I2C_FILTER_THR00C (rw) register accessor: I2CFLT\\_THR0\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr00c`] module"]
#[doc(alias = "I2C_FILTER_THR00C")]
pub type I2cFilterThr00c = crate::Reg<i2c_filter_thr00c::I2cFilterThr00cSpec>;
#[doc = "I2CFLT\\_THR0\\_CFG"]
pub mod i2c_filter_thr00c;
#[doc = "I2C_FILTER_THR010 (rw) register accessor: I2CFLT\\_THR0\\_TMR\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr010`] module"]
#[doc(alias = "I2C_FILTER_THR010")]
pub type I2cFilterThr010 = crate::Reg<i2c_filter_thr010::I2cFilterThr010Spec>;
#[doc = "I2CFLT\\_THR0\\_TMR"]
pub mod i2c_filter_thr010;
#[doc = "I2C_FILTER_THR014 (rw) register accessor: I2CFLT\\_THR0\\_INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr014`] module"]
#[doc(alias = "I2C_FILTER_THR014")]
pub type I2cFilterThr014 = crate::Reg<i2c_filter_thr014::I2cFilterThr014Spec>;
#[doc = "I2CFLT\\_THR0\\_INTEN"]
pub mod i2c_filter_thr014;
#[doc = "I2C_FILTER_THR018 (rw) register accessor: I2CFLT\\_THR0\\_INTS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr018`] module"]
#[doc(alias = "I2C_FILTER_THR018")]
pub type I2cFilterThr018 = crate::Reg<i2c_filter_thr018::I2cFilterThr018Spec>;
#[doc = "I2CFLT\\_THR0\\_INTS"]
pub mod i2c_filter_thr018;
#[doc = "I2C_FILTER_THR020 (rw) register accessor: I2CFLT\\_THR0\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr020`] module"]
#[doc(alias = "I2C_FILTER_THR020")]
pub type I2cFilterThr020 = crate::Reg<i2c_filter_thr020::I2cFilterThr020Spec>;
#[doc = "I2CFLT\\_THR0\\_STATUS"]
pub mod i2c_filter_thr020;
#[doc = "I2C_FILTER_THR024 (rw) register accessor: I2CFLT\\_THR0\\_SEQ\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr024`] module"]
#[doc(alias = "I2C_FILTER_THR024")]
pub type I2cFilterThr024 = crate::Reg<i2c_filter_thr024::I2cFilterThr024Spec>;
#[doc = "I2CFLT\\_THR0\\_SEQ"]
pub mod i2c_filter_thr024;
#[doc = "I2C_FILTER_THR040 (rw) register accessor: I2CFLT\\_THR0\\_MAP0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr040`] module"]
#[doc(alias = "I2C_FILTER_THR040")]
pub type I2cFilterThr040 = crate::Reg<i2c_filter_thr040::I2cFilterThr040Spec>;
#[doc = "I2CFLT\\_THR0\\_MAP0"]
pub mod i2c_filter_thr040;
#[doc = "I2C_FILTER_THR044 (rw) register accessor: I2CFLT\\_THR0\\_MAP1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr044`] module"]
#[doc(alias = "I2C_FILTER_THR044")]
pub type I2cFilterThr044 = crate::Reg<i2c_filter_thr044::I2cFilterThr044Spec>;
#[doc = "I2CFLT\\_THR0\\_MAP1"]
pub mod i2c_filter_thr044;
#[doc = "I2C_FILTER_THR048 (rw) register accessor: I2CFLT\\_THR0\\_MAP2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr048`] module"]
#[doc(alias = "I2C_FILTER_THR048")]
pub type I2cFilterThr048 = crate::Reg<i2c_filter_thr048::I2cFilterThr048Spec>;
#[doc = "I2CFLT\\_THR0\\_MAP2"]
pub mod i2c_filter_thr048;
#[doc = "I2C_FILTER_THR04C (rw) register accessor: I2CFLT\\_THR0\\_MAP3\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr04c`] module"]
#[doc(alias = "I2C_FILTER_THR04C")]
pub type I2cFilterThr04c = crate::Reg<i2c_filter_thr04c::I2cFilterThr04cSpec>;
#[doc = "I2CFLT\\_THR0\\_MAP3"]
pub mod i2c_filter_thr04c;
#[doc = "I2C_FILTER_THR060 (rw) register accessor: I2CFLT\\_THR0\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr060`] module"]
#[doc(alias = "I2C_FILTER_THR060")]
pub type I2cFilterThr060 = crate::Reg<i2c_filter_thr060::I2cFilterThr060Spec>;
#[doc = "I2CFLT\\_THR0\\_INFO"]
pub mod i2c_filter_thr060;
#[doc = "I2C_FILTER_THR070 (rw) register accessor: I2CFLT\\_THR0\\_ADR\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr070`] module"]
#[doc(alias = "I2C_FILTER_THR070")]
pub type I2cFilterThr070 = crate::Reg<i2c_filter_thr070::I2cFilterThr070Spec>;
#[doc = "I2CFLT\\_THR0\\_ADR\\_HI"]
pub mod i2c_filter_thr070;
#[doc = "I2C_FILTER_THR07C (rw) register accessor: I2CF\\_ELOG\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr07c`] module"]
#[doc(alias = "I2C_FILTER_THR07C")]
pub type I2cFilterThr07c = crate::Reg<i2c_filter_thr07c::I2cFilterThr07cSpec>;
#[doc = "I2CF\\_ELOG\\_CNT"]
pub mod i2c_filter_thr07c;
#[doc = "I2C_FILTER_THR080 (rw) register accessor: I2CF\\_ELOG00\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr080`] module"]
#[doc(alias = "I2C_FILTER_THR080")]
pub type I2cFilterThr080 = crate::Reg<i2c_filter_thr080::I2cFilterThr080Spec>;
#[doc = "I2CF\\_ELOG00"]
pub mod i2c_filter_thr080;
#[doc = "I2C_FILTER_THR084 (rw) register accessor: I2CF\\_ELOG01\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr084`] module"]
#[doc(alias = "I2C_FILTER_THR084")]
pub type I2cFilterThr084 = crate::Reg<i2c_filter_thr084::I2cFilterThr084Spec>;
#[doc = "I2CF\\_ELOG01"]
pub mod i2c_filter_thr084;
#[doc = "I2C_FILTER_THR088 (rw) register accessor: I2CF\\_ELOG02\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr088`] module"]
#[doc(alias = "I2C_FILTER_THR088")]
pub type I2cFilterThr088 = crate::Reg<i2c_filter_thr088::I2cFilterThr088Spec>;
#[doc = "I2CF\\_ELOG02"]
pub mod i2c_filter_thr088;
#[doc = "I2C_FILTER_THR08C (rw) register accessor: I2CF\\_ELOG03\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr08c`] module"]
#[doc(alias = "I2C_FILTER_THR08C")]
pub type I2cFilterThr08c = crate::Reg<i2c_filter_thr08c::I2cFilterThr08cSpec>;
#[doc = "I2CF\\_ELOG03"]
pub mod i2c_filter_thr08c;
#[doc = "I2C_FILTER_THR090 (rw) register accessor: I2CF\\_ELOG04\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr090`] module"]
#[doc(alias = "I2C_FILTER_THR090")]
pub type I2cFilterThr090 = crate::Reg<i2c_filter_thr090::I2cFilterThr090Spec>;
#[doc = "I2CF\\_ELOG04"]
pub mod i2c_filter_thr090;
#[doc = "I2C_FILTER_THR094 (rw) register accessor: I2CF\\_ELOG05\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr094`] module"]
#[doc(alias = "I2C_FILTER_THR094")]
pub type I2cFilterThr094 = crate::Reg<i2c_filter_thr094::I2cFilterThr094Spec>;
#[doc = "I2CF\\_ELOG05"]
pub mod i2c_filter_thr094;
#[doc = "I2C_FILTER_THR098 (rw) register accessor: I2CF\\_ELOG06\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr098`] module"]
#[doc(alias = "I2C_FILTER_THR098")]
pub type I2cFilterThr098 = crate::Reg<i2c_filter_thr098::I2cFilterThr098Spec>;
#[doc = "I2CF\\_ELOG06"]
pub mod i2c_filter_thr098;
#[doc = "I2C_FILTER_THR09C (rw) register accessor: I2CF\\_ELOG07\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter_thr09c`] module"]
#[doc(alias = "I2C_FILTER_THR09C")]
pub type I2cFilterThr09c = crate::Reg<i2c_filter_thr09c::I2cFilterThr09cSpec>;
#[doc = "I2CF\\_ELOG07"]
pub mod i2c_filter_thr09c;
