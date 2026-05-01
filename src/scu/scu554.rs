#[doc = "Register `SCU554` reader"]
pub type R = crate::R<Scu554Spec>;
#[doc = "Register `SCU554` writer"]
pub type W = crate::W<Scu554Spec>;
#[doc = "Field `SCUDISPDIO106` reader - SCU_DIS_PD_IO106"]
pub type Scudispdio106R = crate::BitReader;
#[doc = "Field `SCUDISPDIO106` writer - SCU_DIS_PD_IO106"]
pub type Scudispdio106W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO106` reader - SCU_DIS_PU_IO106"]
pub type Scudispuio106R = crate::BitReader;
#[doc = "Field `SCUDISPUIO106` writer - SCU_DIS_PU_IO106"]
pub type Scudispuio106W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO106` reader - SCU_DRV_IO106"]
pub type Scudrvio106R = crate::FieldReader;
#[doc = "Field `SCUDRVIO106` writer - SCU_DRV_IO106"]
pub type Scudrvio106W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO106` reader - SCU_EN_SMT_IO106"]
pub type Scuensmtio106R = crate::BitReader;
#[doc = "Field `SCUENSMTIO106` writer - SCU_EN_SMT_IO106"]
pub type Scuensmtio106W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO106` reader - SCU_EN_HV_IO106"]
pub type Scuenhvio106R = crate::BitReader;
#[doc = "Field `SCUENHVIO106` writer - SCU_EN_HV_IO106"]
pub type Scuenhvio106W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO107` reader - SCU_DIS_PD_IO107"]
pub type Scudispdio107R = crate::BitReader;
#[doc = "Field `SCUDISPDIO107` writer - SCU_DIS_PD_IO107"]
pub type Scudispdio107W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO107` reader - SCU_DIS_PU_IO107"]
pub type Scudispuio107R = crate::BitReader;
#[doc = "Field `SCUDISPUIO107` writer - SCU_DIS_PU_IO107"]
pub type Scudispuio107W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO107` reader - SCU_DRV_IO107"]
pub type Scudrvio107R = crate::FieldReader;
#[doc = "Field `SCUDRVIO107` writer - SCU_DRV_IO107"]
pub type Scudrvio107W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO107` reader - SCU_EN_SMT_IO107"]
pub type Scuensmtio107R = crate::BitReader;
#[doc = "Field `SCUENSMTIO107` writer - SCU_EN_SMT_IO107"]
pub type Scuensmtio107W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO107` reader - SCU_EN_HV_IO107"]
pub type Scuenhvio107R = crate::BitReader;
#[doc = "Field `SCUENHVIO107` writer - SCU_EN_HV_IO107"]
pub type Scuenhvio107W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO106"]
    #[inline(always)]
    pub fn scudispdio106(&self) -> Scudispdio106R {
        Scudispdio106R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO106"]
    #[inline(always)]
    pub fn scudispuio106(&self) -> Scudispuio106R {
        Scudispuio106R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO106"]
    #[inline(always)]
    pub fn scudrvio106(&self) -> Scudrvio106R {
        Scudrvio106R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO106"]
    #[inline(always)]
    pub fn scuensmtio106(&self) -> Scuensmtio106R {
        Scuensmtio106R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO106"]
    #[inline(always)]
    pub fn scuenhvio106(&self) -> Scuenhvio106R {
        Scuenhvio106R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO107"]
    #[inline(always)]
    pub fn scudispdio107(&self) -> Scudispdio107R {
        Scudispdio107R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO107"]
    #[inline(always)]
    pub fn scudispuio107(&self) -> Scudispuio107R {
        Scudispuio107R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO107"]
    #[inline(always)]
    pub fn scudrvio107(&self) -> Scudrvio107R {
        Scudrvio107R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO107"]
    #[inline(always)]
    pub fn scuensmtio107(&self) -> Scuensmtio107R {
        Scuensmtio107R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO107"]
    #[inline(always)]
    pub fn scuenhvio107(&self) -> Scuenhvio107R {
        Scuenhvio107R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO106"]
    #[inline(always)]
    pub fn scudispdio106(&mut self) -> Scudispdio106W<Scu554Spec> {
        Scudispdio106W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO106"]
    #[inline(always)]
    pub fn scudispuio106(&mut self) -> Scudispuio106W<Scu554Spec> {
        Scudispuio106W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO106"]
    #[inline(always)]
    pub fn scudrvio106(&mut self) -> Scudrvio106W<Scu554Spec> {
        Scudrvio106W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO106"]
    #[inline(always)]
    pub fn scuensmtio106(&mut self) -> Scuensmtio106W<Scu554Spec> {
        Scuensmtio106W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO106"]
    #[inline(always)]
    pub fn scuenhvio106(&mut self) -> Scuenhvio106W<Scu554Spec> {
        Scuenhvio106W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO107"]
    #[inline(always)]
    pub fn scudispdio107(&mut self) -> Scudispdio107W<Scu554Spec> {
        Scudispdio107W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO107"]
    #[inline(always)]
    pub fn scudispuio107(&mut self) -> Scudispuio107W<Scu554Spec> {
        Scudispuio107W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO107"]
    #[inline(always)]
    pub fn scudrvio107(&mut self) -> Scudrvio107W<Scu554Spec> {
        Scudrvio107W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO107"]
    #[inline(always)]
    pub fn scuensmtio107(&mut self) -> Scuensmtio107W<Scu554Spec> {
        Scuensmtio107W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO107"]
    #[inline(always)]
    pub fn scuenhvio107(&mut self) -> Scuenhvio107W<Scu554Spec> {
        Scuenhvio107W::new(self, 25)
    }
}
#[doc = "IO Control \\#54\n\nYou can [`read`](crate::Reg::read) this register and get [`scu554::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu554::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu554Spec;
impl crate::RegisterSpec for Scu554Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu554::R`](R) reader structure"]
impl crate::Readable for Scu554Spec {}
#[doc = "`write(|w| ..)` method takes [`scu554::W`](W) writer structure"]
impl crate::Writable for Scu554Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU554 to value 0x0204_0204"]
impl crate::Resettable for Scu554Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
