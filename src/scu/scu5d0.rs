#[doc = "Register `SCU5D0` reader"]
pub type R = crate::R<Scu5d0Spec>;
#[doc = "Register `SCU5D0` writer"]
pub type W = crate::W<Scu5d0Spec>;
#[doc = "Field `SCUDISPDIO168` reader - SCU_DIS_PD_IO168"]
pub type Scudispdio168R = crate::BitReader;
#[doc = "Field `SCUDISPDIO168` writer - SCU_DIS_PD_IO168"]
pub type Scudispdio168W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO168` reader - SCU_DIS_PU_IO168"]
pub type Scudispuio168R = crate::BitReader;
#[doc = "Field `SCUDISPUIO168` writer - SCU_DIS_PU_IO168"]
pub type Scudispuio168W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO168` reader - SCU_DRV_IO168"]
pub type Scudrvio168R = crate::FieldReader;
#[doc = "Field `SCUDRVIO168` writer - SCU_DRV_IO168"]
pub type Scudrvio168W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO168` reader - SCU_EN_SMT_IO168"]
pub type Scuensmtio168R = crate::BitReader;
#[doc = "Field `SCUENSMTIO168` writer - SCU_EN_SMT_IO168"]
pub type Scuensmtio168W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO168` reader - SCU_EN_HV_IO168"]
pub type Scuenhvio168R = crate::BitReader;
#[doc = "Field `SCUENHVIO168` writer - SCU_EN_HV_IO168"]
pub type Scuenhvio168W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO169` reader - SCU_DIS_PD_IO169"]
pub type Scudispdio169R = crate::BitReader;
#[doc = "Field `SCUDISPDIO169` writer - SCU_DIS_PD_IO169"]
pub type Scudispdio169W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO169` reader - SCU_DIS_PU_IO169"]
pub type Scudispuio169R = crate::BitReader;
#[doc = "Field `SCUDISPUIO169` writer - SCU_DIS_PU_IO169"]
pub type Scudispuio169W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO169` reader - SCU_DRV_IO169"]
pub type Scudrvio169R = crate::FieldReader;
#[doc = "Field `SCUDRVIO169` writer - SCU_DRV_IO169"]
pub type Scudrvio169W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO169` reader - SCU_EN_SMT_IO169"]
pub type Scuensmtio169R = crate::BitReader;
#[doc = "Field `SCUENSMTIO169` writer - SCU_EN_SMT_IO169"]
pub type Scuensmtio169W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO169` reader - SCU_EN_HV_IO169"]
pub type Scuenhvio169R = crate::BitReader;
#[doc = "Field `SCUENHVIO169` writer - SCU_EN_HV_IO169"]
pub type Scuenhvio169W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO168"]
    #[inline(always)]
    pub fn scudispdio168(&self) -> Scudispdio168R {
        Scudispdio168R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO168"]
    #[inline(always)]
    pub fn scudispuio168(&self) -> Scudispuio168R {
        Scudispuio168R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO168"]
    #[inline(always)]
    pub fn scudrvio168(&self) -> Scudrvio168R {
        Scudrvio168R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO168"]
    #[inline(always)]
    pub fn scuensmtio168(&self) -> Scuensmtio168R {
        Scuensmtio168R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO168"]
    #[inline(always)]
    pub fn scuenhvio168(&self) -> Scuenhvio168R {
        Scuenhvio168R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO169"]
    #[inline(always)]
    pub fn scudispdio169(&self) -> Scudispdio169R {
        Scudispdio169R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO169"]
    #[inline(always)]
    pub fn scudispuio169(&self) -> Scudispuio169R {
        Scudispuio169R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO169"]
    #[inline(always)]
    pub fn scudrvio169(&self) -> Scudrvio169R {
        Scudrvio169R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO169"]
    #[inline(always)]
    pub fn scuensmtio169(&self) -> Scuensmtio169R {
        Scuensmtio169R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO169"]
    #[inline(always)]
    pub fn scuenhvio169(&self) -> Scuenhvio169R {
        Scuenhvio169R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO168"]
    #[inline(always)]
    pub fn scudispdio168(&mut self) -> Scudispdio168W<Scu5d0Spec> {
        Scudispdio168W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO168"]
    #[inline(always)]
    pub fn scudispuio168(&mut self) -> Scudispuio168W<Scu5d0Spec> {
        Scudispuio168W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO168"]
    #[inline(always)]
    pub fn scudrvio168(&mut self) -> Scudrvio168W<Scu5d0Spec> {
        Scudrvio168W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO168"]
    #[inline(always)]
    pub fn scuensmtio168(&mut self) -> Scuensmtio168W<Scu5d0Spec> {
        Scuensmtio168W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO168"]
    #[inline(always)]
    pub fn scuenhvio168(&mut self) -> Scuenhvio168W<Scu5d0Spec> {
        Scuenhvio168W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO169"]
    #[inline(always)]
    pub fn scudispdio169(&mut self) -> Scudispdio169W<Scu5d0Spec> {
        Scudispdio169W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO169"]
    #[inline(always)]
    pub fn scudispuio169(&mut self) -> Scudispuio169W<Scu5d0Spec> {
        Scudispuio169W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO169"]
    #[inline(always)]
    pub fn scudrvio169(&mut self) -> Scudrvio169W<Scu5d0Spec> {
        Scudrvio169W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO169"]
    #[inline(always)]
    pub fn scuensmtio169(&mut self) -> Scuensmtio169W<Scu5d0Spec> {
        Scuensmtio169W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO169"]
    #[inline(always)]
    pub fn scuenhvio169(&mut self) -> Scuenhvio169W<Scu5d0Spec> {
        Scuenhvio169W::new(self, 25)
    }
}
#[doc = "IO Control \\#85\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5d0Spec;
impl crate::RegisterSpec for Scu5d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5d0::R`](R) reader structure"]
impl crate::Readable for Scu5d0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5d0::W`](W) writer structure"]
impl crate::Writable for Scu5d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5D0 to value 0x0201_0204"]
impl crate::Resettable for Scu5d0Spec {
    const RESET_VALUE: u32 = 0x0201_0204;
}
