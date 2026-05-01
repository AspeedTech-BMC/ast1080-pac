#[doc = "Register `SCU540` reader"]
pub type R = crate::R<Scu540Spec>;
#[doc = "Register `SCU540` writer"]
pub type W = crate::W<Scu540Spec>;
#[doc = "Field `SCUDISPDIO096` reader - SCU_DIS_PD_IO096"]
pub type Scudispdio096R = crate::BitReader;
#[doc = "Field `SCUDISPDIO096` writer - SCU_DIS_PD_IO096"]
pub type Scudispdio096W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO096` reader - SCU_DIS_PU_IO096"]
pub type Scudispuio096R = crate::BitReader;
#[doc = "Field `SCUDISPUIO096` writer - SCU_DIS_PU_IO096"]
pub type Scudispuio096W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO096` reader - SCU_DRV_IO096"]
pub type Scudrvio096R = crate::FieldReader;
#[doc = "Field `SCUDRVIO096` writer - SCU_DRV_IO096"]
pub type Scudrvio096W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO096` reader - SCU_EN_SMT_IO096"]
pub type Scuensmtio096R = crate::BitReader;
#[doc = "Field `SCUENSMTIO096` writer - SCU_EN_SMT_IO096"]
pub type Scuensmtio096W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO096` reader - SCU_EN_HV_IO096"]
pub type Scuenhvio096R = crate::BitReader;
#[doc = "Field `SCUENHVIO096` writer - SCU_EN_HV_IO096"]
pub type Scuenhvio096W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO097` reader - SCU_DIS_PD_IO097"]
pub type Scudispdio097R = crate::BitReader;
#[doc = "Field `SCUDISPDIO097` writer - SCU_DIS_PD_IO097"]
pub type Scudispdio097W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO097` reader - SCU_DIS_PU_IO097"]
pub type Scudispuio097R = crate::BitReader;
#[doc = "Field `SCUDISPUIO097` writer - SCU_DIS_PU_IO097"]
pub type Scudispuio097W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO097` reader - SCU_DRV_IO097"]
pub type Scudrvio097R = crate::FieldReader;
#[doc = "Field `SCUDRVIO097` writer - SCU_DRV_IO097"]
pub type Scudrvio097W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO097` reader - SCU_EN_SMT_IO097"]
pub type Scuensmtio097R = crate::BitReader;
#[doc = "Field `SCUENSMTIO097` writer - SCU_EN_SMT_IO097"]
pub type Scuensmtio097W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO097` reader - SCU_EN_HV_IO097"]
pub type Scuenhvio097R = crate::BitReader;
#[doc = "Field `SCUENHVIO097` writer - SCU_EN_HV_IO097"]
pub type Scuenhvio097W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO096"]
    #[inline(always)]
    pub fn scudispdio096(&self) -> Scudispdio096R {
        Scudispdio096R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO096"]
    #[inline(always)]
    pub fn scudispuio096(&self) -> Scudispuio096R {
        Scudispuio096R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO096"]
    #[inline(always)]
    pub fn scudrvio096(&self) -> Scudrvio096R {
        Scudrvio096R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO096"]
    #[inline(always)]
    pub fn scuensmtio096(&self) -> Scuensmtio096R {
        Scuensmtio096R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO096"]
    #[inline(always)]
    pub fn scuenhvio096(&self) -> Scuenhvio096R {
        Scuenhvio096R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO097"]
    #[inline(always)]
    pub fn scudispdio097(&self) -> Scudispdio097R {
        Scudispdio097R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO097"]
    #[inline(always)]
    pub fn scudispuio097(&self) -> Scudispuio097R {
        Scudispuio097R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO097"]
    #[inline(always)]
    pub fn scudrvio097(&self) -> Scudrvio097R {
        Scudrvio097R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO097"]
    #[inline(always)]
    pub fn scuensmtio097(&self) -> Scuensmtio097R {
        Scuensmtio097R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO097"]
    #[inline(always)]
    pub fn scuenhvio097(&self) -> Scuenhvio097R {
        Scuenhvio097R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO096"]
    #[inline(always)]
    pub fn scudispdio096(&mut self) -> Scudispdio096W<Scu540Spec> {
        Scudispdio096W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO096"]
    #[inline(always)]
    pub fn scudispuio096(&mut self) -> Scudispuio096W<Scu540Spec> {
        Scudispuio096W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO096"]
    #[inline(always)]
    pub fn scudrvio096(&mut self) -> Scudrvio096W<Scu540Spec> {
        Scudrvio096W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO096"]
    #[inline(always)]
    pub fn scuensmtio096(&mut self) -> Scuensmtio096W<Scu540Spec> {
        Scuensmtio096W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO096"]
    #[inline(always)]
    pub fn scuenhvio096(&mut self) -> Scuenhvio096W<Scu540Spec> {
        Scuenhvio096W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO097"]
    #[inline(always)]
    pub fn scudispdio097(&mut self) -> Scudispdio097W<Scu540Spec> {
        Scudispdio097W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO097"]
    #[inline(always)]
    pub fn scudispuio097(&mut self) -> Scudispuio097W<Scu540Spec> {
        Scudispuio097W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO097"]
    #[inline(always)]
    pub fn scudrvio097(&mut self) -> Scudrvio097W<Scu540Spec> {
        Scudrvio097W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO097"]
    #[inline(always)]
    pub fn scuensmtio097(&mut self) -> Scuensmtio097W<Scu540Spec> {
        Scuensmtio097W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO097"]
    #[inline(always)]
    pub fn scuenhvio097(&mut self) -> Scuenhvio097W<Scu540Spec> {
        Scuenhvio097W::new(self, 25)
    }
}
#[doc = "IO Control \\#49\n\nYou can [`read`](crate::Reg::read) this register and get [`scu540::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu540::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu540Spec;
impl crate::RegisterSpec for Scu540Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu540::R`](R) reader structure"]
impl crate::Readable for Scu540Spec {}
#[doc = "`write(|w| ..)` method takes [`scu540::W`](W) writer structure"]
impl crate::Writable for Scu540Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU540 to value 0x0204_0204"]
impl crate::Resettable for Scu540Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
