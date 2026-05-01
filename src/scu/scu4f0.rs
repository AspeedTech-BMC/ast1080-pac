#[doc = "Register `SCU4F0` reader"]
pub type R = crate::R<Scu4f0Spec>;
#[doc = "Register `SCU4F0` writer"]
pub type W = crate::W<Scu4f0Spec>;
#[doc = "Field `SCUDISPDIO056` reader - SCU_DIS_PD_IO056"]
pub type Scudispdio056R = crate::BitReader;
#[doc = "Field `SCUDISPDIO056` writer - SCU_DIS_PD_IO056"]
pub type Scudispdio056W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO056` reader - SCU_DIS_PU_IO056"]
pub type Scudispuio056R = crate::BitReader;
#[doc = "Field `SCUDISPUIO056` writer - SCU_DIS_PU_IO056"]
pub type Scudispuio056W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO056` reader - SCU_DRV_IO056"]
pub type Scudrvio056R = crate::FieldReader;
#[doc = "Field `SCUDRVIO056` writer - SCU_DRV_IO056"]
pub type Scudrvio056W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO056` reader - SCU_EN_SMT_IO056"]
pub type Scuensmtio056R = crate::BitReader;
#[doc = "Field `SCUENSMTIO056` writer - SCU_EN_SMT_IO056"]
pub type Scuensmtio056W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO056` reader - SCU_EN_HV_IO056"]
pub type Scuenhvio056R = crate::BitReader;
#[doc = "Field `SCUENHVIO056` writer - SCU_EN_HV_IO056"]
pub type Scuenhvio056W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO057` reader - SCU_DIS_PD_IO057"]
pub type Scudispdio057R = crate::BitReader;
#[doc = "Field `SCUDISPDIO057` writer - SCU_DIS_PD_IO057"]
pub type Scudispdio057W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO057` reader - SCU_DIS_PU_IO057"]
pub type Scudispuio057R = crate::BitReader;
#[doc = "Field `SCUDISPUIO057` writer - SCU_DIS_PU_IO057"]
pub type Scudispuio057W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO057` reader - SCU_DRV_IO057"]
pub type Scudrvio057R = crate::FieldReader;
#[doc = "Field `SCUDRVIO057` writer - SCU_DRV_IO057"]
pub type Scudrvio057W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO057` reader - SCU_EN_SMT_IO057"]
pub type Scuensmtio057R = crate::BitReader;
#[doc = "Field `SCUENSMTIO057` writer - SCU_EN_SMT_IO057"]
pub type Scuensmtio057W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO057` reader - SCU_EN_HV_IO057"]
pub type Scuenhvio057R = crate::BitReader;
#[doc = "Field `SCUENHVIO057` writer - SCU_EN_HV_IO057"]
pub type Scuenhvio057W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO056"]
    #[inline(always)]
    pub fn scudispdio056(&self) -> Scudispdio056R {
        Scudispdio056R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO056"]
    #[inline(always)]
    pub fn scudispuio056(&self) -> Scudispuio056R {
        Scudispuio056R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO056"]
    #[inline(always)]
    pub fn scudrvio056(&self) -> Scudrvio056R {
        Scudrvio056R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO056"]
    #[inline(always)]
    pub fn scuensmtio056(&self) -> Scuensmtio056R {
        Scuensmtio056R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO056"]
    #[inline(always)]
    pub fn scuenhvio056(&self) -> Scuenhvio056R {
        Scuenhvio056R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO057"]
    #[inline(always)]
    pub fn scudispdio057(&self) -> Scudispdio057R {
        Scudispdio057R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO057"]
    #[inline(always)]
    pub fn scudispuio057(&self) -> Scudispuio057R {
        Scudispuio057R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO057"]
    #[inline(always)]
    pub fn scudrvio057(&self) -> Scudrvio057R {
        Scudrvio057R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO057"]
    #[inline(always)]
    pub fn scuensmtio057(&self) -> Scuensmtio057R {
        Scuensmtio057R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO057"]
    #[inline(always)]
    pub fn scuenhvio057(&self) -> Scuenhvio057R {
        Scuenhvio057R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO056"]
    #[inline(always)]
    pub fn scudispdio056(&mut self) -> Scudispdio056W<Scu4f0Spec> {
        Scudispdio056W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO056"]
    #[inline(always)]
    pub fn scudispuio056(&mut self) -> Scudispuio056W<Scu4f0Spec> {
        Scudispuio056W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO056"]
    #[inline(always)]
    pub fn scudrvio056(&mut self) -> Scudrvio056W<Scu4f0Spec> {
        Scudrvio056W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO056"]
    #[inline(always)]
    pub fn scuensmtio056(&mut self) -> Scuensmtio056W<Scu4f0Spec> {
        Scuensmtio056W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO056"]
    #[inline(always)]
    pub fn scuenhvio056(&mut self) -> Scuenhvio056W<Scu4f0Spec> {
        Scuenhvio056W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO057"]
    #[inline(always)]
    pub fn scudispdio057(&mut self) -> Scudispdio057W<Scu4f0Spec> {
        Scudispdio057W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO057"]
    #[inline(always)]
    pub fn scudispuio057(&mut self) -> Scudispuio057W<Scu4f0Spec> {
        Scudispuio057W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO057"]
    #[inline(always)]
    pub fn scudrvio057(&mut self) -> Scudrvio057W<Scu4f0Spec> {
        Scudrvio057W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO057"]
    #[inline(always)]
    pub fn scuensmtio057(&mut self) -> Scuensmtio057W<Scu4f0Spec> {
        Scuensmtio057W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO057"]
    #[inline(always)]
    pub fn scuenhvio057(&mut self) -> Scuenhvio057W<Scu4f0Spec> {
        Scuenhvio057W::new(self, 25)
    }
}
#[doc = "IO Control \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4f0Spec;
impl crate::RegisterSpec for Scu4f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4f0::R`](R) reader structure"]
impl crate::Readable for Scu4f0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4f0::W`](W) writer structure"]
impl crate::Writable for Scu4f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4F0 to value 0x0204_0204"]
impl crate::Resettable for Scu4f0Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
