#[doc = "Register `SCU5CC` reader"]
pub type R = crate::R<Scu5ccSpec>;
#[doc = "Register `SCU5CC` writer"]
pub type W = crate::W<Scu5ccSpec>;
#[doc = "Field `SCUDISPDIO166` reader - SCU_DIS_PD_IO166"]
pub type Scudispdio166R = crate::BitReader;
#[doc = "Field `SCUDISPDIO166` writer - SCU_DIS_PD_IO166"]
pub type Scudispdio166W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO166` reader - SCU_DIS_PU_IO166"]
pub type Scudispuio166R = crate::BitReader;
#[doc = "Field `SCUDISPUIO166` writer - SCU_DIS_PU_IO166"]
pub type Scudispuio166W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO166` reader - SCU_DRV_IO166"]
pub type Scudrvio166R = crate::FieldReader;
#[doc = "Field `SCUDRVIO166` writer - SCU_DRV_IO166"]
pub type Scudrvio166W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO166` reader - SCU_EN_SMT_IO166"]
pub type Scuensmtio166R = crate::BitReader;
#[doc = "Field `SCUENSMTIO166` writer - SCU_EN_SMT_IO166"]
pub type Scuensmtio166W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO166` reader - SCU_EN_HV_IO166"]
pub type Scuenhvio166R = crate::BitReader;
#[doc = "Field `SCUENHVIO166` writer - SCU_EN_HV_IO166"]
pub type Scuenhvio166W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO167` reader - SCU_DIS_PD_IO167"]
pub type Scudispdio167R = crate::BitReader;
#[doc = "Field `SCUDISPDIO167` writer - SCU_DIS_PD_IO167"]
pub type Scudispdio167W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO167` reader - SCU_DIS_PU_IO167"]
pub type Scudispuio167R = crate::BitReader;
#[doc = "Field `SCUDISPUIO167` writer - SCU_DIS_PU_IO167"]
pub type Scudispuio167W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO167` reader - SCU_DRV_IO167"]
pub type Scudrvio167R = crate::FieldReader;
#[doc = "Field `SCUDRVIO167` writer - SCU_DRV_IO167"]
pub type Scudrvio167W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO167` reader - SCU_EN_SMT_IO167"]
pub type Scuensmtio167R = crate::BitReader;
#[doc = "Field `SCUENSMTIO167` writer - SCU_EN_SMT_IO167"]
pub type Scuensmtio167W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO167` reader - SCU_EN_HV_IO167"]
pub type Scuenhvio167R = crate::BitReader;
#[doc = "Field `SCUENHVIO167` writer - SCU_EN_HV_IO167"]
pub type Scuenhvio167W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO166"]
    #[inline(always)]
    pub fn scudispdio166(&self) -> Scudispdio166R {
        Scudispdio166R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO166"]
    #[inline(always)]
    pub fn scudispuio166(&self) -> Scudispuio166R {
        Scudispuio166R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO166"]
    #[inline(always)]
    pub fn scudrvio166(&self) -> Scudrvio166R {
        Scudrvio166R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO166"]
    #[inline(always)]
    pub fn scuensmtio166(&self) -> Scuensmtio166R {
        Scuensmtio166R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO166"]
    #[inline(always)]
    pub fn scuenhvio166(&self) -> Scuenhvio166R {
        Scuenhvio166R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO167"]
    #[inline(always)]
    pub fn scudispdio167(&self) -> Scudispdio167R {
        Scudispdio167R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO167"]
    #[inline(always)]
    pub fn scudispuio167(&self) -> Scudispuio167R {
        Scudispuio167R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO167"]
    #[inline(always)]
    pub fn scudrvio167(&self) -> Scudrvio167R {
        Scudrvio167R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO167"]
    #[inline(always)]
    pub fn scuensmtio167(&self) -> Scuensmtio167R {
        Scuensmtio167R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO167"]
    #[inline(always)]
    pub fn scuenhvio167(&self) -> Scuenhvio167R {
        Scuenhvio167R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO166"]
    #[inline(always)]
    pub fn scudispdio166(&mut self) -> Scudispdio166W<Scu5ccSpec> {
        Scudispdio166W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO166"]
    #[inline(always)]
    pub fn scudispuio166(&mut self) -> Scudispuio166W<Scu5ccSpec> {
        Scudispuio166W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO166"]
    #[inline(always)]
    pub fn scudrvio166(&mut self) -> Scudrvio166W<Scu5ccSpec> {
        Scudrvio166W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO166"]
    #[inline(always)]
    pub fn scuensmtio166(&mut self) -> Scuensmtio166W<Scu5ccSpec> {
        Scuensmtio166W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO166"]
    #[inline(always)]
    pub fn scuenhvio166(&mut self) -> Scuenhvio166W<Scu5ccSpec> {
        Scuenhvio166W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO167"]
    #[inline(always)]
    pub fn scudispdio167(&mut self) -> Scudispdio167W<Scu5ccSpec> {
        Scudispdio167W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO167"]
    #[inline(always)]
    pub fn scudispuio167(&mut self) -> Scudispuio167W<Scu5ccSpec> {
        Scudispuio167W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO167"]
    #[inline(always)]
    pub fn scudrvio167(&mut self) -> Scudrvio167W<Scu5ccSpec> {
        Scudrvio167W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO167"]
    #[inline(always)]
    pub fn scuensmtio167(&mut self) -> Scuensmtio167W<Scu5ccSpec> {
        Scuensmtio167W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO167"]
    #[inline(always)]
    pub fn scuenhvio167(&mut self) -> Scuenhvio167W<Scu5ccSpec> {
        Scuenhvio167W::new(self, 25)
    }
}
#[doc = "IO Control \\#84\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5ccSpec;
impl crate::RegisterSpec for Scu5ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5cc::R`](R) reader structure"]
impl crate::Readable for Scu5ccSpec {}
#[doc = "`write(|w| ..)` method takes [`scu5cc::W`](W) writer structure"]
impl crate::Writable for Scu5ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5CC to value 0x0204_0204"]
impl crate::Resettable for Scu5ccSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
