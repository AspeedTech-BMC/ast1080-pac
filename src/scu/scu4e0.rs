#[doc = "Register `SCU4E0` reader"]
pub type R = crate::R<Scu4e0Spec>;
#[doc = "Register `SCU4E0` writer"]
pub type W = crate::W<Scu4e0Spec>;
#[doc = "Field `SCUDISPDIO048` reader - SCU_DIS_PD_IO048"]
pub type Scudispdio048R = crate::BitReader;
#[doc = "Field `SCUDISPDIO048` writer - SCU_DIS_PD_IO048"]
pub type Scudispdio048W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO048` reader - SCU_DIS_PU_IO048"]
pub type Scudispuio048R = crate::BitReader;
#[doc = "Field `SCUDISPUIO048` writer - SCU_DIS_PU_IO048"]
pub type Scudispuio048W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO048` reader - SCU_DRV_IO048"]
pub type Scudrvio048R = crate::FieldReader;
#[doc = "Field `SCUDRVIO048` writer - SCU_DRV_IO048"]
pub type Scudrvio048W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO048` reader - SCU_EN_SMT_IO048"]
pub type Scuensmtio048R = crate::BitReader;
#[doc = "Field `SCUENSMTIO048` writer - SCU_EN_SMT_IO048"]
pub type Scuensmtio048W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO048` reader - SCU_EN_HV_IO048"]
pub type Scuenhvio048R = crate::BitReader;
#[doc = "Field `SCUENHVIO048` writer - SCU_EN_HV_IO048"]
pub type Scuenhvio048W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO049` reader - SCU_DIS_PD_IO049"]
pub type Scudispdio049R = crate::BitReader;
#[doc = "Field `SCUDISPDIO049` writer - SCU_DIS_PD_IO049"]
pub type Scudispdio049W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO049` reader - SCU_DIS_PU_IO049"]
pub type Scudispuio049R = crate::BitReader;
#[doc = "Field `SCUDISPUIO049` writer - SCU_DIS_PU_IO049"]
pub type Scudispuio049W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO049` reader - SCU_DRV_IO049"]
pub type Scudrvio049R = crate::FieldReader;
#[doc = "Field `SCUDRVIO049` writer - SCU_DRV_IO049"]
pub type Scudrvio049W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO049` reader - SCU_EN_SMT_IO049"]
pub type Scuensmtio049R = crate::BitReader;
#[doc = "Field `SCUENSMTIO049` writer - SCU_EN_SMT_IO049"]
pub type Scuensmtio049W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO049` reader - SCU_EN_HV_IO049"]
pub type Scuenhvio049R = crate::BitReader;
#[doc = "Field `SCUENHVIO049` writer - SCU_EN_HV_IO049"]
pub type Scuenhvio049W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO048"]
    #[inline(always)]
    pub fn scudispdio048(&self) -> Scudispdio048R {
        Scudispdio048R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO048"]
    #[inline(always)]
    pub fn scudispuio048(&self) -> Scudispuio048R {
        Scudispuio048R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO048"]
    #[inline(always)]
    pub fn scudrvio048(&self) -> Scudrvio048R {
        Scudrvio048R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO048"]
    #[inline(always)]
    pub fn scuensmtio048(&self) -> Scuensmtio048R {
        Scuensmtio048R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO048"]
    #[inline(always)]
    pub fn scuenhvio048(&self) -> Scuenhvio048R {
        Scuenhvio048R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO049"]
    #[inline(always)]
    pub fn scudispdio049(&self) -> Scudispdio049R {
        Scudispdio049R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO049"]
    #[inline(always)]
    pub fn scudispuio049(&self) -> Scudispuio049R {
        Scudispuio049R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO049"]
    #[inline(always)]
    pub fn scudrvio049(&self) -> Scudrvio049R {
        Scudrvio049R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO049"]
    #[inline(always)]
    pub fn scuensmtio049(&self) -> Scuensmtio049R {
        Scuensmtio049R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO049"]
    #[inline(always)]
    pub fn scuenhvio049(&self) -> Scuenhvio049R {
        Scuenhvio049R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO048"]
    #[inline(always)]
    pub fn scudispdio048(&mut self) -> Scudispdio048W<Scu4e0Spec> {
        Scudispdio048W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO048"]
    #[inline(always)]
    pub fn scudispuio048(&mut self) -> Scudispuio048W<Scu4e0Spec> {
        Scudispuio048W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO048"]
    #[inline(always)]
    pub fn scudrvio048(&mut self) -> Scudrvio048W<Scu4e0Spec> {
        Scudrvio048W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO048"]
    #[inline(always)]
    pub fn scuensmtio048(&mut self) -> Scuensmtio048W<Scu4e0Spec> {
        Scuensmtio048W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO048"]
    #[inline(always)]
    pub fn scuenhvio048(&mut self) -> Scuenhvio048W<Scu4e0Spec> {
        Scuenhvio048W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO049"]
    #[inline(always)]
    pub fn scudispdio049(&mut self) -> Scudispdio049W<Scu4e0Spec> {
        Scudispdio049W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO049"]
    #[inline(always)]
    pub fn scudispuio049(&mut self) -> Scudispuio049W<Scu4e0Spec> {
        Scudispuio049W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO049"]
    #[inline(always)]
    pub fn scudrvio049(&mut self) -> Scudrvio049W<Scu4e0Spec> {
        Scudrvio049W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO049"]
    #[inline(always)]
    pub fn scuensmtio049(&mut self) -> Scuensmtio049W<Scu4e0Spec> {
        Scuensmtio049W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO049"]
    #[inline(always)]
    pub fn scuenhvio049(&mut self) -> Scuenhvio049W<Scu4e0Spec> {
        Scuenhvio049W::new(self, 25)
    }
}
#[doc = "IO Control \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4e0Spec;
impl crate::RegisterSpec for Scu4e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4e0::R`](R) reader structure"]
impl crate::Readable for Scu4e0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4e0::W`](W) writer structure"]
impl crate::Writable for Scu4e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4E0 to value 0x0204_0204"]
impl crate::Resettable for Scu4e0Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
