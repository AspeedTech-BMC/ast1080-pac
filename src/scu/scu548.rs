#[doc = "Register `SCU548` reader"]
pub type R = crate::R<Scu548Spec>;
#[doc = "Register `SCU548` writer"]
pub type W = crate::W<Scu548Spec>;
#[doc = "Field `SCUDISPDIO100` reader - SCU_DIS_PD_IO100"]
pub type Scudispdio100R = crate::BitReader;
#[doc = "Field `SCUDISPDIO100` writer - SCU_DIS_PD_IO100"]
pub type Scudispdio100W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO100` reader - SCU_DIS_PU_IO100"]
pub type Scudispuio100R = crate::BitReader;
#[doc = "Field `SCUDISPUIO100` writer - SCU_DIS_PU_IO100"]
pub type Scudispuio100W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO100` reader - SCU_DRV_IO100"]
pub type Scudrvio100R = crate::FieldReader;
#[doc = "Field `SCUDRVIO100` writer - SCU_DRV_IO100"]
pub type Scudrvio100W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO100` reader - SCU_EN_SMT_IO100"]
pub type Scuensmtio100R = crate::BitReader;
#[doc = "Field `SCUENSMTIO100` writer - SCU_EN_SMT_IO100"]
pub type Scuensmtio100W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO100` reader - SCU_EN_HV_IO100"]
pub type Scuenhvio100R = crate::BitReader;
#[doc = "Field `SCUENHVIO100` writer - SCU_EN_HV_IO100"]
pub type Scuenhvio100W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO101` reader - SCU_DIS_PD_IO101"]
pub type Scudispdio101R = crate::BitReader;
#[doc = "Field `SCUDISPDIO101` writer - SCU_DIS_PD_IO101"]
pub type Scudispdio101W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO101` reader - SCU_DIS_PU_IO101"]
pub type Scudispuio101R = crate::BitReader;
#[doc = "Field `SCUDISPUIO101` writer - SCU_DIS_PU_IO101"]
pub type Scudispuio101W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO101` reader - SCU_DRV_IO101"]
pub type Scudrvio101R = crate::FieldReader;
#[doc = "Field `SCUDRVIO101` writer - SCU_DRV_IO101"]
pub type Scudrvio101W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO101` reader - SCU_EN_SMT_IO101"]
pub type Scuensmtio101R = crate::BitReader;
#[doc = "Field `SCUENSMTIO101` writer - SCU_EN_SMT_IO101"]
pub type Scuensmtio101W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO101` reader - SCU_EN_HV_IO101"]
pub type Scuenhvio101R = crate::BitReader;
#[doc = "Field `SCUENHVIO101` writer - SCU_EN_HV_IO101"]
pub type Scuenhvio101W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO100"]
    #[inline(always)]
    pub fn scudispdio100(&self) -> Scudispdio100R {
        Scudispdio100R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO100"]
    #[inline(always)]
    pub fn scudispuio100(&self) -> Scudispuio100R {
        Scudispuio100R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO100"]
    #[inline(always)]
    pub fn scudrvio100(&self) -> Scudrvio100R {
        Scudrvio100R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO100"]
    #[inline(always)]
    pub fn scuensmtio100(&self) -> Scuensmtio100R {
        Scuensmtio100R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO100"]
    #[inline(always)]
    pub fn scuenhvio100(&self) -> Scuenhvio100R {
        Scuenhvio100R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO101"]
    #[inline(always)]
    pub fn scudispdio101(&self) -> Scudispdio101R {
        Scudispdio101R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO101"]
    #[inline(always)]
    pub fn scudispuio101(&self) -> Scudispuio101R {
        Scudispuio101R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO101"]
    #[inline(always)]
    pub fn scudrvio101(&self) -> Scudrvio101R {
        Scudrvio101R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO101"]
    #[inline(always)]
    pub fn scuensmtio101(&self) -> Scuensmtio101R {
        Scuensmtio101R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO101"]
    #[inline(always)]
    pub fn scuenhvio101(&self) -> Scuenhvio101R {
        Scuenhvio101R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO100"]
    #[inline(always)]
    pub fn scudispdio100(&mut self) -> Scudispdio100W<Scu548Spec> {
        Scudispdio100W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO100"]
    #[inline(always)]
    pub fn scudispuio100(&mut self) -> Scudispuio100W<Scu548Spec> {
        Scudispuio100W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO100"]
    #[inline(always)]
    pub fn scudrvio100(&mut self) -> Scudrvio100W<Scu548Spec> {
        Scudrvio100W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO100"]
    #[inline(always)]
    pub fn scuensmtio100(&mut self) -> Scuensmtio100W<Scu548Spec> {
        Scuensmtio100W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO100"]
    #[inline(always)]
    pub fn scuenhvio100(&mut self) -> Scuenhvio100W<Scu548Spec> {
        Scuenhvio100W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO101"]
    #[inline(always)]
    pub fn scudispdio101(&mut self) -> Scudispdio101W<Scu548Spec> {
        Scudispdio101W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO101"]
    #[inline(always)]
    pub fn scudispuio101(&mut self) -> Scudispuio101W<Scu548Spec> {
        Scudispuio101W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO101"]
    #[inline(always)]
    pub fn scudrvio101(&mut self) -> Scudrvio101W<Scu548Spec> {
        Scudrvio101W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO101"]
    #[inline(always)]
    pub fn scuensmtio101(&mut self) -> Scuensmtio101W<Scu548Spec> {
        Scuensmtio101W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO101"]
    #[inline(always)]
    pub fn scuenhvio101(&mut self) -> Scuenhvio101W<Scu548Spec> {
        Scuenhvio101W::new(self, 25)
    }
}
#[doc = "IO Control \\#51\n\nYou can [`read`](crate::Reg::read) this register and get [`scu548::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu548::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu548Spec;
impl crate::RegisterSpec for Scu548Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu548::R`](R) reader structure"]
impl crate::Readable for Scu548Spec {}
#[doc = "`write(|w| ..)` method takes [`scu548::W`](W) writer structure"]
impl crate::Writable for Scu548Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU548 to value 0x0204_0204"]
impl crate::Resettable for Scu548Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
