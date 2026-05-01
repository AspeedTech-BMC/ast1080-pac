#[doc = "Register `SCU558` reader"]
pub type R = crate::R<Scu558Spec>;
#[doc = "Register `SCU558` writer"]
pub type W = crate::W<Scu558Spec>;
#[doc = "Field `SCUDISPDIO108` reader - SCU_DIS_PD_IO108"]
pub type Scudispdio108R = crate::BitReader;
#[doc = "Field `SCUDISPDIO108` writer - SCU_DIS_PD_IO108"]
pub type Scudispdio108W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO108` reader - SCU_DIS_PU_IO108"]
pub type Scudispuio108R = crate::BitReader;
#[doc = "Field `SCUDISPUIO108` writer - SCU_DIS_PU_IO108"]
pub type Scudispuio108W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO108` reader - SCU_DRV_IO108"]
pub type Scudrvio108R = crate::FieldReader;
#[doc = "Field `SCUDRVIO108` writer - SCU_DRV_IO108"]
pub type Scudrvio108W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO108` reader - SCU_EN_SMT_IO108"]
pub type Scuensmtio108R = crate::BitReader;
#[doc = "Field `SCUENSMTIO108` writer - SCU_EN_SMT_IO108"]
pub type Scuensmtio108W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO108` reader - SCU_EN_HV_IO108"]
pub type Scuenhvio108R = crate::BitReader;
#[doc = "Field `SCUENHVIO108` writer - SCU_EN_HV_IO108"]
pub type Scuenhvio108W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO109` reader - SCU_DIS_PD_IO109"]
pub type Scudispdio109R = crate::BitReader;
#[doc = "Field `SCUDISPDIO109` writer - SCU_DIS_PD_IO109"]
pub type Scudispdio109W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO109` reader - SCU_DIS_PU_IO109"]
pub type Scudispuio109R = crate::BitReader;
#[doc = "Field `SCUDISPUIO109` writer - SCU_DIS_PU_IO109"]
pub type Scudispuio109W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO109` reader - SCU_DRV_IO109"]
pub type Scudrvio109R = crate::FieldReader;
#[doc = "Field `SCUDRVIO109` writer - SCU_DRV_IO109"]
pub type Scudrvio109W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO109` reader - SCU_EN_SMT_IO109"]
pub type Scuensmtio109R = crate::BitReader;
#[doc = "Field `SCUENSMTIO109` writer - SCU_EN_SMT_IO109"]
pub type Scuensmtio109W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO109` reader - SCU_EN_HV_IO109"]
pub type Scuenhvio109R = crate::BitReader;
#[doc = "Field `SCUENHVIO109` writer - SCU_EN_HV_IO109"]
pub type Scuenhvio109W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO108"]
    #[inline(always)]
    pub fn scudispdio108(&self) -> Scudispdio108R {
        Scudispdio108R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO108"]
    #[inline(always)]
    pub fn scudispuio108(&self) -> Scudispuio108R {
        Scudispuio108R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO108"]
    #[inline(always)]
    pub fn scudrvio108(&self) -> Scudrvio108R {
        Scudrvio108R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO108"]
    #[inline(always)]
    pub fn scuensmtio108(&self) -> Scuensmtio108R {
        Scuensmtio108R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO108"]
    #[inline(always)]
    pub fn scuenhvio108(&self) -> Scuenhvio108R {
        Scuenhvio108R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO109"]
    #[inline(always)]
    pub fn scudispdio109(&self) -> Scudispdio109R {
        Scudispdio109R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO109"]
    #[inline(always)]
    pub fn scudispuio109(&self) -> Scudispuio109R {
        Scudispuio109R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO109"]
    #[inline(always)]
    pub fn scudrvio109(&self) -> Scudrvio109R {
        Scudrvio109R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO109"]
    #[inline(always)]
    pub fn scuensmtio109(&self) -> Scuensmtio109R {
        Scuensmtio109R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO109"]
    #[inline(always)]
    pub fn scuenhvio109(&self) -> Scuenhvio109R {
        Scuenhvio109R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO108"]
    #[inline(always)]
    pub fn scudispdio108(&mut self) -> Scudispdio108W<Scu558Spec> {
        Scudispdio108W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO108"]
    #[inline(always)]
    pub fn scudispuio108(&mut self) -> Scudispuio108W<Scu558Spec> {
        Scudispuio108W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO108"]
    #[inline(always)]
    pub fn scudrvio108(&mut self) -> Scudrvio108W<Scu558Spec> {
        Scudrvio108W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO108"]
    #[inline(always)]
    pub fn scuensmtio108(&mut self) -> Scuensmtio108W<Scu558Spec> {
        Scuensmtio108W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO108"]
    #[inline(always)]
    pub fn scuenhvio108(&mut self) -> Scuenhvio108W<Scu558Spec> {
        Scuenhvio108W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO109"]
    #[inline(always)]
    pub fn scudispdio109(&mut self) -> Scudispdio109W<Scu558Spec> {
        Scudispdio109W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO109"]
    #[inline(always)]
    pub fn scudispuio109(&mut self) -> Scudispuio109W<Scu558Spec> {
        Scudispuio109W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO109"]
    #[inline(always)]
    pub fn scudrvio109(&mut self) -> Scudrvio109W<Scu558Spec> {
        Scudrvio109W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO109"]
    #[inline(always)]
    pub fn scuensmtio109(&mut self) -> Scuensmtio109W<Scu558Spec> {
        Scuensmtio109W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO109"]
    #[inline(always)]
    pub fn scuenhvio109(&mut self) -> Scuenhvio109W<Scu558Spec> {
        Scuenhvio109W::new(self, 25)
    }
}
#[doc = "IO Control \\#55\n\nYou can [`read`](crate::Reg::read) this register and get [`scu558::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu558::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu558Spec;
impl crate::RegisterSpec for Scu558Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu558::R`](R) reader structure"]
impl crate::Readable for Scu558Spec {}
#[doc = "`write(|w| ..)` method takes [`scu558::W`](W) writer structure"]
impl crate::Writable for Scu558Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU558 to value 0x0204_0204"]
impl crate::Resettable for Scu558Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
