#[doc = "Register `PRIC_IO4C4` reader"]
pub type R = crate::R<PricIo4c4Spec>;
#[doc = "Register `PRIC_IO4C4` writer"]
pub type W = crate::W<PricIo4c4Spec>;
#[doc = "Field `RegionNWrProt12` reader - Region #N Write Protection"]
pub type RegionNwrProt12R = crate::BitReader;
#[doc = "Field `RegionNWrProt12` writer - Region #N Write Protection"]
pub type RegionNwrProt12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance12 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance12> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance12` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance12R = crate::BitReader<EnblRstTolerance12>;
impl EnblRstTolerance12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance12 {
        match self.bits {
            false => EnblRstTolerance12::ResetBySrst,
            true => EnblRstTolerance12::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance12::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance12::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance12` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance12W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance12>;
impl<'a, REG> EnblRstTolerance12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance12::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance12::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup12` reader - Region #N Read Group"]
pub type RegionNreadGroup12R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup12` writer - Region #N Read Group"]
pub type RegionNreadGroup12W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr351212` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351212R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr351212` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351212W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot12(&self) -> RegionNwrProt12R {
        RegionNwrProt12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance12(&self) -> EnblRstTolerance12R {
        EnblRstTolerance12R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group12(&self) -> RegionNreadGroup12R {
        RegionNreadGroup12R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351212(&self) -> RegionNendAddr351212R {
        RegionNendAddr351212R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot12(&mut self) -> RegionNwrProt12W<PricIo4c4Spec> {
        RegionNwrProt12W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance12(&mut self) -> EnblRstTolerance12W<PricIo4c4Spec> {
        EnblRstTolerance12W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group12(&mut self) -> RegionNreadGroup12W<PricIo4c4Spec> {
        RegionNreadGroup12W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351212(&mut self) -> RegionNendAddr351212W<PricIo4c4Spec> {
        RegionNendAddr351212W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#49\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4c4Spec;
impl crate::RegisterSpec for PricIo4c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4c4::R`](R) reader structure"]
impl crate::Readable for PricIo4c4Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4c4::W`](W) writer structure"]
impl crate::Writable for PricIo4c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4C4 to value 0"]
impl crate::Resettable for PricIo4c4Spec {}
