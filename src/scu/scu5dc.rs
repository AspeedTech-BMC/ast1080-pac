#[doc = "Register `SCU5DC` reader"]
pub type R = crate::R<Scu5dcSpec>;
#[doc = "Register `SCU5DC` writer"]
pub type W = crate::W<Scu5dcSpec>;
#[doc = "Field `SCUDISPDIO174` reader - SCU_DIS_PD_IO174"]
pub type Scudispdio174R = crate::BitReader;
#[doc = "Field `SCUDISPDIO174` writer - SCU_DIS_PD_IO174"]
pub type Scudispdio174W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO174` reader - SCU_DIS_PU_IO174"]
pub type Scudispuio174R = crate::BitReader;
#[doc = "Field `SCUDISPUIO174` writer - SCU_DIS_PU_IO174"]
pub type Scudispuio174W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO174` reader - SCU_DRV_IO174"]
pub type Scudrvio174R = crate::FieldReader;
#[doc = "Field `SCUDRVIO174` writer - SCU_DRV_IO174"]
pub type Scudrvio174W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO174` reader - SCU_EN_SMT_IO174"]
pub type Scuensmtio174R = crate::BitReader;
#[doc = "Field `SCUENSMTIO174` writer - SCU_EN_SMT_IO174"]
pub type Scuensmtio174W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO174` reader - SCU_EN_HV_IO174"]
pub type Scuenhvio174R = crate::BitReader;
#[doc = "Field `SCUENHVIO174` writer - SCU_EN_HV_IO174"]
pub type Scuenhvio174W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO175` reader - SCU_DIS_PD_IO175"]
pub type Scudispdio175R = crate::BitReader;
#[doc = "Field `SCUDISPDIO175` writer - SCU_DIS_PD_IO175"]
pub type Scudispdio175W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO175` reader - SCU_DIS_PU_IO175"]
pub type Scudispuio175R = crate::BitReader;
#[doc = "Field `SCUDISPUIO175` writer - SCU_DIS_PU_IO175"]
pub type Scudispuio175W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO175` reader - SCU_DRV_IO175"]
pub type Scudrvio175R = crate::FieldReader;
#[doc = "Field `SCUDRVIO175` writer - SCU_DRV_IO175"]
pub type Scudrvio175W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO175` reader - SCU_EN_SMT_IO175"]
pub type Scuensmtio175R = crate::BitReader;
#[doc = "Field `SCUENSMTIO175` writer - SCU_EN_SMT_IO175"]
pub type Scuensmtio175W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO175` reader - SCU_EN_HV_IO175"]
pub type Scuenhvio175R = crate::BitReader;
#[doc = "Field `SCUENHVIO175` writer - SCU_EN_HV_IO175"]
pub type Scuenhvio175W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO174"]
    #[inline(always)]
    pub fn scudispdio174(&self) -> Scudispdio174R {
        Scudispdio174R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO174"]
    #[inline(always)]
    pub fn scudispuio174(&self) -> Scudispuio174R {
        Scudispuio174R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO174"]
    #[inline(always)]
    pub fn scudrvio174(&self) -> Scudrvio174R {
        Scudrvio174R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO174"]
    #[inline(always)]
    pub fn scuensmtio174(&self) -> Scuensmtio174R {
        Scuensmtio174R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO174"]
    #[inline(always)]
    pub fn scuenhvio174(&self) -> Scuenhvio174R {
        Scuenhvio174R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO175"]
    #[inline(always)]
    pub fn scudispdio175(&self) -> Scudispdio175R {
        Scudispdio175R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO175"]
    #[inline(always)]
    pub fn scudispuio175(&self) -> Scudispuio175R {
        Scudispuio175R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO175"]
    #[inline(always)]
    pub fn scudrvio175(&self) -> Scudrvio175R {
        Scudrvio175R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO175"]
    #[inline(always)]
    pub fn scuensmtio175(&self) -> Scuensmtio175R {
        Scuensmtio175R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO175"]
    #[inline(always)]
    pub fn scuenhvio175(&self) -> Scuenhvio175R {
        Scuenhvio175R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO174"]
    #[inline(always)]
    pub fn scudispdio174(&mut self) -> Scudispdio174W<Scu5dcSpec> {
        Scudispdio174W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO174"]
    #[inline(always)]
    pub fn scudispuio174(&mut self) -> Scudispuio174W<Scu5dcSpec> {
        Scudispuio174W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO174"]
    #[inline(always)]
    pub fn scudrvio174(&mut self) -> Scudrvio174W<Scu5dcSpec> {
        Scudrvio174W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO174"]
    #[inline(always)]
    pub fn scuensmtio174(&mut self) -> Scuensmtio174W<Scu5dcSpec> {
        Scuensmtio174W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO174"]
    #[inline(always)]
    pub fn scuenhvio174(&mut self) -> Scuenhvio174W<Scu5dcSpec> {
        Scuenhvio174W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO175"]
    #[inline(always)]
    pub fn scudispdio175(&mut self) -> Scudispdio175W<Scu5dcSpec> {
        Scudispdio175W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO175"]
    #[inline(always)]
    pub fn scudispuio175(&mut self) -> Scudispuio175W<Scu5dcSpec> {
        Scudispuio175W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO175"]
    #[inline(always)]
    pub fn scudrvio175(&mut self) -> Scudrvio175W<Scu5dcSpec> {
        Scudrvio175W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO175"]
    #[inline(always)]
    pub fn scuensmtio175(&mut self) -> Scuensmtio175W<Scu5dcSpec> {
        Scuensmtio175W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO175"]
    #[inline(always)]
    pub fn scuenhvio175(&mut self) -> Scuenhvio175W<Scu5dcSpec> {
        Scuenhvio175W::new(self, 25)
    }
}
#[doc = "IO Control \\#88\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5dcSpec;
impl crate::RegisterSpec for Scu5dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5dc::R`](R) reader structure"]
impl crate::Readable for Scu5dcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu5dc::W`](W) writer structure"]
impl crate::Writable for Scu5dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5DC to value 0x0204_0204"]
impl crate::Resettable for Scu5dcSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
