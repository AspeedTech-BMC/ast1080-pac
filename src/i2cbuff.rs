#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    buff: [Buff; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - I2C Buffer Mode Buff"]
    #[inline(always)]
    pub const fn buff(&self, n: usize) -> &Buff {
        &self.buff[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - I2C Buffer Mode Buff"]
    #[inline(always)]
    pub fn buff_iter(&self) -> impl Iterator<Item = &Buff> {
        self.buff.iter()
    }
}
#[doc = "BUFF (rw) register accessor: I2C Buffer Mode Buff\n\nYou can [`read`](crate::Reg::read) this register and get [`buff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff`] module"]
#[doc(alias = "BUFF")]
pub type Buff = crate::Reg<buff::BuffSpec>;
#[doc = "I2C Buffer Mode Buff"]
pub mod buff;
