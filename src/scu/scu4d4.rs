#[doc = "Register `SCU4D4` reader"]
pub type R = crate::R<Scu4d4Spec>;
#[doc = "Register `SCU4D4` writer"]
pub type W = crate::W<Scu4d4Spec>;
#[doc = "Field `SCUDISPDIO042` reader - SCU_DIS_PD_IO042"]
pub type Scudispdio042R = crate::BitReader;
#[doc = "Field `SCUDISPDIO042` writer - SCU_DIS_PD_IO042"]
pub type Scudispdio042W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO042` reader - SCU_DIS_PU_IO042"]
pub type Scudispuio042R = crate::BitReader;
#[doc = "Field `SCUDISPUIO042` writer - SCU_DIS_PU_IO042"]
pub type Scudispuio042W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO042` reader - SCU_DRV_IO042"]
pub type Scudrvio042R = crate::FieldReader;
#[doc = "Field `SCUDRVIO042` writer - SCU_DRV_IO042"]
pub type Scudrvio042W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO042` reader - SCU_EN_SMT_IO042"]
pub type Scuensmtio042R = crate::BitReader;
#[doc = "Field `SCUENSMTIO042` writer - SCU_EN_SMT_IO042"]
pub type Scuensmtio042W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO042` reader - SCU_EN_HV_IO042"]
pub type Scuenhvio042R = crate::BitReader;
#[doc = "Field `SCUENHVIO042` writer - SCU_EN_HV_IO042"]
pub type Scuenhvio042W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO043` reader - SCU_DIS_PD_IO043"]
pub type Scudispdio043R = crate::BitReader;
#[doc = "Field `SCUDISPDIO043` writer - SCU_DIS_PD_IO043"]
pub type Scudispdio043W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO043` reader - SCU_DIS_PU_IO043"]
pub type Scudispuio043R = crate::BitReader;
#[doc = "Field `SCUDISPUIO043` writer - SCU_DIS_PU_IO043"]
pub type Scudispuio043W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO043` reader - SCU_DRV_IO043"]
pub type Scudrvio043R = crate::FieldReader;
#[doc = "Field `SCUDRVIO043` writer - SCU_DRV_IO043"]
pub type Scudrvio043W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO043` reader - SCU_EN_SMT_IO043"]
pub type Scuensmtio043R = crate::BitReader;
#[doc = "Field `SCUENSMTIO043` writer - SCU_EN_SMT_IO043"]
pub type Scuensmtio043W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO043` reader - SCU_EN_HV_IO043"]
pub type Scuenhvio043R = crate::BitReader;
#[doc = "Field `SCUENHVIO043` writer - SCU_EN_HV_IO043"]
pub type Scuenhvio043W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO042"]
    #[inline(always)]
    pub fn scudispdio042(&self) -> Scudispdio042R {
        Scudispdio042R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO042"]
    #[inline(always)]
    pub fn scudispuio042(&self) -> Scudispuio042R {
        Scudispuio042R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO042"]
    #[inline(always)]
    pub fn scudrvio042(&self) -> Scudrvio042R {
        Scudrvio042R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO042"]
    #[inline(always)]
    pub fn scuensmtio042(&self) -> Scuensmtio042R {
        Scuensmtio042R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO042"]
    #[inline(always)]
    pub fn scuenhvio042(&self) -> Scuenhvio042R {
        Scuenhvio042R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO043"]
    #[inline(always)]
    pub fn scudispdio043(&self) -> Scudispdio043R {
        Scudispdio043R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO043"]
    #[inline(always)]
    pub fn scudispuio043(&self) -> Scudispuio043R {
        Scudispuio043R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO043"]
    #[inline(always)]
    pub fn scudrvio043(&self) -> Scudrvio043R {
        Scudrvio043R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO043"]
    #[inline(always)]
    pub fn scuensmtio043(&self) -> Scuensmtio043R {
        Scuensmtio043R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO043"]
    #[inline(always)]
    pub fn scuenhvio043(&self) -> Scuenhvio043R {
        Scuenhvio043R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO042"]
    #[inline(always)]
    pub fn scudispdio042(&mut self) -> Scudispdio042W<Scu4d4Spec> {
        Scudispdio042W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO042"]
    #[inline(always)]
    pub fn scudispuio042(&mut self) -> Scudispuio042W<Scu4d4Spec> {
        Scudispuio042W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO042"]
    #[inline(always)]
    pub fn scudrvio042(&mut self) -> Scudrvio042W<Scu4d4Spec> {
        Scudrvio042W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO042"]
    #[inline(always)]
    pub fn scuensmtio042(&mut self) -> Scuensmtio042W<Scu4d4Spec> {
        Scuensmtio042W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO042"]
    #[inline(always)]
    pub fn scuenhvio042(&mut self) -> Scuenhvio042W<Scu4d4Spec> {
        Scuenhvio042W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO043"]
    #[inline(always)]
    pub fn scudispdio043(&mut self) -> Scudispdio043W<Scu4d4Spec> {
        Scudispdio043W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO043"]
    #[inline(always)]
    pub fn scudispuio043(&mut self) -> Scudispuio043W<Scu4d4Spec> {
        Scudispuio043W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO043"]
    #[inline(always)]
    pub fn scudrvio043(&mut self) -> Scudrvio043W<Scu4d4Spec> {
        Scudrvio043W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO043"]
    #[inline(always)]
    pub fn scuensmtio043(&mut self) -> Scuensmtio043W<Scu4d4Spec> {
        Scuensmtio043W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO043"]
    #[inline(always)]
    pub fn scuenhvio043(&mut self) -> Scuenhvio043W<Scu4d4Spec> {
        Scuenhvio043W::new(self, 25)
    }
}
#[doc = "IO Control \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4d4Spec;
impl crate::RegisterSpec for Scu4d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4d4::R`](R) reader structure"]
impl crate::Readable for Scu4d4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4d4::W`](W) writer structure"]
impl crate::Writable for Scu4d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4D4 to value 0x0204_0204"]
impl crate::Resettable for Scu4d4Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
