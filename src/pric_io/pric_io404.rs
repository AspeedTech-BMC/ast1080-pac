#[doc = "Register `PRIC_IO404` reader"]
pub type R = crate::R<PricIo404Spec>;
#[doc = "Register `PRIC_IO404` writer"]
pub type W = crate::W<PricIo404Spec>;
#[doc = "Field `RegionNWrProt` reader - Region #N Write Protection"]
pub type RegionNwrProtR = crate::BitReader;
#[doc = "Field `RegionNWrProt` writer - Region #N Write Protection"]
pub type RegionNwrProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance` reader - Enable Reset Tolerance"]
pub type EnblRstToleranceR = crate::BitReader<EnblRstTolerance>;
impl EnblRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance {
        match self.bits {
            false => EnblRstTolerance::ResetBySrst,
            true => EnblRstTolerance::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance` writer - Enable Reset Tolerance"]
pub type EnblRstToleranceW<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance>;
impl<'a, REG> EnblRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup` reader - Region #N Read Group"]
pub type RegionNreadGroupR = crate::FieldReader;
#[doc = "Field `RegionNReadGroup` writer - Region #N Read Group"]
pub type RegionNreadGroupW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr3512` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr3512R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr3512` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr3512W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot(&self) -> RegionNwrProtR {
        RegionNwrProtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance(&self) -> EnblRstToleranceR {
        EnblRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group(&self) -> RegionNreadGroupR {
        RegionNreadGroupR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr3512(&self) -> RegionNendAddr3512R {
        RegionNendAddr3512R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot(&mut self) -> RegionNwrProtW<PricIo404Spec> {
        RegionNwrProtW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance(&mut self) -> EnblRstToleranceW<PricIo404Spec> {
        EnblRstToleranceW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group(&mut self) -> RegionNreadGroupW<PricIo404Spec> {
        RegionNreadGroupW::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr3512(&mut self) -> RegionNendAddr3512W<PricIo404Spec> {
        RegionNendAddr3512W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io404::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io404::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo404Spec;
impl crate::RegisterSpec for PricIo404Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io404::R`](R) reader structure"]
impl crate::Readable for PricIo404Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io404::W`](W) writer structure"]
impl crate::Writable for PricIo404Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO404 to value 0"]
impl crate::Resettable for PricIo404Spec {}
