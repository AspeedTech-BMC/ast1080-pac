#[doc = "Register `PRIC_IO464` reader"]
pub type R = crate::R<PricIo464Spec>;
#[doc = "Register `PRIC_IO464` writer"]
pub type W = crate::W<PricIo464Spec>;
#[doc = "Field `RegionNWrProt6` reader - Region #N Write Protection"]
pub type RegionNwrProt6R = crate::BitReader;
#[doc = "Field `RegionNWrProt6` writer - Region #N Write Protection"]
pub type RegionNwrProt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance6 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance6> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance6` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance6R = crate::BitReader<EnblRstTolerance6>;
impl EnblRstTolerance6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance6 {
        match self.bits {
            false => EnblRstTolerance6::ResetBySrst,
            true => EnblRstTolerance6::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance6::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance6::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance6` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance6W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance6>;
impl<'a, REG> EnblRstTolerance6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance6::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance6::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup6` reader - Region #N Read Group"]
pub type RegionNreadGroup6R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup6` writer - Region #N Read Group"]
pub type RegionNreadGroup6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr35126` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35126R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr35126` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35126W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot6(&self) -> RegionNwrProt6R {
        RegionNwrProt6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance6(&self) -> EnblRstTolerance6R {
        EnblRstTolerance6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group6(&self) -> RegionNreadGroup6R {
        RegionNreadGroup6R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35126(&self) -> RegionNendAddr35126R {
        RegionNendAddr35126R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot6(&mut self) -> RegionNwrProt6W<PricIo464Spec> {
        RegionNwrProt6W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance6(&mut self) -> EnblRstTolerance6W<PricIo464Spec> {
        EnblRstTolerance6W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group6(&mut self) -> RegionNreadGroup6W<PricIo464Spec> {
        RegionNreadGroup6W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35126(&mut self) -> RegionNendAddr35126W<PricIo464Spec> {
        RegionNendAddr35126W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io464::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io464::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo464Spec;
impl crate::RegisterSpec for PricIo464Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io464::R`](R) reader structure"]
impl crate::Readable for PricIo464Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io464::W`](W) writer structure"]
impl crate::Writable for PricIo464Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO464 to value 0"]
impl crate::Resettable for PricIo464Spec {}
