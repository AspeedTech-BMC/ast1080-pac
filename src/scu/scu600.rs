#[doc = "Register `SCU600` reader"]
pub type R = crate::R<Scu600Spec>;
#[doc = "Register `SCU600` writer"]
pub type W = crate::W<Scu600Spec>;
#[doc = "Field `SCUDISPDIO192` reader - SCU_DIS_PD_IO192"]
pub type Scudispdio192R = crate::BitReader;
#[doc = "Field `SCUDISPDIO192` writer - SCU_DIS_PD_IO192"]
pub type Scudispdio192W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO192` reader - SCU_DIS_PU_IO192"]
pub type Scudispuio192R = crate::BitReader;
#[doc = "Field `SCUDISPUIO192` writer - SCU_DIS_PU_IO192"]
pub type Scudispuio192W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO192` reader - SCU_DRV_IO192"]
pub type Scudrvio192R = crate::FieldReader;
#[doc = "Field `SCUDRVIO192` writer - SCU_DRV_IO192"]
pub type Scudrvio192W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO192` reader - SCU_EN_SMT_IO192"]
pub type Scuensmtio192R = crate::BitReader;
#[doc = "Field `SCUENSMTIO192` writer - SCU_EN_SMT_IO192"]
pub type Scuensmtio192W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO192` reader - SCU_EN_HV_IO192"]
pub type Scuenhvio192R = crate::BitReader;
#[doc = "Field `SCUENHVIO192` writer - SCU_EN_HV_IO192"]
pub type Scuenhvio192W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO193` reader - SCU_DIS_PD_IO193"]
pub type Scudispdio193R = crate::BitReader;
#[doc = "Field `SCUDISPDIO193` writer - SCU_DIS_PD_IO193"]
pub type Scudispdio193W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO193` reader - SCU_DIS_PU_IO193"]
pub type Scudispuio193R = crate::BitReader;
#[doc = "Field `SCUDISPUIO193` writer - SCU_DIS_PU_IO193"]
pub type Scudispuio193W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO193` reader - SCU_DRV_IO193"]
pub type Scudrvio193R = crate::FieldReader;
#[doc = "Field `SCUDRVIO193` writer - SCU_DRV_IO193"]
pub type Scudrvio193W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO193` reader - SCU_EN_SMT_IO193"]
pub type Scuensmtio193R = crate::BitReader;
#[doc = "Field `SCUENSMTIO193` writer - SCU_EN_SMT_IO193"]
pub type Scuensmtio193W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO193` reader - SCU_EN_HV_IO193"]
pub type Scuenhvio193R = crate::BitReader;
#[doc = "Field `SCUENHVIO193` writer - SCU_EN_HV_IO193"]
pub type Scuenhvio193W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO192"]
    #[inline(always)]
    pub fn scudispdio192(&self) -> Scudispdio192R {
        Scudispdio192R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO192"]
    #[inline(always)]
    pub fn scudispuio192(&self) -> Scudispuio192R {
        Scudispuio192R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO192"]
    #[inline(always)]
    pub fn scudrvio192(&self) -> Scudrvio192R {
        Scudrvio192R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO192"]
    #[inline(always)]
    pub fn scuensmtio192(&self) -> Scuensmtio192R {
        Scuensmtio192R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO192"]
    #[inline(always)]
    pub fn scuenhvio192(&self) -> Scuenhvio192R {
        Scuenhvio192R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO193"]
    #[inline(always)]
    pub fn scudispdio193(&self) -> Scudispdio193R {
        Scudispdio193R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO193"]
    #[inline(always)]
    pub fn scudispuio193(&self) -> Scudispuio193R {
        Scudispuio193R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO193"]
    #[inline(always)]
    pub fn scudrvio193(&self) -> Scudrvio193R {
        Scudrvio193R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO193"]
    #[inline(always)]
    pub fn scuensmtio193(&self) -> Scuensmtio193R {
        Scuensmtio193R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO193"]
    #[inline(always)]
    pub fn scuenhvio193(&self) -> Scuenhvio193R {
        Scuenhvio193R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO192"]
    #[inline(always)]
    pub fn scudispdio192(&mut self) -> Scudispdio192W<Scu600Spec> {
        Scudispdio192W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO192"]
    #[inline(always)]
    pub fn scudispuio192(&mut self) -> Scudispuio192W<Scu600Spec> {
        Scudispuio192W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO192"]
    #[inline(always)]
    pub fn scudrvio192(&mut self) -> Scudrvio192W<Scu600Spec> {
        Scudrvio192W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO192"]
    #[inline(always)]
    pub fn scuensmtio192(&mut self) -> Scuensmtio192W<Scu600Spec> {
        Scuensmtio192W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO192"]
    #[inline(always)]
    pub fn scuenhvio192(&mut self) -> Scuenhvio192W<Scu600Spec> {
        Scuenhvio192W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO193"]
    #[inline(always)]
    pub fn scudispdio193(&mut self) -> Scudispdio193W<Scu600Spec> {
        Scudispdio193W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO193"]
    #[inline(always)]
    pub fn scudispuio193(&mut self) -> Scudispuio193W<Scu600Spec> {
        Scudispuio193W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO193"]
    #[inline(always)]
    pub fn scudrvio193(&mut self) -> Scudrvio193W<Scu600Spec> {
        Scudrvio193W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO193"]
    #[inline(always)]
    pub fn scuensmtio193(&mut self) -> Scuensmtio193W<Scu600Spec> {
        Scuensmtio193W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO193"]
    #[inline(always)]
    pub fn scuenhvio193(&mut self) -> Scuenhvio193W<Scu600Spec> {
        Scuenhvio193W::new(self, 25)
    }
}
#[doc = "IO Control \\#97\n\nYou can [`read`](crate::Reg::read) this register and get [`scu600::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu600::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu600Spec;
impl crate::RegisterSpec for Scu600Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu600::R`](R) reader structure"]
impl crate::Readable for Scu600Spec {}
#[doc = "`write(|w| ..)` method takes [`scu600::W`](W) writer structure"]
impl crate::Writable for Scu600Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU600 to value 0x0204_0201"]
impl crate::Resettable for Scu600Spec {
    const RESET_VALUE: u32 = 0x0204_0201;
}
