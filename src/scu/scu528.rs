#[doc = "Register `SCU528` reader"]
pub type R = crate::R<Scu528Spec>;
#[doc = "Register `SCU528` writer"]
pub type W = crate::W<Scu528Spec>;
#[doc = "Field `SCUDISPDIO084` reader - SCU_DIS_PD_IO084"]
pub type Scudispdio084R = crate::BitReader;
#[doc = "Field `SCUDISPDIO084` writer - SCU_DIS_PD_IO084"]
pub type Scudispdio084W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO084` reader - SCU_DIS_PU_IO084"]
pub type Scudispuio084R = crate::BitReader;
#[doc = "Field `SCUDISPUIO084` writer - SCU_DIS_PU_IO084"]
pub type Scudispuio084W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO084` reader - SCU_DRV_IO084"]
pub type Scudrvio084R = crate::FieldReader;
#[doc = "Field `SCUDRVIO084` writer - SCU_DRV_IO084"]
pub type Scudrvio084W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO084` reader - SCU_EN_SMT_IO084"]
pub type Scuensmtio084R = crate::BitReader;
#[doc = "Field `SCUENSMTIO084` writer - SCU_EN_SMT_IO084"]
pub type Scuensmtio084W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO084` reader - SCU_EN_HV_IO084"]
pub type Scuenhvio084R = crate::BitReader;
#[doc = "Field `SCUENHVIO084` writer - SCU_EN_HV_IO084"]
pub type Scuenhvio084W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO085` reader - SCU_DIS_PD_IO085"]
pub type Scudispdio085R = crate::BitReader;
#[doc = "Field `SCUDISPDIO085` writer - SCU_DIS_PD_IO085"]
pub type Scudispdio085W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO085` reader - SCU_DIS_PU_IO085"]
pub type Scudispuio085R = crate::BitReader;
#[doc = "Field `SCUDISPUIO085` writer - SCU_DIS_PU_IO085"]
pub type Scudispuio085W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO085` reader - SCU_DRV_IO085"]
pub type Scudrvio085R = crate::FieldReader;
#[doc = "Field `SCUDRVIO085` writer - SCU_DRV_IO085"]
pub type Scudrvio085W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO085` reader - SCU_EN_SMT_IO085"]
pub type Scuensmtio085R = crate::BitReader;
#[doc = "Field `SCUENSMTIO085` writer - SCU_EN_SMT_IO085"]
pub type Scuensmtio085W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO085` reader - SCU_EN_HV_IO085"]
pub type Scuenhvio085R = crate::BitReader;
#[doc = "Field `SCUENHVIO085` writer - SCU_EN_HV_IO085"]
pub type Scuenhvio085W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO084"]
    #[inline(always)]
    pub fn scudispdio084(&self) -> Scudispdio084R {
        Scudispdio084R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO084"]
    #[inline(always)]
    pub fn scudispuio084(&self) -> Scudispuio084R {
        Scudispuio084R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO084"]
    #[inline(always)]
    pub fn scudrvio084(&self) -> Scudrvio084R {
        Scudrvio084R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO084"]
    #[inline(always)]
    pub fn scuensmtio084(&self) -> Scuensmtio084R {
        Scuensmtio084R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO084"]
    #[inline(always)]
    pub fn scuenhvio084(&self) -> Scuenhvio084R {
        Scuenhvio084R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO085"]
    #[inline(always)]
    pub fn scudispdio085(&self) -> Scudispdio085R {
        Scudispdio085R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO085"]
    #[inline(always)]
    pub fn scudispuio085(&self) -> Scudispuio085R {
        Scudispuio085R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO085"]
    #[inline(always)]
    pub fn scudrvio085(&self) -> Scudrvio085R {
        Scudrvio085R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO085"]
    #[inline(always)]
    pub fn scuensmtio085(&self) -> Scuensmtio085R {
        Scuensmtio085R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO085"]
    #[inline(always)]
    pub fn scuenhvio085(&self) -> Scuenhvio085R {
        Scuenhvio085R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO084"]
    #[inline(always)]
    pub fn scudispdio084(&mut self) -> Scudispdio084W<Scu528Spec> {
        Scudispdio084W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO084"]
    #[inline(always)]
    pub fn scudispuio084(&mut self) -> Scudispuio084W<Scu528Spec> {
        Scudispuio084W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO084"]
    #[inline(always)]
    pub fn scudrvio084(&mut self) -> Scudrvio084W<Scu528Spec> {
        Scudrvio084W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO084"]
    #[inline(always)]
    pub fn scuensmtio084(&mut self) -> Scuensmtio084W<Scu528Spec> {
        Scuensmtio084W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO084"]
    #[inline(always)]
    pub fn scuenhvio084(&mut self) -> Scuenhvio084W<Scu528Spec> {
        Scuenhvio084W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO085"]
    #[inline(always)]
    pub fn scudispdio085(&mut self) -> Scudispdio085W<Scu528Spec> {
        Scudispdio085W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO085"]
    #[inline(always)]
    pub fn scudispuio085(&mut self) -> Scudispuio085W<Scu528Spec> {
        Scudispuio085W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO085"]
    #[inline(always)]
    pub fn scudrvio085(&mut self) -> Scudrvio085W<Scu528Spec> {
        Scudrvio085W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO085"]
    #[inline(always)]
    pub fn scuensmtio085(&mut self) -> Scuensmtio085W<Scu528Spec> {
        Scuensmtio085W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO085"]
    #[inline(always)]
    pub fn scuenhvio085(&mut self) -> Scuenhvio085W<Scu528Spec> {
        Scuenhvio085W::new(self, 25)
    }
}
#[doc = "IO Control \\#43\n\nYou can [`read`](crate::Reg::read) this register and get [`scu528::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu528::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu528Spec;
impl crate::RegisterSpec for Scu528Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu528::R`](R) reader structure"]
impl crate::Readable for Scu528Spec {}
#[doc = "`write(|w| ..)` method takes [`scu528::W`](W) writer structure"]
impl crate::Writable for Scu528Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU528 to value 0x0204_0204"]
impl crate::Resettable for Scu528Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
