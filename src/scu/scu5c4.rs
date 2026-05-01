#[doc = "Register `SCU5C4` reader"]
pub type R = crate::R<Scu5c4Spec>;
#[doc = "Register `SCU5C4` writer"]
pub type W = crate::W<Scu5c4Spec>;
#[doc = "Field `SCUDISPDIO162` reader - SCU_DIS_PD_IO162"]
pub type Scudispdio162R = crate::BitReader;
#[doc = "Field `SCUDISPDIO162` writer - SCU_DIS_PD_IO162"]
pub type Scudispdio162W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO162` reader - SCU_DIS_PU_IO162"]
pub type Scudispuio162R = crate::BitReader;
#[doc = "Field `SCUDISPUIO162` writer - SCU_DIS_PU_IO162"]
pub type Scudispuio162W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO162` reader - SCU_DRV_IO162"]
pub type Scudrvio162R = crate::FieldReader;
#[doc = "Field `SCUDRVIO162` writer - SCU_DRV_IO162"]
pub type Scudrvio162W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO162` reader - SCU_EN_SMT_IO162"]
pub type Scuensmtio162R = crate::BitReader;
#[doc = "Field `SCUENSMTIO162` writer - SCU_EN_SMT_IO162"]
pub type Scuensmtio162W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO162` reader - SCU_EN_HV_IO162"]
pub type Scuenhvio162R = crate::BitReader;
#[doc = "Field `SCUENHVIO162` writer - SCU_EN_HV_IO162"]
pub type Scuenhvio162W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO163` reader - SCU_DIS_PD_IO163"]
pub type Scudispdio163R = crate::BitReader;
#[doc = "Field `SCUDISPDIO163` writer - SCU_DIS_PD_IO163"]
pub type Scudispdio163W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO163` reader - SCU_DIS_PU_IO163"]
pub type Scudispuio163R = crate::BitReader;
#[doc = "Field `SCUDISPUIO163` writer - SCU_DIS_PU_IO163"]
pub type Scudispuio163W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO163` reader - SCU_DRV_IO163"]
pub type Scudrvio163R = crate::FieldReader;
#[doc = "Field `SCUDRVIO163` writer - SCU_DRV_IO163"]
pub type Scudrvio163W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO163` reader - SCU_EN_SMT_IO163"]
pub type Scuensmtio163R = crate::BitReader;
#[doc = "Field `SCUENSMTIO163` writer - SCU_EN_SMT_IO163"]
pub type Scuensmtio163W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO163` reader - SCU_EN_HV_IO163"]
pub type Scuenhvio163R = crate::BitReader;
#[doc = "Field `SCUENHVIO163` writer - SCU_EN_HV_IO163"]
pub type Scuenhvio163W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO162"]
    #[inline(always)]
    pub fn scudispdio162(&self) -> Scudispdio162R {
        Scudispdio162R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO162"]
    #[inline(always)]
    pub fn scudispuio162(&self) -> Scudispuio162R {
        Scudispuio162R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO162"]
    #[inline(always)]
    pub fn scudrvio162(&self) -> Scudrvio162R {
        Scudrvio162R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO162"]
    #[inline(always)]
    pub fn scuensmtio162(&self) -> Scuensmtio162R {
        Scuensmtio162R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO162"]
    #[inline(always)]
    pub fn scuenhvio162(&self) -> Scuenhvio162R {
        Scuenhvio162R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO163"]
    #[inline(always)]
    pub fn scudispdio163(&self) -> Scudispdio163R {
        Scudispdio163R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO163"]
    #[inline(always)]
    pub fn scudispuio163(&self) -> Scudispuio163R {
        Scudispuio163R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO163"]
    #[inline(always)]
    pub fn scudrvio163(&self) -> Scudrvio163R {
        Scudrvio163R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO163"]
    #[inline(always)]
    pub fn scuensmtio163(&self) -> Scuensmtio163R {
        Scuensmtio163R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO163"]
    #[inline(always)]
    pub fn scuenhvio163(&self) -> Scuenhvio163R {
        Scuenhvio163R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO162"]
    #[inline(always)]
    pub fn scudispdio162(&mut self) -> Scudispdio162W<Scu5c4Spec> {
        Scudispdio162W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO162"]
    #[inline(always)]
    pub fn scudispuio162(&mut self) -> Scudispuio162W<Scu5c4Spec> {
        Scudispuio162W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO162"]
    #[inline(always)]
    pub fn scudrvio162(&mut self) -> Scudrvio162W<Scu5c4Spec> {
        Scudrvio162W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO162"]
    #[inline(always)]
    pub fn scuensmtio162(&mut self) -> Scuensmtio162W<Scu5c4Spec> {
        Scuensmtio162W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO162"]
    #[inline(always)]
    pub fn scuenhvio162(&mut self) -> Scuenhvio162W<Scu5c4Spec> {
        Scuenhvio162W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO163"]
    #[inline(always)]
    pub fn scudispdio163(&mut self) -> Scudispdio163W<Scu5c4Spec> {
        Scudispdio163W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO163"]
    #[inline(always)]
    pub fn scudispuio163(&mut self) -> Scudispuio163W<Scu5c4Spec> {
        Scudispuio163W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO163"]
    #[inline(always)]
    pub fn scudrvio163(&mut self) -> Scudrvio163W<Scu5c4Spec> {
        Scudrvio163W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO163"]
    #[inline(always)]
    pub fn scuensmtio163(&mut self) -> Scuensmtio163W<Scu5c4Spec> {
        Scuensmtio163W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO163"]
    #[inline(always)]
    pub fn scuenhvio163(&mut self) -> Scuenhvio163W<Scu5c4Spec> {
        Scuenhvio163W::new(self, 25)
    }
}
#[doc = "IO Control \\#82\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5c4Spec;
impl crate::RegisterSpec for Scu5c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5c4::R`](R) reader structure"]
impl crate::Readable for Scu5c4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5c4::W`](W) writer structure"]
impl crate::Writable for Scu5c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5C4 to value 0x0201_0204"]
impl crate::Resettable for Scu5c4Spec {
    const RESET_VALUE: u32 = 0x0201_0204;
}
