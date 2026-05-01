#[doc = "Register `SCU574` reader"]
pub type R = crate::R<Scu574Spec>;
#[doc = "Register `SCU574` writer"]
pub type W = crate::W<Scu574Spec>;
#[doc = "Field `SCUDISPDIO122` reader - SCU_DIS_PD_IO122"]
pub type Scudispdio122R = crate::BitReader;
#[doc = "Field `SCUDISPDIO122` writer - SCU_DIS_PD_IO122"]
pub type Scudispdio122W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO122` reader - SCU_DIS_PU_IO122"]
pub type Scudispuio122R = crate::BitReader;
#[doc = "Field `SCUDISPUIO122` writer - SCU_DIS_PU_IO122"]
pub type Scudispuio122W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO122` reader - SCU_DRV_IO122"]
pub type Scudrvio122R = crate::FieldReader;
#[doc = "Field `SCUDRVIO122` writer - SCU_DRV_IO122"]
pub type Scudrvio122W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO122` reader - SCU_EN_SMT_IO122"]
pub type Scuensmtio122R = crate::BitReader;
#[doc = "Field `SCUENSMTIO122` writer - SCU_EN_SMT_IO122"]
pub type Scuensmtio122W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO122` reader - SCU_EN_HV_IO122"]
pub type Scuenhvio122R = crate::BitReader;
#[doc = "Field `SCUENHVIO122` writer - SCU_EN_HV_IO122"]
pub type Scuenhvio122W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO123` reader - SCU_DIS_PD_IO123"]
pub type Scudispdio123R = crate::BitReader;
#[doc = "Field `SCUDISPDIO123` writer - SCU_DIS_PD_IO123"]
pub type Scudispdio123W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO123` reader - SCU_DIS_PU_IO123"]
pub type Scudispuio123R = crate::BitReader;
#[doc = "Field `SCUDISPUIO123` writer - SCU_DIS_PU_IO123"]
pub type Scudispuio123W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO123` reader - SCU_DRV_IO123"]
pub type Scudrvio123R = crate::FieldReader;
#[doc = "Field `SCUDRVIO123` writer - SCU_DRV_IO123"]
pub type Scudrvio123W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO123` reader - SCU_EN_SMT_IO123"]
pub type Scuensmtio123R = crate::BitReader;
#[doc = "Field `SCUENSMTIO123` writer - SCU_EN_SMT_IO123"]
pub type Scuensmtio123W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO123` reader - SCU_EN_HV_IO123"]
pub type Scuenhvio123R = crate::BitReader;
#[doc = "Field `SCUENHVIO123` writer - SCU_EN_HV_IO123"]
pub type Scuenhvio123W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO122"]
    #[inline(always)]
    pub fn scudispdio122(&self) -> Scudispdio122R {
        Scudispdio122R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO122"]
    #[inline(always)]
    pub fn scudispuio122(&self) -> Scudispuio122R {
        Scudispuio122R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO122"]
    #[inline(always)]
    pub fn scudrvio122(&self) -> Scudrvio122R {
        Scudrvio122R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO122"]
    #[inline(always)]
    pub fn scuensmtio122(&self) -> Scuensmtio122R {
        Scuensmtio122R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO122"]
    #[inline(always)]
    pub fn scuenhvio122(&self) -> Scuenhvio122R {
        Scuenhvio122R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO123"]
    #[inline(always)]
    pub fn scudispdio123(&self) -> Scudispdio123R {
        Scudispdio123R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO123"]
    #[inline(always)]
    pub fn scudispuio123(&self) -> Scudispuio123R {
        Scudispuio123R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO123"]
    #[inline(always)]
    pub fn scudrvio123(&self) -> Scudrvio123R {
        Scudrvio123R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO123"]
    #[inline(always)]
    pub fn scuensmtio123(&self) -> Scuensmtio123R {
        Scuensmtio123R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO123"]
    #[inline(always)]
    pub fn scuenhvio123(&self) -> Scuenhvio123R {
        Scuenhvio123R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO122"]
    #[inline(always)]
    pub fn scudispdio122(&mut self) -> Scudispdio122W<Scu574Spec> {
        Scudispdio122W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO122"]
    #[inline(always)]
    pub fn scudispuio122(&mut self) -> Scudispuio122W<Scu574Spec> {
        Scudispuio122W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO122"]
    #[inline(always)]
    pub fn scudrvio122(&mut self) -> Scudrvio122W<Scu574Spec> {
        Scudrvio122W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO122"]
    #[inline(always)]
    pub fn scuensmtio122(&mut self) -> Scuensmtio122W<Scu574Spec> {
        Scuensmtio122W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO122"]
    #[inline(always)]
    pub fn scuenhvio122(&mut self) -> Scuenhvio122W<Scu574Spec> {
        Scuenhvio122W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO123"]
    #[inline(always)]
    pub fn scudispdio123(&mut self) -> Scudispdio123W<Scu574Spec> {
        Scudispdio123W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO123"]
    #[inline(always)]
    pub fn scudispuio123(&mut self) -> Scudispuio123W<Scu574Spec> {
        Scudispuio123W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO123"]
    #[inline(always)]
    pub fn scudrvio123(&mut self) -> Scudrvio123W<Scu574Spec> {
        Scudrvio123W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO123"]
    #[inline(always)]
    pub fn scuensmtio123(&mut self) -> Scuensmtio123W<Scu574Spec> {
        Scuensmtio123W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO123"]
    #[inline(always)]
    pub fn scuenhvio123(&mut self) -> Scuenhvio123W<Scu574Spec> {
        Scuenhvio123W::new(self, 25)
    }
}
#[doc = "IO Control \\#62\n\nYou can [`read`](crate::Reg::read) this register and get [`scu574::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu574::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu574Spec;
impl crate::RegisterSpec for Scu574Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu574::R`](R) reader structure"]
impl crate::Readable for Scu574Spec {}
#[doc = "`write(|w| ..)` method takes [`scu574::W`](W) writer structure"]
impl crate::Writable for Scu574Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU574 to value 0x0204_0204"]
impl crate::Resettable for Scu574Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
