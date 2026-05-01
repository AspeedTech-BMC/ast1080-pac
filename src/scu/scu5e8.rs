#[doc = "Register `SCU5E8` reader"]
pub type R = crate::R<Scu5e8Spec>;
#[doc = "Register `SCU5E8` writer"]
pub type W = crate::W<Scu5e8Spec>;
#[doc = "Field `SCUDISPDIO180` reader - SCU_DIS_PD_IO180"]
pub type Scudispdio180R = crate::BitReader;
#[doc = "Field `SCUDISPDIO180` writer - SCU_DIS_PD_IO180"]
pub type Scudispdio180W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO180` reader - SCU_DIS_PU_IO180"]
pub type Scudispuio180R = crate::BitReader;
#[doc = "Field `SCUDISPUIO180` writer - SCU_DIS_PU_IO180"]
pub type Scudispuio180W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO180` reader - SCU_DRV_IO180"]
pub type Scudrvio180R = crate::FieldReader;
#[doc = "Field `SCUDRVIO180` writer - SCU_DRV_IO180"]
pub type Scudrvio180W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO180` reader - SCU_EN_SMT_IO180"]
pub type Scuensmtio180R = crate::BitReader;
#[doc = "Field `SCUENSMTIO180` writer - SCU_EN_SMT_IO180"]
pub type Scuensmtio180W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO180` reader - SCU_EN_HV_IO180"]
pub type Scuenhvio180R = crate::BitReader;
#[doc = "Field `SCUENHVIO180` writer - SCU_EN_HV_IO180"]
pub type Scuenhvio180W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO181` reader - SCU_DIS_PD_IO181"]
pub type Scudispdio181R = crate::BitReader;
#[doc = "Field `SCUDISPDIO181` writer - SCU_DIS_PD_IO181"]
pub type Scudispdio181W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO181` reader - SCU_DIS_PU_IO181"]
pub type Scudispuio181R = crate::BitReader;
#[doc = "Field `SCUDISPUIO181` writer - SCU_DIS_PU_IO181"]
pub type Scudispuio181W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO181` reader - SCU_DRV_IO181"]
pub type Scudrvio181R = crate::FieldReader;
#[doc = "Field `SCUDRVIO181` writer - SCU_DRV_IO181"]
pub type Scudrvio181W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO181` reader - SCU_EN_SMT_IO181"]
pub type Scuensmtio181R = crate::BitReader;
#[doc = "Field `SCUENSMTIO181` writer - SCU_EN_SMT_IO181"]
pub type Scuensmtio181W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO181` reader - SCU_EN_HV_IO181"]
pub type Scuenhvio181R = crate::BitReader;
#[doc = "Field `SCUENHVIO181` writer - SCU_EN_HV_IO181"]
pub type Scuenhvio181W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO180"]
    #[inline(always)]
    pub fn scudispdio180(&self) -> Scudispdio180R {
        Scudispdio180R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO180"]
    #[inline(always)]
    pub fn scudispuio180(&self) -> Scudispuio180R {
        Scudispuio180R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO180"]
    #[inline(always)]
    pub fn scudrvio180(&self) -> Scudrvio180R {
        Scudrvio180R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO180"]
    #[inline(always)]
    pub fn scuensmtio180(&self) -> Scuensmtio180R {
        Scuensmtio180R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO180"]
    #[inline(always)]
    pub fn scuenhvio180(&self) -> Scuenhvio180R {
        Scuenhvio180R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO181"]
    #[inline(always)]
    pub fn scudispdio181(&self) -> Scudispdio181R {
        Scudispdio181R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO181"]
    #[inline(always)]
    pub fn scudispuio181(&self) -> Scudispuio181R {
        Scudispuio181R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO181"]
    #[inline(always)]
    pub fn scudrvio181(&self) -> Scudrvio181R {
        Scudrvio181R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO181"]
    #[inline(always)]
    pub fn scuensmtio181(&self) -> Scuensmtio181R {
        Scuensmtio181R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO181"]
    #[inline(always)]
    pub fn scuenhvio181(&self) -> Scuenhvio181R {
        Scuenhvio181R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO180"]
    #[inline(always)]
    pub fn scudispdio180(&mut self) -> Scudispdio180W<Scu5e8Spec> {
        Scudispdio180W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO180"]
    #[inline(always)]
    pub fn scudispuio180(&mut self) -> Scudispuio180W<Scu5e8Spec> {
        Scudispuio180W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO180"]
    #[inline(always)]
    pub fn scudrvio180(&mut self) -> Scudrvio180W<Scu5e8Spec> {
        Scudrvio180W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO180"]
    #[inline(always)]
    pub fn scuensmtio180(&mut self) -> Scuensmtio180W<Scu5e8Spec> {
        Scuensmtio180W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO180"]
    #[inline(always)]
    pub fn scuenhvio180(&mut self) -> Scuenhvio180W<Scu5e8Spec> {
        Scuenhvio180W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO181"]
    #[inline(always)]
    pub fn scudispdio181(&mut self) -> Scudispdio181W<Scu5e8Spec> {
        Scudispdio181W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO181"]
    #[inline(always)]
    pub fn scudispuio181(&mut self) -> Scudispuio181W<Scu5e8Spec> {
        Scudispuio181W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO181"]
    #[inline(always)]
    pub fn scudrvio181(&mut self) -> Scudrvio181W<Scu5e8Spec> {
        Scudrvio181W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO181"]
    #[inline(always)]
    pub fn scuensmtio181(&mut self) -> Scuensmtio181W<Scu5e8Spec> {
        Scuensmtio181W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO181"]
    #[inline(always)]
    pub fn scuenhvio181(&mut self) -> Scuenhvio181W<Scu5e8Spec> {
        Scuenhvio181W::new(self, 25)
    }
}
#[doc = "IO Control \\#91\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5e8Spec;
impl crate::RegisterSpec for Scu5e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5e8::R`](R) reader structure"]
impl crate::Readable for Scu5e8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5e8::W`](W) writer structure"]
impl crate::Writable for Scu5e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5E8 to value 0x0204_0204"]
impl crate::Resettable for Scu5e8Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
