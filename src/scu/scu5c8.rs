#[doc = "Register `SCU5C8` reader"]
pub type R = crate::R<Scu5c8Spec>;
#[doc = "Register `SCU5C8` writer"]
pub type W = crate::W<Scu5c8Spec>;
#[doc = "Field `SCUDISPDIO164` reader - SCU_DIS_PD_IO164"]
pub type Scudispdio164R = crate::BitReader;
#[doc = "Field `SCUDISPDIO164` writer - SCU_DIS_PD_IO164"]
pub type Scudispdio164W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO164` reader - SCU_DIS_PU_IO164"]
pub type Scudispuio164R = crate::BitReader;
#[doc = "Field `SCUDISPUIO164` writer - SCU_DIS_PU_IO164"]
pub type Scudispuio164W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO164` reader - SCU_DRV_IO164"]
pub type Scudrvio164R = crate::FieldReader;
#[doc = "Field `SCUDRVIO164` writer - SCU_DRV_IO164"]
pub type Scudrvio164W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO164` reader - SCU_EN_SMT_IO164"]
pub type Scuensmtio164R = crate::BitReader;
#[doc = "Field `SCUENSMTIO164` writer - SCU_EN_SMT_IO164"]
pub type Scuensmtio164W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO164` reader - SCU_EN_HV_IO164"]
pub type Scuenhvio164R = crate::BitReader;
#[doc = "Field `SCUENHVIO164` writer - SCU_EN_HV_IO164"]
pub type Scuenhvio164W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO165` reader - SCU_DIS_PD_IO165"]
pub type Scudispdio165R = crate::BitReader;
#[doc = "Field `SCUDISPDIO165` writer - SCU_DIS_PD_IO165"]
pub type Scudispdio165W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO165` reader - SCU_DIS_PU_IO165"]
pub type Scudispuio165R = crate::BitReader;
#[doc = "Field `SCUDISPUIO165` writer - SCU_DIS_PU_IO165"]
pub type Scudispuio165W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO165` reader - SCU_DRV_IO165"]
pub type Scudrvio165R = crate::FieldReader;
#[doc = "Field `SCUDRVIO165` writer - SCU_DRV_IO165"]
pub type Scudrvio165W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO165` reader - SCU_EN_SMT_IO165"]
pub type Scuensmtio165R = crate::BitReader;
#[doc = "Field `SCUENSMTIO165` writer - SCU_EN_SMT_IO165"]
pub type Scuensmtio165W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO165` reader - SCU_EN_HV_IO165"]
pub type Scuenhvio165R = crate::BitReader;
#[doc = "Field `SCUENHVIO165` writer - SCU_EN_HV_IO165"]
pub type Scuenhvio165W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO164"]
    #[inline(always)]
    pub fn scudispdio164(&self) -> Scudispdio164R {
        Scudispdio164R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO164"]
    #[inline(always)]
    pub fn scudispuio164(&self) -> Scudispuio164R {
        Scudispuio164R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO164"]
    #[inline(always)]
    pub fn scudrvio164(&self) -> Scudrvio164R {
        Scudrvio164R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO164"]
    #[inline(always)]
    pub fn scuensmtio164(&self) -> Scuensmtio164R {
        Scuensmtio164R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO164"]
    #[inline(always)]
    pub fn scuenhvio164(&self) -> Scuenhvio164R {
        Scuenhvio164R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO165"]
    #[inline(always)]
    pub fn scudispdio165(&self) -> Scudispdio165R {
        Scudispdio165R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO165"]
    #[inline(always)]
    pub fn scudispuio165(&self) -> Scudispuio165R {
        Scudispuio165R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO165"]
    #[inline(always)]
    pub fn scudrvio165(&self) -> Scudrvio165R {
        Scudrvio165R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO165"]
    #[inline(always)]
    pub fn scuensmtio165(&self) -> Scuensmtio165R {
        Scuensmtio165R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO165"]
    #[inline(always)]
    pub fn scuenhvio165(&self) -> Scuenhvio165R {
        Scuenhvio165R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO164"]
    #[inline(always)]
    pub fn scudispdio164(&mut self) -> Scudispdio164W<Scu5c8Spec> {
        Scudispdio164W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO164"]
    #[inline(always)]
    pub fn scudispuio164(&mut self) -> Scudispuio164W<Scu5c8Spec> {
        Scudispuio164W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO164"]
    #[inline(always)]
    pub fn scudrvio164(&mut self) -> Scudrvio164W<Scu5c8Spec> {
        Scudrvio164W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO164"]
    #[inline(always)]
    pub fn scuensmtio164(&mut self) -> Scuensmtio164W<Scu5c8Spec> {
        Scuensmtio164W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO164"]
    #[inline(always)]
    pub fn scuenhvio164(&mut self) -> Scuenhvio164W<Scu5c8Spec> {
        Scuenhvio164W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO165"]
    #[inline(always)]
    pub fn scudispdio165(&mut self) -> Scudispdio165W<Scu5c8Spec> {
        Scudispdio165W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO165"]
    #[inline(always)]
    pub fn scudispuio165(&mut self) -> Scudispuio165W<Scu5c8Spec> {
        Scudispuio165W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO165"]
    #[inline(always)]
    pub fn scudrvio165(&mut self) -> Scudrvio165W<Scu5c8Spec> {
        Scudrvio165W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO165"]
    #[inline(always)]
    pub fn scuensmtio165(&mut self) -> Scuensmtio165W<Scu5c8Spec> {
        Scuensmtio165W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO165"]
    #[inline(always)]
    pub fn scuenhvio165(&mut self) -> Scuenhvio165W<Scu5c8Spec> {
        Scuenhvio165W::new(self, 25)
    }
}
#[doc = "IO Control \\#83\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5c8Spec;
impl crate::RegisterSpec for Scu5c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5c8::R`](R) reader structure"]
impl crate::Readable for Scu5c8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5c8::W`](W) writer structure"]
impl crate::Writable for Scu5c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5C8 to value 0x0204_0201"]
impl crate::Resettable for Scu5c8Spec {
    const RESET_VALUE: u32 = 0x0204_0201;
}
