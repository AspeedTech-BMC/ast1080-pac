#[doc = "Register `SCU510` reader"]
pub type R = crate::R<Scu510Spec>;
#[doc = "Register `SCU510` writer"]
pub type W = crate::W<Scu510Spec>;
#[doc = "Field `SCUDISPDIO072` reader - SCU_DIS_PD_IO072"]
pub type Scudispdio072R = crate::BitReader;
#[doc = "Field `SCUDISPDIO072` writer - SCU_DIS_PD_IO072"]
pub type Scudispdio072W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO072` reader - SCU_DIS_PU_IO072"]
pub type Scudispuio072R = crate::BitReader;
#[doc = "Field `SCUDISPUIO072` writer - SCU_DIS_PU_IO072"]
pub type Scudispuio072W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO072` reader - SCU_DRV_IO072"]
pub type Scudrvio072R = crate::FieldReader;
#[doc = "Field `SCUDRVIO072` writer - SCU_DRV_IO072"]
pub type Scudrvio072W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO072` reader - SCU_EN_SMT_IO072"]
pub type Scuensmtio072R = crate::BitReader;
#[doc = "Field `SCUENSMTIO072` writer - SCU_EN_SMT_IO072"]
pub type Scuensmtio072W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO072` reader - SCU_EN_HV_IO072"]
pub type Scuenhvio072R = crate::BitReader;
#[doc = "Field `SCUENHVIO072` writer - SCU_EN_HV_IO072"]
pub type Scuenhvio072W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO073` reader - SCU_DIS_PD_IO073"]
pub type Scudispdio073R = crate::BitReader;
#[doc = "Field `SCUDISPDIO073` writer - SCU_DIS_PD_IO073"]
pub type Scudispdio073W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO073` reader - SCU_DIS_PU_IO073"]
pub type Scudispuio073R = crate::BitReader;
#[doc = "Field `SCUDISPUIO073` writer - SCU_DIS_PU_IO073"]
pub type Scudispuio073W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO073` reader - SCU_DRV_IO073"]
pub type Scudrvio073R = crate::FieldReader;
#[doc = "Field `SCUDRVIO073` writer - SCU_DRV_IO073"]
pub type Scudrvio073W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO073` reader - SCU_EN_SMT_IO073"]
pub type Scuensmtio073R = crate::BitReader;
#[doc = "Field `SCUENSMTIO073` writer - SCU_EN_SMT_IO073"]
pub type Scuensmtio073W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO073` reader - SCU_EN_HV_IO073"]
pub type Scuenhvio073R = crate::BitReader;
#[doc = "Field `SCUENHVIO073` writer - SCU_EN_HV_IO073"]
pub type Scuenhvio073W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO072"]
    #[inline(always)]
    pub fn scudispdio072(&self) -> Scudispdio072R {
        Scudispdio072R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO072"]
    #[inline(always)]
    pub fn scudispuio072(&self) -> Scudispuio072R {
        Scudispuio072R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO072"]
    #[inline(always)]
    pub fn scudrvio072(&self) -> Scudrvio072R {
        Scudrvio072R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO072"]
    #[inline(always)]
    pub fn scuensmtio072(&self) -> Scuensmtio072R {
        Scuensmtio072R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO072"]
    #[inline(always)]
    pub fn scuenhvio072(&self) -> Scuenhvio072R {
        Scuenhvio072R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO073"]
    #[inline(always)]
    pub fn scudispdio073(&self) -> Scudispdio073R {
        Scudispdio073R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO073"]
    #[inline(always)]
    pub fn scudispuio073(&self) -> Scudispuio073R {
        Scudispuio073R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO073"]
    #[inline(always)]
    pub fn scudrvio073(&self) -> Scudrvio073R {
        Scudrvio073R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO073"]
    #[inline(always)]
    pub fn scuensmtio073(&self) -> Scuensmtio073R {
        Scuensmtio073R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO073"]
    #[inline(always)]
    pub fn scuenhvio073(&self) -> Scuenhvio073R {
        Scuenhvio073R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO072"]
    #[inline(always)]
    pub fn scudispdio072(&mut self) -> Scudispdio072W<Scu510Spec> {
        Scudispdio072W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO072"]
    #[inline(always)]
    pub fn scudispuio072(&mut self) -> Scudispuio072W<Scu510Spec> {
        Scudispuio072W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO072"]
    #[inline(always)]
    pub fn scudrvio072(&mut self) -> Scudrvio072W<Scu510Spec> {
        Scudrvio072W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO072"]
    #[inline(always)]
    pub fn scuensmtio072(&mut self) -> Scuensmtio072W<Scu510Spec> {
        Scuensmtio072W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO072"]
    #[inline(always)]
    pub fn scuenhvio072(&mut self) -> Scuenhvio072W<Scu510Spec> {
        Scuenhvio072W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO073"]
    #[inline(always)]
    pub fn scudispdio073(&mut self) -> Scudispdio073W<Scu510Spec> {
        Scudispdio073W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO073"]
    #[inline(always)]
    pub fn scudispuio073(&mut self) -> Scudispuio073W<Scu510Spec> {
        Scudispuio073W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO073"]
    #[inline(always)]
    pub fn scudrvio073(&mut self) -> Scudrvio073W<Scu510Spec> {
        Scudrvio073W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO073"]
    #[inline(always)]
    pub fn scuensmtio073(&mut self) -> Scuensmtio073W<Scu510Spec> {
        Scuensmtio073W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO073"]
    #[inline(always)]
    pub fn scuenhvio073(&mut self) -> Scuenhvio073W<Scu510Spec> {
        Scuenhvio073W::new(self, 25)
    }
}
#[doc = "IO Control \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`scu510::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu510::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu510Spec;
impl crate::RegisterSpec for Scu510Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu510::R`](R) reader structure"]
impl crate::Readable for Scu510Spec {}
#[doc = "`write(|w| ..)` method takes [`scu510::W`](W) writer structure"]
impl crate::Writable for Scu510Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU510 to value 0x0204_0204"]
impl crate::Resettable for Scu510Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
