#[doc = "Register `SCU520` reader"]
pub type R = crate::R<Scu520Spec>;
#[doc = "Register `SCU520` writer"]
pub type W = crate::W<Scu520Spec>;
#[doc = "Field `SCUDISPDIO080` reader - SCU_DIS_PD_IO080"]
pub type Scudispdio080R = crate::BitReader;
#[doc = "Field `SCUDISPDIO080` writer - SCU_DIS_PD_IO080"]
pub type Scudispdio080W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO080` reader - SCU_DIS_PU_IO080"]
pub type Scudispuio080R = crate::BitReader;
#[doc = "Field `SCUDISPUIO080` writer - SCU_DIS_PU_IO080"]
pub type Scudispuio080W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO080` reader - SCU_DRV_IO080"]
pub type Scudrvio080R = crate::FieldReader;
#[doc = "Field `SCUDRVIO080` writer - SCU_DRV_IO080"]
pub type Scudrvio080W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO080` reader - SCU_EN_SMT_IO080"]
pub type Scuensmtio080R = crate::BitReader;
#[doc = "Field `SCUENSMTIO080` writer - SCU_EN_SMT_IO080"]
pub type Scuensmtio080W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO080` reader - SCU_EN_HV_IO080"]
pub type Scuenhvio080R = crate::BitReader;
#[doc = "Field `SCUENHVIO080` writer - SCU_EN_HV_IO080"]
pub type Scuenhvio080W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO081` reader - SCU_DIS_PD_IO081"]
pub type Scudispdio081R = crate::BitReader;
#[doc = "Field `SCUDISPDIO081` writer - SCU_DIS_PD_IO081"]
pub type Scudispdio081W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO081` reader - SCU_DIS_PU_IO081"]
pub type Scudispuio081R = crate::BitReader;
#[doc = "Field `SCUDISPUIO081` writer - SCU_DIS_PU_IO081"]
pub type Scudispuio081W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO081` reader - SCU_DRV_IO081"]
pub type Scudrvio081R = crate::FieldReader;
#[doc = "Field `SCUDRVIO081` writer - SCU_DRV_IO081"]
pub type Scudrvio081W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO081` reader - SCU_EN_SMT_IO081"]
pub type Scuensmtio081R = crate::BitReader;
#[doc = "Field `SCUENSMTIO081` writer - SCU_EN_SMT_IO081"]
pub type Scuensmtio081W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO081` reader - SCU_EN_HV_IO081"]
pub type Scuenhvio081R = crate::BitReader;
#[doc = "Field `SCUENHVIO081` writer - SCU_EN_HV_IO081"]
pub type Scuenhvio081W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO080"]
    #[inline(always)]
    pub fn scudispdio080(&self) -> Scudispdio080R {
        Scudispdio080R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO080"]
    #[inline(always)]
    pub fn scudispuio080(&self) -> Scudispuio080R {
        Scudispuio080R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO080"]
    #[inline(always)]
    pub fn scudrvio080(&self) -> Scudrvio080R {
        Scudrvio080R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO080"]
    #[inline(always)]
    pub fn scuensmtio080(&self) -> Scuensmtio080R {
        Scuensmtio080R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO080"]
    #[inline(always)]
    pub fn scuenhvio080(&self) -> Scuenhvio080R {
        Scuenhvio080R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO081"]
    #[inline(always)]
    pub fn scudispdio081(&self) -> Scudispdio081R {
        Scudispdio081R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO081"]
    #[inline(always)]
    pub fn scudispuio081(&self) -> Scudispuio081R {
        Scudispuio081R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO081"]
    #[inline(always)]
    pub fn scudrvio081(&self) -> Scudrvio081R {
        Scudrvio081R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO081"]
    #[inline(always)]
    pub fn scuensmtio081(&self) -> Scuensmtio081R {
        Scuensmtio081R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO081"]
    #[inline(always)]
    pub fn scuenhvio081(&self) -> Scuenhvio081R {
        Scuenhvio081R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO080"]
    #[inline(always)]
    pub fn scudispdio080(&mut self) -> Scudispdio080W<Scu520Spec> {
        Scudispdio080W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO080"]
    #[inline(always)]
    pub fn scudispuio080(&mut self) -> Scudispuio080W<Scu520Spec> {
        Scudispuio080W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO080"]
    #[inline(always)]
    pub fn scudrvio080(&mut self) -> Scudrvio080W<Scu520Spec> {
        Scudrvio080W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO080"]
    #[inline(always)]
    pub fn scuensmtio080(&mut self) -> Scuensmtio080W<Scu520Spec> {
        Scuensmtio080W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO080"]
    #[inline(always)]
    pub fn scuenhvio080(&mut self) -> Scuenhvio080W<Scu520Spec> {
        Scuenhvio080W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO081"]
    #[inline(always)]
    pub fn scudispdio081(&mut self) -> Scudispdio081W<Scu520Spec> {
        Scudispdio081W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO081"]
    #[inline(always)]
    pub fn scudispuio081(&mut self) -> Scudispuio081W<Scu520Spec> {
        Scudispuio081W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO081"]
    #[inline(always)]
    pub fn scudrvio081(&mut self) -> Scudrvio081W<Scu520Spec> {
        Scudrvio081W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO081"]
    #[inline(always)]
    pub fn scuensmtio081(&mut self) -> Scuensmtio081W<Scu520Spec> {
        Scuensmtio081W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO081"]
    #[inline(always)]
    pub fn scuenhvio081(&mut self) -> Scuenhvio081W<Scu520Spec> {
        Scuenhvio081W::new(self, 25)
    }
}
#[doc = "IO Control \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`scu520::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu520::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu520Spec;
impl crate::RegisterSpec for Scu520Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu520::R`](R) reader structure"]
impl crate::Readable for Scu520Spec {}
#[doc = "`write(|w| ..)` method takes [`scu520::W`](W) writer structure"]
impl crate::Writable for Scu520Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU520 to value 0x0204_0204"]
impl crate::Resettable for Scu520Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
