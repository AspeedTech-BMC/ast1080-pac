#[doc = "Register `SCU5C0` reader"]
pub type R = crate::R<Scu5c0Spec>;
#[doc = "Register `SCU5C0` writer"]
pub type W = crate::W<Scu5c0Spec>;
#[doc = "Field `SCUDISPDIO160` reader - SCU_DIS_PD_IO160"]
pub type Scudispdio160R = crate::BitReader;
#[doc = "Field `SCUDISPDIO160` writer - SCU_DIS_PD_IO160"]
pub type Scudispdio160W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO160` reader - SCU_DIS_PU_IO160"]
pub type Scudispuio160R = crate::BitReader;
#[doc = "Field `SCUDISPUIO160` writer - SCU_DIS_PU_IO160"]
pub type Scudispuio160W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO160` reader - SCU_DRV_IO160"]
pub type Scudrvio160R = crate::FieldReader;
#[doc = "Field `SCUDRVIO160` writer - SCU_DRV_IO160"]
pub type Scudrvio160W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO160` reader - SCU_EN_SMT_IO160"]
pub type Scuensmtio160R = crate::BitReader;
#[doc = "Field `SCUENSMTIO160` writer - SCU_EN_SMT_IO160"]
pub type Scuensmtio160W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO160` reader - SCU_EN_HV_IO160"]
pub type Scuenhvio160R = crate::BitReader;
#[doc = "Field `SCUENHVIO160` writer - SCU_EN_HV_IO160"]
pub type Scuenhvio160W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO161` reader - SCU_DIS_PD_IO161"]
pub type Scudispdio161R = crate::BitReader;
#[doc = "Field `SCUDISPDIO161` writer - SCU_DIS_PD_IO161"]
pub type Scudispdio161W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO161` reader - SCU_DIS_PU_IO161"]
pub type Scudispuio161R = crate::BitReader;
#[doc = "Field `SCUDISPUIO161` writer - SCU_DIS_PU_IO161"]
pub type Scudispuio161W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO161` reader - SCU_DRV_IO161"]
pub type Scudrvio161R = crate::FieldReader;
#[doc = "Field `SCUDRVIO161` writer - SCU_DRV_IO161"]
pub type Scudrvio161W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO161` reader - SCU_EN_SMT_IO161"]
pub type Scuensmtio161R = crate::BitReader;
#[doc = "Field `SCUENSMTIO161` writer - SCU_EN_SMT_IO161"]
pub type Scuensmtio161W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO161` reader - SCU_EN_HV_IO161"]
pub type Scuenhvio161R = crate::BitReader;
#[doc = "Field `SCUENHVIO161` writer - SCU_EN_HV_IO161"]
pub type Scuenhvio161W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO160"]
    #[inline(always)]
    pub fn scudispdio160(&self) -> Scudispdio160R {
        Scudispdio160R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO160"]
    #[inline(always)]
    pub fn scudispuio160(&self) -> Scudispuio160R {
        Scudispuio160R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO160"]
    #[inline(always)]
    pub fn scudrvio160(&self) -> Scudrvio160R {
        Scudrvio160R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO160"]
    #[inline(always)]
    pub fn scuensmtio160(&self) -> Scuensmtio160R {
        Scuensmtio160R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO160"]
    #[inline(always)]
    pub fn scuenhvio160(&self) -> Scuenhvio160R {
        Scuenhvio160R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO161"]
    #[inline(always)]
    pub fn scudispdio161(&self) -> Scudispdio161R {
        Scudispdio161R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO161"]
    #[inline(always)]
    pub fn scudispuio161(&self) -> Scudispuio161R {
        Scudispuio161R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO161"]
    #[inline(always)]
    pub fn scudrvio161(&self) -> Scudrvio161R {
        Scudrvio161R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO161"]
    #[inline(always)]
    pub fn scuensmtio161(&self) -> Scuensmtio161R {
        Scuensmtio161R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO161"]
    #[inline(always)]
    pub fn scuenhvio161(&self) -> Scuenhvio161R {
        Scuenhvio161R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO160"]
    #[inline(always)]
    pub fn scudispdio160(&mut self) -> Scudispdio160W<Scu5c0Spec> {
        Scudispdio160W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO160"]
    #[inline(always)]
    pub fn scudispuio160(&mut self) -> Scudispuio160W<Scu5c0Spec> {
        Scudispuio160W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO160"]
    #[inline(always)]
    pub fn scudrvio160(&mut self) -> Scudrvio160W<Scu5c0Spec> {
        Scudrvio160W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO160"]
    #[inline(always)]
    pub fn scuensmtio160(&mut self) -> Scuensmtio160W<Scu5c0Spec> {
        Scuensmtio160W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO160"]
    #[inline(always)]
    pub fn scuenhvio160(&mut self) -> Scuenhvio160W<Scu5c0Spec> {
        Scuenhvio160W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO161"]
    #[inline(always)]
    pub fn scudispdio161(&mut self) -> Scudispdio161W<Scu5c0Spec> {
        Scudispdio161W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO161"]
    #[inline(always)]
    pub fn scudispuio161(&mut self) -> Scudispuio161W<Scu5c0Spec> {
        Scudispuio161W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO161"]
    #[inline(always)]
    pub fn scudrvio161(&mut self) -> Scudrvio161W<Scu5c0Spec> {
        Scudrvio161W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO161"]
    #[inline(always)]
    pub fn scuensmtio161(&mut self) -> Scuensmtio161W<Scu5c0Spec> {
        Scuensmtio161W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO161"]
    #[inline(always)]
    pub fn scuenhvio161(&mut self) -> Scuenhvio161W<Scu5c0Spec> {
        Scuenhvio161W::new(self, 25)
    }
}
#[doc = "IO Control \\#81\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5c0Spec;
impl crate::RegisterSpec for Scu5c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5c0::R`](R) reader structure"]
impl crate::Readable for Scu5c0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5c0::W`](W) writer structure"]
impl crate::Writable for Scu5c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5C0 to value 0x0204_0204"]
impl crate::Resettable for Scu5c0Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
