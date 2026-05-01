#[doc = "Register `SCU578` reader"]
pub type R = crate::R<Scu578Spec>;
#[doc = "Register `SCU578` writer"]
pub type W = crate::W<Scu578Spec>;
#[doc = "Field `SCUDISPDIO124` reader - SCU_DIS_PD_IO124"]
pub type Scudispdio124R = crate::BitReader;
#[doc = "Field `SCUDISPDIO124` writer - SCU_DIS_PD_IO124"]
pub type Scudispdio124W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO124` reader - SCU_DIS_PU_IO124"]
pub type Scudispuio124R = crate::BitReader;
#[doc = "Field `SCUDISPUIO124` writer - SCU_DIS_PU_IO124"]
pub type Scudispuio124W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO124` reader - SCU_DRV_IO124"]
pub type Scudrvio124R = crate::FieldReader;
#[doc = "Field `SCUDRVIO124` writer - SCU_DRV_IO124"]
pub type Scudrvio124W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO124` reader - SCU_EN_SMT_IO124"]
pub type Scuensmtio124R = crate::BitReader;
#[doc = "Field `SCUENSMTIO124` writer - SCU_EN_SMT_IO124"]
pub type Scuensmtio124W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO124` reader - SCU_EN_HV_IO124"]
pub type Scuenhvio124R = crate::BitReader;
#[doc = "Field `SCUENHVIO124` writer - SCU_EN_HV_IO124"]
pub type Scuenhvio124W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO125` reader - SCU_DIS_PD_IO125"]
pub type Scudispdio125R = crate::BitReader;
#[doc = "Field `SCUDISPDIO125` writer - SCU_DIS_PD_IO125"]
pub type Scudispdio125W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO125` reader - SCU_DIS_PU_IO125"]
pub type Scudispuio125R = crate::BitReader;
#[doc = "Field `SCUDISPUIO125` writer - SCU_DIS_PU_IO125"]
pub type Scudispuio125W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO125` reader - SCU_DRV_IO125"]
pub type Scudrvio125R = crate::FieldReader;
#[doc = "Field `SCUDRVIO125` writer - SCU_DRV_IO125"]
pub type Scudrvio125W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO125` reader - SCU_EN_SMT_IO125"]
pub type Scuensmtio125R = crate::BitReader;
#[doc = "Field `SCUENSMTIO125` writer - SCU_EN_SMT_IO125"]
pub type Scuensmtio125W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO125` reader - SCU_EN_HV_IO125"]
pub type Scuenhvio125R = crate::BitReader;
#[doc = "Field `SCUENHVIO125` writer - SCU_EN_HV_IO125"]
pub type Scuenhvio125W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO124"]
    #[inline(always)]
    pub fn scudispdio124(&self) -> Scudispdio124R {
        Scudispdio124R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO124"]
    #[inline(always)]
    pub fn scudispuio124(&self) -> Scudispuio124R {
        Scudispuio124R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO124"]
    #[inline(always)]
    pub fn scudrvio124(&self) -> Scudrvio124R {
        Scudrvio124R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO124"]
    #[inline(always)]
    pub fn scuensmtio124(&self) -> Scuensmtio124R {
        Scuensmtio124R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO124"]
    #[inline(always)]
    pub fn scuenhvio124(&self) -> Scuenhvio124R {
        Scuenhvio124R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO125"]
    #[inline(always)]
    pub fn scudispdio125(&self) -> Scudispdio125R {
        Scudispdio125R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO125"]
    #[inline(always)]
    pub fn scudispuio125(&self) -> Scudispuio125R {
        Scudispuio125R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO125"]
    #[inline(always)]
    pub fn scudrvio125(&self) -> Scudrvio125R {
        Scudrvio125R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO125"]
    #[inline(always)]
    pub fn scuensmtio125(&self) -> Scuensmtio125R {
        Scuensmtio125R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO125"]
    #[inline(always)]
    pub fn scuenhvio125(&self) -> Scuenhvio125R {
        Scuenhvio125R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO124"]
    #[inline(always)]
    pub fn scudispdio124(&mut self) -> Scudispdio124W<Scu578Spec> {
        Scudispdio124W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO124"]
    #[inline(always)]
    pub fn scudispuio124(&mut self) -> Scudispuio124W<Scu578Spec> {
        Scudispuio124W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO124"]
    #[inline(always)]
    pub fn scudrvio124(&mut self) -> Scudrvio124W<Scu578Spec> {
        Scudrvio124W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO124"]
    #[inline(always)]
    pub fn scuensmtio124(&mut self) -> Scuensmtio124W<Scu578Spec> {
        Scuensmtio124W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO124"]
    #[inline(always)]
    pub fn scuenhvio124(&mut self) -> Scuenhvio124W<Scu578Spec> {
        Scuenhvio124W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO125"]
    #[inline(always)]
    pub fn scudispdio125(&mut self) -> Scudispdio125W<Scu578Spec> {
        Scudispdio125W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO125"]
    #[inline(always)]
    pub fn scudispuio125(&mut self) -> Scudispuio125W<Scu578Spec> {
        Scudispuio125W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO125"]
    #[inline(always)]
    pub fn scudrvio125(&mut self) -> Scudrvio125W<Scu578Spec> {
        Scudrvio125W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO125"]
    #[inline(always)]
    pub fn scuensmtio125(&mut self) -> Scuensmtio125W<Scu578Spec> {
        Scuensmtio125W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO125"]
    #[inline(always)]
    pub fn scuenhvio125(&mut self) -> Scuenhvio125W<Scu578Spec> {
        Scuenhvio125W::new(self, 25)
    }
}
#[doc = "IO Control \\#63\n\nYou can [`read`](crate::Reg::read) this register and get [`scu578::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu578::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu578Spec;
impl crate::RegisterSpec for Scu578Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu578::R`](R) reader structure"]
impl crate::Readable for Scu578Spec {}
#[doc = "`write(|w| ..)` method takes [`scu578::W`](W) writer structure"]
impl crate::Writable for Scu578Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU578 to value 0x0204_0204"]
impl crate::Resettable for Scu578Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
