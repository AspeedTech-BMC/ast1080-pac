#[doc = "Register `SCU480` reader"]
pub type R = crate::R<Scu480Spec>;
#[doc = "Register `SCU480` writer"]
pub type W = crate::W<Scu480Spec>;
#[doc = "Field `SCUDISPDIO000` reader - SCU_DIS_PD_IO000"]
pub type Scudispdio000R = crate::BitReader;
#[doc = "Field `SCUDISPDIO000` writer - SCU_DIS_PD_IO000"]
pub type Scudispdio000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO000` reader - SCU_DIS_PU_IO000"]
pub type Scudispuio000R = crate::BitReader;
#[doc = "Field `SCUDISPUIO000` writer - SCU_DIS_PU_IO000"]
pub type Scudispuio000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO000` reader - SCU_DRV_IO000"]
pub type Scudrvio000R = crate::FieldReader;
#[doc = "Field `SCUDRVIO000` writer - SCU_DRV_IO000"]
pub type Scudrvio000W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO000` reader - SCU_EN_SMT_IO000"]
pub type Scuensmtio000R = crate::BitReader;
#[doc = "Field `SCUENSMTIO000` writer - SCU_EN_SMT_IO000"]
pub type Scuensmtio000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO000` reader - SCU_EN_HV_IO000"]
pub type Scuenhvio000R = crate::BitReader;
#[doc = "Field `SCUENHVIO000` writer - SCU_EN_HV_IO000"]
pub type Scuenhvio000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO001` reader - SCU_DIS_PD_IO001"]
pub type Scudispdio001R = crate::BitReader;
#[doc = "Field `SCUDISPDIO001` writer - SCU_DIS_PD_IO001"]
pub type Scudispdio001W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO001` reader - SCU_DIS_PU_IO001"]
pub type Scudispuio001R = crate::BitReader;
#[doc = "Field `SCUDISPUIO001` writer - SCU_DIS_PU_IO001"]
pub type Scudispuio001W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO001` reader - SCU_DRV_IO001"]
pub type Scudrvio001R = crate::FieldReader;
#[doc = "Field `SCUDRVIO001` writer - SCU_DRV_IO001"]
pub type Scudrvio001W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO001` reader - SCU_EN_SMT_IO001"]
pub type Scuensmtio001R = crate::BitReader;
#[doc = "Field `SCUENSMTIO001` writer - SCU_EN_SMT_IO001"]
pub type Scuensmtio001W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO001` reader - SCU_EN_HV_IO001"]
pub type Scuenhvio001R = crate::BitReader;
#[doc = "Field `SCUENHVIO001` writer - SCU_EN_HV_IO001"]
pub type Scuenhvio001W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO000"]
    #[inline(always)]
    pub fn scudispdio000(&self) -> Scudispdio000R {
        Scudispdio000R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO000"]
    #[inline(always)]
    pub fn scudispuio000(&self) -> Scudispuio000R {
        Scudispuio000R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO000"]
    #[inline(always)]
    pub fn scudrvio000(&self) -> Scudrvio000R {
        Scudrvio000R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO000"]
    #[inline(always)]
    pub fn scuensmtio000(&self) -> Scuensmtio000R {
        Scuensmtio000R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO000"]
    #[inline(always)]
    pub fn scuenhvio000(&self) -> Scuenhvio000R {
        Scuenhvio000R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO001"]
    #[inline(always)]
    pub fn scudispdio001(&self) -> Scudispdio001R {
        Scudispdio001R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO001"]
    #[inline(always)]
    pub fn scudispuio001(&self) -> Scudispuio001R {
        Scudispuio001R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO001"]
    #[inline(always)]
    pub fn scudrvio001(&self) -> Scudrvio001R {
        Scudrvio001R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO001"]
    #[inline(always)]
    pub fn scuensmtio001(&self) -> Scuensmtio001R {
        Scuensmtio001R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO001"]
    #[inline(always)]
    pub fn scuenhvio001(&self) -> Scuenhvio001R {
        Scuenhvio001R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO000"]
    #[inline(always)]
    pub fn scudispdio000(&mut self) -> Scudispdio000W<Scu480Spec> {
        Scudispdio000W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO000"]
    #[inline(always)]
    pub fn scudispuio000(&mut self) -> Scudispuio000W<Scu480Spec> {
        Scudispuio000W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO000"]
    #[inline(always)]
    pub fn scudrvio000(&mut self) -> Scudrvio000W<Scu480Spec> {
        Scudrvio000W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO000"]
    #[inline(always)]
    pub fn scuensmtio000(&mut self) -> Scuensmtio000W<Scu480Spec> {
        Scuensmtio000W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO000"]
    #[inline(always)]
    pub fn scuenhvio000(&mut self) -> Scuenhvio000W<Scu480Spec> {
        Scuenhvio000W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO001"]
    #[inline(always)]
    pub fn scudispdio001(&mut self) -> Scudispdio001W<Scu480Spec> {
        Scudispdio001W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO001"]
    #[inline(always)]
    pub fn scudispuio001(&mut self) -> Scudispuio001W<Scu480Spec> {
        Scudispuio001W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO001"]
    #[inline(always)]
    pub fn scudrvio001(&mut self) -> Scudrvio001W<Scu480Spec> {
        Scudrvio001W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO001"]
    #[inline(always)]
    pub fn scuensmtio001(&mut self) -> Scuensmtio001W<Scu480Spec> {
        Scuensmtio001W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO001"]
    #[inline(always)]
    pub fn scuenhvio001(&mut self) -> Scuenhvio001W<Scu480Spec> {
        Scuenhvio001W::new(self, 25)
    }
}
#[doc = "IO Control \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu480::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu480::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu480Spec;
impl crate::RegisterSpec for Scu480Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu480::R`](R) reader structure"]
impl crate::Readable for Scu480Spec {}
#[doc = "`write(|w| ..)` method takes [`scu480::W`](W) writer structure"]
impl crate::Writable for Scu480Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU480 to value 0x0204_0204"]
impl crate::Resettable for Scu480Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
