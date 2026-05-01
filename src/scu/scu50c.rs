#[doc = "Register `SCU50C` reader"]
pub type R = crate::R<Scu50cSpec>;
#[doc = "Register `SCU50C` writer"]
pub type W = crate::W<Scu50cSpec>;
#[doc = "Field `SCUDISPDIO070` reader - SCU_DIS_PD_IO070"]
pub type Scudispdio070R = crate::BitReader;
#[doc = "Field `SCUDISPDIO070` writer - SCU_DIS_PD_IO070"]
pub type Scudispdio070W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO070` reader - SCU_DIS_PU_IO070"]
pub type Scudispuio070R = crate::BitReader;
#[doc = "Field `SCUDISPUIO070` writer - SCU_DIS_PU_IO070"]
pub type Scudispuio070W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO070` reader - SCU_DRV_IO070"]
pub type Scudrvio070R = crate::FieldReader;
#[doc = "Field `SCUDRVIO070` writer - SCU_DRV_IO070"]
pub type Scudrvio070W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO070` reader - SCU_EN_SMT_IO070"]
pub type Scuensmtio070R = crate::BitReader;
#[doc = "Field `SCUENSMTIO070` writer - SCU_EN_SMT_IO070"]
pub type Scuensmtio070W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO070` reader - SCU_EN_HV_IO070"]
pub type Scuenhvio070R = crate::BitReader;
#[doc = "Field `SCUENHVIO070` writer - SCU_EN_HV_IO070"]
pub type Scuenhvio070W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO071` reader - SCU_DIS_PD_IO071"]
pub type Scudispdio071R = crate::BitReader;
#[doc = "Field `SCUDISPDIO071` writer - SCU_DIS_PD_IO071"]
pub type Scudispdio071W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO071` reader - SCU_DIS_PU_IO071"]
pub type Scudispuio071R = crate::BitReader;
#[doc = "Field `SCUDISPUIO071` writer - SCU_DIS_PU_IO071"]
pub type Scudispuio071W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO071` reader - SCU_DRV_IO071"]
pub type Scudrvio071R = crate::FieldReader;
#[doc = "Field `SCUDRVIO071` writer - SCU_DRV_IO071"]
pub type Scudrvio071W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO071` reader - SCU_EN_SMT_IO071"]
pub type Scuensmtio071R = crate::BitReader;
#[doc = "Field `SCUENSMTIO071` writer - SCU_EN_SMT_IO071"]
pub type Scuensmtio071W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO071` reader - SCU_EN_HV_IO071"]
pub type Scuenhvio071R = crate::BitReader;
#[doc = "Field `SCUENHVIO071` writer - SCU_EN_HV_IO071"]
pub type Scuenhvio071W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO070"]
    #[inline(always)]
    pub fn scudispdio070(&self) -> Scudispdio070R {
        Scudispdio070R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO070"]
    #[inline(always)]
    pub fn scudispuio070(&self) -> Scudispuio070R {
        Scudispuio070R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO070"]
    #[inline(always)]
    pub fn scudrvio070(&self) -> Scudrvio070R {
        Scudrvio070R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO070"]
    #[inline(always)]
    pub fn scuensmtio070(&self) -> Scuensmtio070R {
        Scuensmtio070R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO070"]
    #[inline(always)]
    pub fn scuenhvio070(&self) -> Scuenhvio070R {
        Scuenhvio070R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO071"]
    #[inline(always)]
    pub fn scudispdio071(&self) -> Scudispdio071R {
        Scudispdio071R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO071"]
    #[inline(always)]
    pub fn scudispuio071(&self) -> Scudispuio071R {
        Scudispuio071R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO071"]
    #[inline(always)]
    pub fn scudrvio071(&self) -> Scudrvio071R {
        Scudrvio071R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO071"]
    #[inline(always)]
    pub fn scuensmtio071(&self) -> Scuensmtio071R {
        Scuensmtio071R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO071"]
    #[inline(always)]
    pub fn scuenhvio071(&self) -> Scuenhvio071R {
        Scuenhvio071R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO070"]
    #[inline(always)]
    pub fn scudispdio070(&mut self) -> Scudispdio070W<Scu50cSpec> {
        Scudispdio070W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO070"]
    #[inline(always)]
    pub fn scudispuio070(&mut self) -> Scudispuio070W<Scu50cSpec> {
        Scudispuio070W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO070"]
    #[inline(always)]
    pub fn scudrvio070(&mut self) -> Scudrvio070W<Scu50cSpec> {
        Scudrvio070W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO070"]
    #[inline(always)]
    pub fn scuensmtio070(&mut self) -> Scuensmtio070W<Scu50cSpec> {
        Scuensmtio070W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO070"]
    #[inline(always)]
    pub fn scuenhvio070(&mut self) -> Scuenhvio070W<Scu50cSpec> {
        Scuenhvio070W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO071"]
    #[inline(always)]
    pub fn scudispdio071(&mut self) -> Scudispdio071W<Scu50cSpec> {
        Scudispdio071W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO071"]
    #[inline(always)]
    pub fn scudispuio071(&mut self) -> Scudispuio071W<Scu50cSpec> {
        Scudispuio071W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO071"]
    #[inline(always)]
    pub fn scudrvio071(&mut self) -> Scudrvio071W<Scu50cSpec> {
        Scudrvio071W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO071"]
    #[inline(always)]
    pub fn scuensmtio071(&mut self) -> Scuensmtio071W<Scu50cSpec> {
        Scuensmtio071W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO071"]
    #[inline(always)]
    pub fn scuenhvio071(&mut self) -> Scuenhvio071W<Scu50cSpec> {
        Scuenhvio071W::new(self, 25)
    }
}
#[doc = "IO Control \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`scu50c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu50c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu50cSpec;
impl crate::RegisterSpec for Scu50cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu50c::R`](R) reader structure"]
impl crate::Readable for Scu50cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu50c::W`](W) writer structure"]
impl crate::Writable for Scu50cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU50C to value 0x0204_0204"]
impl crate::Resettable for Scu50cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
