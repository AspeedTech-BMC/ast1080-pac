#[doc = "Register `SCU498` reader"]
pub type R = crate::R<Scu498Spec>;
#[doc = "Register `SCU498` writer"]
pub type W = crate::W<Scu498Spec>;
#[doc = "Field `SCUDISPDIO012` reader - SCU_DIS_PD_IO012"]
pub type Scudispdio012R = crate::BitReader;
#[doc = "Field `SCUDISPDIO012` writer - SCU_DIS_PD_IO012"]
pub type Scudispdio012W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO012` reader - SCU_DIS_PU_IO012"]
pub type Scudispuio012R = crate::BitReader;
#[doc = "Field `SCUDISPUIO012` writer - SCU_DIS_PU_IO012"]
pub type Scudispuio012W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO012` reader - SCU_DRV_IO012"]
pub type Scudrvio012R = crate::FieldReader;
#[doc = "Field `SCUDRVIO012` writer - SCU_DRV_IO012"]
pub type Scudrvio012W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO012` reader - SCU_EN_SMT_IO012"]
pub type Scuensmtio012R = crate::BitReader;
#[doc = "Field `SCUENSMTIO012` writer - SCU_EN_SMT_IO012"]
pub type Scuensmtio012W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO012` reader - SCU_EN_HV_IO012"]
pub type Scuenhvio012R = crate::BitReader;
#[doc = "Field `SCUENHVIO012` writer - SCU_EN_HV_IO012"]
pub type Scuenhvio012W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO013` reader - SCU_DIS_PD_IO013"]
pub type Scudispdio013R = crate::BitReader;
#[doc = "Field `SCUDISPDIO013` writer - SCU_DIS_PD_IO013"]
pub type Scudispdio013W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO013` reader - SCU_DIS_PU_IO013"]
pub type Scudispuio013R = crate::BitReader;
#[doc = "Field `SCUDISPUIO013` writer - SCU_DIS_PU_IO013"]
pub type Scudispuio013W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO013` reader - SCU_DRV_IO013"]
pub type Scudrvio013R = crate::FieldReader;
#[doc = "Field `SCUDRVIO013` writer - SCU_DRV_IO013"]
pub type Scudrvio013W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO013` reader - SCU_EN_SMT_IO013"]
pub type Scuensmtio013R = crate::BitReader;
#[doc = "Field `SCUENSMTIO013` writer - SCU_EN_SMT_IO013"]
pub type Scuensmtio013W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO013` reader - SCU_EN_HV_IO013"]
pub type Scuenhvio013R = crate::BitReader;
#[doc = "Field `SCUENHVIO013` writer - SCU_EN_HV_IO013"]
pub type Scuenhvio013W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO012"]
    #[inline(always)]
    pub fn scudispdio012(&self) -> Scudispdio012R {
        Scudispdio012R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO012"]
    #[inline(always)]
    pub fn scudispuio012(&self) -> Scudispuio012R {
        Scudispuio012R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO012"]
    #[inline(always)]
    pub fn scudrvio012(&self) -> Scudrvio012R {
        Scudrvio012R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO012"]
    #[inline(always)]
    pub fn scuensmtio012(&self) -> Scuensmtio012R {
        Scuensmtio012R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO012"]
    #[inline(always)]
    pub fn scuenhvio012(&self) -> Scuenhvio012R {
        Scuenhvio012R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO013"]
    #[inline(always)]
    pub fn scudispdio013(&self) -> Scudispdio013R {
        Scudispdio013R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO013"]
    #[inline(always)]
    pub fn scudispuio013(&self) -> Scudispuio013R {
        Scudispuio013R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO013"]
    #[inline(always)]
    pub fn scudrvio013(&self) -> Scudrvio013R {
        Scudrvio013R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO013"]
    #[inline(always)]
    pub fn scuensmtio013(&self) -> Scuensmtio013R {
        Scuensmtio013R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO013"]
    #[inline(always)]
    pub fn scuenhvio013(&self) -> Scuenhvio013R {
        Scuenhvio013R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO012"]
    #[inline(always)]
    pub fn scudispdio012(&mut self) -> Scudispdio012W<Scu498Spec> {
        Scudispdio012W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO012"]
    #[inline(always)]
    pub fn scudispuio012(&mut self) -> Scudispuio012W<Scu498Spec> {
        Scudispuio012W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO012"]
    #[inline(always)]
    pub fn scudrvio012(&mut self) -> Scudrvio012W<Scu498Spec> {
        Scudrvio012W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO012"]
    #[inline(always)]
    pub fn scuensmtio012(&mut self) -> Scuensmtio012W<Scu498Spec> {
        Scuensmtio012W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO012"]
    #[inline(always)]
    pub fn scuenhvio012(&mut self) -> Scuenhvio012W<Scu498Spec> {
        Scuenhvio012W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO013"]
    #[inline(always)]
    pub fn scudispdio013(&mut self) -> Scudispdio013W<Scu498Spec> {
        Scudispdio013W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO013"]
    #[inline(always)]
    pub fn scudispuio013(&mut self) -> Scudispuio013W<Scu498Spec> {
        Scudispuio013W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO013"]
    #[inline(always)]
    pub fn scudrvio013(&mut self) -> Scudrvio013W<Scu498Spec> {
        Scudrvio013W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO013"]
    #[inline(always)]
    pub fn scuensmtio013(&mut self) -> Scuensmtio013W<Scu498Spec> {
        Scuensmtio013W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO013"]
    #[inline(always)]
    pub fn scuenhvio013(&mut self) -> Scuenhvio013W<Scu498Spec> {
        Scuenhvio013W::new(self, 25)
    }
}
#[doc = "IO Control \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu498::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu498::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu498Spec;
impl crate::RegisterSpec for Scu498Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu498::R`](R) reader structure"]
impl crate::Readable for Scu498Spec {}
#[doc = "`write(|w| ..)` method takes [`scu498::W`](W) writer structure"]
impl crate::Writable for Scu498Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU498 to value 0x0204_0204"]
impl crate::Resettable for Scu498Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
