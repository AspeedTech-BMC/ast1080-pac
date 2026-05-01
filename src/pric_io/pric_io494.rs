#[doc = "Register `PRIC_IO494` reader"]
pub type R = crate::R<PricIo494Spec>;
#[doc = "Register `PRIC_IO494` writer"]
pub type W = crate::W<PricIo494Spec>;
#[doc = "Field `RegionNWrProt9` reader - Region #N Write Protection"]
pub type RegionNwrProt9R = crate::BitReader;
#[doc = "Field `RegionNWrProt9` writer - Region #N Write Protection"]
pub type RegionNwrProt9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance9 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance9> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance9` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance9R = crate::BitReader<EnblRstTolerance9>;
impl EnblRstTolerance9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance9 {
        match self.bits {
            false => EnblRstTolerance9::ResetBySrst,
            true => EnblRstTolerance9::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance9::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance9::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance9` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance9W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance9>;
impl<'a, REG> EnblRstTolerance9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance9::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance9::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup9` reader - Region #N Read Group"]
pub type RegionNreadGroup9R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup9` writer - Region #N Read Group"]
pub type RegionNreadGroup9W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr35129` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35129R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr35129` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr35129W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot9(&self) -> RegionNwrProt9R {
        RegionNwrProt9R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance9(&self) -> EnblRstTolerance9R {
        EnblRstTolerance9R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group9(&self) -> RegionNreadGroup9R {
        RegionNreadGroup9R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35129(&self) -> RegionNendAddr35129R {
        RegionNendAddr35129R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot9(&mut self) -> RegionNwrProt9W<PricIo494Spec> {
        RegionNwrProt9W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance9(&mut self) -> EnblRstTolerance9W<PricIo494Spec> {
        EnblRstTolerance9W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group9(&mut self) -> RegionNreadGroup9W<PricIo494Spec> {
        RegionNreadGroup9W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr35129(&mut self) -> RegionNendAddr35129W<PricIo494Spec> {
        RegionNendAddr35129W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io494::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io494::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo494Spec;
impl crate::RegisterSpec for PricIo494Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io494::R`](R) reader structure"]
impl crate::Readable for PricIo494Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io494::W`](W) writer structure"]
impl crate::Writable for PricIo494Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO494 to value 0"]
impl crate::Resettable for PricIo494Spec {}
