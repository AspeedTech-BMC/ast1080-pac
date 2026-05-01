#[doc = "Register `SCU4E8` reader"]
pub type R = crate::R<Scu4e8Spec>;
#[doc = "Register `SCU4E8` writer"]
pub type W = crate::W<Scu4e8Spec>;
#[doc = "Field `SCUDISPDIO052` reader - SCU_DIS_PD_IO052"]
pub type Scudispdio052R = crate::BitReader;
#[doc = "Field `SCUDISPDIO052` writer - SCU_DIS_PD_IO052"]
pub type Scudispdio052W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO052` reader - SCU_DIS_PU_IO052"]
pub type Scudispuio052R = crate::BitReader;
#[doc = "Field `SCUDISPUIO052` writer - SCU_DIS_PU_IO052"]
pub type Scudispuio052W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO052` reader - SCU_DRV_IO052"]
pub type Scudrvio052R = crate::FieldReader;
#[doc = "Field `SCUDRVIO052` writer - SCU_DRV_IO052"]
pub type Scudrvio052W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO052` reader - SCU_EN_SMT_IO052"]
pub type Scuensmtio052R = crate::BitReader;
#[doc = "Field `SCUENSMTIO052` writer - SCU_EN_SMT_IO052"]
pub type Scuensmtio052W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO052` reader - SCU_EN_HV_IO052"]
pub type Scuenhvio052R = crate::BitReader;
#[doc = "Field `SCUENHVIO052` writer - SCU_EN_HV_IO052"]
pub type Scuenhvio052W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO053` reader - SCU_DIS_PD_IO053"]
pub type Scudispdio053R = crate::BitReader;
#[doc = "Field `SCUDISPDIO053` writer - SCU_DIS_PD_IO053"]
pub type Scudispdio053W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO053` reader - SCU_DIS_PU_IO053"]
pub type Scudispuio053R = crate::BitReader;
#[doc = "Field `SCUDISPUIO053` writer - SCU_DIS_PU_IO053"]
pub type Scudispuio053W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO053` reader - SCU_DRV_IO053"]
pub type Scudrvio053R = crate::FieldReader;
#[doc = "Field `SCUDRVIO053` writer - SCU_DRV_IO053"]
pub type Scudrvio053W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO053` reader - SCU_EN_SMT_IO053"]
pub type Scuensmtio053R = crate::BitReader;
#[doc = "Field `SCUENSMTIO053` writer - SCU_EN_SMT_IO053"]
pub type Scuensmtio053W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO053` reader - SCU_EN_HV_IO053"]
pub type Scuenhvio053R = crate::BitReader;
#[doc = "Field `SCUENHVIO053` writer - SCU_EN_HV_IO053"]
pub type Scuenhvio053W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO052"]
    #[inline(always)]
    pub fn scudispdio052(&self) -> Scudispdio052R {
        Scudispdio052R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO052"]
    #[inline(always)]
    pub fn scudispuio052(&self) -> Scudispuio052R {
        Scudispuio052R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO052"]
    #[inline(always)]
    pub fn scudrvio052(&self) -> Scudrvio052R {
        Scudrvio052R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO052"]
    #[inline(always)]
    pub fn scuensmtio052(&self) -> Scuensmtio052R {
        Scuensmtio052R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO052"]
    #[inline(always)]
    pub fn scuenhvio052(&self) -> Scuenhvio052R {
        Scuenhvio052R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO053"]
    #[inline(always)]
    pub fn scudispdio053(&self) -> Scudispdio053R {
        Scudispdio053R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO053"]
    #[inline(always)]
    pub fn scudispuio053(&self) -> Scudispuio053R {
        Scudispuio053R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO053"]
    #[inline(always)]
    pub fn scudrvio053(&self) -> Scudrvio053R {
        Scudrvio053R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO053"]
    #[inline(always)]
    pub fn scuensmtio053(&self) -> Scuensmtio053R {
        Scuensmtio053R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO053"]
    #[inline(always)]
    pub fn scuenhvio053(&self) -> Scuenhvio053R {
        Scuenhvio053R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO052"]
    #[inline(always)]
    pub fn scudispdio052(&mut self) -> Scudispdio052W<Scu4e8Spec> {
        Scudispdio052W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO052"]
    #[inline(always)]
    pub fn scudispuio052(&mut self) -> Scudispuio052W<Scu4e8Spec> {
        Scudispuio052W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO052"]
    #[inline(always)]
    pub fn scudrvio052(&mut self) -> Scudrvio052W<Scu4e8Spec> {
        Scudrvio052W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO052"]
    #[inline(always)]
    pub fn scuensmtio052(&mut self) -> Scuensmtio052W<Scu4e8Spec> {
        Scuensmtio052W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO052"]
    #[inline(always)]
    pub fn scuenhvio052(&mut self) -> Scuenhvio052W<Scu4e8Spec> {
        Scuenhvio052W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO053"]
    #[inline(always)]
    pub fn scudispdio053(&mut self) -> Scudispdio053W<Scu4e8Spec> {
        Scudispdio053W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO053"]
    #[inline(always)]
    pub fn scudispuio053(&mut self) -> Scudispuio053W<Scu4e8Spec> {
        Scudispuio053W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO053"]
    #[inline(always)]
    pub fn scudrvio053(&mut self) -> Scudrvio053W<Scu4e8Spec> {
        Scudrvio053W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO053"]
    #[inline(always)]
    pub fn scuensmtio053(&mut self) -> Scuensmtio053W<Scu4e8Spec> {
        Scuensmtio053W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO053"]
    #[inline(always)]
    pub fn scuenhvio053(&mut self) -> Scuenhvio053W<Scu4e8Spec> {
        Scuenhvio053W::new(self, 25)
    }
}
#[doc = "IO Control \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4e8Spec;
impl crate::RegisterSpec for Scu4e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4e8::R`](R) reader structure"]
impl crate::Readable for Scu4e8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4e8::W`](W) writer structure"]
impl crate::Writable for Scu4e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4E8 to value 0x0204_0204"]
impl crate::Resettable for Scu4e8Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
