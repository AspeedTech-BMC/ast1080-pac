#[doc = "Register `PRIC_IO454` reader"]
pub type R = crate::R<PricIo454Spec>;
#[doc = "Register `PRIC_IO454` writer"]
pub type W = crate::W<PricIo454Spec>;
#[doc = "Field `RegionNWrProt5` reader - Region #N Write Protection"]
pub type RegionNwrProt5R = crate::BitReader;
#[doc = "Field `RegionNWrProt5` writer - Region #N Write Protection"]
pub type RegionNwrProt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance5 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance5> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance5` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance5R = crate::BitReader<EnblRstTolerance5>;
impl EnblRstTolerance5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance5 {
        match self.bits {
            false => EnblRstTolerance5::ResetBySrst,
            true => EnblRstTolerance5::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance5::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance5::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance5` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance5W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance5>;
impl<'a, REG> EnblRstTolerance5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance5::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance5::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup5` reader - Region #N Read Group"]
pub type RegionNreadGroup5R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup5` writer - Region #N Read Group"]
pub type RegionNreadGroup5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr35125` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35125R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr35125` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35125W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot5(&self) -> RegionNwrProt5R {
        RegionNwrProt5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance5(&self) -> EnblRstTolerance5R {
        EnblRstTolerance5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group5(&self) -> RegionNreadGroup5R {
        RegionNreadGroup5R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35125(&self) -> RegionNendAddr35125R {
        RegionNendAddr35125R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot5(&mut self) -> RegionNwrProt5W<PricIo454Spec> {
        RegionNwrProt5W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance5(&mut self) -> EnblRstTolerance5W<PricIo454Spec> {
        EnblRstTolerance5W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group5(&mut self) -> RegionNreadGroup5W<PricIo454Spec> {
        RegionNreadGroup5W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35125(&mut self) -> RegionNendAddr35125W<PricIo454Spec> {
        RegionNendAddr35125W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io454::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io454::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo454Spec;
impl crate::RegisterSpec for PricIo454Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io454::R`](R) reader structure"]
impl crate::Readable for PricIo454Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io454::W`](W) writer structure"]
impl crate::Writable for PricIo454Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO454 to value 0"]
impl crate::Resettable for PricIo454Spec {}
