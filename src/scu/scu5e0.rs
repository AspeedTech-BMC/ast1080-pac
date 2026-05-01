#[doc = "Register `SCU5E0` reader"]
pub type R = crate::R<Scu5e0Spec>;
#[doc = "Register `SCU5E0` writer"]
pub type W = crate::W<Scu5e0Spec>;
#[doc = "Field `SCUDISPDIO176` reader - SCU_DIS_PD_IO176"]
pub type Scudispdio176R = crate::BitReader;
#[doc = "Field `SCUDISPDIO176` writer - SCU_DIS_PD_IO176"]
pub type Scudispdio176W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO176` reader - SCU_DIS_PU_IO176"]
pub type Scudispuio176R = crate::BitReader;
#[doc = "Field `SCUDISPUIO176` writer - SCU_DIS_PU_IO176"]
pub type Scudispuio176W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO176` reader - SCU_DRV_IO176"]
pub type Scudrvio176R = crate::FieldReader;
#[doc = "Field `SCUDRVIO176` writer - SCU_DRV_IO176"]
pub type Scudrvio176W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO176` reader - SCU_EN_SMT_IO176"]
pub type Scuensmtio176R = crate::BitReader;
#[doc = "Field `SCUENSMTIO176` writer - SCU_EN_SMT_IO176"]
pub type Scuensmtio176W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO176` reader - SCU_EN_HV_IO176"]
pub type Scuenhvio176R = crate::BitReader;
#[doc = "Field `SCUENHVIO176` writer - SCU_EN_HV_IO176"]
pub type Scuenhvio176W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO177` reader - SCU_DIS_PD_IO177"]
pub type Scudispdio177R = crate::BitReader;
#[doc = "Field `SCUDISPDIO177` writer - SCU_DIS_PD_IO177"]
pub type Scudispdio177W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO177` reader - SCU_DIS_PU_IO177"]
pub type Scudispuio177R = crate::BitReader;
#[doc = "Field `SCUDISPUIO177` writer - SCU_DIS_PU_IO177"]
pub type Scudispuio177W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO177` reader - SCU_DRV_IO177"]
pub type Scudrvio177R = crate::FieldReader;
#[doc = "Field `SCUDRVIO177` writer - SCU_DRV_IO177"]
pub type Scudrvio177W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO177` reader - SCU_EN_SMT_IO177"]
pub type Scuensmtio177R = crate::BitReader;
#[doc = "Field `SCUENSMTIO177` writer - SCU_EN_SMT_IO177"]
pub type Scuensmtio177W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO177` reader - SCU_EN_HV_IO177"]
pub type Scuenhvio177R = crate::BitReader;
#[doc = "Field `SCUENHVIO177` writer - SCU_EN_HV_IO177"]
pub type Scuenhvio177W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO176"]
    #[inline(always)]
    pub fn scudispdio176(&self) -> Scudispdio176R {
        Scudispdio176R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO176"]
    #[inline(always)]
    pub fn scudispuio176(&self) -> Scudispuio176R {
        Scudispuio176R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO176"]
    #[inline(always)]
    pub fn scudrvio176(&self) -> Scudrvio176R {
        Scudrvio176R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO176"]
    #[inline(always)]
    pub fn scuensmtio176(&self) -> Scuensmtio176R {
        Scuensmtio176R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO176"]
    #[inline(always)]
    pub fn scuenhvio176(&self) -> Scuenhvio176R {
        Scuenhvio176R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO177"]
    #[inline(always)]
    pub fn scudispdio177(&self) -> Scudispdio177R {
        Scudispdio177R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO177"]
    #[inline(always)]
    pub fn scudispuio177(&self) -> Scudispuio177R {
        Scudispuio177R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO177"]
    #[inline(always)]
    pub fn scudrvio177(&self) -> Scudrvio177R {
        Scudrvio177R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO177"]
    #[inline(always)]
    pub fn scuensmtio177(&self) -> Scuensmtio177R {
        Scuensmtio177R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO177"]
    #[inline(always)]
    pub fn scuenhvio177(&self) -> Scuenhvio177R {
        Scuenhvio177R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO176"]
    #[inline(always)]
    pub fn scudispdio176(&mut self) -> Scudispdio176W<Scu5e0Spec> {
        Scudispdio176W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO176"]
    #[inline(always)]
    pub fn scudispuio176(&mut self) -> Scudispuio176W<Scu5e0Spec> {
        Scudispuio176W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO176"]
    #[inline(always)]
    pub fn scudrvio176(&mut self) -> Scudrvio176W<Scu5e0Spec> {
        Scudrvio176W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO176"]
    #[inline(always)]
    pub fn scuensmtio176(&mut self) -> Scuensmtio176W<Scu5e0Spec> {
        Scuensmtio176W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO176"]
    #[inline(always)]
    pub fn scuenhvio176(&mut self) -> Scuenhvio176W<Scu5e0Spec> {
        Scuenhvio176W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO177"]
    #[inline(always)]
    pub fn scudispdio177(&mut self) -> Scudispdio177W<Scu5e0Spec> {
        Scudispdio177W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO177"]
    #[inline(always)]
    pub fn scudispuio177(&mut self) -> Scudispuio177W<Scu5e0Spec> {
        Scudispuio177W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO177"]
    #[inline(always)]
    pub fn scudrvio177(&mut self) -> Scudrvio177W<Scu5e0Spec> {
        Scudrvio177W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO177"]
    #[inline(always)]
    pub fn scuensmtio177(&mut self) -> Scuensmtio177W<Scu5e0Spec> {
        Scuensmtio177W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO177"]
    #[inline(always)]
    pub fn scuenhvio177(&mut self) -> Scuenhvio177W<Scu5e0Spec> {
        Scuenhvio177W::new(self, 25)
    }
}
#[doc = "IO Control \\#89\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5e0Spec;
impl crate::RegisterSpec for Scu5e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5e0::R`](R) reader structure"]
impl crate::Readable for Scu5e0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5e0::W`](W) writer structure"]
impl crate::Writable for Scu5e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5E0 to value 0x0204_0204"]
impl crate::Resettable for Scu5e0Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
