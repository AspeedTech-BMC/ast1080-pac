#[doc = "Register `SCU4B8` reader"]
pub type R = crate::R<Scu4b8Spec>;
#[doc = "Register `SCU4B8` writer"]
pub type W = crate::W<Scu4b8Spec>;
#[doc = "Field `SCUDISPDIO028` reader - SCU_DIS_PD_IO028"]
pub type Scudispdio028R = crate::BitReader;
#[doc = "Field `SCUDISPDIO028` writer - SCU_DIS_PD_IO028"]
pub type Scudispdio028W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO028` reader - SCU_DIS_PU_IO028"]
pub type Scudispuio028R = crate::BitReader;
#[doc = "Field `SCUDISPUIO028` writer - SCU_DIS_PU_IO028"]
pub type Scudispuio028W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO028` reader - SCU_DRV_IO028"]
pub type Scudrvio028R = crate::FieldReader;
#[doc = "Field `SCUDRVIO028` writer - SCU_DRV_IO028"]
pub type Scudrvio028W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO028` reader - SCU_EN_SMT_IO028"]
pub type Scuensmtio028R = crate::BitReader;
#[doc = "Field `SCUENSMTIO028` writer - SCU_EN_SMT_IO028"]
pub type Scuensmtio028W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO028` reader - SCU_EN_HV_IO028"]
pub type Scuenhvio028R = crate::BitReader;
#[doc = "Field `SCUENHVIO028` writer - SCU_EN_HV_IO028"]
pub type Scuenhvio028W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO029` reader - SCU_DIS_PD_IO029"]
pub type Scudispdio029R = crate::BitReader;
#[doc = "Field `SCUDISPDIO029` writer - SCU_DIS_PD_IO029"]
pub type Scudispdio029W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO029` reader - SCU_DIS_PU_IO029"]
pub type Scudispuio029R = crate::BitReader;
#[doc = "Field `SCUDISPUIO029` writer - SCU_DIS_PU_IO029"]
pub type Scudispuio029W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO029` reader - SCU_DRV_IO029"]
pub type Scudrvio029R = crate::FieldReader;
#[doc = "Field `SCUDRVIO029` writer - SCU_DRV_IO029"]
pub type Scudrvio029W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO029` reader - SCU_EN_SMT_IO029"]
pub type Scuensmtio029R = crate::BitReader;
#[doc = "Field `SCUENSMTIO029` writer - SCU_EN_SMT_IO029"]
pub type Scuensmtio029W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO029` reader - SCU_EN_HV_IO029"]
pub type Scuenhvio029R = crate::BitReader;
#[doc = "Field `SCUENHVIO029` writer - SCU_EN_HV_IO029"]
pub type Scuenhvio029W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO028"]
    #[inline(always)]
    pub fn scudispdio028(&self) -> Scudispdio028R {
        Scudispdio028R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO028"]
    #[inline(always)]
    pub fn scudispuio028(&self) -> Scudispuio028R {
        Scudispuio028R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO028"]
    #[inline(always)]
    pub fn scudrvio028(&self) -> Scudrvio028R {
        Scudrvio028R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO028"]
    #[inline(always)]
    pub fn scuensmtio028(&self) -> Scuensmtio028R {
        Scuensmtio028R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO028"]
    #[inline(always)]
    pub fn scuenhvio028(&self) -> Scuenhvio028R {
        Scuenhvio028R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO029"]
    #[inline(always)]
    pub fn scudispdio029(&self) -> Scudispdio029R {
        Scudispdio029R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO029"]
    #[inline(always)]
    pub fn scudispuio029(&self) -> Scudispuio029R {
        Scudispuio029R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO029"]
    #[inline(always)]
    pub fn scudrvio029(&self) -> Scudrvio029R {
        Scudrvio029R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO029"]
    #[inline(always)]
    pub fn scuensmtio029(&self) -> Scuensmtio029R {
        Scuensmtio029R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO029"]
    #[inline(always)]
    pub fn scuenhvio029(&self) -> Scuenhvio029R {
        Scuenhvio029R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO028"]
    #[inline(always)]
    pub fn scudispdio028(&mut self) -> Scudispdio028W<Scu4b8Spec> {
        Scudispdio028W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO028"]
    #[inline(always)]
    pub fn scudispuio028(&mut self) -> Scudispuio028W<Scu4b8Spec> {
        Scudispuio028W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO028"]
    #[inline(always)]
    pub fn scudrvio028(&mut self) -> Scudrvio028W<Scu4b8Spec> {
        Scudrvio028W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO028"]
    #[inline(always)]
    pub fn scuensmtio028(&mut self) -> Scuensmtio028W<Scu4b8Spec> {
        Scuensmtio028W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO028"]
    #[inline(always)]
    pub fn scuenhvio028(&mut self) -> Scuenhvio028W<Scu4b8Spec> {
        Scuenhvio028W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO029"]
    #[inline(always)]
    pub fn scudispdio029(&mut self) -> Scudispdio029W<Scu4b8Spec> {
        Scudispdio029W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO029"]
    #[inline(always)]
    pub fn scudispuio029(&mut self) -> Scudispuio029W<Scu4b8Spec> {
        Scudispuio029W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO029"]
    #[inline(always)]
    pub fn scudrvio029(&mut self) -> Scudrvio029W<Scu4b8Spec> {
        Scudrvio029W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO029"]
    #[inline(always)]
    pub fn scuensmtio029(&mut self) -> Scuensmtio029W<Scu4b8Spec> {
        Scuensmtio029W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO029"]
    #[inline(always)]
    pub fn scuenhvio029(&mut self) -> Scuenhvio029W<Scu4b8Spec> {
        Scuenhvio029W::new(self, 25)
    }
}
#[doc = "IO Control \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4b8Spec;
impl crate::RegisterSpec for Scu4b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4b8::R`](R) reader structure"]
impl crate::Readable for Scu4b8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4b8::W`](W) writer structure"]
impl crate::Writable for Scu4b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4B8 to value 0x0204_0204"]
impl crate::Resettable for Scu4b8Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
