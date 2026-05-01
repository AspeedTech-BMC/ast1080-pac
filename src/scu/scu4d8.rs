#[doc = "Register `SCU4D8` reader"]
pub type R = crate::R<Scu4d8Spec>;
#[doc = "Register `SCU4D8` writer"]
pub type W = crate::W<Scu4d8Spec>;
#[doc = "Field `SCUDISPDIO044` reader - SCU_DIS_PD_IO044"]
pub type Scudispdio044R = crate::BitReader;
#[doc = "Field `SCUDISPDIO044` writer - SCU_DIS_PD_IO044"]
pub type Scudispdio044W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO044` reader - SCU_DIS_PU_IO044"]
pub type Scudispuio044R = crate::BitReader;
#[doc = "Field `SCUDISPUIO044` writer - SCU_DIS_PU_IO044"]
pub type Scudispuio044W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO044` reader - SCU_DRV_IO044"]
pub type Scudrvio044R = crate::FieldReader;
#[doc = "Field `SCUDRVIO044` writer - SCU_DRV_IO044"]
pub type Scudrvio044W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO044` reader - SCU_EN_SMT_IO044"]
pub type Scuensmtio044R = crate::BitReader;
#[doc = "Field `SCUENSMTIO044` writer - SCU_EN_SMT_IO044"]
pub type Scuensmtio044W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO044` reader - SCU_EN_HV_IO044"]
pub type Scuenhvio044R = crate::BitReader;
#[doc = "Field `SCUENHVIO044` writer - SCU_EN_HV_IO044"]
pub type Scuenhvio044W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO045` reader - SCU_DIS_PD_IO045"]
pub type Scudispdio045R = crate::BitReader;
#[doc = "Field `SCUDISPDIO045` writer - SCU_DIS_PD_IO045"]
pub type Scudispdio045W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO045` reader - SCU_DIS_PU_IO045"]
pub type Scudispuio045R = crate::BitReader;
#[doc = "Field `SCUDISPUIO045` writer - SCU_DIS_PU_IO045"]
pub type Scudispuio045W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO045` reader - SCU_DRV_IO045"]
pub type Scudrvio045R = crate::FieldReader;
#[doc = "Field `SCUDRVIO045` writer - SCU_DRV_IO045"]
pub type Scudrvio045W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO045` reader - SCU_EN_SMT_IO045"]
pub type Scuensmtio045R = crate::BitReader;
#[doc = "Field `SCUENSMTIO045` writer - SCU_EN_SMT_IO045"]
pub type Scuensmtio045W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO045` reader - SCU_EN_HV_IO045"]
pub type Scuenhvio045R = crate::BitReader;
#[doc = "Field `SCUENHVIO045` writer - SCU_EN_HV_IO045"]
pub type Scuenhvio045W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO044"]
    #[inline(always)]
    pub fn scudispdio044(&self) -> Scudispdio044R {
        Scudispdio044R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO044"]
    #[inline(always)]
    pub fn scudispuio044(&self) -> Scudispuio044R {
        Scudispuio044R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO044"]
    #[inline(always)]
    pub fn scudrvio044(&self) -> Scudrvio044R {
        Scudrvio044R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO044"]
    #[inline(always)]
    pub fn scuensmtio044(&self) -> Scuensmtio044R {
        Scuensmtio044R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO044"]
    #[inline(always)]
    pub fn scuenhvio044(&self) -> Scuenhvio044R {
        Scuenhvio044R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO045"]
    #[inline(always)]
    pub fn scudispdio045(&self) -> Scudispdio045R {
        Scudispdio045R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO045"]
    #[inline(always)]
    pub fn scudispuio045(&self) -> Scudispuio045R {
        Scudispuio045R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO045"]
    #[inline(always)]
    pub fn scudrvio045(&self) -> Scudrvio045R {
        Scudrvio045R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO045"]
    #[inline(always)]
    pub fn scuensmtio045(&self) -> Scuensmtio045R {
        Scuensmtio045R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO045"]
    #[inline(always)]
    pub fn scuenhvio045(&self) -> Scuenhvio045R {
        Scuenhvio045R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO044"]
    #[inline(always)]
    pub fn scudispdio044(&mut self) -> Scudispdio044W<Scu4d8Spec> {
        Scudispdio044W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO044"]
    #[inline(always)]
    pub fn scudispuio044(&mut self) -> Scudispuio044W<Scu4d8Spec> {
        Scudispuio044W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO044"]
    #[inline(always)]
    pub fn scudrvio044(&mut self) -> Scudrvio044W<Scu4d8Spec> {
        Scudrvio044W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO044"]
    #[inline(always)]
    pub fn scuensmtio044(&mut self) -> Scuensmtio044W<Scu4d8Spec> {
        Scuensmtio044W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO044"]
    #[inline(always)]
    pub fn scuenhvio044(&mut self) -> Scuenhvio044W<Scu4d8Spec> {
        Scuenhvio044W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO045"]
    #[inline(always)]
    pub fn scudispdio045(&mut self) -> Scudispdio045W<Scu4d8Spec> {
        Scudispdio045W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO045"]
    #[inline(always)]
    pub fn scudispuio045(&mut self) -> Scudispuio045W<Scu4d8Spec> {
        Scudispuio045W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO045"]
    #[inline(always)]
    pub fn scudrvio045(&mut self) -> Scudrvio045W<Scu4d8Spec> {
        Scudrvio045W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO045"]
    #[inline(always)]
    pub fn scuensmtio045(&mut self) -> Scuensmtio045W<Scu4d8Spec> {
        Scuensmtio045W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO045"]
    #[inline(always)]
    pub fn scuenhvio045(&mut self) -> Scuenhvio045W<Scu4d8Spec> {
        Scuenhvio045W::new(self, 25)
    }
}
#[doc = "IO Control \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4d8Spec;
impl crate::RegisterSpec for Scu4d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4d8::R`](R) reader structure"]
impl crate::Readable for Scu4d8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4d8::W`](W) writer structure"]
impl crate::Writable for Scu4d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4D8 to value 0x0204_0204"]
impl crate::Resettable for Scu4d8Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
