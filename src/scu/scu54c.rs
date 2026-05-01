#[doc = "Register `SCU54C` reader"]
pub type R = crate::R<Scu54cSpec>;
#[doc = "Register `SCU54C` writer"]
pub type W = crate::W<Scu54cSpec>;
#[doc = "Field `SCUDISPDIO102` reader - SCU_DIS_PD_IO102"]
pub type Scudispdio102R = crate::BitReader;
#[doc = "Field `SCUDISPDIO102` writer - SCU_DIS_PD_IO102"]
pub type Scudispdio102W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO102` reader - SCU_DIS_PU_IO102"]
pub type Scudispuio102R = crate::BitReader;
#[doc = "Field `SCUDISPUIO102` writer - SCU_DIS_PU_IO102"]
pub type Scudispuio102W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO102` reader - SCU_DRV_IO102"]
pub type Scudrvio102R = crate::FieldReader;
#[doc = "Field `SCUDRVIO102` writer - SCU_DRV_IO102"]
pub type Scudrvio102W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO102` reader - SCU_EN_SMT_IO102"]
pub type Scuensmtio102R = crate::BitReader;
#[doc = "Field `SCUENSMTIO102` writer - SCU_EN_SMT_IO102"]
pub type Scuensmtio102W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO102` reader - SCU_EN_HV_IO102"]
pub type Scuenhvio102R = crate::BitReader;
#[doc = "Field `SCUENHVIO102` writer - SCU_EN_HV_IO102"]
pub type Scuenhvio102W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO103` reader - SCU_DIS_PD_IO103"]
pub type Scudispdio103R = crate::BitReader;
#[doc = "Field `SCUDISPDIO103` writer - SCU_DIS_PD_IO103"]
pub type Scudispdio103W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO103` reader - SCU_DIS_PU_IO103"]
pub type Scudispuio103R = crate::BitReader;
#[doc = "Field `SCUDISPUIO103` writer - SCU_DIS_PU_IO103"]
pub type Scudispuio103W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO103` reader - SCU_DRV_IO103"]
pub type Scudrvio103R = crate::FieldReader;
#[doc = "Field `SCUDRVIO103` writer - SCU_DRV_IO103"]
pub type Scudrvio103W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO103` reader - SCU_EN_SMT_IO103"]
pub type Scuensmtio103R = crate::BitReader;
#[doc = "Field `SCUENSMTIO103` writer - SCU_EN_SMT_IO103"]
pub type Scuensmtio103W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO103` reader - SCU_EN_HV_IO103"]
pub type Scuenhvio103R = crate::BitReader;
#[doc = "Field `SCUENHVIO103` writer - SCU_EN_HV_IO103"]
pub type Scuenhvio103W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO102"]
    #[inline(always)]
    pub fn scudispdio102(&self) -> Scudispdio102R {
        Scudispdio102R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO102"]
    #[inline(always)]
    pub fn scudispuio102(&self) -> Scudispuio102R {
        Scudispuio102R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO102"]
    #[inline(always)]
    pub fn scudrvio102(&self) -> Scudrvio102R {
        Scudrvio102R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO102"]
    #[inline(always)]
    pub fn scuensmtio102(&self) -> Scuensmtio102R {
        Scuensmtio102R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO102"]
    #[inline(always)]
    pub fn scuenhvio102(&self) -> Scuenhvio102R {
        Scuenhvio102R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO103"]
    #[inline(always)]
    pub fn scudispdio103(&self) -> Scudispdio103R {
        Scudispdio103R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO103"]
    #[inline(always)]
    pub fn scudispuio103(&self) -> Scudispuio103R {
        Scudispuio103R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO103"]
    #[inline(always)]
    pub fn scudrvio103(&self) -> Scudrvio103R {
        Scudrvio103R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO103"]
    #[inline(always)]
    pub fn scuensmtio103(&self) -> Scuensmtio103R {
        Scuensmtio103R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO103"]
    #[inline(always)]
    pub fn scuenhvio103(&self) -> Scuenhvio103R {
        Scuenhvio103R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO102"]
    #[inline(always)]
    pub fn scudispdio102(&mut self) -> Scudispdio102W<Scu54cSpec> {
        Scudispdio102W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO102"]
    #[inline(always)]
    pub fn scudispuio102(&mut self) -> Scudispuio102W<Scu54cSpec> {
        Scudispuio102W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO102"]
    #[inline(always)]
    pub fn scudrvio102(&mut self) -> Scudrvio102W<Scu54cSpec> {
        Scudrvio102W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO102"]
    #[inline(always)]
    pub fn scuensmtio102(&mut self) -> Scuensmtio102W<Scu54cSpec> {
        Scuensmtio102W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO102"]
    #[inline(always)]
    pub fn scuenhvio102(&mut self) -> Scuenhvio102W<Scu54cSpec> {
        Scuenhvio102W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO103"]
    #[inline(always)]
    pub fn scudispdio103(&mut self) -> Scudispdio103W<Scu54cSpec> {
        Scudispdio103W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO103"]
    #[inline(always)]
    pub fn scudispuio103(&mut self) -> Scudispuio103W<Scu54cSpec> {
        Scudispuio103W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO103"]
    #[inline(always)]
    pub fn scudrvio103(&mut self) -> Scudrvio103W<Scu54cSpec> {
        Scudrvio103W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO103"]
    #[inline(always)]
    pub fn scuensmtio103(&mut self) -> Scuensmtio103W<Scu54cSpec> {
        Scuensmtio103W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO103"]
    #[inline(always)]
    pub fn scuenhvio103(&mut self) -> Scuenhvio103W<Scu54cSpec> {
        Scuenhvio103W::new(self, 25)
    }
}
#[doc = "IO Control \\#52\n\nYou can [`read`](crate::Reg::read) this register and get [`scu54c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu54c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu54cSpec;
impl crate::RegisterSpec for Scu54cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu54c::R`](R) reader structure"]
impl crate::Readable for Scu54cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu54c::W`](W) writer structure"]
impl crate::Writable for Scu54cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU54C to value 0x0204_0204"]
impl crate::Resettable for Scu54cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
