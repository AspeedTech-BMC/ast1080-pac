#[doc = "Register `PRIC_IO254` reader"]
pub type R = crate::R<PricIo254Spec>;
#[doc = "Register `PRIC_IO254` writer"]
pub type W = crate::W<PricIo254Spec>;
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
#[doc = "Field `EnblWrGroup0OfSysMemroyArbiter` reader - Enable Write Group #0 of System Memroy Arbiter"]
pub type EnblWrGroup0ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSysMemroyArbiter` writer - Enable Write Group #0 of System Memroy Arbiter"]
pub type EnblWrGroup0ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSysMemroyArbiter` reader - Enable Write Group #1 of System Memroy Arbiter"]
pub type EnblWrGroup1ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSysMemroyArbiter` writer - Enable Write Group #1 of System Memroy Arbiter"]
pub type EnblWrGroup1ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSysMemroyArbiter` reader - Enable Write Group #2 of System Memroy Arbiter"]
pub type EnblWrGroup2ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSysMemroyArbiter` writer - Enable Write Group #2 of System Memroy Arbiter"]
pub type EnblWrGroup2ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSysMemroyArbiter` reader - Enable Write Group #3 of System Memroy Arbiter"]
pub type EnblWrGroup3ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSysMemroyArbiter` writer - Enable Write Group #3 of System Memroy Arbiter"]
pub type EnblWrGroup3ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSysMemroyArbiter` reader - Enable Write Group #4 of System Memroy Arbiter"]
pub type EnblWrGroup4ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSysMemroyArbiter` writer - Enable Write Group #4 of System Memroy Arbiter"]
pub type EnblWrGroup4ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSysMemroyArbiter` reader - Enable Write Group #5 of System Memroy Arbiter"]
pub type EnblWrGroup5ofSysMemroyArbiterR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSysMemroyArbiter` writer - Enable Write Group #5 of System Memroy Arbiter"]
pub type EnblWrGroup5ofSysMemroyArbiterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1254PRIC1_254\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1254pric12542116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1254pric12542116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1254pric12542116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1254PRIC12542116` reader - Enable Reset Tolerance of PRIC1254PRIC1_254\\[21:16\\]"]
pub type EnblRstToleranceOfPric1254pric12542116R =
    crate::BitReader<EnblRstToleranceOfPric1254pric12542116>;
impl EnblRstToleranceOfPric1254pric12542116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1254pric12542116 {
        match self.bits {
            false => EnblRstToleranceOfPric1254pric12542116::ResetBySrst,
            true => EnblRstToleranceOfPric1254pric12542116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1254pric12542116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1254pric12542116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1254PRIC12542116` writer - Enable Reset Tolerance of PRIC1254PRIC1_254\\[21:16\\]"]
pub type EnblRstToleranceOfPric1254pric12542116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1254pric12542116>;
impl<'a, REG> EnblRstToleranceOfPric1254pric12542116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1254pric12542116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1254pric12542116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1254PRIC12542216` reader - Enable Write Protection of PRIC1254PRIC1_254\\[22:16\\]"]
pub type EnblWrProtOfPric1254pric12542216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1254PRIC12542216` writer - Enable Write Protection of PRIC1254PRIC1_254\\[22:16\\]"]
pub type EnblWrProtOfPric1254pric12542216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSysMemoryCtrl` reader - Enable Write Group #0 of System Memory Controller"]
pub type EnblWrGroup0ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSysMemoryCtrl` writer - Enable Write Group #0 of System Memory Controller"]
pub type EnblWrGroup0ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSysMemoryCtrl` reader - Enable Write Group #1 of System Memory Controller"]
pub type EnblWrGroup1ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSysMemoryCtrl` writer - Enable Write Group #1 of System Memory Controller"]
pub type EnblWrGroup1ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSysMemoryCtrl` reader - Enable Write Group #2 of System Memory Controller"]
pub type EnblWrGroup2ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSysMemoryCtrl` writer - Enable Write Group #2 of System Memory Controller"]
pub type EnblWrGroup2ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSysMemoryCtrl` reader - Enable Write Group #3 of System Memory Controller"]
pub type EnblWrGroup3ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSysMemoryCtrl` writer - Enable Write Group #3 of System Memory Controller"]
pub type EnblWrGroup3ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSysMemoryCtrl` reader - Enable Write Group #4 of System Memory Controller"]
pub type EnblWrGroup4ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSysMemoryCtrl` writer - Enable Write Group #4 of System Memory Controller"]
pub type EnblWrGroup4ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSysMemoryCtrl` reader - Enable Write Group #5 of System Memory Controller"]
pub type EnblWrGroup5ofSysMemoryCtrlR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSysMemoryCtrl` writer - Enable Write Group #5 of System Memory Controller"]
pub type EnblWrGroup5ofSysMemoryCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1254PRIC1_254\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1254pric12542924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1254pric12542924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1254pric12542924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1254PRIC12542924` reader - Enable Reset Tolerance of PRIC1254PRIC1_254\\[29:24\\]"]
pub type EnblRstToleranceOfPric1254pric12542924R =
    crate::BitReader<EnblRstToleranceOfPric1254pric12542924>;
impl EnblRstToleranceOfPric1254pric12542924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1254pric12542924 {
        match self.bits {
            false => EnblRstToleranceOfPric1254pric12542924::ResetBySrst,
            true => EnblRstToleranceOfPric1254pric12542924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1254pric12542924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1254pric12542924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1254PRIC12542924` writer - Enable Reset Tolerance of PRIC1254PRIC1_254\\[29:24\\]"]
pub type EnblRstToleranceOfPric1254pric12542924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1254pric12542924>;
impl<'a, REG> EnblRstToleranceOfPric1254pric12542924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1254pric12542924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1254pric12542924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1254PRIC12543024` reader - Enable Write Protection of PRIC1254PRIC1_254\\[30:24\\]"]
pub type EnblWrProtOfPric1254pric12543024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1254PRIC12543024` writer - Enable Write Protection of PRIC1254PRIC1_254\\[30:24\\]"]
pub type EnblWrProtOfPric1254pric12543024W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16 - Enable Write Group #0 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sys_memroy_arbiter(&self) -> EnblWrGroup0ofSysMemroyArbiterR {
        EnblWrGroup0ofSysMemroyArbiterR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sys_memroy_arbiter(&self) -> EnblWrGroup1ofSysMemroyArbiterR {
        EnblWrGroup1ofSysMemroyArbiterR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sys_memroy_arbiter(&self) -> EnblWrGroup2ofSysMemroyArbiterR {
        EnblWrGroup2ofSysMemroyArbiterR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sys_memroy_arbiter(&self) -> EnblWrGroup3ofSysMemroyArbiterR {
        EnblWrGroup3ofSysMemroyArbiterR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sys_memroy_arbiter(&self) -> EnblWrGroup4ofSysMemroyArbiterR {
        EnblWrGroup4ofSysMemroyArbiterR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sys_memroy_arbiter(&self) -> EnblWrGroup5ofSysMemroyArbiterR {
        EnblWrGroup5ofSysMemroyArbiterR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1254PRIC1_254\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1254pric12542116(
        &self,
    ) -> EnblRstToleranceOfPric1254pric12542116R {
        EnblRstToleranceOfPric1254pric12542116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1254PRIC1_254\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1254pric12542216(&self) -> EnblWrProtOfPric1254pric12542216R {
        EnblWrProtOfPric1254pric12542216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sys_memory_ctrl(&self) -> EnblWrGroup0ofSysMemoryCtrlR {
        EnblWrGroup0ofSysMemoryCtrlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sys_memory_ctrl(&self) -> EnblWrGroup1ofSysMemoryCtrlR {
        EnblWrGroup1ofSysMemoryCtrlR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sys_memory_ctrl(&self) -> EnblWrGroup2ofSysMemoryCtrlR {
        EnblWrGroup2ofSysMemoryCtrlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sys_memory_ctrl(&self) -> EnblWrGroup3ofSysMemoryCtrlR {
        EnblWrGroup3ofSysMemoryCtrlR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sys_memory_ctrl(&self) -> EnblWrGroup4ofSysMemoryCtrlR {
        EnblWrGroup4ofSysMemoryCtrlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sys_memory_ctrl(&self) -> EnblWrGroup5ofSysMemoryCtrlR {
        EnblWrGroup5ofSysMemoryCtrlR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1254PRIC1_254\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1254pric12542924(
        &self,
    ) -> EnblRstToleranceOfPric1254pric12542924R {
        EnblRstToleranceOfPric1254pric12542924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1254PRIC1_254\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1254pric12543024(&self) -> EnblWrProtOfPric1254pric12543024R {
        EnblWrProtOfPric1254pric12543024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo254Spec> {
        Reserved9W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo254Spec> {
        Reserved8W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo254Spec> {
        Reserved7W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo254Spec> {
        Reserved6W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo254Spec> {
        Reserved5W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo254Spec> {
        Reserved4W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo254Spec> {
        Reserved3W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo254Spec> {
        Reserved2W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo254Spec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblWrGroup0ofSysMemroyArbiterW<PricIo254Spec> {
        EnblWrGroup0ofSysMemroyArbiterW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblWrGroup1ofSysMemroyArbiterW<PricIo254Spec> {
        EnblWrGroup1ofSysMemroyArbiterW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblWrGroup2ofSysMemroyArbiterW<PricIo254Spec> {
        EnblWrGroup2ofSysMemroyArbiterW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblWrGroup3ofSysMemroyArbiterW<PricIo254Spec> {
        EnblWrGroup3ofSysMemroyArbiterW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblWrGroup4ofSysMemroyArbiterW<PricIo254Spec> {
        EnblWrGroup4ofSysMemroyArbiterW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of System Memroy Arbiter"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sys_memroy_arbiter(
        &mut self,
    ) -> EnblWrGroup5ofSysMemroyArbiterW<PricIo254Spec> {
        EnblWrGroup5ofSysMemroyArbiterW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1254PRIC1_254\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1254pric12542116(
        &mut self,
    ) -> EnblRstToleranceOfPric1254pric12542116W<PricIo254Spec> {
        EnblRstToleranceOfPric1254pric12542116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1254PRIC1_254\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1254pric12542216(
        &mut self,
    ) -> EnblWrProtOfPric1254pric12542216W<PricIo254Spec> {
        EnblWrProtOfPric1254pric12542216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group0of_sys_memory_ctrl(
        &mut self,
    ) -> EnblWrGroup0ofSysMemoryCtrlW<PricIo254Spec> {
        EnblWrGroup0ofSysMemoryCtrlW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group1of_sys_memory_ctrl(
        &mut self,
    ) -> EnblWrGroup1ofSysMemoryCtrlW<PricIo254Spec> {
        EnblWrGroup1ofSysMemoryCtrlW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group2of_sys_memory_ctrl(
        &mut self,
    ) -> EnblWrGroup2ofSysMemoryCtrlW<PricIo254Spec> {
        EnblWrGroup2ofSysMemoryCtrlW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group3of_sys_memory_ctrl(
        &mut self,
    ) -> EnblWrGroup3ofSysMemoryCtrlW<PricIo254Spec> {
        EnblWrGroup3ofSysMemoryCtrlW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group4of_sys_memory_ctrl(
        &mut self,
    ) -> EnblWrGroup4ofSysMemoryCtrlW<PricIo254Spec> {
        EnblWrGroup4ofSysMemoryCtrlW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of System Memory Controller"]
    #[inline(always)]
    pub fn enbl_wr_group5of_sys_memory_ctrl(
        &mut self,
    ) -> EnblWrGroup5ofSysMemoryCtrlW<PricIo254Spec> {
        EnblWrGroup5ofSysMemoryCtrlW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1254PRIC1_254\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1254pric12542924(
        &mut self,
    ) -> EnblRstToleranceOfPric1254pric12542924W<PricIo254Spec> {
        EnblRstToleranceOfPric1254pric12542924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1254PRIC1_254\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1254pric12543024(
        &mut self,
    ) -> EnblWrProtOfPric1254pric12543024W<PricIo254Spec> {
        EnblWrProtOfPric1254pric12543024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io254::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io254::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo254Spec;
impl crate::RegisterSpec for PricIo254Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io254::R`](R) reader structure"]
impl crate::Readable for PricIo254Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io254::W`](W) writer structure"]
impl crate::Writable for PricIo254Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO254 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo254Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
