#[doc = "Register `PRIC_IO444` reader"]
pub type R = crate::R<PricIo444Spec>;
#[doc = "Register `PRIC_IO444` writer"]
pub type W = crate::W<PricIo444Spec>;
#[doc = "Field `RegionNWrProt4` reader - Region #N Write Protection"]
pub type RegionNwrProt4R = crate::BitReader;
#[doc = "Field `RegionNWrProt4` writer - Region #N Write Protection"]
pub type RegionNwrProt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance4 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance4> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance4` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance4R = crate::BitReader<EnblRstTolerance4>;
impl EnblRstTolerance4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance4 {
        match self.bits {
            false => EnblRstTolerance4::ResetBySrst,
            true => EnblRstTolerance4::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance4::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance4::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance4` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance4W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance4>;
impl<'a, REG> EnblRstTolerance4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance4::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance4::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup4` reader - Region #N Read Group"]
pub type RegionNreadGroup4R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup4` writer - Region #N Read Group"]
pub type RegionNreadGroup4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr35124` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35124R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr35124` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35124W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot4(&self) -> RegionNwrProt4R {
        RegionNwrProt4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance4(&self) -> EnblRstTolerance4R {
        EnblRstTolerance4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group4(&self) -> RegionNreadGroup4R {
        RegionNreadGroup4R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35124(&self) -> RegionNendAddr35124R {
        RegionNendAddr35124R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot4(&mut self) -> RegionNwrProt4W<PricIo444Spec> {
        RegionNwrProt4W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance4(&mut self) -> EnblRstTolerance4W<PricIo444Spec> {
        EnblRstTolerance4W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group4(&mut self) -> RegionNreadGroup4W<PricIo444Spec> {
        RegionNreadGroup4W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35124(&mut self) -> RegionNendAddr35124W<PricIo444Spec> {
        RegionNendAddr35124W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io444::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io444::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo444Spec;
impl crate::RegisterSpec for PricIo444Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io444::R`](R) reader structure"]
impl crate::Readable for PricIo444Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io444::W`](W) writer structure"]
impl crate::Writable for PricIo444Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO444 to value 0"]
impl crate::Resettable for PricIo444Spec {}
