#[doc = "Register `SCU594` reader"]
pub type R = crate::R<Scu594Spec>;
#[doc = "Register `SCU594` writer"]
pub type W = crate::W<Scu594Spec>;
#[doc = "Field `SCUDISPDIO138` reader - SCU_DIS_PD_IO138"]
pub type Scudispdio138R = crate::BitReader;
#[doc = "Field `SCUDISPDIO138` writer - SCU_DIS_PD_IO138"]
pub type Scudispdio138W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO138` reader - SCU_DIS_PU_IO138"]
pub type Scudispuio138R = crate::BitReader;
#[doc = "Field `SCUDISPUIO138` writer - SCU_DIS_PU_IO138"]
pub type Scudispuio138W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO138` reader - SCU_DRV_IO138"]
pub type Scudrvio138R = crate::FieldReader;
#[doc = "Field `SCUDRVIO138` writer - SCU_DRV_IO138"]
pub type Scudrvio138W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO138` reader - SCU_EN_SMT_IO138"]
pub type Scuensmtio138R = crate::BitReader;
#[doc = "Field `SCUENSMTIO138` writer - SCU_EN_SMT_IO138"]
pub type Scuensmtio138W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO138` reader - SCU_EN_HV_IO138"]
pub type Scuenhvio138R = crate::BitReader;
#[doc = "Field `SCUENHVIO138` writer - SCU_EN_HV_IO138"]
pub type Scuenhvio138W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO139` reader - SCU_DIS_PD_IO139"]
pub type Scudispdio139R = crate::BitReader;
#[doc = "Field `SCUDISPDIO139` writer - SCU_DIS_PD_IO139"]
pub type Scudispdio139W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO139` reader - SCU_DIS_PU_IO139"]
pub type Scudispuio139R = crate::BitReader;
#[doc = "Field `SCUDISPUIO139` writer - SCU_DIS_PU_IO139"]
pub type Scudispuio139W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO139` reader - SCU_DRV_IO139"]
pub type Scudrvio139R = crate::FieldReader;
#[doc = "Field `SCUDRVIO139` writer - SCU_DRV_IO139"]
pub type Scudrvio139W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO139` reader - SCU_EN_SMT_IO139"]
pub type Scuensmtio139R = crate::BitReader;
#[doc = "Field `SCUENSMTIO139` writer - SCU_EN_SMT_IO139"]
pub type Scuensmtio139W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO139` reader - SCU_EN_HV_IO139"]
pub type Scuenhvio139R = crate::BitReader;
#[doc = "Field `SCUENHVIO139` writer - SCU_EN_HV_IO139"]
pub type Scuenhvio139W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO138"]
    #[inline(always)]
    pub fn scudispdio138(&self) -> Scudispdio138R {
        Scudispdio138R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO138"]
    #[inline(always)]
    pub fn scudispuio138(&self) -> Scudispuio138R {
        Scudispuio138R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO138"]
    #[inline(always)]
    pub fn scudrvio138(&self) -> Scudrvio138R {
        Scudrvio138R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO138"]
    #[inline(always)]
    pub fn scuensmtio138(&self) -> Scuensmtio138R {
        Scuensmtio138R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO138"]
    #[inline(always)]
    pub fn scuenhvio138(&self) -> Scuenhvio138R {
        Scuenhvio138R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO139"]
    #[inline(always)]
    pub fn scudispdio139(&self) -> Scudispdio139R {
        Scudispdio139R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO139"]
    #[inline(always)]
    pub fn scudispuio139(&self) -> Scudispuio139R {
        Scudispuio139R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO139"]
    #[inline(always)]
    pub fn scudrvio139(&self) -> Scudrvio139R {
        Scudrvio139R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO139"]
    #[inline(always)]
    pub fn scuensmtio139(&self) -> Scuensmtio139R {
        Scuensmtio139R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO139"]
    #[inline(always)]
    pub fn scuenhvio139(&self) -> Scuenhvio139R {
        Scuenhvio139R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO138"]
    #[inline(always)]
    pub fn scudispdio138(&mut self) -> Scudispdio138W<Scu594Spec> {
        Scudispdio138W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO138"]
    #[inline(always)]
    pub fn scudispuio138(&mut self) -> Scudispuio138W<Scu594Spec> {
        Scudispuio138W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO138"]
    #[inline(always)]
    pub fn scudrvio138(&mut self) -> Scudrvio138W<Scu594Spec> {
        Scudrvio138W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO138"]
    #[inline(always)]
    pub fn scuensmtio138(&mut self) -> Scuensmtio138W<Scu594Spec> {
        Scuensmtio138W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO138"]
    #[inline(always)]
    pub fn scuenhvio138(&mut self) -> Scuenhvio138W<Scu594Spec> {
        Scuenhvio138W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO139"]
    #[inline(always)]
    pub fn scudispdio139(&mut self) -> Scudispdio139W<Scu594Spec> {
        Scudispdio139W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO139"]
    #[inline(always)]
    pub fn scudispuio139(&mut self) -> Scudispuio139W<Scu594Spec> {
        Scudispuio139W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO139"]
    #[inline(always)]
    pub fn scudrvio139(&mut self) -> Scudrvio139W<Scu594Spec> {
        Scudrvio139W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO139"]
    #[inline(always)]
    pub fn scuensmtio139(&mut self) -> Scuensmtio139W<Scu594Spec> {
        Scuensmtio139W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO139"]
    #[inline(always)]
    pub fn scuenhvio139(&mut self) -> Scuenhvio139W<Scu594Spec> {
        Scuenhvio139W::new(self, 25)
    }
}
#[doc = "IO Control \\#70\n\nYou can [`read`](crate::Reg::read) this register and get [`scu594::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu594::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu594Spec;
impl crate::RegisterSpec for Scu594Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu594::R`](R) reader structure"]
impl crate::Readable for Scu594Spec {}
#[doc = "`write(|w| ..)` method takes [`scu594::W`](W) writer structure"]
impl crate::Writable for Scu594Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU594 to value 0x0204_0204"]
impl crate::Resettable for Scu594Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
