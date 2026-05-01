#[doc = "Register `SCU5F4` reader"]
pub type R = crate::R<Scu5f4Spec>;
#[doc = "Register `SCU5F4` writer"]
pub type W = crate::W<Scu5f4Spec>;
#[doc = "Field `SCUDISPDIO186` reader - SCU_DIS_PD_IO186"]
pub type Scudispdio186R = crate::BitReader;
#[doc = "Field `SCUDISPDIO186` writer - SCU_DIS_PD_IO186"]
pub type Scudispdio186W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO186` reader - SCU_DIS_PU_IO186"]
pub type Scudispuio186R = crate::BitReader;
#[doc = "Field `SCUDISPUIO186` writer - SCU_DIS_PU_IO186"]
pub type Scudispuio186W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO186` reader - SCU_DRV_IO186"]
pub type Scudrvio186R = crate::FieldReader;
#[doc = "Field `SCUDRVIO186` writer - SCU_DRV_IO186"]
pub type Scudrvio186W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO186` reader - SCU_EN_SMT_IO186"]
pub type Scuensmtio186R = crate::BitReader;
#[doc = "Field `SCUENSMTIO186` writer - SCU_EN_SMT_IO186"]
pub type Scuensmtio186W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO186` reader - SCU_EN_HV_IO186"]
pub type Scuenhvio186R = crate::BitReader;
#[doc = "Field `SCUENHVIO186` writer - SCU_EN_HV_IO186"]
pub type Scuenhvio186W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO187` reader - SCU_DIS_PD_IO187"]
pub type Scudispdio187R = crate::BitReader;
#[doc = "Field `SCUDISPDIO187` writer - SCU_DIS_PD_IO187"]
pub type Scudispdio187W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO187` reader - SCU_DIS_PU_IO187"]
pub type Scudispuio187R = crate::BitReader;
#[doc = "Field `SCUDISPUIO187` writer - SCU_DIS_PU_IO187"]
pub type Scudispuio187W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO187` reader - SCU_DRV_IO187"]
pub type Scudrvio187R = crate::FieldReader;
#[doc = "Field `SCUDRVIO187` writer - SCU_DRV_IO187"]
pub type Scudrvio187W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO187` reader - SCU_EN_SMT_IO187"]
pub type Scuensmtio187R = crate::BitReader;
#[doc = "Field `SCUENSMTIO187` writer - SCU_EN_SMT_IO187"]
pub type Scuensmtio187W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO187` reader - SCU_EN_HV_IO187"]
pub type Scuenhvio187R = crate::BitReader;
#[doc = "Field `SCUENHVIO187` writer - SCU_EN_HV_IO187"]
pub type Scuenhvio187W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO186"]
    #[inline(always)]
    pub fn scudispdio186(&self) -> Scudispdio186R {
        Scudispdio186R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO186"]
    #[inline(always)]
    pub fn scudispuio186(&self) -> Scudispuio186R {
        Scudispuio186R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO186"]
    #[inline(always)]
    pub fn scudrvio186(&self) -> Scudrvio186R {
        Scudrvio186R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO186"]
    #[inline(always)]
    pub fn scuensmtio186(&self) -> Scuensmtio186R {
        Scuensmtio186R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO186"]
    #[inline(always)]
    pub fn scuenhvio186(&self) -> Scuenhvio186R {
        Scuenhvio186R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO187"]
    #[inline(always)]
    pub fn scudispdio187(&self) -> Scudispdio187R {
        Scudispdio187R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO187"]
    #[inline(always)]
    pub fn scudispuio187(&self) -> Scudispuio187R {
        Scudispuio187R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO187"]
    #[inline(always)]
    pub fn scudrvio187(&self) -> Scudrvio187R {
        Scudrvio187R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO187"]
    #[inline(always)]
    pub fn scuensmtio187(&self) -> Scuensmtio187R {
        Scuensmtio187R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO187"]
    #[inline(always)]
    pub fn scuenhvio187(&self) -> Scuenhvio187R {
        Scuenhvio187R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO186"]
    #[inline(always)]
    pub fn scudispdio186(&mut self) -> Scudispdio186W<Scu5f4Spec> {
        Scudispdio186W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO186"]
    #[inline(always)]
    pub fn scudispuio186(&mut self) -> Scudispuio186W<Scu5f4Spec> {
        Scudispuio186W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO186"]
    #[inline(always)]
    pub fn scudrvio186(&mut self) -> Scudrvio186W<Scu5f4Spec> {
        Scudrvio186W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO186"]
    #[inline(always)]
    pub fn scuensmtio186(&mut self) -> Scuensmtio186W<Scu5f4Spec> {
        Scuensmtio186W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO186"]
    #[inline(always)]
    pub fn scuenhvio186(&mut self) -> Scuenhvio186W<Scu5f4Spec> {
        Scuenhvio186W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO187"]
    #[inline(always)]
    pub fn scudispdio187(&mut self) -> Scudispdio187W<Scu5f4Spec> {
        Scudispdio187W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO187"]
    #[inline(always)]
    pub fn scudispuio187(&mut self) -> Scudispuio187W<Scu5f4Spec> {
        Scudispuio187W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO187"]
    #[inline(always)]
    pub fn scudrvio187(&mut self) -> Scudrvio187W<Scu5f4Spec> {
        Scudrvio187W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO187"]
    #[inline(always)]
    pub fn scuensmtio187(&mut self) -> Scuensmtio187W<Scu5f4Spec> {
        Scuensmtio187W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO187"]
    #[inline(always)]
    pub fn scuenhvio187(&mut self) -> Scuenhvio187W<Scu5f4Spec> {
        Scuenhvio187W::new(self, 25)
    }
}
#[doc = "IO Control \\#94\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5f4Spec;
impl crate::RegisterSpec for Scu5f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5f4::R`](R) reader structure"]
impl crate::Readable for Scu5f4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5f4::W`](W) writer structure"]
impl crate::Writable for Scu5f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5F4 to value 0x0201_0204"]
impl crate::Resettable for Scu5f4Spec {
    const RESET_VALUE: u32 = 0x0201_0204;
}
