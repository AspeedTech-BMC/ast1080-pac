#[doc = "Register `PRIC_IO354` reader"]
pub type R = crate::R<PricIo354Spec>;
#[doc = "Register `PRIC_IO354` writer"]
pub type W = crate::W<PricIo354Spec>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `Reserved9` writer - Reserved"]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EnblReadGroup0OfSysMemroyArbiter` reader - Enable Read Group #0 of System Memroy Arbiter"]
pub type EnblReadGroup0ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSysMemroyArbiter` writer - Enable Read Group #0 of System Memroy Arbiter"]
pub type EnblReadGroup0ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSysMemroyArbiter` reader - Enable Read Group #1 of System Memroy Arbiter"]
pub type EnblReadGroup1ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSysMemroyArbiter` writer - Enable Read Group #1 of System Memroy Arbiter"]
pub type EnblReadGroup1ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSysMemroyArbiter` reader - Enable Read Group #2 of System Memroy Arbiter"]
pub type EnblReadGroup2ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSysMemroyArbiter` writer - Enable Read Group #2 of System Memroy Arbiter"]
pub type EnblReadGroup2ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSysMemroyArbiter` reader - Enable Read Group #3 of System Memroy Arbiter"]
pub type EnblReadGroup3ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSysMemroyArbiter` writer - Enable Read Group #3 of System Memroy Arbiter"]
pub type EnblReadGroup3ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSysMemroyArbiter` reader - Enable Read Group #4 of System Memroy Arbiter"]
pub type EnblReadGroup4ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSysMemroyArbiter` writer - Enable Read Group #4 of System Memroy Arbiter"]
pub type EnblReadGroup4ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSysMemroyArbiter` reader - Enable Read Group #5 of System Memroy Arbiter"]
pub type EnblReadGroup5ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSysMemroyArbiter` writer - Enable Read Group #5 of System Memroy Arbiter"]
pub type EnblReadGroup5ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1354PRIC1_354\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1354pric13542116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1354pric13542116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1354pric13542116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1354PRIC13542116` reader - Enable Reset Tolerance of PRIC1354PRIC1_354\\[21:16\\]"]
pub type EnblRstToleranceOfPric1354pric13542116R =
    crate::BitReader<EnblRstToleranceOfPric1354pric13542116>;
impl EnblRstToleranceOfPric1354pric13542116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1354pric13542116 {
        match self.bits {
            false => EnblRstToleranceOfPric1354pric13542116::ResetBySrst,
            true => EnblRstToleranceOfPric1354pric13542116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1354pric13542116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1354pric13542116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1354PRIC13542116` writer - Enable Reset Tolerance of PRIC1354PRIC1_354\\[21:16\\]"]
pub type EnblRstToleranceOfPric1354pric13542116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1354pric13542116>;
impl<'a, REG> EnblRstToleranceOfPric1354pric13542116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1354pric13542116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1354pric13542116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1354PRIC13542216` reader - Enable Write Protection of PRIC1354PRIC1_354\\[22:16\\]"]
pub type EnblWrProtOfPric1354pric13542216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1354PRIC13542216` writer - Enable Write Protection of PRIC1354PRIC1_354\\[22:16\\]"]
pub type EnblWrProtOfPric1354pric13542216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSysMemoryCtrl` reader - Enable Read Group #0 of System Memory Controller"]
pub type EnblReadGroup0ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSysMemoryCtrl` writer - Enable Read Group #0 of System Memory Controller"]
pub type EnblReadGroup0ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSysMemoryCtrl` reader - Enable Read Group #1 of System Memory Controller"]
pub type EnblReadGroup1ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSysMemoryCtrl` writer - Enable Read Group #1 of System Memory Controller"]
pub type EnblReadGroup1ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSysMemoryCtrl` reader - Enable Read Group #2 of System Memory Controller"]
pub type EnblReadGroup2ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSysMemoryCtrl` writer - Enable Read Group #2 of System Memory Controller"]
pub type EnblReadGroup2ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSysMemoryCtrl` reader - Enable Read Group #3 of System Memory Controller"]
pub type EnblReadGroup3ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSysMemoryCtrl` writer - Enable Read Group #3 of System Memory Controller"]
pub type EnblReadGroup3ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSysMemoryCtrl` reader - Enable Read Group #4 of System Memory Controller"]
pub type EnblReadGroup4ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSysMemoryCtrl` writer - Enable Read Group #4 of System Memory Controller"]
pub type EnblReadGroup4ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSysMemoryCtrl` reader - Enable Read Group #5 of System Memory Controller"]
pub type EnblReadGroup5ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSysMemoryCtrl` writer - Enable Read Group #5 of System Memory Controller"]
pub type EnblReadGroup5ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1354PRIC1_354\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1354pric13542924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1354pric13542924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1354pric13542924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1354PRIC13542924` reader - Enable Reset Tolerance of PRIC1354PRIC1_354\\[29:24\\]"]
pub type EnblRstToleranceOfPric1354pric13542924R =
    crate::BitReader<EnblRstToleranceOfPric1354pric13542924>;
impl EnblRstToleranceOfPric1354pric13542924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1354pric13542924 {
        match self.bits {
            false => EnblRstToleranceOfPric1354pric13542924::ResetBySrst,
            true => EnblRstToleranceOfPric1354pric13542924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1354pric13542924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1354pric13542924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1354PRIC13542924` writer - Enable Reset Tolerance of PRIC1354PRIC1_354\\[29:24\\]"]
pub type EnblRstToleranceOfPric1354pric13542924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1354pric13542924>;
impl<'a, REG> EnblRstToleranceOfPric1354pric13542924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1354pric13542924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1354pric13542924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1354PRIC13543024` reader - Enable Write Protection of PRIC1354PRIC1_354\\[30:24\\]"]
pub type EnblWrProtOfPric1354pric13543024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1354PRIC13543024` writer - Enable Write Protection of PRIC1354PRIC1_354\\[30:24\\]"]
pub type EnblWrProtOfPric1354pric13543024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group0of_sys_memroy_arbiter(&self) -> EnblReadGroup0ofSysMemroyArbiterR {
        EnblReadGroup0ofSysMemroyArbiterR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group1of_sys_memroy_arbiter(&self) -> EnblReadGroup1ofSysMemroyArbiterR {
        EnblReadGroup1ofSysMemroyArbiterR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group2of_sys_memroy_arbiter(&self) -> EnblReadGroup2ofSysMemroyArbiterR {
        EnblReadGroup2ofSysMemroyArbiterR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group3of_sys_memroy_arbiter(&self) -> EnblReadGroup3ofSysMemroyArbiterR {
        EnblReadGroup3ofSysMemroyArbiterR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group4of_sys_memroy_arbiter(&self) -> EnblReadGroup4ofSysMemroyArbiterR {
        EnblReadGroup4ofSysMemroyArbiterR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group5of_sys_memroy_arbiter(&self) -> EnblReadGroup5ofSysMemroyArbiterR {
        EnblReadGroup5ofSysMemroyArbiterR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1354PRIC1_354\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1354pric13542116(
        &self,
    ) -> EnblRstToleranceOfPric1354pric13542116R {
        EnblRstToleranceOfPric1354pric13542116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1354PRIC1_354\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1354pric13542216(&self) -> EnblWrProtOfPric1354pric13542216R {
        EnblWrProtOfPric1354pric13542216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group0of_sys_memory_ctrl(&self) -> EnblReadGroup0ofSysMemoryCtrlR {
        EnblReadGroup0ofSysMemoryCtrlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group1of_sys_memory_ctrl(&self) -> EnblReadGroup1ofSysMemoryCtrlR {
        EnblReadGroup1ofSysMemoryCtrlR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group2of_sys_memory_ctrl(&self) -> EnblReadGroup2ofSysMemoryCtrlR {
        EnblReadGroup2ofSysMemoryCtrlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group3of_sys_memory_ctrl(&self) -> EnblReadGroup3ofSysMemoryCtrlR {
        EnblReadGroup3ofSysMemoryCtrlR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group4of_sys_memory_ctrl(&self) -> EnblReadGroup4ofSysMemoryCtrlR {
        EnblReadGroup4ofSysMemoryCtrlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group5of_sys_memory_ctrl(&self) -> EnblReadGroup5ofSysMemoryCtrlR {
        EnblReadGroup5ofSysMemoryCtrlR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1354PRIC1_354\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1354pric13542924(
        &self,
    ) -> EnblRstToleranceOfPric1354pric13542924R {
        EnblRstToleranceOfPric1354pric13542924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1354PRIC1_354\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1354pric13543024(&self) -> EnblWrProtOfPric1354pric13543024R {
        EnblWrProtOfPric1354pric13543024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo354Spec> {
        Reserved9W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo354Spec> {
        Reserved8W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo354Spec> {
        Reserved7W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo354Spec> {
        Reserved6W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo354Spec> {
        Reserved5W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo354Spec> {
        Reserved4W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo354Spec> {
        Reserved3W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo354Spec> {
        Reserved2W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo354Spec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group0of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblReadGroup0ofSysMemroyArbiterW<PricIo354Spec> {
        EnblReadGroup0ofSysMemroyArbiterW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group1of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblReadGroup1ofSysMemroyArbiterW<PricIo354Spec> {
        EnblReadGroup1ofSysMemroyArbiterW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group2of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblReadGroup2ofSysMemroyArbiterW<PricIo354Spec> {
        EnblReadGroup2ofSysMemroyArbiterW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group3of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblReadGroup3ofSysMemroyArbiterW<PricIo354Spec> {
        EnblReadGroup3ofSysMemroyArbiterW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group4of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblReadGroup4ofSysMemroyArbiterW<PricIo354Spec> {
        EnblReadGroup4ofSysMemroyArbiterW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_read_group5of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblReadGroup5ofSysMemroyArbiterW<PricIo354Spec> {
        EnblReadGroup5ofSysMemroyArbiterW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1354PRIC1_354\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1354pric13542116(
        &mut self,
    ) -> EnblRstToleranceOfPric1354pric13542116W<PricIo354Spec> {
        EnblRstToleranceOfPric1354pric13542116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1354PRIC1_354\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1354pric13542216(
        &mut self,
    ) -> EnblWrProtOfPric1354pric13542216W<PricIo354Spec> {
        EnblWrProtOfPric1354pric13542216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group0of_sys_memory_ctrl(
        &mut self,
    ) -> EnblReadGroup0ofSysMemoryCtrlW<PricIo354Spec> {
        EnblReadGroup0ofSysMemoryCtrlW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group1of_sys_memory_ctrl(
        &mut self,
    ) -> EnblReadGroup1ofSysMemoryCtrlW<PricIo354Spec> {
        EnblReadGroup1ofSysMemoryCtrlW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group2of_sys_memory_ctrl(
        &mut self,
    ) -> EnblReadGroup2ofSysMemoryCtrlW<PricIo354Spec> {
        EnblReadGroup2ofSysMemoryCtrlW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group3of_sys_memory_ctrl(
        &mut self,
    ) -> EnblReadGroup3ofSysMemoryCtrlW<PricIo354Spec> {
        EnblReadGroup3ofSysMemoryCtrlW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group4of_sys_memory_ctrl(
        &mut self,
    ) -> EnblReadGroup4ofSysMemoryCtrlW<PricIo354Spec> {
        EnblReadGroup4ofSysMemoryCtrlW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_read_group5of_sys_memory_ctrl(
        &mut self,
    ) -> EnblReadGroup5ofSysMemoryCtrlW<PricIo354Spec> {
        EnblReadGroup5ofSysMemoryCtrlW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1354PRIC1_354\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1354pric13542924(
        &mut self,
    ) -> EnblRstToleranceOfPric1354pric13542924W<PricIo354Spec> {
        EnblRstToleranceOfPric1354pric13542924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1354PRIC1_354\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1354pric13543024(
        &mut self,
    ) -> EnblWrProtOfPric1354pric13543024W<PricIo354Spec> {
        EnblWrProtOfPric1354pric13543024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io354::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io354::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo354Spec;
impl crate::RegisterSpec for PricIo354Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io354::R`](R) reader structure"]
impl crate::Readable for PricIo354Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io354::W`](W) writer structure"]
impl crate::Writable for PricIo354Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO354 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo354Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
