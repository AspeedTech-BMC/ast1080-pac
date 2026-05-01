#[doc = "Register `SCU484` reader"]
pub type R = crate::R<Scu484Spec>;
#[doc = "Register `SCU484` writer"]
pub type W = crate::W<Scu484Spec>;
#[doc = "Field `SCUDISPDIO002` reader - SCU_DIS_PD_IO002"]
pub type Scudispdio002R = crate::BitReader;
#[doc = "Field `SCUDISPDIO002` writer - SCU_DIS_PD_IO002"]
pub type Scudispdio002W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO002` reader - SCU_DIS_PU_IO002"]
pub type Scudispuio002R = crate::BitReader;
#[doc = "Field `SCUDISPUIO002` writer - SCU_DIS_PU_IO002"]
pub type Scudispuio002W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO002` reader - SCU_DRV_IO002"]
pub type Scudrvio002R = crate::FieldReader;
#[doc = "Field `SCUDRVIO002` writer - SCU_DRV_IO002"]
pub type Scudrvio002W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO002` reader - SCU_EN_SMT_IO002"]
pub type Scuensmtio002R = crate::BitReader;
#[doc = "Field `SCUENSMTIO002` writer - SCU_EN_SMT_IO002"]
pub type Scuensmtio002W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO002` reader - SCU_EN_HV_IO002"]
pub type Scuenhvio002R = crate::BitReader;
#[doc = "Field `SCUENHVIO002` writer - SCU_EN_HV_IO002"]
pub type Scuenhvio002W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO003` reader - SCU_DIS_PD_IO003"]
pub type Scudispdio003R = crate::BitReader;
#[doc = "Field `SCUDISPDIO003` writer - SCU_DIS_PD_IO003"]
pub type Scudispdio003W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO003` reader - SCU_DIS_PU_IO003"]
pub type Scudispuio003R = crate::BitReader;
#[doc = "Field `SCUDISPUIO003` writer - SCU_DIS_PU_IO003"]
pub type Scudispuio003W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO003` reader - SCU_DRV_IO003"]
pub type Scudrvio003R = crate::FieldReader;
#[doc = "Field `SCUDRVIO003` writer - SCU_DRV_IO003"]
pub type Scudrvio003W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO003` reader - SCU_EN_SMT_IO003"]
pub type Scuensmtio003R = crate::BitReader;
#[doc = "Field `SCUENSMTIO003` writer - SCU_EN_SMT_IO003"]
pub type Scuensmtio003W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO003` reader - SCU_EN_HV_IO003"]
pub type Scuenhvio003R = crate::BitReader;
#[doc = "Field `SCUENHVIO003` writer - SCU_EN_HV_IO003"]
pub type Scuenhvio003W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO002"]
    #[inline(always)]
    pub fn scudispdio002(&self) -> Scudispdio002R {
        Scudispdio002R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO002"]
    #[inline(always)]
    pub fn scudispuio002(&self) -> Scudispuio002R {
        Scudispuio002R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO002"]
    #[inline(always)]
    pub fn scudrvio002(&self) -> Scudrvio002R {
        Scudrvio002R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO002"]
    #[inline(always)]
    pub fn scuensmtio002(&self) -> Scuensmtio002R {
        Scuensmtio002R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO002"]
    #[inline(always)]
    pub fn scuenhvio002(&self) -> Scuenhvio002R {
        Scuenhvio002R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO003"]
    #[inline(always)]
    pub fn scudispdio003(&self) -> Scudispdio003R {
        Scudispdio003R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO003"]
    #[inline(always)]
    pub fn scudispuio003(&self) -> Scudispuio003R {
        Scudispuio003R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO003"]
    #[inline(always)]
    pub fn scudrvio003(&self) -> Scudrvio003R {
        Scudrvio003R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO003"]
    #[inline(always)]
    pub fn scuensmtio003(&self) -> Scuensmtio003R {
        Scuensmtio003R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO003"]
    #[inline(always)]
    pub fn scuenhvio003(&self) -> Scuenhvio003R {
        Scuenhvio003R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO002"]
    #[inline(always)]
    pub fn scudispdio002(&mut self) -> Scudispdio002W<Scu484Spec> {
        Scudispdio002W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO002"]
    #[inline(always)]
    pub fn scudispuio002(&mut self) -> Scudispuio002W<Scu484Spec> {
        Scudispuio002W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO002"]
    #[inline(always)]
    pub fn scudrvio002(&mut self) -> Scudrvio002W<Scu484Spec> {
        Scudrvio002W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO002"]
    #[inline(always)]
    pub fn scuensmtio002(&mut self) -> Scuensmtio002W<Scu484Spec> {
        Scuensmtio002W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO002"]
    #[inline(always)]
    pub fn scuenhvio002(&mut self) -> Scuenhvio002W<Scu484Spec> {
        Scuenhvio002W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO003"]
    #[inline(always)]
    pub fn scudispdio003(&mut self) -> Scudispdio003W<Scu484Spec> {
        Scudispdio003W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO003"]
    #[inline(always)]
    pub fn scudispuio003(&mut self) -> Scudispuio003W<Scu484Spec> {
        Scudispuio003W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO003"]
    #[inline(always)]
    pub fn scudrvio003(&mut self) -> Scudrvio003W<Scu484Spec> {
        Scudrvio003W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO003"]
    #[inline(always)]
    pub fn scuensmtio003(&mut self) -> Scuensmtio003W<Scu484Spec> {
        Scuensmtio003W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO003"]
    #[inline(always)]
    pub fn scuenhvio003(&mut self) -> Scuenhvio003W<Scu484Spec> {
        Scuenhvio003W::new(self, 25)
    }
}
#[doc = "IO Control \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu484::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu484::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu484Spec;
impl crate::RegisterSpec for Scu484Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu484::R`](R) reader structure"]
impl crate::Readable for Scu484Spec {}
#[doc = "`write(|w| ..)` method takes [`scu484::W`](W) writer structure"]
impl crate::Writable for Scu484Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU484 to value 0x0204_0204"]
impl crate::Resettable for Scu484Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
