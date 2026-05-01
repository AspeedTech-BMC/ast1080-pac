#[doc = "Register `SCU51C` reader"]
pub type R = crate::R<Scu51cSpec>;
#[doc = "Register `SCU51C` writer"]
pub type W = crate::W<Scu51cSpec>;
#[doc = "Field `SCUDISPDIO078` reader - SCU_DIS_PD_IO078"]
pub type Scudispdio078R = crate::BitReader;
#[doc = "Field `SCUDISPDIO078` writer - SCU_DIS_PD_IO078"]
pub type Scudispdio078W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO078` reader - SCU_DIS_PU_IO078"]
pub type Scudispuio078R = crate::BitReader;
#[doc = "Field `SCUDISPUIO078` writer - SCU_DIS_PU_IO078"]
pub type Scudispuio078W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO078` reader - SCU_DRV_IO078"]
pub type Scudrvio078R = crate::FieldReader;
#[doc = "Field `SCUDRVIO078` writer - SCU_DRV_IO078"]
pub type Scudrvio078W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO078` reader - SCU_EN_SMT_IO078"]
pub type Scuensmtio078R = crate::BitReader;
#[doc = "Field `SCUENSMTIO078` writer - SCU_EN_SMT_IO078"]
pub type Scuensmtio078W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO078` reader - SCU_EN_HV_IO078"]
pub type Scuenhvio078R = crate::BitReader;
#[doc = "Field `SCUENHVIO078` writer - SCU_EN_HV_IO078"]
pub type Scuenhvio078W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO079` reader - SCU_DIS_PD_IO079"]
pub type Scudispdio079R = crate::BitReader;
#[doc = "Field `SCUDISPDIO079` writer - SCU_DIS_PD_IO079"]
pub type Scudispdio079W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO079` reader - SCU_DIS_PU_IO079"]
pub type Scudispuio079R = crate::BitReader;
#[doc = "Field `SCUDISPUIO079` writer - SCU_DIS_PU_IO079"]
pub type Scudispuio079W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO079` reader - SCU_DRV_IO079"]
pub type Scudrvio079R = crate::FieldReader;
#[doc = "Field `SCUDRVIO079` writer - SCU_DRV_IO079"]
pub type Scudrvio079W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO079` reader - SCU_EN_SMT_IO079"]
pub type Scuensmtio079R = crate::BitReader;
#[doc = "Field `SCUENSMTIO079` writer - SCU_EN_SMT_IO079"]
pub type Scuensmtio079W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO079` reader - SCU_EN_HV_IO079"]
pub type Scuenhvio079R = crate::BitReader;
#[doc = "Field `SCUENHVIO079` writer - SCU_EN_HV_IO079"]
pub type Scuenhvio079W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO078"]
    #[inline(always)]
    pub fn scudispdio078(&self) -> Scudispdio078R {
        Scudispdio078R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO078"]
    #[inline(always)]
    pub fn scudispuio078(&self) -> Scudispuio078R {
        Scudispuio078R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO078"]
    #[inline(always)]
    pub fn scudrvio078(&self) -> Scudrvio078R {
        Scudrvio078R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO078"]
    #[inline(always)]
    pub fn scuensmtio078(&self) -> Scuensmtio078R {
        Scuensmtio078R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO078"]
    #[inline(always)]
    pub fn scuenhvio078(&self) -> Scuenhvio078R {
        Scuenhvio078R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO079"]
    #[inline(always)]
    pub fn scudispdio079(&self) -> Scudispdio079R {
        Scudispdio079R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO079"]
    #[inline(always)]
    pub fn scudispuio079(&self) -> Scudispuio079R {
        Scudispuio079R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO079"]
    #[inline(always)]
    pub fn scudrvio079(&self) -> Scudrvio079R {
        Scudrvio079R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO079"]
    #[inline(always)]
    pub fn scuensmtio079(&self) -> Scuensmtio079R {
        Scuensmtio079R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO079"]
    #[inline(always)]
    pub fn scuenhvio079(&self) -> Scuenhvio079R {
        Scuenhvio079R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO078"]
    #[inline(always)]
    pub fn scudispdio078(&mut self) -> Scudispdio078W<Scu51cSpec> {
        Scudispdio078W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO078"]
    #[inline(always)]
    pub fn scudispuio078(&mut self) -> Scudispuio078W<Scu51cSpec> {
        Scudispuio078W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO078"]
    #[inline(always)]
    pub fn scudrvio078(&mut self) -> Scudrvio078W<Scu51cSpec> {
        Scudrvio078W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO078"]
    #[inline(always)]
    pub fn scuensmtio078(&mut self) -> Scuensmtio078W<Scu51cSpec> {
        Scuensmtio078W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO078"]
    #[inline(always)]
    pub fn scuenhvio078(&mut self) -> Scuenhvio078W<Scu51cSpec> {
        Scuenhvio078W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO079"]
    #[inline(always)]
    pub fn scudispdio079(&mut self) -> Scudispdio079W<Scu51cSpec> {
        Scudispdio079W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO079"]
    #[inline(always)]
    pub fn scudispuio079(&mut self) -> Scudispuio079W<Scu51cSpec> {
        Scudispuio079W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO079"]
    #[inline(always)]
    pub fn scudrvio079(&mut self) -> Scudrvio079W<Scu51cSpec> {
        Scudrvio079W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO079"]
    #[inline(always)]
    pub fn scuensmtio079(&mut self) -> Scuensmtio079W<Scu51cSpec> {
        Scuensmtio079W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO079"]
    #[inline(always)]
    pub fn scuenhvio079(&mut self) -> Scuenhvio079W<Scu51cSpec> {
        Scuenhvio079W::new(self, 25)
    }
}
#[doc = "IO Control \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`scu51c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu51c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu51cSpec;
impl crate::RegisterSpec for Scu51cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu51c::R`](R) reader structure"]
impl crate::Readable for Scu51cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu51c::W`](W) writer structure"]
impl crate::Writable for Scu51cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU51C to value 0x0204_0204"]
impl crate::Resettable for Scu51cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
