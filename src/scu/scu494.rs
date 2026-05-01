#[doc = "Register `SCU494` reader"]
pub type R = crate::R<Scu494Spec>;
#[doc = "Register `SCU494` writer"]
pub type W = crate::W<Scu494Spec>;
#[doc = "Field `SCUDISPDIO010` reader - SCU_DIS_PD_IO010"]
pub type Scudispdio010R = crate::BitReader;
#[doc = "Field `SCUDISPDIO010` writer - SCU_DIS_PD_IO010"]
pub type Scudispdio010W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO010` reader - SCU_DIS_PU_IO010"]
pub type Scudispuio010R = crate::BitReader;
#[doc = "Field `SCUDISPUIO010` writer - SCU_DIS_PU_IO010"]
pub type Scudispuio010W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO010` reader - SCU_DRV_IO010"]
pub type Scudrvio010R = crate::FieldReader;
#[doc = "Field `SCUDRVIO010` writer - SCU_DRV_IO010"]
pub type Scudrvio010W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO010` reader - SCU_EN_SMT_IO010"]
pub type Scuensmtio010R = crate::BitReader;
#[doc = "Field `SCUENSMTIO010` writer - SCU_EN_SMT_IO010"]
pub type Scuensmtio010W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO010` reader - SCU_EN_HV_IO010"]
pub type Scuenhvio010R = crate::BitReader;
#[doc = "Field `SCUENHVIO010` writer - SCU_EN_HV_IO010"]
pub type Scuenhvio010W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO011` reader - SCU_DIS_PD_IO011"]
pub type Scudispdio011R = crate::BitReader;
#[doc = "Field `SCUDISPDIO011` writer - SCU_DIS_PD_IO011"]
pub type Scudispdio011W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO011` reader - SCU_DIS_PU_IO011"]
pub type Scudispuio011R = crate::BitReader;
#[doc = "Field `SCUDISPUIO011` writer - SCU_DIS_PU_IO011"]
pub type Scudispuio011W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO011` reader - SCU_DRV_IO011"]
pub type Scudrvio011R = crate::FieldReader;
#[doc = "Field `SCUDRVIO011` writer - SCU_DRV_IO011"]
pub type Scudrvio011W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO011` reader - SCU_EN_SMT_IO011"]
pub type Scuensmtio011R = crate::BitReader;
#[doc = "Field `SCUENSMTIO011` writer - SCU_EN_SMT_IO011"]
pub type Scuensmtio011W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO011` reader - SCU_EN_HV_IO011"]
pub type Scuenhvio011R = crate::BitReader;
#[doc = "Field `SCUENHVIO011` writer - SCU_EN_HV_IO011"]
pub type Scuenhvio011W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO010"]
    #[inline(always)]
    pub fn scudispdio010(&self) -> Scudispdio010R {
        Scudispdio010R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO010"]
    #[inline(always)]
    pub fn scudispuio010(&self) -> Scudispuio010R {
        Scudispuio010R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO010"]
    #[inline(always)]
    pub fn scudrvio010(&self) -> Scudrvio010R {
        Scudrvio010R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO010"]
    #[inline(always)]
    pub fn scuensmtio010(&self) -> Scuensmtio010R {
        Scuensmtio010R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO010"]
    #[inline(always)]
    pub fn scuenhvio010(&self) -> Scuenhvio010R {
        Scuenhvio010R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO011"]
    #[inline(always)]
    pub fn scudispdio011(&self) -> Scudispdio011R {
        Scudispdio011R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO011"]
    #[inline(always)]
    pub fn scudispuio011(&self) -> Scudispuio011R {
        Scudispuio011R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO011"]
    #[inline(always)]
    pub fn scudrvio011(&self) -> Scudrvio011R {
        Scudrvio011R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO011"]
    #[inline(always)]
    pub fn scuensmtio011(&self) -> Scuensmtio011R {
        Scuensmtio011R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO011"]
    #[inline(always)]
    pub fn scuenhvio011(&self) -> Scuenhvio011R {
        Scuenhvio011R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO010"]
    #[inline(always)]
    pub fn scudispdio010(&mut self) -> Scudispdio010W<Scu494Spec> {
        Scudispdio010W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO010"]
    #[inline(always)]
    pub fn scudispuio010(&mut self) -> Scudispuio010W<Scu494Spec> {
        Scudispuio010W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO010"]
    #[inline(always)]
    pub fn scudrvio010(&mut self) -> Scudrvio010W<Scu494Spec> {
        Scudrvio010W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO010"]
    #[inline(always)]
    pub fn scuensmtio010(&mut self) -> Scuensmtio010W<Scu494Spec> {
        Scuensmtio010W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO010"]
    #[inline(always)]
    pub fn scuenhvio010(&mut self) -> Scuenhvio010W<Scu494Spec> {
        Scuenhvio010W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO011"]
    #[inline(always)]
    pub fn scudispdio011(&mut self) -> Scudispdio011W<Scu494Spec> {
        Scudispdio011W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO011"]
    #[inline(always)]
    pub fn scudispuio011(&mut self) -> Scudispuio011W<Scu494Spec> {
        Scudispuio011W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO011"]
    #[inline(always)]
    pub fn scudrvio011(&mut self) -> Scudrvio011W<Scu494Spec> {
        Scudrvio011W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO011"]
    #[inline(always)]
    pub fn scuensmtio011(&mut self) -> Scuensmtio011W<Scu494Spec> {
        Scuensmtio011W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO011"]
    #[inline(always)]
    pub fn scuenhvio011(&mut self) -> Scuenhvio011W<Scu494Spec> {
        Scuenhvio011W::new(self, 25)
    }
}
#[doc = "IO Control \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu494::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu494::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu494Spec;
impl crate::RegisterSpec for Scu494Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu494::R`](R) reader structure"]
impl crate::Readable for Scu494Spec {}
#[doc = "`write(|w| ..)` method takes [`scu494::W`](W) writer structure"]
impl crate::Writable for Scu494Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU494 to value 0x0204_0204"]
impl crate::Resettable for Scu494Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
