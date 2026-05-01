#[doc = "Register `SCU5B4` reader"]
pub type R = crate::R<Scu5b4Spec>;
#[doc = "Register `SCU5B4` writer"]
pub type W = crate::W<Scu5b4Spec>;
#[doc = "Field `SCUDISPDIO154` reader - SCU_DIS_PD_IO154"]
pub type Scudispdio154R = crate::BitReader;
#[doc = "Field `SCUDISPDIO154` writer - SCU_DIS_PD_IO154"]
pub type Scudispdio154W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO154` reader - SCU_DIS_PU_IO154"]
pub type Scudispuio154R = crate::BitReader;
#[doc = "Field `SCUDISPUIO154` writer - SCU_DIS_PU_IO154"]
pub type Scudispuio154W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO154` reader - SCU_DRV_IO154"]
pub type Scudrvio154R = crate::FieldReader;
#[doc = "Field `SCUDRVIO154` writer - SCU_DRV_IO154"]
pub type Scudrvio154W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO154` reader - SCU_EN_SMT_IO154"]
pub type Scuensmtio154R = crate::BitReader;
#[doc = "Field `SCUENSMTIO154` writer - SCU_EN_SMT_IO154"]
pub type Scuensmtio154W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO154` reader - SCU_EN_HV_IO154"]
pub type Scuenhvio154R = crate::BitReader;
#[doc = "Field `SCUENHVIO154` writer - SCU_EN_HV_IO154"]
pub type Scuenhvio154W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO155` reader - SCU_DIS_PD_IO155"]
pub type Scudispdio155R = crate::BitReader;
#[doc = "Field `SCUDISPDIO155` writer - SCU_DIS_PD_IO155"]
pub type Scudispdio155W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO155` reader - SCU_DIS_PU_IO155"]
pub type Scudispuio155R = crate::BitReader;
#[doc = "Field `SCUDISPUIO155` writer - SCU_DIS_PU_IO155"]
pub type Scudispuio155W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO155` reader - SCU_DRV_IO155"]
pub type Scudrvio155R = crate::FieldReader;
#[doc = "Field `SCUDRVIO155` writer - SCU_DRV_IO155"]
pub type Scudrvio155W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO155` reader - SCU_EN_SMT_IO155"]
pub type Scuensmtio155R = crate::BitReader;
#[doc = "Field `SCUENSMTIO155` writer - SCU_EN_SMT_IO155"]
pub type Scuensmtio155W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO155` reader - SCU_EN_HV_IO155"]
pub type Scuenhvio155R = crate::BitReader;
#[doc = "Field `SCUENHVIO155` writer - SCU_EN_HV_IO155"]
pub type Scuenhvio155W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO154"]
    #[inline(always)]
    pub fn scudispdio154(&self) -> Scudispdio154R {
        Scudispdio154R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO154"]
    #[inline(always)]
    pub fn scudispuio154(&self) -> Scudispuio154R {
        Scudispuio154R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO154"]
    #[inline(always)]
    pub fn scudrvio154(&self) -> Scudrvio154R {
        Scudrvio154R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO154"]
    #[inline(always)]
    pub fn scuensmtio154(&self) -> Scuensmtio154R {
        Scuensmtio154R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO154"]
    #[inline(always)]
    pub fn scuenhvio154(&self) -> Scuenhvio154R {
        Scuenhvio154R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO155"]
    #[inline(always)]
    pub fn scudispdio155(&self) -> Scudispdio155R {
        Scudispdio155R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO155"]
    #[inline(always)]
    pub fn scudispuio155(&self) -> Scudispuio155R {
        Scudispuio155R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO155"]
    #[inline(always)]
    pub fn scudrvio155(&self) -> Scudrvio155R {
        Scudrvio155R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO155"]
    #[inline(always)]
    pub fn scuensmtio155(&self) -> Scuensmtio155R {
        Scuensmtio155R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO155"]
    #[inline(always)]
    pub fn scuenhvio155(&self) -> Scuenhvio155R {
        Scuenhvio155R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO154"]
    #[inline(always)]
    pub fn scudispdio154(&mut self) -> Scudispdio154W<Scu5b4Spec> {
        Scudispdio154W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO154"]
    #[inline(always)]
    pub fn scudispuio154(&mut self) -> Scudispuio154W<Scu5b4Spec> {
        Scudispuio154W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO154"]
    #[inline(always)]
    pub fn scudrvio154(&mut self) -> Scudrvio154W<Scu5b4Spec> {
        Scudrvio154W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO154"]
    #[inline(always)]
    pub fn scuensmtio154(&mut self) -> Scuensmtio154W<Scu5b4Spec> {
        Scuensmtio154W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO154"]
    #[inline(always)]
    pub fn scuenhvio154(&mut self) -> Scuenhvio154W<Scu5b4Spec> {
        Scuenhvio154W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO155"]
    #[inline(always)]
    pub fn scudispdio155(&mut self) -> Scudispdio155W<Scu5b4Spec> {
        Scudispdio155W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO155"]
    #[inline(always)]
    pub fn scudispuio155(&mut self) -> Scudispuio155W<Scu5b4Spec> {
        Scudispuio155W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO155"]
    #[inline(always)]
    pub fn scudrvio155(&mut self) -> Scudrvio155W<Scu5b4Spec> {
        Scudrvio155W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO155"]
    #[inline(always)]
    pub fn scuensmtio155(&mut self) -> Scuensmtio155W<Scu5b4Spec> {
        Scuensmtio155W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO155"]
    #[inline(always)]
    pub fn scuenhvio155(&mut self) -> Scuenhvio155W<Scu5b4Spec> {
        Scuenhvio155W::new(self, 25)
    }
}
#[doc = "IO Control \\#78\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5b4Spec;
impl crate::RegisterSpec for Scu5b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5b4::R`](R) reader structure"]
impl crate::Readable for Scu5b4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5b4::W`](W) writer structure"]
impl crate::Writable for Scu5b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5B4 to value 0x0204_0204"]
impl crate::Resettable for Scu5b4Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
