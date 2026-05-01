#[doc = "Register `SCU4E4` reader"]
pub type R = crate::R<Scu4e4Spec>;
#[doc = "Register `SCU4E4` writer"]
pub type W = crate::W<Scu4e4Spec>;
#[doc = "Field `SCUDISPDIO050` reader - SCU_DIS_PD_IO050"]
pub type Scudispdio050R = crate::BitReader;
#[doc = "Field `SCUDISPDIO050` writer - SCU_DIS_PD_IO050"]
pub type Scudispdio050W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO050` reader - SCU_DIS_PU_IO050"]
pub type Scudispuio050R = crate::BitReader;
#[doc = "Field `SCUDISPUIO050` writer - SCU_DIS_PU_IO050"]
pub type Scudispuio050W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO050` reader - SCU_DRV_IO050"]
pub type Scudrvio050R = crate::FieldReader;
#[doc = "Field `SCUDRVIO050` writer - SCU_DRV_IO050"]
pub type Scudrvio050W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO050` reader - SCU_EN_SMT_IO050"]
pub type Scuensmtio050R = crate::BitReader;
#[doc = "Field `SCUENSMTIO050` writer - SCU_EN_SMT_IO050"]
pub type Scuensmtio050W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO050` reader - SCU_EN_HV_IO050"]
pub type Scuenhvio050R = crate::BitReader;
#[doc = "Field `SCUENHVIO050` writer - SCU_EN_HV_IO050"]
pub type Scuenhvio050W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO051` reader - SCU_DIS_PD_IO051"]
pub type Scudispdio051R = crate::BitReader;
#[doc = "Field `SCUDISPDIO051` writer - SCU_DIS_PD_IO051"]
pub type Scudispdio051W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO051` reader - SCU_DIS_PU_IO051"]
pub type Scudispuio051R = crate::BitReader;
#[doc = "Field `SCUDISPUIO051` writer - SCU_DIS_PU_IO051"]
pub type Scudispuio051W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO051` reader - SCU_DRV_IO051"]
pub type Scudrvio051R = crate::FieldReader;
#[doc = "Field `SCUDRVIO051` writer - SCU_DRV_IO051"]
pub type Scudrvio051W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO051` reader - SCU_EN_SMT_IO051"]
pub type Scuensmtio051R = crate::BitReader;
#[doc = "Field `SCUENSMTIO051` writer - SCU_EN_SMT_IO051"]
pub type Scuensmtio051W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO051` reader - SCU_EN_HV_IO051"]
pub type Scuenhvio051R = crate::BitReader;
#[doc = "Field `SCUENHVIO051` writer - SCU_EN_HV_IO051"]
pub type Scuenhvio051W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO050"]
    #[inline(always)]
    pub fn scudispdio050(&self) -> Scudispdio050R {
        Scudispdio050R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO050"]
    #[inline(always)]
    pub fn scudispuio050(&self) -> Scudispuio050R {
        Scudispuio050R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO050"]
    #[inline(always)]
    pub fn scudrvio050(&self) -> Scudrvio050R {
        Scudrvio050R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO050"]
    #[inline(always)]
    pub fn scuensmtio050(&self) -> Scuensmtio050R {
        Scuensmtio050R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO050"]
    #[inline(always)]
    pub fn scuenhvio050(&self) -> Scuenhvio050R {
        Scuenhvio050R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO051"]
    #[inline(always)]
    pub fn scudispdio051(&self) -> Scudispdio051R {
        Scudispdio051R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO051"]
    #[inline(always)]
    pub fn scudispuio051(&self) -> Scudispuio051R {
        Scudispuio051R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO051"]
    #[inline(always)]
    pub fn scudrvio051(&self) -> Scudrvio051R {
        Scudrvio051R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO051"]
    #[inline(always)]
    pub fn scuensmtio051(&self) -> Scuensmtio051R {
        Scuensmtio051R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO051"]
    #[inline(always)]
    pub fn scuenhvio051(&self) -> Scuenhvio051R {
        Scuenhvio051R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO050"]
    #[inline(always)]
    pub fn scudispdio050(&mut self) -> Scudispdio050W<Scu4e4Spec> {
        Scudispdio050W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO050"]
    #[inline(always)]
    pub fn scudispuio050(&mut self) -> Scudispuio050W<Scu4e4Spec> {
        Scudispuio050W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO050"]
    #[inline(always)]
    pub fn scudrvio050(&mut self) -> Scudrvio050W<Scu4e4Spec> {
        Scudrvio050W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO050"]
    #[inline(always)]
    pub fn scuensmtio050(&mut self) -> Scuensmtio050W<Scu4e4Spec> {
        Scuensmtio050W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO050"]
    #[inline(always)]
    pub fn scuenhvio050(&mut self) -> Scuenhvio050W<Scu4e4Spec> {
        Scuenhvio050W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO051"]
    #[inline(always)]
    pub fn scudispdio051(&mut self) -> Scudispdio051W<Scu4e4Spec> {
        Scudispdio051W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO051"]
    #[inline(always)]
    pub fn scudispuio051(&mut self) -> Scudispuio051W<Scu4e4Spec> {
        Scudispuio051W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO051"]
    #[inline(always)]
    pub fn scudrvio051(&mut self) -> Scudrvio051W<Scu4e4Spec> {
        Scudrvio051W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO051"]
    #[inline(always)]
    pub fn scuensmtio051(&mut self) -> Scuensmtio051W<Scu4e4Spec> {
        Scuensmtio051W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO051"]
    #[inline(always)]
    pub fn scuenhvio051(&mut self) -> Scuenhvio051W<Scu4e4Spec> {
        Scuenhvio051W::new(self, 25)
    }
}
#[doc = "IO Control \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4e4Spec;
impl crate::RegisterSpec for Scu4e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4e4::R`](R) reader structure"]
impl crate::Readable for Scu4e4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4e4::W`](W) writer structure"]
impl crate::Writable for Scu4e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4E4 to value 0x0204_0204"]
impl crate::Resettable for Scu4e4Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
