#[doc = "Register `PRIC_IO4E4` reader"]
pub type R = crate::R<PricIo4e4Spec>;
#[doc = "Register `PRIC_IO4E4` writer"]
pub type W = crate::W<PricIo4e4Spec>;
#[doc = "Field `RegionNWrProt14` reader - Region #N Write Protection"]
pub type RegionNwrProt14R = crate::BitReader;
#[doc = "Field `RegionNWrProt14` writer - Region #N Write Protection"]
pub type RegionNwrProt14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstTolerance14 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstTolerance14> for bool {
    #[inline(always)]
    fn from(variant: EnblRstTolerance14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstTolerance14` reader - Enable Reset Tolerance"]
pub type EnblRstTolerance14R = crate::BitReader<EnblRstTolerance14>;
impl EnblRstTolerance14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstTolerance14 {
        match self.bits {
            false => EnblRstTolerance14::ResetBySrst,
            true => EnblRstTolerance14::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstTolerance14::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstTolerance14::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstTolerance14` writer - Enable Reset Tolerance"]
pub type EnblRstTolerance14W<'a, REG> = crate::BitWriter<'a, REG, EnblRstTolerance14>;
impl<'a, REG> EnblRstTolerance14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance14::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstTolerance14::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `RegionNReadGroup14` reader - Region #N Read Group"]
pub type RegionNreadGroup14R = crate::FieldReader;
#[doc = "Field `RegionNReadGroup14` writer - Region #N Read Group"]
pub type RegionNreadGroup14W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNEndAddr351214` reader - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351214R = crate::FieldReader<u32>;
#[doc = "Field `RegionNEndAddr351214` writer - Region #N End Address\\[35:12\\]"]
pub type RegionNendAddr351214W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot14(&self) -> RegionNwrProt14R {
        RegionNwrProt14R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance14(&self) -> EnblRstTolerance14R {
        EnblRstTolerance14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group14(&self) -> RegionNreadGroup14R {
        RegionNreadGroup14R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351214(&self) -> RegionNendAddr351214R {
        RegionNendAddr351214R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Write Protection"]
    #[inline(always)]
    pub fn region_nwr_prot14(&mut self) -> RegionNwrProt14W<PricIo4e4Spec> {
        RegionNwrProt14W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Reset Tolerance"]
    #[inline(always)]
    pub fn enbl_rst_tolerance14(&mut self) -> EnblRstTolerance14W<PricIo4e4Spec> {
        EnblRstTolerance14W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Region #N Read Group"]
    #[inline(always)]
    pub fn region_nread_group14(&mut self) -> RegionNreadGroup14W<PricIo4e4Spec> {
        RegionNreadGroup14W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N End Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nend_addr351214(&mut self) -> RegionNendAddr351214W<PricIo4e4Spec> {
        RegionNendAddr351214W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#57\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4e4Spec;
impl crate::RegisterSpec for PricIo4e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4e4::R`](R) reader structure"]
impl crate::Readable for PricIo4e4Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4e4::W`](W) writer structure"]
impl crate::Writable for PricIo4e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4E4 to value 0"]
impl crate::Resettable for PricIo4e4Spec {}
