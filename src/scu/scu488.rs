#[doc = "Register `SCU488` reader"]
pub type R = crate::R<Scu488Spec>;
#[doc = "Register `SCU488` writer"]
pub type W = crate::W<Scu488Spec>;
#[doc = "Field `SCUDISPDIO004` reader - SCU_DIS_PD_IO004"]
pub type Scudispdio004R = crate::BitReader;
#[doc = "Field `SCUDISPDIO004` writer - SCU_DIS_PD_IO004"]
pub type Scudispdio004W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO004` reader - SCU_DIS_PU_IO004"]
pub type Scudispuio004R = crate::BitReader;
#[doc = "Field `SCUDISPUIO004` writer - SCU_DIS_PU_IO004"]
pub type Scudispuio004W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO004` reader - SCU_DRV_IO004"]
pub type Scudrvio004R = crate::FieldReader;
#[doc = "Field `SCUDRVIO004` writer - SCU_DRV_IO004"]
pub type Scudrvio004W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO004` reader - SCU_EN_SMT_IO004"]
pub type Scuensmtio004R = crate::BitReader;
#[doc = "Field `SCUENSMTIO004` writer - SCU_EN_SMT_IO004"]
pub type Scuensmtio004W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO004` reader - SCU_EN_HV_IO004"]
pub type Scuenhvio004R = crate::BitReader;
#[doc = "Field `SCUENHVIO004` writer - SCU_EN_HV_IO004"]
pub type Scuenhvio004W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO005` reader - SCU_DIS_PD_IO005"]
pub type Scudispdio005R = crate::BitReader;
#[doc = "Field `SCUDISPDIO005` writer - SCU_DIS_PD_IO005"]
pub type Scudispdio005W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO005` reader - SCU_DIS_PU_IO005"]
pub type Scudispuio005R = crate::BitReader;
#[doc = "Field `SCUDISPUIO005` writer - SCU_DIS_PU_IO005"]
pub type Scudispuio005W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO005` reader - SCU_DRV_IO005"]
pub type Scudrvio005R = crate::FieldReader;
#[doc = "Field `SCUDRVIO005` writer - SCU_DRV_IO005"]
pub type Scudrvio005W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO005` reader - SCU_EN_SMT_IO005"]
pub type Scuensmtio005R = crate::BitReader;
#[doc = "Field `SCUENSMTIO005` writer - SCU_EN_SMT_IO005"]
pub type Scuensmtio005W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO005` reader - SCU_EN_HV_IO005"]
pub type Scuenhvio005R = crate::BitReader;
#[doc = "Field `SCUENHVIO005` writer - SCU_EN_HV_IO005"]
pub type Scuenhvio005W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO004"]
    #[inline(always)]
    pub fn scudispdio004(&self) -> Scudispdio004R {
        Scudispdio004R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO004"]
    #[inline(always)]
    pub fn scudispuio004(&self) -> Scudispuio004R {
        Scudispuio004R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO004"]
    #[inline(always)]
    pub fn scudrvio004(&self) -> Scudrvio004R {
        Scudrvio004R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO004"]
    #[inline(always)]
    pub fn scuensmtio004(&self) -> Scuensmtio004R {
        Scuensmtio004R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO004"]
    #[inline(always)]
    pub fn scuenhvio004(&self) -> Scuenhvio004R {
        Scuenhvio004R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO005"]
    #[inline(always)]
    pub fn scudispdio005(&self) -> Scudispdio005R {
        Scudispdio005R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO005"]
    #[inline(always)]
    pub fn scudispuio005(&self) -> Scudispuio005R {
        Scudispuio005R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO005"]
    #[inline(always)]
    pub fn scudrvio005(&self) -> Scudrvio005R {
        Scudrvio005R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO005"]
    #[inline(always)]
    pub fn scuensmtio005(&self) -> Scuensmtio005R {
        Scuensmtio005R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO005"]
    #[inline(always)]
    pub fn scuenhvio005(&self) -> Scuenhvio005R {
        Scuenhvio005R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO004"]
    #[inline(always)]
    pub fn scudispdio004(&mut self) -> Scudispdio004W<Scu488Spec> {
        Scudispdio004W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO004"]
    #[inline(always)]
    pub fn scudispuio004(&mut self) -> Scudispuio004W<Scu488Spec> {
        Scudispuio004W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO004"]
    #[inline(always)]
    pub fn scudrvio004(&mut self) -> Scudrvio004W<Scu488Spec> {
        Scudrvio004W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO004"]
    #[inline(always)]
    pub fn scuensmtio004(&mut self) -> Scuensmtio004W<Scu488Spec> {
        Scuensmtio004W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO004"]
    #[inline(always)]
    pub fn scuenhvio004(&mut self) -> Scuenhvio004W<Scu488Spec> {
        Scuenhvio004W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO005"]
    #[inline(always)]
    pub fn scudispdio005(&mut self) -> Scudispdio005W<Scu488Spec> {
        Scudispdio005W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO005"]
    #[inline(always)]
    pub fn scudispuio005(&mut self) -> Scudispuio005W<Scu488Spec> {
        Scudispuio005W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO005"]
    #[inline(always)]
    pub fn scudrvio005(&mut self) -> Scudrvio005W<Scu488Spec> {
        Scudrvio005W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO005"]
    #[inline(always)]
    pub fn scuensmtio005(&mut self) -> Scuensmtio005W<Scu488Spec> {
        Scuensmtio005W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO005"]
    #[inline(always)]
    pub fn scuenhvio005(&mut self) -> Scuenhvio005W<Scu488Spec> {
        Scuenhvio005W::new(self, 25)
    }
}
#[doc = "IO Control \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu488::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu488::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu488Spec;
impl crate::RegisterSpec for Scu488Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu488::R`](R) reader structure"]
impl crate::Readable for Scu488Spec {}
#[doc = "`write(|w| ..)` method takes [`scu488::W`](W) writer structure"]
impl crate::Writable for Scu488Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU488 to value 0x0204_0204"]
impl crate::Resettable for Scu488Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
