#[doc = "Register `PRIC_IO4F4` reader"]
pub type R = crate::R<PricIo4f4Spec>;
#[doc = "Register `PRIC_IO4F4` writer"]
pub type W = crate::W<PricIo4f4Spec>;
#[doc = "Field `RegionNWrProt15` reader - Region #N Write Protection"]
pub type RegionNwrProt15R = crate::BitReader;
#[doc = "Field `RegionNWrProt15` writer - Region #N Write Protection"]
pub type RegionNwrProt15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance15 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance15> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance15` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance15R = crate::BitReader<EnblRstTolerance15>;
impl EnblRstTolerance15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance15 {
        match self.bits {
            false => EnblRstTolerance15::ResetBySrst,
            true => EnblRstTolerance15::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance15::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance15::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance15` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance15W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance15>;
impl<'a, REG> EnblRstTolerance15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance15::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance15::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup15` reader - Region #N Read Group"]
pub type RegionNreadGroup15R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup15` writer - Region #N Read Group"]
pub type RegionNreadGroup15W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr351215` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351215R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr351215` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351215W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot15(&self) -> RegionNwrProt15R {
        RegionNwrProt15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance15(&self) -> EnblRstTolerance15R {
        EnblRstTolerance15R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group15(&self) -> RegionNreadGroup15R {
        RegionNreadGroup15R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351215(&self) -> RegionNendAddr351215R {
        RegionNendAddr351215R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot15(&mut self) -> RegionNwrProt15W<PricIo4f4Spec> {
        RegionNwrProt15W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance15(&mut self) -> EnblRstTolerance15W<PricIo4f4Spec> {
        EnblRstTolerance15W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group15(&mut self) -> RegionNreadGroup15W<PricIo4f4Spec> {
        RegionNreadGroup15W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351215(&mut self) -> RegionNendAddr351215W<PricIo4f4Spec> {
        RegionNendAddr351215W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#61\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4f4Spec;
impl crate::RegisterSpec for PricIo4f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4f4::R`](R) reader structure"]
impl crate::Readable for PricIo4f4Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4f4::W`](W) writer structure"]
impl crate::Writable for PricIo4f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4F4 to value 0"]
impl crate::Resettable for PricIo4f4Spec {}
