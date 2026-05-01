#[doc = "Register `SCU530` reader"]
pub type R = crate::R<Scu530Spec>;
#[doc = "Register `SCU530` writer"]
pub type W = crate::W<Scu530Spec>;
#[doc = "Field `SCUDISPDIO088` reader - SCU_DIS_PD_IO088"]
pub type Scudispdio088R = crate::BitReader;
#[doc = "Field `SCUDISPDIO088` writer - SCU_DIS_PD_IO088"]
pub type Scudispdio088W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO088` reader - SCU_DIS_PU_IO088"]
pub type Scudispuio088R = crate::BitReader;
#[doc = "Field `SCUDISPUIO088` writer - SCU_DIS_PU_IO088"]
pub type Scudispuio088W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO088` reader - SCU_DRV_IO088"]
pub type Scudrvio088R = crate::FieldReader;
#[doc = "Field `SCUDRVIO088` writer - SCU_DRV_IO088"]
pub type Scudrvio088W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO088` reader - SCU_EN_SMT_IO088"]
pub type Scuensmtio088R = crate::BitReader;
#[doc = "Field `SCUENSMTIO088` writer - SCU_EN_SMT_IO088"]
pub type Scuensmtio088W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO088` reader - SCU_EN_HV_IO088"]
pub type Scuenhvio088R = crate::BitReader;
#[doc = "Field `SCUENHVIO088` writer - SCU_EN_HV_IO088"]
pub type Scuenhvio088W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO089` reader - SCU_DIS_PD_IO089"]
pub type Scudispdio089R = crate::BitReader;
#[doc = "Field `SCUDISPDIO089` writer - SCU_DIS_PD_IO089"]
pub type Scudispdio089W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO089` reader - SCU_DIS_PU_IO089"]
pub type Scudispuio089R = crate::BitReader;
#[doc = "Field `SCUDISPUIO089` writer - SCU_DIS_PU_IO089"]
pub type Scudispuio089W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO089` reader - SCU_DRV_IO089"]
pub type Scudrvio089R = crate::FieldReader;
#[doc = "Field `SCUDRVIO089` writer - SCU_DRV_IO089"]
pub type Scudrvio089W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO089` reader - SCU_EN_SMT_IO089"]
pub type Scuensmtio089R = crate::BitReader;
#[doc = "Field `SCUENSMTIO089` writer - SCU_EN_SMT_IO089"]
pub type Scuensmtio089W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO089` reader - SCU_EN_HV_IO089"]
pub type Scuenhvio089R = crate::BitReader;
#[doc = "Field `SCUENHVIO089` writer - SCU_EN_HV_IO089"]
pub type Scuenhvio089W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO088"]
    #[inline(always)]
    pub fn scudispdio088(&self) -> Scudispdio088R {
        Scudispdio088R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO088"]
    #[inline(always)]
    pub fn scudispuio088(&self) -> Scudispuio088R {
        Scudispuio088R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO088"]
    #[inline(always)]
    pub fn scudrvio088(&self) -> Scudrvio088R {
        Scudrvio088R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO088"]
    #[inline(always)]
    pub fn scuensmtio088(&self) -> Scuensmtio088R {
        Scuensmtio088R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO088"]
    #[inline(always)]
    pub fn scuenhvio088(&self) -> Scuenhvio088R {
        Scuenhvio088R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO089"]
    #[inline(always)]
    pub fn scudispdio089(&self) -> Scudispdio089R {
        Scudispdio089R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO089"]
    #[inline(always)]
    pub fn scudispuio089(&self) -> Scudispuio089R {
        Scudispuio089R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO089"]
    #[inline(always)]
    pub fn scudrvio089(&self) -> Scudrvio089R {
        Scudrvio089R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO089"]
    #[inline(always)]
    pub fn scuensmtio089(&self) -> Scuensmtio089R {
        Scuensmtio089R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO089"]
    #[inline(always)]
    pub fn scuenhvio089(&self) -> Scuenhvio089R {
        Scuenhvio089R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO088"]
    #[inline(always)]
    pub fn scudispdio088(&mut self) -> Scudispdio088W<Scu530Spec> {
        Scudispdio088W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO088"]
    #[inline(always)]
    pub fn scudispuio088(&mut self) -> Scudispuio088W<Scu530Spec> {
        Scudispuio088W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO088"]
    #[inline(always)]
    pub fn scudrvio088(&mut self) -> Scudrvio088W<Scu530Spec> {
        Scudrvio088W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO088"]
    #[inline(always)]
    pub fn scuensmtio088(&mut self) -> Scuensmtio088W<Scu530Spec> {
        Scuensmtio088W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO088"]
    #[inline(always)]
    pub fn scuenhvio088(&mut self) -> Scuenhvio088W<Scu530Spec> {
        Scuenhvio088W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO089"]
    #[inline(always)]
    pub fn scudispdio089(&mut self) -> Scudispdio089W<Scu530Spec> {
        Scudispdio089W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO089"]
    #[inline(always)]
    pub fn scudispuio089(&mut self) -> Scudispuio089W<Scu530Spec> {
        Scudispuio089W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO089"]
    #[inline(always)]
    pub fn scudrvio089(&mut self) -> Scudrvio089W<Scu530Spec> {
        Scudrvio089W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO089"]
    #[inline(always)]
    pub fn scuensmtio089(&mut self) -> Scuensmtio089W<Scu530Spec> {
        Scuensmtio089W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO089"]
    #[inline(always)]
    pub fn scuenhvio089(&mut self) -> Scuenhvio089W<Scu530Spec> {
        Scuenhvio089W::new(self, 25)
    }
}
#[doc = "IO Control \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`scu530::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu530::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu530Spec;
impl crate::RegisterSpec for Scu530Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu530::R`](R) reader structure"]
impl crate::Readable for Scu530Spec {}
#[doc = "`write(|w| ..)` method takes [`scu530::W`](W) writer structure"]
impl crate::Writable for Scu530Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU530 to value 0x0204_0204"]
impl crate::Resettable for Scu530Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
