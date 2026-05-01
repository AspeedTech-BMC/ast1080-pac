#[doc = "Register `SCU5A0` reader"]
pub type R = crate::R<Scu5a0Spec>;
#[doc = "Register `SCU5A0` writer"]
pub type W = crate::W<Scu5a0Spec>;
#[doc = "Field `SCUDISPDIO144` reader - SCU_DIS_PD_IO144"]
pub type Scudispdio144R = crate::BitReader;
#[doc = "Field `SCUDISPDIO144` writer - SCU_DIS_PD_IO144"]
pub type Scudispdio144W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO144` reader - SCU_DIS_PU_IO144"]
pub type Scudispuio144R = crate::BitReader;
#[doc = "Field `SCUDISPUIO144` writer - SCU_DIS_PU_IO144"]
pub type Scudispuio144W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO144` reader - SCU_DRV_IO144"]
pub type Scudrvio144R = crate::FieldReader;
#[doc = "Field `SCUDRVIO144` writer - SCU_DRV_IO144"]
pub type Scudrvio144W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO144` reader - SCU_EN_SMT_IO144"]
pub type Scuensmtio144R = crate::BitReader;
#[doc = "Field `SCUENSMTIO144` writer - SCU_EN_SMT_IO144"]
pub type Scuensmtio144W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO144` reader - SCU_EN_HV_IO144"]
pub type Scuenhvio144R = crate::BitReader;
#[doc = "Field `SCUENHVIO144` writer - SCU_EN_HV_IO144"]
pub type Scuenhvio144W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO145` reader - SCU_DIS_PD_IO145"]
pub type Scudispdio145R = crate::BitReader;
#[doc = "Field `SCUDISPDIO145` writer - SCU_DIS_PD_IO145"]
pub type Scudispdio145W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO145` reader - SCU_DIS_PU_IO145"]
pub type Scudispuio145R = crate::BitReader;
#[doc = "Field `SCUDISPUIO145` writer - SCU_DIS_PU_IO145"]
pub type Scudispuio145W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO145` reader - SCU_DRV_IO145"]
pub type Scudrvio145R = crate::FieldReader;
#[doc = "Field `SCUDRVIO145` writer - SCU_DRV_IO145"]
pub type Scudrvio145W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO145` reader - SCU_EN_SMT_IO145"]
pub type Scuensmtio145R = crate::BitReader;
#[doc = "Field `SCUENSMTIO145` writer - SCU_EN_SMT_IO145"]
pub type Scuensmtio145W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO145` reader - SCU_EN_HV_IO145"]
pub type Scuenhvio145R = crate::BitReader;
#[doc = "Field `SCUENHVIO145` writer - SCU_EN_HV_IO145"]
pub type Scuenhvio145W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO144"]
    #[inline(always)]
    pub fn scudispdio144(&self) -> Scudispdio144R {
        Scudispdio144R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO144"]
    #[inline(always)]
    pub fn scudispuio144(&self) -> Scudispuio144R {
        Scudispuio144R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO144"]
    #[inline(always)]
    pub fn scudrvio144(&self) -> Scudrvio144R {
        Scudrvio144R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO144"]
    #[inline(always)]
    pub fn scuensmtio144(&self) -> Scuensmtio144R {
        Scuensmtio144R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO144"]
    #[inline(always)]
    pub fn scuenhvio144(&self) -> Scuenhvio144R {
        Scuenhvio144R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO145"]
    #[inline(always)]
    pub fn scudispdio145(&self) -> Scudispdio145R {
        Scudispdio145R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO145"]
    #[inline(always)]
    pub fn scudispuio145(&self) -> Scudispuio145R {
        Scudispuio145R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO145"]
    #[inline(always)]
    pub fn scudrvio145(&self) -> Scudrvio145R {
        Scudrvio145R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO145"]
    #[inline(always)]
    pub fn scuensmtio145(&self) -> Scuensmtio145R {
        Scuensmtio145R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO145"]
    #[inline(always)]
    pub fn scuenhvio145(&self) -> Scuenhvio145R {
        Scuenhvio145R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO144"]
    #[inline(always)]
    pub fn scudispdio144(&mut self) -> Scudispdio144W<Scu5a0Spec> {
        Scudispdio144W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO144"]
    #[inline(always)]
    pub fn scudispuio144(&mut self) -> Scudispuio144W<Scu5a0Spec> {
        Scudispuio144W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO144"]
    #[inline(always)]
    pub fn scudrvio144(&mut self) -> Scudrvio144W<Scu5a0Spec> {
        Scudrvio144W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO144"]
    #[inline(always)]
    pub fn scuensmtio144(&mut self) -> Scuensmtio144W<Scu5a0Spec> {
        Scuensmtio144W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO144"]
    #[inline(always)]
    pub fn scuenhvio144(&mut self) -> Scuenhvio144W<Scu5a0Spec> {
        Scuenhvio144W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO145"]
    #[inline(always)]
    pub fn scudispdio145(&mut self) -> Scudispdio145W<Scu5a0Spec> {
        Scudispdio145W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO145"]
    #[inline(always)]
    pub fn scudispuio145(&mut self) -> Scudispuio145W<Scu5a0Spec> {
        Scudispuio145W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO145"]
    #[inline(always)]
    pub fn scudrvio145(&mut self) -> Scudrvio145W<Scu5a0Spec> {
        Scudrvio145W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO145"]
    #[inline(always)]
    pub fn scuensmtio145(&mut self) -> Scuensmtio145W<Scu5a0Spec> {
        Scuensmtio145W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO145"]
    #[inline(always)]
    pub fn scuenhvio145(&mut self) -> Scuenhvio145W<Scu5a0Spec> {
        Scuenhvio145W::new(self, 25)
    }
}
#[doc = "IO Control \\#73\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5a0Spec;
impl crate::RegisterSpec for Scu5a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5a0::R`](R) reader structure"]
impl crate::Readable for Scu5a0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5a0::W`](W) writer structure"]
impl crate::Writable for Scu5a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5A0 to value 0x0201_0201"]
impl crate::Resettable for Scu5a0Spec {
    const RESET_VALUE: u32 = 0x0201_0201;
}
