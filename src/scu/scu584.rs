#[doc = "Register `SCU584` reader"]
pub type R = crate::R<Scu584Spec>;
#[doc = "Register `SCU584` writer"]
pub type W = crate::W<Scu584Spec>;
#[doc = "Field `SCUDISPDIO130` reader - SCU_DIS_PD_IO130"]
pub type Scudispdio130R = crate::BitReader;
#[doc = "Field `SCUDISPDIO130` writer - SCU_DIS_PD_IO130"]
pub type Scudispdio130W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO130` reader - SCU_DIS_PU_IO130"]
pub type Scudispuio130R = crate::BitReader;
#[doc = "Field `SCUDISPUIO130` writer - SCU_DIS_PU_IO130"]
pub type Scudispuio130W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO130` reader - SCU_DRV_IO130"]
pub type Scudrvio130R = crate::FieldReader;
#[doc = "Field `SCUDRVIO130` writer - SCU_DRV_IO130"]
pub type Scudrvio130W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO130` reader - SCU_EN_SMT_IO130"]
pub type Scuensmtio130R = crate::BitReader;
#[doc = "Field `SCUENSMTIO130` writer - SCU_EN_SMT_IO130"]
pub type Scuensmtio130W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO130` reader - SCU_EN_HV_IO130"]
pub type Scuenhvio130R = crate::BitReader;
#[doc = "Field `SCUENHVIO130` writer - SCU_EN_HV_IO130"]
pub type Scuenhvio130W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO131` reader - SCU_DIS_PD_IO131"]
pub type Scudispdio131R = crate::BitReader;
#[doc = "Field `SCUDISPDIO131` writer - SCU_DIS_PD_IO131"]
pub type Scudispdio131W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO131` reader - SCU_DIS_PU_IO131"]
pub type Scudispuio131R = crate::BitReader;
#[doc = "Field `SCUDISPUIO131` writer - SCU_DIS_PU_IO131"]
pub type Scudispuio131W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO131` reader - SCU_DRV_IO131"]
pub type Scudrvio131R = crate::FieldReader;
#[doc = "Field `SCUDRVIO131` writer - SCU_DRV_IO131"]
pub type Scudrvio131W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO131` reader - SCU_EN_SMT_IO131"]
pub type Scuensmtio131R = crate::BitReader;
#[doc = "Field `SCUENSMTIO131` writer - SCU_EN_SMT_IO131"]
pub type Scuensmtio131W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO131` reader - SCU_EN_HV_IO131"]
pub type Scuenhvio131R = crate::BitReader;
#[doc = "Field `SCUENHVIO131` writer - SCU_EN_HV_IO131"]
pub type Scuenhvio131W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO130"]
    #[inline(always)]
    pub fn scudispdio130(&self) -> Scudispdio130R {
        Scudispdio130R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO130"]
    #[inline(always)]
    pub fn scudispuio130(&self) -> Scudispuio130R {
        Scudispuio130R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO130"]
    #[inline(always)]
    pub fn scudrvio130(&self) -> Scudrvio130R {
        Scudrvio130R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO130"]
    #[inline(always)]
    pub fn scuensmtio130(&self) -> Scuensmtio130R {
        Scuensmtio130R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO130"]
    #[inline(always)]
    pub fn scuenhvio130(&self) -> Scuenhvio130R {
        Scuenhvio130R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO131"]
    #[inline(always)]
    pub fn scudispdio131(&self) -> Scudispdio131R {
        Scudispdio131R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO131"]
    #[inline(always)]
    pub fn scudispuio131(&self) -> Scudispuio131R {
        Scudispuio131R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO131"]
    #[inline(always)]
    pub fn scudrvio131(&self) -> Scudrvio131R {
        Scudrvio131R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO131"]
    #[inline(always)]
    pub fn scuensmtio131(&self) -> Scuensmtio131R {
        Scuensmtio131R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO131"]
    #[inline(always)]
    pub fn scuenhvio131(&self) -> Scuenhvio131R {
        Scuenhvio131R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO130"]
    #[inline(always)]
    pub fn scudispdio130(&mut self) -> Scudispdio130W<Scu584Spec> {
        Scudispdio130W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO130"]
    #[inline(always)]
    pub fn scudispuio130(&mut self) -> Scudispuio130W<Scu584Spec> {
        Scudispuio130W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO130"]
    #[inline(always)]
    pub fn scudrvio130(&mut self) -> Scudrvio130W<Scu584Spec> {
        Scudrvio130W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO130"]
    #[inline(always)]
    pub fn scuensmtio130(&mut self) -> Scuensmtio130W<Scu584Spec> {
        Scuensmtio130W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO130"]
    #[inline(always)]
    pub fn scuenhvio130(&mut self) -> Scuenhvio130W<Scu584Spec> {
        Scuenhvio130W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO131"]
    #[inline(always)]
    pub fn scudispdio131(&mut self) -> Scudispdio131W<Scu584Spec> {
        Scudispdio131W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO131"]
    #[inline(always)]
    pub fn scudispuio131(&mut self) -> Scudispuio131W<Scu584Spec> {
        Scudispuio131W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO131"]
    #[inline(always)]
    pub fn scudrvio131(&mut self) -> Scudrvio131W<Scu584Spec> {
        Scudrvio131W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO131"]
    #[inline(always)]
    pub fn scuensmtio131(&mut self) -> Scuensmtio131W<Scu584Spec> {
        Scuensmtio131W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO131"]
    #[inline(always)]
    pub fn scuenhvio131(&mut self) -> Scuenhvio131W<Scu584Spec> {
        Scuenhvio131W::new(self, 25)
    }
}
#[doc = "IO Control \\#66\n\nYou can [`read`](crate::Reg::read) this register and get [`scu584::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu584::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu584Spec;
impl crate::RegisterSpec for Scu584Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu584::R`](R) reader structure"]
impl crate::Readable for Scu584Spec {}
#[doc = "`write(|w| ..)` method takes [`scu584::W`](W) writer structure"]
impl crate::Writable for Scu584Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU584 to value 0x0204_0204"]
impl crate::Resettable for Scu584Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
