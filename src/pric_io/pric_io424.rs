#[doc = "Register `PRIC_IO424` reader"]
pub type R = crate::R<PricIo424Spec>;
#[doc = "Register `PRIC_IO424` writer"]
pub type W = crate::W<PricIo424Spec>;
#[doc = "Field `RegionNWrProt2` reader - Region #N Write Protection"]
pub type RegionNwrProt2R = crate::BitReader;
#[doc = "Field `RegionNWrProt2` writer - Region #N Write Protection"]
pub type RegionNwrProt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance2 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance2> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance2` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance2R = crate::BitReader<EnblRstTolerance2>;
impl EnblRstTolerance2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance2 {
        match self.bits {
            false => EnblRstTolerance2::ResetBySrst,
            true => EnblRstTolerance2::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance2::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance2::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance2` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance2W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance2>;
impl<'a, REG> EnblRstTolerance2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance2::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance2::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup2` reader - Region #N Read Group"]
pub type RegionNreadGroup2R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup2` writer - Region #N Read Group"]
pub type RegionNreadGroup2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr35122` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35122R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr35122` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35122W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot2(&self) -> RegionNwrProt2R {
        RegionNwrProt2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance2(&self) -> EnblRstTolerance2R {
        EnblRstTolerance2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group2(&self) -> RegionNreadGroup2R {
        RegionNreadGroup2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35122(&self) -> RegionNendAddr35122R {
        RegionNendAddr35122R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot2(&mut self) -> RegionNwrProt2W<PricIo424Spec> {
        RegionNwrProt2W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance2(&mut self) -> EnblRstTolerance2W<PricIo424Spec> {
        EnblRstTolerance2W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group2(&mut self) -> RegionNreadGroup2W<PricIo424Spec> {
        RegionNreadGroup2W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35122(&mut self) -> RegionNendAddr35122W<PricIo424Spec> {
        RegionNendAddr35122W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io424::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io424::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo424Spec;
impl crate::RegisterSpec for PricIo424Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io424::R`](R) reader structure"]
impl crate::Readable for PricIo424Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io424::W`](W) writer structure"]
impl crate::Writable for PricIo424Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO424 to value 0"]
impl crate::Resettable for PricIo424Spec {}
