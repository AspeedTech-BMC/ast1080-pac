#[doc = "Register `SCU5D4` reader"]
pub type R = crate::R<Scu5d4Spec>;
#[doc = "Register `SCU5D4` writer"]
pub type W = crate::W<Scu5d4Spec>;
#[doc = "Field `SCUDISPDIO170` reader - SCU_DIS_PD_IO170"]
pub type Scudispdio170R = crate::BitReader;
#[doc = "Field `SCUDISPDIO170` writer - SCU_DIS_PD_IO170"]
pub type Scudispdio170W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO170` reader - SCU_DIS_PU_IO170"]
pub type Scudispuio170R = crate::BitReader;
#[doc = "Field `SCUDISPUIO170` writer - SCU_DIS_PU_IO170"]
pub type Scudispuio170W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO170` reader - SCU_DRV_IO170"]
pub type Scudrvio170R = crate::FieldReader;
#[doc = "Field `SCUDRVIO170` writer - SCU_DRV_IO170"]
pub type Scudrvio170W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO170` reader - SCU_EN_SMT_IO170"]
pub type Scuensmtio170R = crate::BitReader;
#[doc = "Field `SCUENSMTIO170` writer - SCU_EN_SMT_IO170"]
pub type Scuensmtio170W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO170` reader - SCU_EN_HV_IO170"]
pub type Scuenhvio170R = crate::BitReader;
#[doc = "Field `SCUENHVIO170` writer - SCU_EN_HV_IO170"]
pub type Scuenhvio170W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO171` reader - SCU_DIS_PD_IO171"]
pub type Scudispdio171R = crate::BitReader;
#[doc = "Field `SCUDISPDIO171` writer - SCU_DIS_PD_IO171"]
pub type Scudispdio171W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO171` reader - SCU_DIS_PU_IO171"]
pub type Scudispuio171R = crate::BitReader;
#[doc = "Field `SCUDISPUIO171` writer - SCU_DIS_PU_IO171"]
pub type Scudispuio171W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO171` reader - SCU_DRV_IO171"]
pub type Scudrvio171R = crate::FieldReader;
#[doc = "Field `SCUDRVIO171` writer - SCU_DRV_IO171"]
pub type Scudrvio171W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO171` reader - SCU_EN_SMT_IO171"]
pub type Scuensmtio171R = crate::BitReader;
#[doc = "Field `SCUENSMTIO171` writer - SCU_EN_SMT_IO171"]
pub type Scuensmtio171W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO171` reader - SCU_EN_HV_IO171"]
pub type Scuenhvio171R = crate::BitReader;
#[doc = "Field `SCUENHVIO171` writer - SCU_EN_HV_IO171"]
pub type Scuenhvio171W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO170"]
    #[inline(always)]
    pub fn scudispdio170(&self) -> Scudispdio170R {
        Scudispdio170R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO170"]
    #[inline(always)]
    pub fn scudispuio170(&self) -> Scudispuio170R {
        Scudispuio170R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO170"]
    #[inline(always)]
    pub fn scudrvio170(&self) -> Scudrvio170R {
        Scudrvio170R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO170"]
    #[inline(always)]
    pub fn scuensmtio170(&self) -> Scuensmtio170R {
        Scuensmtio170R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO170"]
    #[inline(always)]
    pub fn scuenhvio170(&self) -> Scuenhvio170R {
        Scuenhvio170R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO171"]
    #[inline(always)]
    pub fn scudispdio171(&self) -> Scudispdio171R {
        Scudispdio171R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO171"]
    #[inline(always)]
    pub fn scudispuio171(&self) -> Scudispuio171R {
        Scudispuio171R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO171"]
    #[inline(always)]
    pub fn scudrvio171(&self) -> Scudrvio171R {
        Scudrvio171R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO171"]
    #[inline(always)]
    pub fn scuensmtio171(&self) -> Scuensmtio171R {
        Scuensmtio171R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO171"]
    #[inline(always)]
    pub fn scuenhvio171(&self) -> Scuenhvio171R {
        Scuenhvio171R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO170"]
    #[inline(always)]
    pub fn scudispdio170(&mut self) -> Scudispdio170W<Scu5d4Spec> {
        Scudispdio170W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO170"]
    #[inline(always)]
    pub fn scudispuio170(&mut self) -> Scudispuio170W<Scu5d4Spec> {
        Scudispuio170W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO170"]
    #[inline(always)]
    pub fn scudrvio170(&mut self) -> Scudrvio170W<Scu5d4Spec> {
        Scudrvio170W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO170"]
    #[inline(always)]
    pub fn scuensmtio170(&mut self) -> Scuensmtio170W<Scu5d4Spec> {
        Scuensmtio170W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO170"]
    #[inline(always)]
    pub fn scuenhvio170(&mut self) -> Scuenhvio170W<Scu5d4Spec> {
        Scuenhvio170W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO171"]
    #[inline(always)]
    pub fn scudispdio171(&mut self) -> Scudispdio171W<Scu5d4Spec> {
        Scudispdio171W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO171"]
    #[inline(always)]
    pub fn scudispuio171(&mut self) -> Scudispuio171W<Scu5d4Spec> {
        Scudispuio171W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO171"]
    #[inline(always)]
    pub fn scudrvio171(&mut self) -> Scudrvio171W<Scu5d4Spec> {
        Scudrvio171W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO171"]
    #[inline(always)]
    pub fn scuensmtio171(&mut self) -> Scuensmtio171W<Scu5d4Spec> {
        Scuensmtio171W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO171"]
    #[inline(always)]
    pub fn scuenhvio171(&mut self) -> Scuenhvio171W<Scu5d4Spec> {
        Scuenhvio171W::new(self, 25)
    }
}
#[doc = "IO Control \\#86\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5d4Spec;
impl crate::RegisterSpec for Scu5d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5d4::R`](R) reader structure"]
impl crate::Readable for Scu5d4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5d4::W`](W) writer structure"]
impl crate::Writable for Scu5d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5D4 to value 0x0204_0201"]
impl crate::Resettable for Scu5d4Spec {
    const RESET_VALUE: u32 = 0x0204_0201;
}
