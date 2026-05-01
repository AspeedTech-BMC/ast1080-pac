#[doc = "Register `SCU4D0` reader"]
pub type R = crate::R<Scu4d0Spec>;
#[doc = "Register `SCU4D0` writer"]
pub type W = crate::W<Scu4d0Spec>;
#[doc = "Field `SCUDISPDIO040` reader - SCU_DIS_PD_IO040"]
pub type Scudispdio040R = crate::BitReader;
#[doc = "Field `SCUDISPDIO040` writer - SCU_DIS_PD_IO040"]
pub type Scudispdio040W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO040` reader - SCU_DIS_PU_IO040"]
pub type Scudispuio040R = crate::BitReader;
#[doc = "Field `SCUDISPUIO040` writer - SCU_DIS_PU_IO040"]
pub type Scudispuio040W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO040` reader - SCU_DRV_IO040"]
pub type Scudrvio040R = crate::FieldReader;
#[doc = "Field `SCUDRVIO040` writer - SCU_DRV_IO040"]
pub type Scudrvio040W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO040` reader - SCU_EN_SMT_IO040"]
pub type Scuensmtio040R = crate::BitReader;
#[doc = "Field `SCUENSMTIO040` writer - SCU_EN_SMT_IO040"]
pub type Scuensmtio040W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO040` reader - SCU_EN_HV_IO040"]
pub type Scuenhvio040R = crate::BitReader;
#[doc = "Field `SCUENHVIO040` writer - SCU_EN_HV_IO040"]
pub type Scuenhvio040W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO041` reader - SCU_DIS_PD_IO041"]
pub type Scudispdio041R = crate::BitReader;
#[doc = "Field `SCUDISPDIO041` writer - SCU_DIS_PD_IO041"]
pub type Scudispdio041W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO041` reader - SCU_DIS_PU_IO041"]
pub type Scudispuio041R = crate::BitReader;
#[doc = "Field `SCUDISPUIO041` writer - SCU_DIS_PU_IO041"]
pub type Scudispuio041W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO041` reader - SCU_DRV_IO041"]
pub type Scudrvio041R = crate::FieldReader;
#[doc = "Field `SCUDRVIO041` writer - SCU_DRV_IO041"]
pub type Scudrvio041W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO041` reader - SCU_EN_SMT_IO041"]
pub type Scuensmtio041R = crate::BitReader;
#[doc = "Field `SCUENSMTIO041` writer - SCU_EN_SMT_IO041"]
pub type Scuensmtio041W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO041` reader - SCU_EN_HV_IO041"]
pub type Scuenhvio041R = crate::BitReader;
#[doc = "Field `SCUENHVIO041` writer - SCU_EN_HV_IO041"]
pub type Scuenhvio041W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO040"]
    #[inline(always)]
    pub fn scudispdio040(&self) -> Scudispdio040R {
        Scudispdio040R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO040"]
    #[inline(always)]
    pub fn scudispuio040(&self) -> Scudispuio040R {
        Scudispuio040R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO040"]
    #[inline(always)]
    pub fn scudrvio040(&self) -> Scudrvio040R {
        Scudrvio040R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO040"]
    #[inline(always)]
    pub fn scuensmtio040(&self) -> Scuensmtio040R {
        Scuensmtio040R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO040"]
    #[inline(always)]
    pub fn scuenhvio040(&self) -> Scuenhvio040R {
        Scuenhvio040R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO041"]
    #[inline(always)]
    pub fn scudispdio041(&self) -> Scudispdio041R {
        Scudispdio041R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO041"]
    #[inline(always)]
    pub fn scudispuio041(&self) -> Scudispuio041R {
        Scudispuio041R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO041"]
    #[inline(always)]
    pub fn scudrvio041(&self) -> Scudrvio041R {
        Scudrvio041R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO041"]
    #[inline(always)]
    pub fn scuensmtio041(&self) -> Scuensmtio041R {
        Scuensmtio041R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO041"]
    #[inline(always)]
    pub fn scuenhvio041(&self) -> Scuenhvio041R {
        Scuenhvio041R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO040"]
    #[inline(always)]
    pub fn scudispdio040(&mut self) -> Scudispdio040W<Scu4d0Spec> {
        Scudispdio040W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO040"]
    #[inline(always)]
    pub fn scudispuio040(&mut self) -> Scudispuio040W<Scu4d0Spec> {
        Scudispuio040W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO040"]
    #[inline(always)]
    pub fn scudrvio040(&mut self) -> Scudrvio040W<Scu4d0Spec> {
        Scudrvio040W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO040"]
    #[inline(always)]
    pub fn scuensmtio040(&mut self) -> Scuensmtio040W<Scu4d0Spec> {
        Scuensmtio040W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO040"]
    #[inline(always)]
    pub fn scuenhvio040(&mut self) -> Scuenhvio040W<Scu4d0Spec> {
        Scuenhvio040W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO041"]
    #[inline(always)]
    pub fn scudispdio041(&mut self) -> Scudispdio041W<Scu4d0Spec> {
        Scudispdio041W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO041"]
    #[inline(always)]
    pub fn scudispuio041(&mut self) -> Scudispuio041W<Scu4d0Spec> {
        Scudispuio041W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO041"]
    #[inline(always)]
    pub fn scudrvio041(&mut self) -> Scudrvio041W<Scu4d0Spec> {
        Scudrvio041W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO041"]
    #[inline(always)]
    pub fn scuensmtio041(&mut self) -> Scuensmtio041W<Scu4d0Spec> {
        Scuensmtio041W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO041"]
    #[inline(always)]
    pub fn scuenhvio041(&mut self) -> Scuenhvio041W<Scu4d0Spec> {
        Scuenhvio041W::new(self, 25)
    }
}
#[doc = "IO Control \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4d0Spec;
impl crate::RegisterSpec for Scu4d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4d0::R`](R) reader structure"]
impl crate::Readable for Scu4d0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4d0::W`](W) writer structure"]
impl crate::Writable for Scu4d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4D0 to value 0x0204_0204"]
impl crate::Resettable for Scu4d0Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
