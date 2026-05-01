#[doc = "Register `SCU5B0` reader"]
pub type R = crate::R<Scu5b0Spec>;
#[doc = "Register `SCU5B0` writer"]
pub type W = crate::W<Scu5b0Spec>;
#[doc = "Field `SCUDISPDIO152` reader - SCU_DIS_PD_IO152"]
pub type Scudispdio152R = crate::BitReader;
#[doc = "Field `SCUDISPDIO152` writer - SCU_DIS_PD_IO152"]
pub type Scudispdio152W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO152` reader - SCU_DIS_PU_IO152"]
pub type Scudispuio152R = crate::BitReader;
#[doc = "Field `SCUDISPUIO152` writer - SCU_DIS_PU_IO152"]
pub type Scudispuio152W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO152` reader - SCU_DRV_IO152"]
pub type Scudrvio152R = crate::FieldReader;
#[doc = "Field `SCUDRVIO152` writer - SCU_DRV_IO152"]
pub type Scudrvio152W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO152` reader - SCU_EN_SMT_IO152"]
pub type Scuensmtio152R = crate::BitReader;
#[doc = "Field `SCUENSMTIO152` writer - SCU_EN_SMT_IO152"]
pub type Scuensmtio152W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO152` reader - SCU_EN_HV_IO152"]
pub type Scuenhvio152R = crate::BitReader;
#[doc = "Field `SCUENHVIO152` writer - SCU_EN_HV_IO152"]
pub type Scuenhvio152W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO153` reader - SCU_DIS_PD_IO153"]
pub type Scudispdio153R = crate::BitReader;
#[doc = "Field `SCUDISPDIO153` writer - SCU_DIS_PD_IO153"]
pub type Scudispdio153W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO153` reader - SCU_DIS_PU_IO153"]
pub type Scudispuio153R = crate::BitReader;
#[doc = "Field `SCUDISPUIO153` writer - SCU_DIS_PU_IO153"]
pub type Scudispuio153W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO153` reader - SCU_DRV_IO153"]
pub type Scudrvio153R = crate::FieldReader;
#[doc = "Field `SCUDRVIO153` writer - SCU_DRV_IO153"]
pub type Scudrvio153W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO153` reader - SCU_EN_SMT_IO153"]
pub type Scuensmtio153R = crate::BitReader;
#[doc = "Field `SCUENSMTIO153` writer - SCU_EN_SMT_IO153"]
pub type Scuensmtio153W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO153` reader - SCU_EN_HV_IO153"]
pub type Scuenhvio153R = crate::BitReader;
#[doc = "Field `SCUENHVIO153` writer - SCU_EN_HV_IO153"]
pub type Scuenhvio153W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO152"]
    #[inline(always)]
    pub fn scudispdio152(&self) -> Scudispdio152R {
        Scudispdio152R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO152"]
    #[inline(always)]
    pub fn scudispuio152(&self) -> Scudispuio152R {
        Scudispuio152R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO152"]
    #[inline(always)]
    pub fn scudrvio152(&self) -> Scudrvio152R {
        Scudrvio152R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO152"]
    #[inline(always)]
    pub fn scuensmtio152(&self) -> Scuensmtio152R {
        Scuensmtio152R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO152"]
    #[inline(always)]
    pub fn scuenhvio152(&self) -> Scuenhvio152R {
        Scuenhvio152R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO153"]
    #[inline(always)]
    pub fn scudispdio153(&self) -> Scudispdio153R {
        Scudispdio153R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO153"]
    #[inline(always)]
    pub fn scudispuio153(&self) -> Scudispuio153R {
        Scudispuio153R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO153"]
    #[inline(always)]
    pub fn scudrvio153(&self) -> Scudrvio153R {
        Scudrvio153R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO153"]
    #[inline(always)]
    pub fn scuensmtio153(&self) -> Scuensmtio153R {
        Scuensmtio153R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO153"]
    #[inline(always)]
    pub fn scuenhvio153(&self) -> Scuenhvio153R {
        Scuenhvio153R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO152"]
    #[inline(always)]
    pub fn scudispdio152(&mut self) -> Scudispdio152W<Scu5b0Spec> {
        Scudispdio152W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO152"]
    #[inline(always)]
    pub fn scudispuio152(&mut self) -> Scudispuio152W<Scu5b0Spec> {
        Scudispuio152W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO152"]
    #[inline(always)]
    pub fn scudrvio152(&mut self) -> Scudrvio152W<Scu5b0Spec> {
        Scudrvio152W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO152"]
    #[inline(always)]
    pub fn scuensmtio152(&mut self) -> Scuensmtio152W<Scu5b0Spec> {
        Scuensmtio152W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO152"]
    #[inline(always)]
    pub fn scuenhvio152(&mut self) -> Scuenhvio152W<Scu5b0Spec> {
        Scuenhvio152W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO153"]
    #[inline(always)]
    pub fn scudispdio153(&mut self) -> Scudispdio153W<Scu5b0Spec> {
        Scudispdio153W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO153"]
    #[inline(always)]
    pub fn scudispuio153(&mut self) -> Scudispuio153W<Scu5b0Spec> {
        Scudispuio153W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO153"]
    #[inline(always)]
    pub fn scudrvio153(&mut self) -> Scudrvio153W<Scu5b0Spec> {
        Scudrvio153W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO153"]
    #[inline(always)]
    pub fn scuensmtio153(&mut self) -> Scuensmtio153W<Scu5b0Spec> {
        Scuensmtio153W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO153"]
    #[inline(always)]
    pub fn scuenhvio153(&mut self) -> Scuenhvio153W<Scu5b0Spec> {
        Scuenhvio153W::new(self, 25)
    }
}
#[doc = "IO Control \\#77\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5b0Spec;
impl crate::RegisterSpec for Scu5b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5b0::R`](R) reader structure"]
impl crate::Readable for Scu5b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5b0::W`](W) writer structure"]
impl crate::Writable for Scu5b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5B0 to value 0x0204_0201"]
impl crate::Resettable for Scu5b0Spec {
    const RESET_VALUE: u32 = 0x0204_0201;
}
