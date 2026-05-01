#[doc = "Register `SCU4A0` reader"]
pub type R = crate::R<Scu4a0Spec>;
#[doc = "Register `SCU4A0` writer"]
pub type W = crate::W<Scu4a0Spec>;
#[doc = "Field `SCUDISPDIO016` reader - SCU_DIS_PD_IO016"]
pub type Scudispdio016R = crate::BitReader;
#[doc = "Field `SCUDISPDIO016` writer - SCU_DIS_PD_IO016"]
pub type Scudispdio016W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO016` reader - SCU_DIS_PU_IO016"]
pub type Scudispuio016R = crate::BitReader;
#[doc = "Field `SCUDISPUIO016` writer - SCU_DIS_PU_IO016"]
pub type Scudispuio016W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO016` reader - SCU_DRV_IO016"]
pub type Scudrvio016R = crate::FieldReader;
#[doc = "Field `SCUDRVIO016` writer - SCU_DRV_IO016"]
pub type Scudrvio016W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO016` reader - SCU_EN_SMT_IO016"]
pub type Scuensmtio016R = crate::BitReader;
#[doc = "Field `SCUENSMTIO016` writer - SCU_EN_SMT_IO016"]
pub type Scuensmtio016W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO016` reader - SCU_EN_HV_IO016"]
pub type Scuenhvio016R = crate::BitReader;
#[doc = "Field `SCUENHVIO016` writer - SCU_EN_HV_IO016"]
pub type Scuenhvio016W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO017` reader - SCU_DIS_PD_IO017"]
pub type Scudispdio017R = crate::BitReader;
#[doc = "Field `SCUDISPDIO017` writer - SCU_DIS_PD_IO017"]
pub type Scudispdio017W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO017` reader - SCU_DIS_PU_IO017"]
pub type Scudispuio017R = crate::BitReader;
#[doc = "Field `SCUDISPUIO017` writer - SCU_DIS_PU_IO017"]
pub type Scudispuio017W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO017` reader - SCU_DRV_IO017"]
pub type Scudrvio017R = crate::FieldReader;
#[doc = "Field `SCUDRVIO017` writer - SCU_DRV_IO017"]
pub type Scudrvio017W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO017` reader - SCU_EN_SMT_IO017"]
pub type Scuensmtio017R = crate::BitReader;
#[doc = "Field `SCUENSMTIO017` writer - SCU_EN_SMT_IO017"]
pub type Scuensmtio017W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO017` reader - SCU_EN_HV_IO017"]
pub type Scuenhvio017R = crate::BitReader;
#[doc = "Field `SCUENHVIO017` writer - SCU_EN_HV_IO017"]
pub type Scuenhvio017W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO016"]
    #[inline(always)]
    pub fn scudispdio016(&self) -> Scudispdio016R {
        Scudispdio016R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO016"]
    #[inline(always)]
    pub fn scudispuio016(&self) -> Scudispuio016R {
        Scudispuio016R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO016"]
    #[inline(always)]
    pub fn scudrvio016(&self) -> Scudrvio016R {
        Scudrvio016R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO016"]
    #[inline(always)]
    pub fn scuensmtio016(&self) -> Scuensmtio016R {
        Scuensmtio016R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO016"]
    #[inline(always)]
    pub fn scuenhvio016(&self) -> Scuenhvio016R {
        Scuenhvio016R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO017"]
    #[inline(always)]
    pub fn scudispdio017(&self) -> Scudispdio017R {
        Scudispdio017R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO017"]
    #[inline(always)]
    pub fn scudispuio017(&self) -> Scudispuio017R {
        Scudispuio017R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO017"]
    #[inline(always)]
    pub fn scudrvio017(&self) -> Scudrvio017R {
        Scudrvio017R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO017"]
    #[inline(always)]
    pub fn scuensmtio017(&self) -> Scuensmtio017R {
        Scuensmtio017R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO017"]
    #[inline(always)]
    pub fn scuenhvio017(&self) -> Scuenhvio017R {
        Scuenhvio017R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO016"]
    #[inline(always)]
    pub fn scudispdio016(&mut self) -> Scudispdio016W<Scu4a0Spec> {
        Scudispdio016W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO016"]
    #[inline(always)]
    pub fn scudispuio016(&mut self) -> Scudispuio016W<Scu4a0Spec> {
        Scudispuio016W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO016"]
    #[inline(always)]
    pub fn scudrvio016(&mut self) -> Scudrvio016W<Scu4a0Spec> {
        Scudrvio016W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO016"]
    #[inline(always)]
    pub fn scuensmtio016(&mut self) -> Scuensmtio016W<Scu4a0Spec> {
        Scuensmtio016W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO016"]
    #[inline(always)]
    pub fn scuenhvio016(&mut self) -> Scuenhvio016W<Scu4a0Spec> {
        Scuenhvio016W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO017"]
    #[inline(always)]
    pub fn scudispdio017(&mut self) -> Scudispdio017W<Scu4a0Spec> {
        Scudispdio017W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO017"]
    #[inline(always)]
    pub fn scudispuio017(&mut self) -> Scudispuio017W<Scu4a0Spec> {
        Scudispuio017W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO017"]
    #[inline(always)]
    pub fn scudrvio017(&mut self) -> Scudrvio017W<Scu4a0Spec> {
        Scudrvio017W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO017"]
    #[inline(always)]
    pub fn scuensmtio017(&mut self) -> Scuensmtio017W<Scu4a0Spec> {
        Scuensmtio017W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO017"]
    #[inline(always)]
    pub fn scuenhvio017(&mut self) -> Scuenhvio017W<Scu4a0Spec> {
        Scuenhvio017W::new(self, 25)
    }
}
#[doc = "IO Control \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4a0Spec;
impl crate::RegisterSpec for Scu4a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4a0::R`](R) reader structure"]
impl crate::Readable for Scu4a0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4a0::W`](W) writer structure"]
impl crate::Writable for Scu4a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4A0 to value 0x0204_0204"]
impl crate::Resettable for Scu4a0Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
