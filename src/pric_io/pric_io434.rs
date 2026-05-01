#[doc = "Register `PRIC_IO434` reader"]
pub type R = crate::R<PricIo434Spec>;
#[doc = "Register `PRIC_IO434` writer"]
pub type W = crate::W<PricIo434Spec>;
#[doc = "Field `RegionNWrProt3` reader - Region #N Write Protection"]
pub type RegionNwrProt3R = crate::BitReader;
#[doc = "Field `RegionNWrProt3` writer - Region #N Write Protection"]
pub type RegionNwrProt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance3 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance3> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance3` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance3R = crate::BitReader<EnblRstTolerance3>;
impl EnblRstTolerance3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance3 {
        match self.bits {
            false => EnblRstTolerance3::ResetBySrst,
            true => EnblRstTolerance3::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance3::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance3::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance3` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance3W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance3>;
impl<'a, REG> EnblRstTolerance3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance3::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance3::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup3` reader - Region #N Read Group"]
pub type RegionNreadGroup3R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup3` writer - Region #N Read Group"]
pub type RegionNreadGroup3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr35123` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35123R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr35123` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35123W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot3(&self) -> RegionNwrProt3R {
        RegionNwrProt3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance3(&self) -> EnblRstTolerance3R {
        EnblRstTolerance3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group3(&self) -> RegionNreadGroup3R {
        RegionNreadGroup3R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35123(&self) -> RegionNendAddr35123R {
        RegionNendAddr35123R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot3(&mut self) -> RegionNwrProt3W<PricIo434Spec> {
        RegionNwrProt3W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance3(&mut self) -> EnblRstTolerance3W<PricIo434Spec> {
        EnblRstTolerance3W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group3(&mut self) -> RegionNreadGroup3W<PricIo434Spec> {
        RegionNreadGroup3W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35123(&mut self) -> RegionNendAddr35123W<PricIo434Spec> {
        RegionNendAddr35123W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io434::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io434::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo434Spec;
impl crate::RegisterSpec for PricIo434Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io434::R`](R) reader structure"]
impl crate::Readable for PricIo434Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io434::W`](W) writer structure"]
impl crate::Writable for PricIo434Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO434 to value 0"]
impl crate::Resettable for PricIo434Spec {}
