#[doc = "Register `PRIC_IO4B4` reader"]
pub type R = crate::R<PricIo4b4Spec>;
#[doc = "Register `PRIC_IO4B4` writer"]
pub type W = crate::W<PricIo4b4Spec>;
#[doc = "Field `RegionNWrProt11` reader - Region #N Write Protection"]
pub type RegionNwrProt11R = crate::BitReader;
#[doc = "Field `RegionNWrProt11` writer - Region #N Write Protection"]
pub type RegionNwrProt11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance11 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance11> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance11` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance11R = crate::BitReader<EnblRstTolerance11>;
impl EnblRstTolerance11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance11 {
        match self.bits {
            false => EnblRstTolerance11::ResetBySrst,
            true => EnblRstTolerance11::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance11::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance11::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance11` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance11W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance11>;
impl<'a, REG> EnblRstTolerance11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance11::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance11::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup11` reader - Region #N Read Group"]
pub type RegionNreadGroup11R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup11` writer - Region #N Read Group"]
pub type RegionNreadGroup11W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr351211` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351211R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr351211` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351211W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot11(&self) -> RegionNwrProt11R {
        RegionNwrProt11R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance11(&self) -> EnblRstTolerance11R {
        EnblRstTolerance11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group11(&self) -> RegionNreadGroup11R {
        RegionNreadGroup11R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351211(&self) -> RegionNendAddr351211R {
        RegionNendAddr351211R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot11(&mut self) -> RegionNwrProt11W<PricIo4b4Spec> {
        RegionNwrProt11W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance11(&mut self) -> EnblRstTolerance11W<PricIo4b4Spec> {
        EnblRstTolerance11W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group11(&mut self) -> RegionNreadGroup11W<PricIo4b4Spec> {
        RegionNreadGroup11W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351211(&mut self) -> RegionNendAddr351211W<PricIo4b4Spec> {
        RegionNendAddr351211W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4b4Spec;
impl crate::RegisterSpec for PricIo4b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4b4::R`](R) reader structure"]
impl crate::Readable for PricIo4b4Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4b4::W`](W) writer structure"]
impl crate::Writable for PricIo4b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4B4 to value 0"]
impl crate::Resettable for PricIo4b4Spec {}
