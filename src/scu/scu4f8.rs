#[doc = "Register `SCU4F8` reader"]
pub type R = crate::R<Scu4f8Spec>;
#[doc = "Register `SCU4F8` writer"]
pub type W = crate::W<Scu4f8Spec>;
#[doc = "Field `SCUDISPDIO060` reader - SCU_DIS_PD_IO060"]
pub type Scudispdio060R = crate::BitReader;
#[doc = "Field `SCUDISPDIO060` writer - SCU_DIS_PD_IO060"]
pub type Scudispdio060W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO060` reader - SCU_DIS_PU_IO060"]
pub type Scudispuio060R = crate::BitReader;
#[doc = "Field `SCUDISPUIO060` writer - SCU_DIS_PU_IO060"]
pub type Scudispuio060W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO060` reader - SCU_DRV_IO060"]
pub type Scudrvio060R = crate::FieldReader;
#[doc = "Field `SCUDRVIO060` writer - SCU_DRV_IO060"]
pub type Scudrvio060W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO060` reader - SCU_EN_SMT_IO060"]
pub type Scuensmtio060R = crate::BitReader;
#[doc = "Field `SCUENSMTIO060` writer - SCU_EN_SMT_IO060"]
pub type Scuensmtio060W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO060` reader - SCU_EN_HV_IO060"]
pub type Scuenhvio060R = crate::BitReader;
#[doc = "Field `SCUENHVIO060` writer - SCU_EN_HV_IO060"]
pub type Scuenhvio060W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO061` reader - SCU_DIS_PD_IO061"]
pub type Scudispdio061R = crate::BitReader;
#[doc = "Field `SCUDISPDIO061` writer - SCU_DIS_PD_IO061"]
pub type Scudispdio061W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO061` reader - SCU_DIS_PU_IO061"]
pub type Scudispuio061R = crate::BitReader;
#[doc = "Field `SCUDISPUIO061` writer - SCU_DIS_PU_IO061"]
pub type Scudispuio061W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO061` reader - SCU_DRV_IO061"]
pub type Scudrvio061R = crate::FieldReader;
#[doc = "Field `SCUDRVIO061` writer - SCU_DRV_IO061"]
pub type Scudrvio061W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO061` reader - SCU_EN_SMT_IO061"]
pub type Scuensmtio061R = crate::BitReader;
#[doc = "Field `SCUENSMTIO061` writer - SCU_EN_SMT_IO061"]
pub type Scuensmtio061W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO061` reader - SCU_EN_HV_IO061"]
pub type Scuenhvio061R = crate::BitReader;
#[doc = "Field `SCUENHVIO061` writer - SCU_EN_HV_IO061"]
pub type Scuenhvio061W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO060"]
    #[inline(always)]
    pub fn scudispdio060(&self) -> Scudispdio060R {
        Scudispdio060R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO060"]
    #[inline(always)]
    pub fn scudispuio060(&self) -> Scudispuio060R {
        Scudispuio060R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO060"]
    #[inline(always)]
    pub fn scudrvio060(&self) -> Scudrvio060R {
        Scudrvio060R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO060"]
    #[inline(always)]
    pub fn scuensmtio060(&self) -> Scuensmtio060R {
        Scuensmtio060R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO060"]
    #[inline(always)]
    pub fn scuenhvio060(&self) -> Scuenhvio060R {
        Scuenhvio060R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO061"]
    #[inline(always)]
    pub fn scudispdio061(&self) -> Scudispdio061R {
        Scudispdio061R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO061"]
    #[inline(always)]
    pub fn scudispuio061(&self) -> Scudispuio061R {
        Scudispuio061R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO061"]
    #[inline(always)]
    pub fn scudrvio061(&self) -> Scudrvio061R {
        Scudrvio061R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO061"]
    #[inline(always)]
    pub fn scuensmtio061(&self) -> Scuensmtio061R {
        Scuensmtio061R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO061"]
    #[inline(always)]
    pub fn scuenhvio061(&self) -> Scuenhvio061R {
        Scuenhvio061R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO060"]
    #[inline(always)]
    pub fn scudispdio060(&mut self) -> Scudispdio060W<Scu4f8Spec> {
        Scudispdio060W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO060"]
    #[inline(always)]
    pub fn scudispuio060(&mut self) -> Scudispuio060W<Scu4f8Spec> {
        Scudispuio060W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO060"]
    #[inline(always)]
    pub fn scudrvio060(&mut self) -> Scudrvio060W<Scu4f8Spec> {
        Scudrvio060W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO060"]
    #[inline(always)]
    pub fn scuensmtio060(&mut self) -> Scuensmtio060W<Scu4f8Spec> {
        Scuensmtio060W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO060"]
    #[inline(always)]
    pub fn scuenhvio060(&mut self) -> Scuenhvio060W<Scu4f8Spec> {
        Scuenhvio060W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO061"]
    #[inline(always)]
    pub fn scudispdio061(&mut self) -> Scudispdio061W<Scu4f8Spec> {
        Scudispdio061W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO061"]
    #[inline(always)]
    pub fn scudispuio061(&mut self) -> Scudispuio061W<Scu4f8Spec> {
        Scudispuio061W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO061"]
    #[inline(always)]
    pub fn scudrvio061(&mut self) -> Scudrvio061W<Scu4f8Spec> {
        Scudrvio061W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO061"]
    #[inline(always)]
    pub fn scuensmtio061(&mut self) -> Scuensmtio061W<Scu4f8Spec> {
        Scuensmtio061W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO061"]
    #[inline(always)]
    pub fn scuenhvio061(&mut self) -> Scuenhvio061W<Scu4f8Spec> {
        Scuenhvio061W::new(self, 25)
    }
}
#[doc = "IO Control \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4f8Spec;
impl crate::RegisterSpec for Scu4f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4f8::R`](R) reader structure"]
impl crate::Readable for Scu4f8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4f8::W`](W) writer structure"]
impl crate::Writable for Scu4f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4F8 to value 0x0204_0204"]
impl crate::Resettable for Scu4f8Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
