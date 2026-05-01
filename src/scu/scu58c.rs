#[doc = "Register `SCU58C` reader"]
pub type R = crate::R<Scu58cSpec>;
#[doc = "Register `SCU58C` writer"]
pub type W = crate::W<Scu58cSpec>;
#[doc = "Field `SCUDISPDIO134` reader - SCU_DIS_PD_IO134"]
pub type Scudispdio134R = crate::BitReader;
#[doc = "Field `SCUDISPDIO134` writer - SCU_DIS_PD_IO134"]
pub type Scudispdio134W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO134` reader - SCU_DIS_PU_IO134"]
pub type Scudispuio134R = crate::BitReader;
#[doc = "Field `SCUDISPUIO134` writer - SCU_DIS_PU_IO134"]
pub type Scudispuio134W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO134` reader - SCU_DRV_IO134"]
pub type Scudrvio134R = crate::FieldReader;
#[doc = "Field `SCUDRVIO134` writer - SCU_DRV_IO134"]
pub type Scudrvio134W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO134` reader - SCU_EN_SMT_IO134"]
pub type Scuensmtio134R = crate::BitReader;
#[doc = "Field `SCUENSMTIO134` writer - SCU_EN_SMT_IO134"]
pub type Scuensmtio134W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO134` reader - SCU_EN_HV_IO134"]
pub type Scuenhvio134R = crate::BitReader;
#[doc = "Field `SCUENHVIO134` writer - SCU_EN_HV_IO134"]
pub type Scuenhvio134W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO135` reader - SCU_DIS_PD_IO135"]
pub type Scudispdio135R = crate::BitReader;
#[doc = "Field `SCUDISPDIO135` writer - SCU_DIS_PD_IO135"]
pub type Scudispdio135W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO135` reader - SCU_DIS_PU_IO135"]
pub type Scudispuio135R = crate::BitReader;
#[doc = "Field `SCUDISPUIO135` writer - SCU_DIS_PU_IO135"]
pub type Scudispuio135W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO135` reader - SCU_DRV_IO135"]
pub type Scudrvio135R = crate::FieldReader;
#[doc = "Field `SCUDRVIO135` writer - SCU_DRV_IO135"]
pub type Scudrvio135W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO135` reader - SCU_EN_SMT_IO135"]
pub type Scuensmtio135R = crate::BitReader;
#[doc = "Field `SCUENSMTIO135` writer - SCU_EN_SMT_IO135"]
pub type Scuensmtio135W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO135` reader - SCU_EN_HV_IO135"]
pub type Scuenhvio135R = crate::BitReader;
#[doc = "Field `SCUENHVIO135` writer - SCU_EN_HV_IO135"]
pub type Scuenhvio135W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO134"]
    #[inline(always)]
    pub fn scudispdio134(&self) -> Scudispdio134R {
        Scudispdio134R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO134"]
    #[inline(always)]
    pub fn scudispuio134(&self) -> Scudispuio134R {
        Scudispuio134R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO134"]
    #[inline(always)]
    pub fn scudrvio134(&self) -> Scudrvio134R {
        Scudrvio134R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO134"]
    #[inline(always)]
    pub fn scuensmtio134(&self) -> Scuensmtio134R {
        Scuensmtio134R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO134"]
    #[inline(always)]
    pub fn scuenhvio134(&self) -> Scuenhvio134R {
        Scuenhvio134R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO135"]
    #[inline(always)]
    pub fn scudispdio135(&self) -> Scudispdio135R {
        Scudispdio135R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO135"]
    #[inline(always)]
    pub fn scudispuio135(&self) -> Scudispuio135R {
        Scudispuio135R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO135"]
    #[inline(always)]
    pub fn scudrvio135(&self) -> Scudrvio135R {
        Scudrvio135R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO135"]
    #[inline(always)]
    pub fn scuensmtio135(&self) -> Scuensmtio135R {
        Scuensmtio135R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO135"]
    #[inline(always)]
    pub fn scuenhvio135(&self) -> Scuenhvio135R {
        Scuenhvio135R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO134"]
    #[inline(always)]
    pub fn scudispdio134(&mut self) -> Scudispdio134W<Scu58cSpec> {
        Scudispdio134W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO134"]
    #[inline(always)]
    pub fn scudispuio134(&mut self) -> Scudispuio134W<Scu58cSpec> {
        Scudispuio134W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO134"]
    #[inline(always)]
    pub fn scudrvio134(&mut self) -> Scudrvio134W<Scu58cSpec> {
        Scudrvio134W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO134"]
    #[inline(always)]
    pub fn scuensmtio134(&mut self) -> Scuensmtio134W<Scu58cSpec> {
        Scuensmtio134W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO134"]
    #[inline(always)]
    pub fn scuenhvio134(&mut self) -> Scuenhvio134W<Scu58cSpec> {
        Scuenhvio134W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO135"]
    #[inline(always)]
    pub fn scudispdio135(&mut self) -> Scudispdio135W<Scu58cSpec> {
        Scudispdio135W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO135"]
    #[inline(always)]
    pub fn scudispuio135(&mut self) -> Scudispuio135W<Scu58cSpec> {
        Scudispuio135W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO135"]
    #[inline(always)]
    pub fn scudrvio135(&mut self) -> Scudrvio135W<Scu58cSpec> {
        Scudrvio135W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO135"]
    #[inline(always)]
    pub fn scuensmtio135(&mut self) -> Scuensmtio135W<Scu58cSpec> {
        Scuensmtio135W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO135"]
    #[inline(always)]
    pub fn scuenhvio135(&mut self) -> Scuenhvio135W<Scu58cSpec> {
        Scuenhvio135W::new(self, 25)
    }
}
#[doc = "IO Control \\#68\n\nYou can [`read`](crate::Reg::read) this register and get [`scu58c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu58c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu58cSpec;
impl crate::RegisterSpec for Scu58cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu58c::R`](R) reader structure"]
impl crate::Readable for Scu58cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu58c::W`](W) writer structure"]
impl crate::Writable for Scu58cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU58C to value 0x0204_0204"]
impl crate::Resettable for Scu58cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
