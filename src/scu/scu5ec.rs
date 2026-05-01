#[doc = "Register `SCU5EC` reader"]
pub type R = crate::R<Scu5ecSpec>;
#[doc = "Register `SCU5EC` writer"]
pub type W = crate::W<Scu5ecSpec>;
#[doc = "Field `SCUDISPDIO182` reader - SCU_DIS_PD_IO182"]
pub type Scudispdio182R = crate::BitReader;
#[doc = "Field `SCUDISPDIO182` writer - SCU_DIS_PD_IO182"]
pub type Scudispdio182W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO182` reader - SCU_DIS_PU_IO182"]
pub type Scudispuio182R = crate::BitReader;
#[doc = "Field `SCUDISPUIO182` writer - SCU_DIS_PU_IO182"]
pub type Scudispuio182W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO182` reader - SCU_DRV_IO182"]
pub type Scudrvio182R = crate::FieldReader;
#[doc = "Field `SCUDRVIO182` writer - SCU_DRV_IO182"]
pub type Scudrvio182W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO182` reader - SCU_EN_SMT_IO182"]
pub type Scuensmtio182R = crate::BitReader;
#[doc = "Field `SCUENSMTIO182` writer - SCU_EN_SMT_IO182"]
pub type Scuensmtio182W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO182` reader - SCU_EN_HV_IO182"]
pub type Scuenhvio182R = crate::BitReader;
#[doc = "Field `SCUENHVIO182` writer - SCU_EN_HV_IO182"]
pub type Scuenhvio182W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO183` reader - SCU_DIS_PD_IO183"]
pub type Scudispdio183R = crate::BitReader;
#[doc = "Field `SCUDISPDIO183` writer - SCU_DIS_PD_IO183"]
pub type Scudispdio183W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO183` reader - SCU_DIS_PU_IO183"]
pub type Scudispuio183R = crate::BitReader;
#[doc = "Field `SCUDISPUIO183` writer - SCU_DIS_PU_IO183"]
pub type Scudispuio183W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO183` reader - SCU_DRV_IO183"]
pub type Scudrvio183R = crate::FieldReader;
#[doc = "Field `SCUDRVIO183` writer - SCU_DRV_IO183"]
pub type Scudrvio183W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO183` reader - SCU_EN_SMT_IO183"]
pub type Scuensmtio183R = crate::BitReader;
#[doc = "Field `SCUENSMTIO183` writer - SCU_EN_SMT_IO183"]
pub type Scuensmtio183W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO183` reader - SCU_EN_HV_IO183"]
pub type Scuenhvio183R = crate::BitReader;
#[doc = "Field `SCUENHVIO183` writer - SCU_EN_HV_IO183"]
pub type Scuenhvio183W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO182"]
    #[inline(always)]
    pub fn scudispdio182(&self) -> Scudispdio182R {
        Scudispdio182R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO182"]
    #[inline(always)]
    pub fn scudispuio182(&self) -> Scudispuio182R {
        Scudispuio182R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO182"]
    #[inline(always)]
    pub fn scudrvio182(&self) -> Scudrvio182R {
        Scudrvio182R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO182"]
    #[inline(always)]
    pub fn scuensmtio182(&self) -> Scuensmtio182R {
        Scuensmtio182R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO182"]
    #[inline(always)]
    pub fn scuenhvio182(&self) -> Scuenhvio182R {
        Scuenhvio182R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO183"]
    #[inline(always)]
    pub fn scudispdio183(&self) -> Scudispdio183R {
        Scudispdio183R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO183"]
    #[inline(always)]
    pub fn scudispuio183(&self) -> Scudispuio183R {
        Scudispuio183R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO183"]
    #[inline(always)]
    pub fn scudrvio183(&self) -> Scudrvio183R {
        Scudrvio183R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO183"]
    #[inline(always)]
    pub fn scuensmtio183(&self) -> Scuensmtio183R {
        Scuensmtio183R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO183"]
    #[inline(always)]
    pub fn scuenhvio183(&self) -> Scuenhvio183R {
        Scuenhvio183R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO182"]
    #[inline(always)]
    pub fn scudispdio182(&mut self) -> Scudispdio182W<Scu5ecSpec> {
        Scudispdio182W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO182"]
    #[inline(always)]
    pub fn scudispuio182(&mut self) -> Scudispuio182W<Scu5ecSpec> {
        Scudispuio182W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO182"]
    #[inline(always)]
    pub fn scudrvio182(&mut self) -> Scudrvio182W<Scu5ecSpec> {
        Scudrvio182W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO182"]
    #[inline(always)]
    pub fn scuensmtio182(&mut self) -> Scuensmtio182W<Scu5ecSpec> {
        Scuensmtio182W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO182"]
    #[inline(always)]
    pub fn scuenhvio182(&mut self) -> Scuenhvio182W<Scu5ecSpec> {
        Scuenhvio182W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO183"]
    #[inline(always)]
    pub fn scudispdio183(&mut self) -> Scudispdio183W<Scu5ecSpec> {
        Scudispdio183W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO183"]
    #[inline(always)]
    pub fn scudispuio183(&mut self) -> Scudispuio183W<Scu5ecSpec> {
        Scudispuio183W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO183"]
    #[inline(always)]
    pub fn scudrvio183(&mut self) -> Scudrvio183W<Scu5ecSpec> {
        Scudrvio183W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO183"]
    #[inline(always)]
    pub fn scuensmtio183(&mut self) -> Scuensmtio183W<Scu5ecSpec> {
        Scuensmtio183W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO183"]
    #[inline(always)]
    pub fn scuenhvio183(&mut self) -> Scuenhvio183W<Scu5ecSpec> {
        Scuenhvio183W::new(self, 25)
    }
}
#[doc = "IO Control \\#92\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5ecSpec;
impl crate::RegisterSpec for Scu5ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5ec::R`](R) reader structure"]
impl crate::Readable for Scu5ecSpec {}
#[doc = "`write(|w| ..)` method takes [`scu5ec::W`](W) writer structure"]
impl crate::Writable for Scu5ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5EC to value 0x0204_0204"]
impl crate::Resettable for Scu5ecSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
