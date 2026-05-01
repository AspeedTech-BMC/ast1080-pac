#[doc = "Register `SCU5F0` reader"]
pub type R = crate::R<Scu5f0Spec>;
#[doc = "Register `SCU5F0` writer"]
pub type W = crate::W<Scu5f0Spec>;
#[doc = "Field `SCUDISPDIO184` reader - SCU_DIS_PD_IO184"]
pub type Scudispdio184R = crate::BitReader;
#[doc = "Field `SCUDISPDIO184` writer - SCU_DIS_PD_IO184"]
pub type Scudispdio184W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO184` reader - SCU_DIS_PU_IO184"]
pub type Scudispuio184R = crate::BitReader;
#[doc = "Field `SCUDISPUIO184` writer - SCU_DIS_PU_IO184"]
pub type Scudispuio184W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO184` reader - SCU_DRV_IO184"]
pub type Scudrvio184R = crate::FieldReader;
#[doc = "Field `SCUDRVIO184` writer - SCU_DRV_IO184"]
pub type Scudrvio184W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO184` reader - SCU_EN_SMT_IO184"]
pub type Scuensmtio184R = crate::BitReader;
#[doc = "Field `SCUENSMTIO184` writer - SCU_EN_SMT_IO184"]
pub type Scuensmtio184W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO184` reader - SCU_EN_HV_IO184"]
pub type Scuenhvio184R = crate::BitReader;
#[doc = "Field `SCUENHVIO184` writer - SCU_EN_HV_IO184"]
pub type Scuenhvio184W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO185` reader - SCU_DIS_PD_IO185"]
pub type Scudispdio185R = crate::BitReader;
#[doc = "Field `SCUDISPDIO185` writer - SCU_DIS_PD_IO185"]
pub type Scudispdio185W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO185` reader - SCU_DIS_PU_IO185"]
pub type Scudispuio185R = crate::BitReader;
#[doc = "Field `SCUDISPUIO185` writer - SCU_DIS_PU_IO185"]
pub type Scudispuio185W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO185` reader - SCU_DRV_IO185"]
pub type Scudrvio185R = crate::FieldReader;
#[doc = "Field `SCUDRVIO185` writer - SCU_DRV_IO185"]
pub type Scudrvio185W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO185` reader - SCU_EN_SMT_IO185"]
pub type Scuensmtio185R = crate::BitReader;
#[doc = "Field `SCUENSMTIO185` writer - SCU_EN_SMT_IO185"]
pub type Scuensmtio185W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO185` reader - SCU_EN_HV_IO185"]
pub type Scuenhvio185R = crate::BitReader;
#[doc = "Field `SCUENHVIO185` writer - SCU_EN_HV_IO185"]
pub type Scuenhvio185W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO184"]
    #[inline(always)]
    pub fn scudispdio184(&self) -> Scudispdio184R {
        Scudispdio184R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO184"]
    #[inline(always)]
    pub fn scudispuio184(&self) -> Scudispuio184R {
        Scudispuio184R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO184"]
    #[inline(always)]
    pub fn scudrvio184(&self) -> Scudrvio184R {
        Scudrvio184R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO184"]
    #[inline(always)]
    pub fn scuensmtio184(&self) -> Scuensmtio184R {
        Scuensmtio184R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO184"]
    #[inline(always)]
    pub fn scuenhvio184(&self) -> Scuenhvio184R {
        Scuenhvio184R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO185"]
    #[inline(always)]
    pub fn scudispdio185(&self) -> Scudispdio185R {
        Scudispdio185R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO185"]
    #[inline(always)]
    pub fn scudispuio185(&self) -> Scudispuio185R {
        Scudispuio185R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO185"]
    #[inline(always)]
    pub fn scudrvio185(&self) -> Scudrvio185R {
        Scudrvio185R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO185"]
    #[inline(always)]
    pub fn scuensmtio185(&self) -> Scuensmtio185R {
        Scuensmtio185R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO185"]
    #[inline(always)]
    pub fn scuenhvio185(&self) -> Scuenhvio185R {
        Scuenhvio185R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO184"]
    #[inline(always)]
    pub fn scudispdio184(&mut self) -> Scudispdio184W<Scu5f0Spec> {
        Scudispdio184W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO184"]
    #[inline(always)]
    pub fn scudispuio184(&mut self) -> Scudispuio184W<Scu5f0Spec> {
        Scudispuio184W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO184"]
    #[inline(always)]
    pub fn scudrvio184(&mut self) -> Scudrvio184W<Scu5f0Spec> {
        Scudrvio184W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO184"]
    #[inline(always)]
    pub fn scuensmtio184(&mut self) -> Scuensmtio184W<Scu5f0Spec> {
        Scuensmtio184W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO184"]
    #[inline(always)]
    pub fn scuenhvio184(&mut self) -> Scuenhvio184W<Scu5f0Spec> {
        Scuenhvio184W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO185"]
    #[inline(always)]
    pub fn scudispdio185(&mut self) -> Scudispdio185W<Scu5f0Spec> {
        Scudispdio185W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO185"]
    #[inline(always)]
    pub fn scudispuio185(&mut self) -> Scudispuio185W<Scu5f0Spec> {
        Scudispuio185W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO185"]
    #[inline(always)]
    pub fn scudrvio185(&mut self) -> Scudrvio185W<Scu5f0Spec> {
        Scudrvio185W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO185"]
    #[inline(always)]
    pub fn scuensmtio185(&mut self) -> Scuensmtio185W<Scu5f0Spec> {
        Scuensmtio185W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO185"]
    #[inline(always)]
    pub fn scuenhvio185(&mut self) -> Scuenhvio185W<Scu5f0Spec> {
        Scuenhvio185W::new(self, 25)
    }
}
#[doc = "IO Control \\#93\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5f0Spec;
impl crate::RegisterSpec for Scu5f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5f0::R`](R) reader structure"]
impl crate::Readable for Scu5f0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5f0::W`](W) writer structure"]
impl crate::Writable for Scu5f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5F0 to value 0x0204_0204"]
impl crate::Resettable for Scu5f0Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
