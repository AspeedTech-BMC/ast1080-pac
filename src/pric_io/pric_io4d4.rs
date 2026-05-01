#[doc = "Register `PRIC_IO4D4` reader"]
pub type R = crate::R<PricIo4d4Spec>;
#[doc = "Register `PRIC_IO4D4` writer"]
pub type W = crate::W<PricIo4d4Spec>;
#[doc = "Field `RegionNWrProt13` reader - Region #N Write Protection"]
pub type RegionNwrProt13R = crate::BitReader;
#[doc = "Field `RegionNWrProt13` writer - Region #N Write Protection"]
pub type RegionNwrProt13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance13 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance13> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance13` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance13R = crate::BitReader<EnblRstTolerance13>;
impl EnblRstTolerance13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance13 {
        match self.bits {
            false => EnblRstTolerance13::ResetBySrst,
            true => EnblRstTolerance13::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance13::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance13::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance13` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance13W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance13>;
impl<'a, REG> EnblRstTolerance13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance13::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance13::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup13` reader - Region #N Read Group"]
pub type RegionNreadGroup13R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup13` writer - Region #N Read Group"]
pub type RegionNreadGroup13W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr351213` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351213R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr351213` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351213W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot13(&self) -> RegionNwrProt13R {
        RegionNwrProt13R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance13(&self) -> EnblRstTolerance13R {
        EnblRstTolerance13R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group13(&self) -> RegionNreadGroup13R {
        RegionNreadGroup13R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351213(&self) -> RegionNendAddr351213R {
        RegionNendAddr351213R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot13(&mut self) -> RegionNwrProt13W<PricIo4d4Spec> {
        RegionNwrProt13W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance13(&mut self) -> EnblRstTolerance13W<PricIo4d4Spec> {
        EnblRstTolerance13W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group13(&mut self) -> RegionNreadGroup13W<PricIo4d4Spec> {
        RegionNreadGroup13W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351213(&mut self) -> RegionNendAddr351213W<PricIo4d4Spec> {
        RegionNendAddr351213W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#53\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4d4Spec;
impl crate::RegisterSpec for PricIo4d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4d4::R`](R) reader structure"]
impl crate::Readable for PricIo4d4Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4d4::W`](W) writer structure"]
impl crate::Writable for PricIo4d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4D4 to value 0"]
impl crate::Resettable for PricIo4d4Spec {}
