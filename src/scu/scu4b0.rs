#[doc = "Register `SCU4B0` reader"]
pub type R = crate::R<Scu4b0Spec>;
#[doc = "Register `SCU4B0` writer"]
pub type W = crate::W<Scu4b0Spec>;
#[doc = "Field `SCUDISPDIO024` reader - SCU_DIS_PD_IO024"]
pub type Scudispdio024R = crate::BitReader;
#[doc = "Field `SCUDISPDIO024` writer - SCU_DIS_PD_IO024"]
pub type Scudispdio024W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO024` reader - SCU_DIS_PU_IO024"]
pub type Scudispuio024R = crate::BitReader;
#[doc = "Field `SCUDISPUIO024` writer - SCU_DIS_PU_IO024"]
pub type Scudispuio024W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO024` reader - SCU_DRV_IO024"]
pub type Scudrvio024R = crate::FieldReader;
#[doc = "Field `SCUDRVIO024` writer - SCU_DRV_IO024"]
pub type Scudrvio024W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO024` reader - SCU_EN_SMT_IO024"]
pub type Scuensmtio024R = crate::BitReader;
#[doc = "Field `SCUENSMTIO024` writer - SCU_EN_SMT_IO024"]
pub type Scuensmtio024W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO024` reader - SCU_EN_HV_IO024"]
pub type Scuenhvio024R = crate::BitReader;
#[doc = "Field `SCUENHVIO024` writer - SCU_EN_HV_IO024"]
pub type Scuenhvio024W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO025` reader - SCU_DIS_PD_IO025"]
pub type Scudispdio025R = crate::BitReader;
#[doc = "Field `SCUDISPDIO025` writer - SCU_DIS_PD_IO025"]
pub type Scudispdio025W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO025` reader - SCU_DIS_PU_IO025"]
pub type Scudispuio025R = crate::BitReader;
#[doc = "Field `SCUDISPUIO025` writer - SCU_DIS_PU_IO025"]
pub type Scudispuio025W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO025` reader - SCU_DRV_IO025"]
pub type Scudrvio025R = crate::FieldReader;
#[doc = "Field `SCUDRVIO025` writer - SCU_DRV_IO025"]
pub type Scudrvio025W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO025` reader - SCU_EN_SMT_IO025"]
pub type Scuensmtio025R = crate::BitReader;
#[doc = "Field `SCUENSMTIO025` writer - SCU_EN_SMT_IO025"]
pub type Scuensmtio025W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO025` reader - SCU_EN_HV_IO025"]
pub type Scuenhvio025R = crate::BitReader;
#[doc = "Field `SCUENHVIO025` writer - SCU_EN_HV_IO025"]
pub type Scuenhvio025W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO024"]
    #[inline(always)]
    pub fn scudispdio024(&self) -> Scudispdio024R {
        Scudispdio024R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO024"]
    #[inline(always)]
    pub fn scudispuio024(&self) -> Scudispuio024R {
        Scudispuio024R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO024"]
    #[inline(always)]
    pub fn scudrvio024(&self) -> Scudrvio024R {
        Scudrvio024R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO024"]
    #[inline(always)]
    pub fn scuensmtio024(&self) -> Scuensmtio024R {
        Scuensmtio024R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO024"]
    #[inline(always)]
    pub fn scuenhvio024(&self) -> Scuenhvio024R {
        Scuenhvio024R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO025"]
    #[inline(always)]
    pub fn scudispdio025(&self) -> Scudispdio025R {
        Scudispdio025R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO025"]
    #[inline(always)]
    pub fn scudispuio025(&self) -> Scudispuio025R {
        Scudispuio025R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO025"]
    #[inline(always)]
    pub fn scudrvio025(&self) -> Scudrvio025R {
        Scudrvio025R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO025"]
    #[inline(always)]
    pub fn scuensmtio025(&self) -> Scuensmtio025R {
        Scuensmtio025R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO025"]
    #[inline(always)]
    pub fn scuenhvio025(&self) -> Scuenhvio025R {
        Scuenhvio025R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO024"]
    #[inline(always)]
    pub fn scudispdio024(&mut self) -> Scudispdio024W<Scu4b0Spec> {
        Scudispdio024W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO024"]
    #[inline(always)]
    pub fn scudispuio024(&mut self) -> Scudispuio024W<Scu4b0Spec> {
        Scudispuio024W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO024"]
    #[inline(always)]
    pub fn scudrvio024(&mut self) -> Scudrvio024W<Scu4b0Spec> {
        Scudrvio024W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO024"]
    #[inline(always)]
    pub fn scuensmtio024(&mut self) -> Scuensmtio024W<Scu4b0Spec> {
        Scuensmtio024W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO024"]
    #[inline(always)]
    pub fn scuenhvio024(&mut self) -> Scuenhvio024W<Scu4b0Spec> {
        Scuenhvio024W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO025"]
    #[inline(always)]
    pub fn scudispdio025(&mut self) -> Scudispdio025W<Scu4b0Spec> {
        Scudispdio025W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO025"]
    #[inline(always)]
    pub fn scudispuio025(&mut self) -> Scudispuio025W<Scu4b0Spec> {
        Scudispuio025W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO025"]
    #[inline(always)]
    pub fn scudrvio025(&mut self) -> Scudrvio025W<Scu4b0Spec> {
        Scudrvio025W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO025"]
    #[inline(always)]
    pub fn scuensmtio025(&mut self) -> Scuensmtio025W<Scu4b0Spec> {
        Scuensmtio025W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO025"]
    #[inline(always)]
    pub fn scuenhvio025(&mut self) -> Scuenhvio025W<Scu4b0Spec> {
        Scuenhvio025W::new(self, 25)
    }
}
#[doc = "IO Control \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4b0Spec;
impl crate::RegisterSpec for Scu4b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4b0::R`](R) reader structure"]
impl crate::Readable for Scu4b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4b0::W`](W) writer structure"]
impl crate::Writable for Scu4b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4B0 to value 0x0204_0204"]
impl crate::Resettable for Scu4b0Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
