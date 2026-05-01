#[doc = "Register `PRIC_IO474` reader"]
pub type R = crate::R<PricIo474Spec>;
#[doc = "Register `PRIC_IO474` writer"]
pub type W = crate::W<PricIo474Spec>;
#[doc = "Field `RegionNWrProt7` reader - Region #N Write Protection"]
pub type RegionNwrProt7R = crate::BitReader;
#[doc = "Field `RegionNWrProt7` writer - Region #N Write Protection"]
pub type RegionNwrProt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance7 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance7> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance7` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance7R = crate::BitReader<EnblRstTolerance7>;
impl EnblRstTolerance7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance7 {
        match self.bits {
            false => EnblRstTolerance7::ResetBySrst,
            true => EnblRstTolerance7::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance7::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance7::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance7` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance7W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance7>;
impl<'a, REG> EnblRstTolerance7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance7::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance7::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup7` reader - Region #N Read Group"]
pub type RegionNreadGroup7R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup7` writer - Region #N Read Group"]
pub type RegionNreadGroup7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr35127` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35127R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr35127` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35127W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot7(&self) -> RegionNwrProt7R {
        RegionNwrProt7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance7(&self) -> EnblRstTolerance7R {
        EnblRstTolerance7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group7(&self) -> RegionNreadGroup7R {
        RegionNreadGroup7R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35127(&self) -> RegionNendAddr35127R {
        RegionNendAddr35127R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot7(&mut self) -> RegionNwrProt7W<PricIo474Spec> {
        RegionNwrProt7W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance7(&mut self) -> EnblRstTolerance7W<PricIo474Spec> {
        EnblRstTolerance7W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group7(&mut self) -> RegionNreadGroup7W<PricIo474Spec> {
        RegionNreadGroup7W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35127(&mut self) -> RegionNendAddr35127W<PricIo474Spec> {
        RegionNendAddr35127W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io474::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io474::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo474Spec;
impl crate::RegisterSpec for PricIo474Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io474::R`](R) reader structure"]
impl crate::Readable for PricIo474Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io474::W`](W) writer structure"]
impl crate::Writable for PricIo474Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO474 to value 0"]
impl crate::Resettable for PricIo474Spec {}
