#[doc = "Register `SCU568` reader"]
pub type R = crate::R<Scu568Spec>;
#[doc = "Register `SCU568` writer"]
pub type W = crate::W<Scu568Spec>;
#[doc = "Field `SCUDISPDIO116` reader - SCU_DIS_PD_IO116"]
pub type Scudispdio116R = crate::BitReader;
#[doc = "Field `SCUDISPDIO116` writer - SCU_DIS_PD_IO116"]
pub type Scudispdio116W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO116` reader - SCU_DIS_PU_IO116"]
pub type Scudispuio116R = crate::BitReader;
#[doc = "Field `SCUDISPUIO116` writer - SCU_DIS_PU_IO116"]
pub type Scudispuio116W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO116` reader - SCU_DRV_IO116"]
pub type Scudrvio116R = crate::FieldReader;
#[doc = "Field `SCUDRVIO116` writer - SCU_DRV_IO116"]
pub type Scudrvio116W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO116` reader - SCU_EN_SMT_IO116"]
pub type Scuensmtio116R = crate::BitReader;
#[doc = "Field `SCUENSMTIO116` writer - SCU_EN_SMT_IO116"]
pub type Scuensmtio116W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO116` reader - SCU_EN_HV_IO116"]
pub type Scuenhvio116R = crate::BitReader;
#[doc = "Field `SCUENHVIO116` writer - SCU_EN_HV_IO116"]
pub type Scuenhvio116W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO117` reader - SCU_DIS_PD_IO117"]
pub type Scudispdio117R = crate::BitReader;
#[doc = "Field `SCUDISPDIO117` writer - SCU_DIS_PD_IO117"]
pub type Scudispdio117W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO117` reader - SCU_DIS_PU_IO117"]
pub type Scudispuio117R = crate::BitReader;
#[doc = "Field `SCUDISPUIO117` writer - SCU_DIS_PU_IO117"]
pub type Scudispuio117W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO117` reader - SCU_DRV_IO117"]
pub type Scudrvio117R = crate::FieldReader;
#[doc = "Field `SCUDRVIO117` writer - SCU_DRV_IO117"]
pub type Scudrvio117W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO117` reader - SCU_EN_SMT_IO117"]
pub type Scuensmtio117R = crate::BitReader;
#[doc = "Field `SCUENSMTIO117` writer - SCU_EN_SMT_IO117"]
pub type Scuensmtio117W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO117` reader - SCU_EN_HV_IO117"]
pub type Scuenhvio117R = crate::BitReader;
#[doc = "Field `SCUENHVIO117` writer - SCU_EN_HV_IO117"]
pub type Scuenhvio117W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO116"]
    #[inline(always)]
    pub fn scudispdio116(&self) -> Scudispdio116R {
        Scudispdio116R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO116"]
    #[inline(always)]
    pub fn scudispuio116(&self) -> Scudispuio116R {
        Scudispuio116R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO116"]
    #[inline(always)]
    pub fn scudrvio116(&self) -> Scudrvio116R {
        Scudrvio116R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO116"]
    #[inline(always)]
    pub fn scuensmtio116(&self) -> Scuensmtio116R {
        Scuensmtio116R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO116"]
    #[inline(always)]
    pub fn scuenhvio116(&self) -> Scuenhvio116R {
        Scuenhvio116R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO117"]
    #[inline(always)]
    pub fn scudispdio117(&self) -> Scudispdio117R {
        Scudispdio117R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO117"]
    #[inline(always)]
    pub fn scudispuio117(&self) -> Scudispuio117R {
        Scudispuio117R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO117"]
    #[inline(always)]
    pub fn scudrvio117(&self) -> Scudrvio117R {
        Scudrvio117R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO117"]
    #[inline(always)]
    pub fn scuensmtio117(&self) -> Scuensmtio117R {
        Scuensmtio117R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO117"]
    #[inline(always)]
    pub fn scuenhvio117(&self) -> Scuenhvio117R {
        Scuenhvio117R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO116"]
    #[inline(always)]
    pub fn scudispdio116(&mut self) -> Scudispdio116W<Scu568Spec> {
        Scudispdio116W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO116"]
    #[inline(always)]
    pub fn scudispuio116(&mut self) -> Scudispuio116W<Scu568Spec> {
        Scudispuio116W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO116"]
    #[inline(always)]
    pub fn scudrvio116(&mut self) -> Scudrvio116W<Scu568Spec> {
        Scudrvio116W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO116"]
    #[inline(always)]
    pub fn scuensmtio116(&mut self) -> Scuensmtio116W<Scu568Spec> {
        Scuensmtio116W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO116"]
    #[inline(always)]
    pub fn scuenhvio116(&mut self) -> Scuenhvio116W<Scu568Spec> {
        Scuenhvio116W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO117"]
    #[inline(always)]
    pub fn scudispdio117(&mut self) -> Scudispdio117W<Scu568Spec> {
        Scudispdio117W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO117"]
    #[inline(always)]
    pub fn scudispuio117(&mut self) -> Scudispuio117W<Scu568Spec> {
        Scudispuio117W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO117"]
    #[inline(always)]
    pub fn scudrvio117(&mut self) -> Scudrvio117W<Scu568Spec> {
        Scudrvio117W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO117"]
    #[inline(always)]
    pub fn scuensmtio117(&mut self) -> Scuensmtio117W<Scu568Spec> {
        Scuensmtio117W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO117"]
    #[inline(always)]
    pub fn scuenhvio117(&mut self) -> Scuenhvio117W<Scu568Spec> {
        Scuenhvio117W::new(self, 25)
    }
}
#[doc = "IO Control \\#59\n\nYou can [`read`](crate::Reg::read) this register and get [`scu568::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu568::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu568Spec;
impl crate::RegisterSpec for Scu568Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu568::R`](R) reader structure"]
impl crate::Readable for Scu568Spec {}
#[doc = "`write(|w| ..)` method takes [`scu568::W`](W) writer structure"]
impl crate::Writable for Scu568Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU568 to value 0x0204_0204"]
impl crate::Resettable for Scu568Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
