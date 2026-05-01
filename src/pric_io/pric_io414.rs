#[doc = "Register `PRIC_IO414` reader"]
pub type R = crate::R<PricIo414Spec>;
#[doc = "Register `PRIC_IO414` writer"]
pub type W = crate::W<PricIo414Spec>;
#[doc = "Field `RegionNWrProt1` reader - Region #N Write Protection"]
pub type RegionNwrProt1R = crate::BitReader;
#[doc = "Field `RegionNWrProt1` writer - Region #N Write Protection"]
pub type RegionNwrProt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance1 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance1> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance1` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance1R = crate::BitReader<EnblRstTolerance1>;
impl EnblRstTolerance1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance1 {
        match self.bits {
            false => EnblRstTolerance1::ResetBySrst,
            true => EnblRstTolerance1::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance1::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance1::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance1` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance1W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance1>;
impl<'a, REG> EnblRstTolerance1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance1::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance1::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup1` reader - Region #N Read Group"]
pub type RegionNreadGroup1R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup1` writer - Region #N Read Group"]
pub type RegionNreadGroup1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr35121` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35121R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr35121` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35121W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot1(&self) -> RegionNwrProt1R {
        RegionNwrProt1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance1(&self) -> EnblRstTolerance1R {
        EnblRstTolerance1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group1(&self) -> RegionNreadGroup1R {
        RegionNreadGroup1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35121(&self) -> RegionNendAddr35121R {
        RegionNendAddr35121R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot1(&mut self) -> RegionNwrProt1W<PricIo414Spec> {
        RegionNwrProt1W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance1(&mut self) -> EnblRstTolerance1W<PricIo414Spec> {
        EnblRstTolerance1W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group1(&mut self) -> RegionNreadGroup1W<PricIo414Spec> {
        RegionNreadGroup1W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35121(&mut self) -> RegionNendAddr35121W<PricIo414Spec> {
        RegionNendAddr35121W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io414::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io414::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo414Spec;
impl crate::RegisterSpec for PricIo414Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io414::R`](R) reader structure"]
impl crate::Readable for PricIo414Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io414::W`](W) writer structure"]
impl crate::Writable for PricIo414Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO414 to value 0"]
impl crate::Resettable for PricIo414Spec {}
