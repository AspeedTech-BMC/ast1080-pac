#[doc = "Register `PRIC_IO484` reader"]
pub type R = crate::R<PricIo484Spec>;
#[doc = "Register `PRIC_IO484` writer"]
pub type W = crate::W<PricIo484Spec>;
#[doc = "Field `RegionNWrProt8` reader - Region #N Write Protection"]
pub type RegionNwrProt8R = crate::BitReader;
#[doc = "Field `RegionNWrProt8` writer - Region #N Write Protection"]
pub type RegionNwrProt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance8 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance8> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance8` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance8R = crate::BitReader<EnblRstTolerance8>;
impl EnblRstTolerance8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance8 {
        match self.bits {
            false => EnblRstTolerance8::ResetBySrst,
            true => EnblRstTolerance8::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance8::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance8::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance8` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance8W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance8>;
impl<'a, REG> EnblRstTolerance8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance8::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance8::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup8` reader - Region #N Read Group"]
pub type RegionNreadGroup8R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup8` writer - Region #N Read Group"]
pub type RegionNreadGroup8W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr35128` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35128R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr35128` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35128W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot8(&self) -> RegionNwrProt8R {
        RegionNwrProt8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance8(&self) -> EnblRstTolerance8R {
        EnblRstTolerance8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group8(&self) -> RegionNreadGroup8R {
        RegionNreadGroup8R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35128(&self) -> RegionNendAddr35128R {
        RegionNendAddr35128R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot8(&mut self) -> RegionNwrProt8W<PricIo484Spec> {
        RegionNwrProt8W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance8(&mut self) -> EnblRstTolerance8W<PricIo484Spec> {
        EnblRstTolerance8W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group8(&mut self) -> RegionNreadGroup8W<PricIo484Spec> {
        RegionNreadGroup8W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35128(&mut self) -> RegionNendAddr35128W<PricIo484Spec> {
        RegionNendAddr35128W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io484::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io484::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo484Spec;
impl crate::RegisterSpec for PricIo484Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io484::R`](R) reader structure"]
impl crate::Readable for PricIo484Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io484::W`](W) writer structure"]
impl crate::Writable for PricIo484Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO484 to value 0"]
impl crate::Resettable for PricIo484Spec {}
