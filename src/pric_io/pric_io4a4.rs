#[doc = "Register `PRIC_IO4A4` reader"]
pub type R = crate::R<PricIo4a4Spec>;
#[doc = "Register `PRIC_IO4A4` writer"]
pub type W = crate::W<PricIo4a4Spec>;
#[doc = "Field `RegionNWrProt10` reader - Region #N Write Protection"]
pub type RegionNwrProt10R = crate::BitReader;
#[doc = "Field `RegionNWrProt10` writer - Region #N Write Protection"]
pub type RegionNwrProt10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance10 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance10> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance10` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance10R = crate::BitReader<EnblRstTolerance10>;
impl EnblRstTolerance10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance10 {
        match self.bits {
            false => EnblRstTolerance10::ResetBySrst,
            true => EnblRstTolerance10::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance10::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance10::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance10` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance10W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance10>;
impl<'a, REG> EnblRstTolerance10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance10::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance10::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup10` reader - Region #N Read Group"]
pub type RegionNreadGroup10R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup10` writer - Region #N Read Group"]
pub type RegionNreadGroup10W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr351210` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351210R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr351210` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351210W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot10(&self) -> RegionNwrProt10R {
        RegionNwrProt10R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance10(&self) -> EnblRstTolerance10R {
        EnblRstTolerance10R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group10(&self) -> RegionNreadGroup10R {
        RegionNreadGroup10R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351210(&self) -> RegionNendAddr351210R {
        RegionNendAddr351210R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot10(&mut self) -> RegionNwrProt10W<PricIo4a4Spec> {
        RegionNwrProt10W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance10(&mut self) -> EnblRstTolerance10W<PricIo4a4Spec> {
        EnblRstTolerance10W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group10(&mut self) -> RegionNreadGroup10W<PricIo4a4Spec> {
        RegionNreadGroup10W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351210(&mut self) -> RegionNendAddr351210W<PricIo4a4Spec> {
        RegionNendAddr351210W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4a4Spec;
impl crate::RegisterSpec for PricIo4a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4a4::R`](R) reader structure"]
impl crate::Readable for PricIo4a4Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4a4::W`](W) writer structure"]
impl crate::Writable for PricIo4a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4A4 to value 0"]
impl crate::Resettable for PricIo4a4Spec {}
