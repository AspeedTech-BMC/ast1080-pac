#[doc = "Register `SCU550` reader"]
pub type R = crate::R<Scu550Spec>;
#[doc = "Register `SCU550` writer"]
pub type W = crate::W<Scu550Spec>;
#[doc = "Field `SCUDISPDIO104` reader - SCU_DIS_PD_IO104"]
pub type Scudispdio104R = crate::BitReader;
#[doc = "Field `SCUDISPDIO104` writer - SCU_DIS_PD_IO104"]
pub type Scudispdio104W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO104` reader - SCU_DIS_PU_IO104"]
pub type Scudispuio104R = crate::BitReader;
#[doc = "Field `SCUDISPUIO104` writer - SCU_DIS_PU_IO104"]
pub type Scudispuio104W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO104` reader - SCU_DRV_IO104"]
pub type Scudrvio104R = crate::FieldReader;
#[doc = "Field `SCUDRVIO104` writer - SCU_DRV_IO104"]
pub type Scudrvio104W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO104` reader - SCU_EN_SMT_IO104"]
pub type Scuensmtio104R = crate::BitReader;
#[doc = "Field `SCUENSMTIO104` writer - SCU_EN_SMT_IO104"]
pub type Scuensmtio104W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO104` reader - SCU_EN_HV_IO104"]
pub type Scuenhvio104R = crate::BitReader;
#[doc = "Field `SCUENHVIO104` writer - SCU_EN_HV_IO104"]
pub type Scuenhvio104W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO105` reader - SCU_DIS_PD_IO105"]
pub type Scudispdio105R = crate::BitReader;
#[doc = "Field `SCUDISPDIO105` writer - SCU_DIS_PD_IO105"]
pub type Scudispdio105W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO105` reader - SCU_DIS_PU_IO105"]
pub type Scudispuio105R = crate::BitReader;
#[doc = "Field `SCUDISPUIO105` writer - SCU_DIS_PU_IO105"]
pub type Scudispuio105W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO105` reader - SCU_DRV_IO105"]
pub type Scudrvio105R = crate::FieldReader;
#[doc = "Field `SCUDRVIO105` writer - SCU_DRV_IO105"]
pub type Scudrvio105W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO105` reader - SCU_EN_SMT_IO105"]
pub type Scuensmtio105R = crate::BitReader;
#[doc = "Field `SCUENSMTIO105` writer - SCU_EN_SMT_IO105"]
pub type Scuensmtio105W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO105` reader - SCU_EN_HV_IO105"]
pub type Scuenhvio105R = crate::BitReader;
#[doc = "Field `SCUENHVIO105` writer - SCU_EN_HV_IO105"]
pub type Scuenhvio105W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO104"]
    #[inline(always)]
    pub fn scudispdio104(&self) -> Scudispdio104R {
        Scudispdio104R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO104"]
    #[inline(always)]
    pub fn scudispuio104(&self) -> Scudispuio104R {
        Scudispuio104R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO104"]
    #[inline(always)]
    pub fn scudrvio104(&self) -> Scudrvio104R {
        Scudrvio104R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO104"]
    #[inline(always)]
    pub fn scuensmtio104(&self) -> Scuensmtio104R {
        Scuensmtio104R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO104"]
    #[inline(always)]
    pub fn scuenhvio104(&self) -> Scuenhvio104R {
        Scuenhvio104R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO105"]
    #[inline(always)]
    pub fn scudispdio105(&self) -> Scudispdio105R {
        Scudispdio105R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO105"]
    #[inline(always)]
    pub fn scudispuio105(&self) -> Scudispuio105R {
        Scudispuio105R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO105"]
    #[inline(always)]
    pub fn scudrvio105(&self) -> Scudrvio105R {
        Scudrvio105R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO105"]
    #[inline(always)]
    pub fn scuensmtio105(&self) -> Scuensmtio105R {
        Scuensmtio105R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO105"]
    #[inline(always)]
    pub fn scuenhvio105(&self) -> Scuenhvio105R {
        Scuenhvio105R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO104"]
    #[inline(always)]
    pub fn scudispdio104(&mut self) -> Scudispdio104W<Scu550Spec> {
        Scudispdio104W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO104"]
    #[inline(always)]
    pub fn scudispuio104(&mut self) -> Scudispuio104W<Scu550Spec> {
        Scudispuio104W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO104"]
    #[inline(always)]
    pub fn scudrvio104(&mut self) -> Scudrvio104W<Scu550Spec> {
        Scudrvio104W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO104"]
    #[inline(always)]
    pub fn scuensmtio104(&mut self) -> Scuensmtio104W<Scu550Spec> {
        Scuensmtio104W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO104"]
    #[inline(always)]
    pub fn scuenhvio104(&mut self) -> Scuenhvio104W<Scu550Spec> {
        Scuenhvio104W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO105"]
    #[inline(always)]
    pub fn scudispdio105(&mut self) -> Scudispdio105W<Scu550Spec> {
        Scudispdio105W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO105"]
    #[inline(always)]
    pub fn scudispuio105(&mut self) -> Scudispuio105W<Scu550Spec> {
        Scudispuio105W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO105"]
    #[inline(always)]
    pub fn scudrvio105(&mut self) -> Scudrvio105W<Scu550Spec> {
        Scudrvio105W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO105"]
    #[inline(always)]
    pub fn scuensmtio105(&mut self) -> Scuensmtio105W<Scu550Spec> {
        Scuensmtio105W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO105"]
    #[inline(always)]
    pub fn scuenhvio105(&mut self) -> Scuenhvio105W<Scu550Spec> {
        Scuenhvio105W::new(self, 25)
    }
}
#[doc = "IO Control \\#53\n\nYou can [`read`](crate::Reg::read) this register and get [`scu550::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu550::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu550Spec;
impl crate::RegisterSpec for Scu550Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu550::R`](R) reader structure"]
impl crate::Readable for Scu550Spec {}
#[doc = "`write(|w| ..)` method takes [`scu550::W`](W) writer structure"]
impl crate::Writable for Scu550Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU550 to value 0x0204_0204"]
impl crate::Resettable for Scu550Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
