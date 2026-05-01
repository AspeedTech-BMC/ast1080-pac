#[doc = "Register `SCU544` reader"]
pub type R = crate::R<Scu544Spec>;
#[doc = "Register `SCU544` writer"]
pub type W = crate::W<Scu544Spec>;
#[doc = "Field `SCUDISPDIO098` reader - SCU_DIS_PD_IO098"]
pub type Scudispdio098R = crate::BitReader;
#[doc = "Field `SCUDISPDIO098` writer - SCU_DIS_PD_IO098"]
pub type Scudispdio098W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO098` reader - SCU_DIS_PU_IO098"]
pub type Scudispuio098R = crate::BitReader;
#[doc = "Field `SCUDISPUIO098` writer - SCU_DIS_PU_IO098"]
pub type Scudispuio098W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO098` reader - SCU_DRV_IO098"]
pub type Scudrvio098R = crate::FieldReader;
#[doc = "Field `SCUDRVIO098` writer - SCU_DRV_IO098"]
pub type Scudrvio098W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO098` reader - SCU_EN_SMT_IO098"]
pub type Scuensmtio098R = crate::BitReader;
#[doc = "Field `SCUENSMTIO098` writer - SCU_EN_SMT_IO098"]
pub type Scuensmtio098W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO098` reader - SCU_EN_HV_IO098"]
pub type Scuenhvio098R = crate::BitReader;
#[doc = "Field `SCUENHVIO098` writer - SCU_EN_HV_IO098"]
pub type Scuenhvio098W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO099` reader - SCU_DIS_PD_IO099"]
pub type Scudispdio099R = crate::BitReader;
#[doc = "Field `SCUDISPDIO099` writer - SCU_DIS_PD_IO099"]
pub type Scudispdio099W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO099` reader - SCU_DIS_PU_IO099"]
pub type Scudispuio099R = crate::BitReader;
#[doc = "Field `SCUDISPUIO099` writer - SCU_DIS_PU_IO099"]
pub type Scudispuio099W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO099` reader - SCU_DRV_IO099"]
pub type Scudrvio099R = crate::FieldReader;
#[doc = "Field `SCUDRVIO099` writer - SCU_DRV_IO099"]
pub type Scudrvio099W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO099` reader - SCU_EN_SMT_IO099"]
pub type Scuensmtio099R = crate::BitReader;
#[doc = "Field `SCUENSMTIO099` writer - SCU_EN_SMT_IO099"]
pub type Scuensmtio099W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO099` reader - SCU_EN_HV_IO099"]
pub type Scuenhvio099R = crate::BitReader;
#[doc = "Field `SCUENHVIO099` writer - SCU_EN_HV_IO099"]
pub type Scuenhvio099W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO098"]
    #[inline(always)]
    pub fn scudispdio098(&self) -> Scudispdio098R {
        Scudispdio098R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO098"]
    #[inline(always)]
    pub fn scudispuio098(&self) -> Scudispuio098R {
        Scudispuio098R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO098"]
    #[inline(always)]
    pub fn scudrvio098(&self) -> Scudrvio098R {
        Scudrvio098R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO098"]
    #[inline(always)]
    pub fn scuensmtio098(&self) -> Scuensmtio098R {
        Scuensmtio098R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO098"]
    #[inline(always)]
    pub fn scuenhvio098(&self) -> Scuenhvio098R {
        Scuenhvio098R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO099"]
    #[inline(always)]
    pub fn scudispdio099(&self) -> Scudispdio099R {
        Scudispdio099R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO099"]
    #[inline(always)]
    pub fn scudispuio099(&self) -> Scudispuio099R {
        Scudispuio099R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO099"]
    #[inline(always)]
    pub fn scudrvio099(&self) -> Scudrvio099R {
        Scudrvio099R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO099"]
    #[inline(always)]
    pub fn scuensmtio099(&self) -> Scuensmtio099R {
        Scuensmtio099R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO099"]
    #[inline(always)]
    pub fn scuenhvio099(&self) -> Scuenhvio099R {
        Scuenhvio099R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO098"]
    #[inline(always)]
    pub fn scudispdio098(&mut self) -> Scudispdio098W<Scu544Spec> {
        Scudispdio098W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO098"]
    #[inline(always)]
    pub fn scudispuio098(&mut self) -> Scudispuio098W<Scu544Spec> {
        Scudispuio098W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO098"]
    #[inline(always)]
    pub fn scudrvio098(&mut self) -> Scudrvio098W<Scu544Spec> {
        Scudrvio098W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO098"]
    #[inline(always)]
    pub fn scuensmtio098(&mut self) -> Scuensmtio098W<Scu544Spec> {
        Scuensmtio098W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO098"]
    #[inline(always)]
    pub fn scuenhvio098(&mut self) -> Scuenhvio098W<Scu544Spec> {
        Scuenhvio098W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO099"]
    #[inline(always)]
    pub fn scudispdio099(&mut self) -> Scudispdio099W<Scu544Spec> {
        Scudispdio099W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO099"]
    #[inline(always)]
    pub fn scudispuio099(&mut self) -> Scudispuio099W<Scu544Spec> {
        Scudispuio099W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO099"]
    #[inline(always)]
    pub fn scudrvio099(&mut self) -> Scudrvio099W<Scu544Spec> {
        Scudrvio099W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO099"]
    #[inline(always)]
    pub fn scuensmtio099(&mut self) -> Scuensmtio099W<Scu544Spec> {
        Scuensmtio099W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO099"]
    #[inline(always)]
    pub fn scuenhvio099(&mut self) -> Scuenhvio099W<Scu544Spec> {
        Scuenhvio099W::new(self, 25)
    }
}
#[doc = "IO Control \\#50\n\nYou can [`read`](crate::Reg::read) this register and get [`scu544::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu544::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu544Spec;
impl crate::RegisterSpec for Scu544Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu544::R`](R) reader structure"]
impl crate::Readable for Scu544Spec {}
#[doc = "`write(|w| ..)` method takes [`scu544::W`](W) writer structure"]
impl crate::Writable for Scu544Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU544 to value 0x0204_0204"]
impl crate::Resettable for Scu544Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
