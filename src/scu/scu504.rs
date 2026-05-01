#[doc = "Register `SCU504` reader"]
pub type R = crate::R<Scu504Spec>;
#[doc = "Register `SCU504` writer"]
pub type W = crate::W<Scu504Spec>;
#[doc = "Field `SCUDISPDIO066` reader - SCU_DIS_PD_IO066"]
pub type Scudispdio066R = crate::BitReader;
#[doc = "Field `SCUDISPDIO066` writer - SCU_DIS_PD_IO066"]
pub type Scudispdio066W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO066` reader - SCU_DIS_PU_IO066"]
pub type Scudispuio066R = crate::BitReader;
#[doc = "Field `SCUDISPUIO066` writer - SCU_DIS_PU_IO066"]
pub type Scudispuio066W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO066` reader - SCU_DRV_IO066"]
pub type Scudrvio066R = crate::FieldReader;
#[doc = "Field `SCUDRVIO066` writer - SCU_DRV_IO066"]
pub type Scudrvio066W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO066` reader - SCU_EN_SMT_IO066"]
pub type Scuensmtio066R = crate::BitReader;
#[doc = "Field `SCUENSMTIO066` writer - SCU_EN_SMT_IO066"]
pub type Scuensmtio066W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO066` reader - SCU_EN_HV_IO066"]
pub type Scuenhvio066R = crate::BitReader;
#[doc = "Field `SCUENHVIO066` writer - SCU_EN_HV_IO066"]
pub type Scuenhvio066W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO067` reader - SCU_DIS_PD_IO067"]
pub type Scudispdio067R = crate::BitReader;
#[doc = "Field `SCUDISPDIO067` writer - SCU_DIS_PD_IO067"]
pub type Scudispdio067W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO067` reader - SCU_DIS_PU_IO067"]
pub type Scudispuio067R = crate::BitReader;
#[doc = "Field `SCUDISPUIO067` writer - SCU_DIS_PU_IO067"]
pub type Scudispuio067W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO067` reader - SCU_DRV_IO067"]
pub type Scudrvio067R = crate::FieldReader;
#[doc = "Field `SCUDRVIO067` writer - SCU_DRV_IO067"]
pub type Scudrvio067W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO067` reader - SCU_EN_SMT_IO067"]
pub type Scuensmtio067R = crate::BitReader;
#[doc = "Field `SCUENSMTIO067` writer - SCU_EN_SMT_IO067"]
pub type Scuensmtio067W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO067` reader - SCU_EN_HV_IO067"]
pub type Scuenhvio067R = crate::BitReader;
#[doc = "Field `SCUENHVIO067` writer - SCU_EN_HV_IO067"]
pub type Scuenhvio067W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO066"]
    #[inline(always)]
    pub fn scudispdio066(&self) -> Scudispdio066R {
        Scudispdio066R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO066"]
    #[inline(always)]
    pub fn scudispuio066(&self) -> Scudispuio066R {
        Scudispuio066R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO066"]
    #[inline(always)]
    pub fn scudrvio066(&self) -> Scudrvio066R {
        Scudrvio066R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO066"]
    #[inline(always)]
    pub fn scuensmtio066(&self) -> Scuensmtio066R {
        Scuensmtio066R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO066"]
    #[inline(always)]
    pub fn scuenhvio066(&self) -> Scuenhvio066R {
        Scuenhvio066R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO067"]
    #[inline(always)]
    pub fn scudispdio067(&self) -> Scudispdio067R {
        Scudispdio067R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO067"]
    #[inline(always)]
    pub fn scudispuio067(&self) -> Scudispuio067R {
        Scudispuio067R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO067"]
    #[inline(always)]
    pub fn scudrvio067(&self) -> Scudrvio067R {
        Scudrvio067R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO067"]
    #[inline(always)]
    pub fn scuensmtio067(&self) -> Scuensmtio067R {
        Scuensmtio067R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO067"]
    #[inline(always)]
    pub fn scuenhvio067(&self) -> Scuenhvio067R {
        Scuenhvio067R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO066"]
    #[inline(always)]
    pub fn scudispdio066(&mut self) -> Scudispdio066W<Scu504Spec> {
        Scudispdio066W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO066"]
    #[inline(always)]
    pub fn scudispuio066(&mut self) -> Scudispuio066W<Scu504Spec> {
        Scudispuio066W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO066"]
    #[inline(always)]
    pub fn scudrvio066(&mut self) -> Scudrvio066W<Scu504Spec> {
        Scudrvio066W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO066"]
    #[inline(always)]
    pub fn scuensmtio066(&mut self) -> Scuensmtio066W<Scu504Spec> {
        Scuensmtio066W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO066"]
    #[inline(always)]
    pub fn scuenhvio066(&mut self) -> Scuenhvio066W<Scu504Spec> {
        Scuenhvio066W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO067"]
    #[inline(always)]
    pub fn scudispdio067(&mut self) -> Scudispdio067W<Scu504Spec> {
        Scudispdio067W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO067"]
    #[inline(always)]
    pub fn scudispuio067(&mut self) -> Scudispuio067W<Scu504Spec> {
        Scudispuio067W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO067"]
    #[inline(always)]
    pub fn scudrvio067(&mut self) -> Scudrvio067W<Scu504Spec> {
        Scudrvio067W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO067"]
    #[inline(always)]
    pub fn scuensmtio067(&mut self) -> Scuensmtio067W<Scu504Spec> {
        Scuensmtio067W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO067"]
    #[inline(always)]
    pub fn scuenhvio067(&mut self) -> Scuenhvio067W<Scu504Spec> {
        Scuenhvio067W::new(self, 25)
    }
}
#[doc = "IO Control \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`scu504::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu504::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu504Spec;
impl crate::RegisterSpec for Scu504Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu504::R`](R) reader structure"]
impl crate::Readable for Scu504Spec {}
#[doc = "`write(|w| ..)` method takes [`scu504::W`](W) writer structure"]
impl crate::Writable for Scu504Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU504 to value 0x0204_0204"]
impl crate::Resettable for Scu504Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
