#[doc = "Register `SCU4C4` reader"]
pub type R = crate::R<Scu4c4Spec>;
#[doc = "Register `SCU4C4` writer"]
pub type W = crate::W<Scu4c4Spec>;
#[doc = "Field `SCUDISPDIO034` reader - SCU_DIS_PD_IO034"]
pub type Scudispdio034R = crate::BitReader;
#[doc = "Field `SCUDISPDIO034` writer - SCU_DIS_PD_IO034"]
pub type Scudispdio034W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO034` reader - SCU_DIS_PU_IO034"]
pub type Scudispuio034R = crate::BitReader;
#[doc = "Field `SCUDISPUIO034` writer - SCU_DIS_PU_IO034"]
pub type Scudispuio034W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO034` reader - SCU_DRV_IO034"]
pub type Scudrvio034R = crate::FieldReader;
#[doc = "Field `SCUDRVIO034` writer - SCU_DRV_IO034"]
pub type Scudrvio034W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO034` reader - SCU_EN_SMT_IO034"]
pub type Scuensmtio034R = crate::BitReader;
#[doc = "Field `SCUENSMTIO034` writer - SCU_EN_SMT_IO034"]
pub type Scuensmtio034W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO034` reader - SCU_EN_HV_IO034"]
pub type Scuenhvio034R = crate::BitReader;
#[doc = "Field `SCUENHVIO034` writer - SCU_EN_HV_IO034"]
pub type Scuenhvio034W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO035` reader - SCU_DIS_PD_IO035"]
pub type Scudispdio035R = crate::BitReader;
#[doc = "Field `SCUDISPDIO035` writer - SCU_DIS_PD_IO035"]
pub type Scudispdio035W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO035` reader - SCU_DIS_PU_IO035"]
pub type Scudispuio035R = crate::BitReader;
#[doc = "Field `SCUDISPUIO035` writer - SCU_DIS_PU_IO035"]
pub type Scudispuio035W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO035` reader - SCU_DRV_IO035"]
pub type Scudrvio035R = crate::FieldReader;
#[doc = "Field `SCUDRVIO035` writer - SCU_DRV_IO035"]
pub type Scudrvio035W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO035` reader - SCU_EN_SMT_IO035"]
pub type Scuensmtio035R = crate::BitReader;
#[doc = "Field `SCUENSMTIO035` writer - SCU_EN_SMT_IO035"]
pub type Scuensmtio035W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO035` reader - SCU_EN_HV_IO035"]
pub type Scuenhvio035R = crate::BitReader;
#[doc = "Field `SCUENHVIO035` writer - SCU_EN_HV_IO035"]
pub type Scuenhvio035W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO034"]
    #[inline(always)]
    pub fn scudispdio034(&self) -> Scudispdio034R {
        Scudispdio034R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO034"]
    #[inline(always)]
    pub fn scudispuio034(&self) -> Scudispuio034R {
        Scudispuio034R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO034"]
    #[inline(always)]
    pub fn scudrvio034(&self) -> Scudrvio034R {
        Scudrvio034R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO034"]
    #[inline(always)]
    pub fn scuensmtio034(&self) -> Scuensmtio034R {
        Scuensmtio034R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO034"]
    #[inline(always)]
    pub fn scuenhvio034(&self) -> Scuenhvio034R {
        Scuenhvio034R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO035"]
    #[inline(always)]
    pub fn scudispdio035(&self) -> Scudispdio035R {
        Scudispdio035R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO035"]
    #[inline(always)]
    pub fn scudispuio035(&self) -> Scudispuio035R {
        Scudispuio035R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO035"]
    #[inline(always)]
    pub fn scudrvio035(&self) -> Scudrvio035R {
        Scudrvio035R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO035"]
    #[inline(always)]
    pub fn scuensmtio035(&self) -> Scuensmtio035R {
        Scuensmtio035R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO035"]
    #[inline(always)]
    pub fn scuenhvio035(&self) -> Scuenhvio035R {
        Scuenhvio035R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO034"]
    #[inline(always)]
    pub fn scudispdio034(&mut self) -> Scudispdio034W<Scu4c4Spec> {
        Scudispdio034W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO034"]
    #[inline(always)]
    pub fn scudispuio034(&mut self) -> Scudispuio034W<Scu4c4Spec> {
        Scudispuio034W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO034"]
    #[inline(always)]
    pub fn scudrvio034(&mut self) -> Scudrvio034W<Scu4c4Spec> {
        Scudrvio034W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO034"]
    #[inline(always)]
    pub fn scuensmtio034(&mut self) -> Scuensmtio034W<Scu4c4Spec> {
        Scuensmtio034W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO034"]
    #[inline(always)]
    pub fn scuenhvio034(&mut self) -> Scuenhvio034W<Scu4c4Spec> {
        Scuenhvio034W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO035"]
    #[inline(always)]
    pub fn scudispdio035(&mut self) -> Scudispdio035W<Scu4c4Spec> {
        Scudispdio035W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO035"]
    #[inline(always)]
    pub fn scudispuio035(&mut self) -> Scudispuio035W<Scu4c4Spec> {
        Scudispuio035W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO035"]
    #[inline(always)]
    pub fn scudrvio035(&mut self) -> Scudrvio035W<Scu4c4Spec> {
        Scudrvio035W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO035"]
    #[inline(always)]
    pub fn scuensmtio035(&mut self) -> Scuensmtio035W<Scu4c4Spec> {
        Scuensmtio035W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO035"]
    #[inline(always)]
    pub fn scuenhvio035(&mut self) -> Scuenhvio035W<Scu4c4Spec> {
        Scuenhvio035W::new(self, 25)
    }
}
#[doc = "IO Control \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4c4Spec;
impl crate::RegisterSpec for Scu4c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4c4::R`](R) reader structure"]
impl crate::Readable for Scu4c4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4c4::W`](W) writer structure"]
impl crate::Writable for Scu4c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4C4 to value 0x0204_0204"]
impl crate::Resettable for Scu4c4Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
