#[doc = "Register `SCU508` reader"]
pub type R = crate::R<Scu508Spec>;
#[doc = "Register `SCU508` writer"]
pub type W = crate::W<Scu508Spec>;
#[doc = "Field `SCUDISPDIO068` reader - SCU_DIS_PD_IO068"]
pub type Scudispdio068R = crate::BitReader;
#[doc = "Field `SCUDISPDIO068` writer - SCU_DIS_PD_IO068"]
pub type Scudispdio068W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO068` reader - SCU_DIS_PU_IO068"]
pub type Scudispuio068R = crate::BitReader;
#[doc = "Field `SCUDISPUIO068` writer - SCU_DIS_PU_IO068"]
pub type Scudispuio068W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO068` reader - SCU_DRV_IO068"]
pub type Scudrvio068R = crate::FieldReader;
#[doc = "Field `SCUDRVIO068` writer - SCU_DRV_IO068"]
pub type Scudrvio068W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO068` reader - SCU_EN_SMT_IO068"]
pub type Scuensmtio068R = crate::BitReader;
#[doc = "Field `SCUENSMTIO068` writer - SCU_EN_SMT_IO068"]
pub type Scuensmtio068W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO068` reader - SCU_EN_HV_IO068"]
pub type Scuenhvio068R = crate::BitReader;
#[doc = "Field `SCUENHVIO068` writer - SCU_EN_HV_IO068"]
pub type Scuenhvio068W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO069` reader - SCU_DIS_PD_IO069"]
pub type Scudispdio069R = crate::BitReader;
#[doc = "Field `SCUDISPDIO069` writer - SCU_DIS_PD_IO069"]
pub type Scudispdio069W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO069` reader - SCU_DIS_PU_IO069"]
pub type Scudispuio069R = crate::BitReader;
#[doc = "Field `SCUDISPUIO069` writer - SCU_DIS_PU_IO069"]
pub type Scudispuio069W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO069` reader - SCU_DRV_IO069"]
pub type Scudrvio069R = crate::FieldReader;
#[doc = "Field `SCUDRVIO069` writer - SCU_DRV_IO069"]
pub type Scudrvio069W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO069` reader - SCU_EN_SMT_IO069"]
pub type Scuensmtio069R = crate::BitReader;
#[doc = "Field `SCUENSMTIO069` writer - SCU_EN_SMT_IO069"]
pub type Scuensmtio069W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO069` reader - SCU_EN_HV_IO069"]
pub type Scuenhvio069R = crate::BitReader;
#[doc = "Field `SCUENHVIO069` writer - SCU_EN_HV_IO069"]
pub type Scuenhvio069W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO068"]
    #[inline(always)]
    pub fn scudispdio068(&self) -> Scudispdio068R {
        Scudispdio068R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO068"]
    #[inline(always)]
    pub fn scudispuio068(&self) -> Scudispuio068R {
        Scudispuio068R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO068"]
    #[inline(always)]
    pub fn scudrvio068(&self) -> Scudrvio068R {
        Scudrvio068R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO068"]
    #[inline(always)]
    pub fn scuensmtio068(&self) -> Scuensmtio068R {
        Scuensmtio068R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO068"]
    #[inline(always)]
    pub fn scuenhvio068(&self) -> Scuenhvio068R {
        Scuenhvio068R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO069"]
    #[inline(always)]
    pub fn scudispdio069(&self) -> Scudispdio069R {
        Scudispdio069R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO069"]
    #[inline(always)]
    pub fn scudispuio069(&self) -> Scudispuio069R {
        Scudispuio069R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO069"]
    #[inline(always)]
    pub fn scudrvio069(&self) -> Scudrvio069R {
        Scudrvio069R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO069"]
    #[inline(always)]
    pub fn scuensmtio069(&self) -> Scuensmtio069R {
        Scuensmtio069R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO069"]
    #[inline(always)]
    pub fn scuenhvio069(&self) -> Scuenhvio069R {
        Scuenhvio069R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO068"]
    #[inline(always)]
    pub fn scudispdio068(&mut self) -> Scudispdio068W<Scu508Spec> {
        Scudispdio068W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO068"]
    #[inline(always)]
    pub fn scudispuio068(&mut self) -> Scudispuio068W<Scu508Spec> {
        Scudispuio068W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO068"]
    #[inline(always)]
    pub fn scudrvio068(&mut self) -> Scudrvio068W<Scu508Spec> {
        Scudrvio068W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO068"]
    #[inline(always)]
    pub fn scuensmtio068(&mut self) -> Scuensmtio068W<Scu508Spec> {
        Scuensmtio068W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO068"]
    #[inline(always)]
    pub fn scuenhvio068(&mut self) -> Scuenhvio068W<Scu508Spec> {
        Scuenhvio068W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO069"]
    #[inline(always)]
    pub fn scudispdio069(&mut self) -> Scudispdio069W<Scu508Spec> {
        Scudispdio069W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO069"]
    #[inline(always)]
    pub fn scudispuio069(&mut self) -> Scudispuio069W<Scu508Spec> {
        Scudispuio069W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO069"]
    #[inline(always)]
    pub fn scudrvio069(&mut self) -> Scudrvio069W<Scu508Spec> {
        Scudrvio069W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO069"]
    #[inline(always)]
    pub fn scuensmtio069(&mut self) -> Scuensmtio069W<Scu508Spec> {
        Scuensmtio069W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO069"]
    #[inline(always)]
    pub fn scuenhvio069(&mut self) -> Scuenhvio069W<Scu508Spec> {
        Scuenhvio069W::new(self, 25)
    }
}
#[doc = "IO Control \\#35\n\nYou can [`read`](crate::Reg::read) this register and get [`scu508::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu508::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu508Spec;
impl crate::RegisterSpec for Scu508Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu508::R`](R) reader structure"]
impl crate::Readable for Scu508Spec {}
#[doc = "`write(|w| ..)` method takes [`scu508::W`](W) writer structure"]
impl crate::Writable for Scu508Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU508 to value 0x0204_0204"]
impl crate::Resettable for Scu508Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
