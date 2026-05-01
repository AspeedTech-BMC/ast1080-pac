#[doc = "Register `SCU5A4` reader"]
pub type R = crate::R<Scu5a4Spec>;
#[doc = "Register `SCU5A4` writer"]
pub type W = crate::W<Scu5a4Spec>;
#[doc = "Field `SCUDISPDIO146` reader - SCU_DIS_PD_IO146"]
pub type Scudispdio146R = crate::BitReader;
#[doc = "Field `SCUDISPDIO146` writer - SCU_DIS_PD_IO146"]
pub type Scudispdio146W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO146` reader - SCU_DIS_PU_IO146"]
pub type Scudispuio146R = crate::BitReader;
#[doc = "Field `SCUDISPUIO146` writer - SCU_DIS_PU_IO146"]
pub type Scudispuio146W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO146` reader - SCU_DRV_IO146"]
pub type Scudrvio146R = crate::FieldReader;
#[doc = "Field `SCUDRVIO146` writer - SCU_DRV_IO146"]
pub type Scudrvio146W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO146` reader - SCU_EN_SMT_IO146"]
pub type Scuensmtio146R = crate::BitReader;
#[doc = "Field `SCUENSMTIO146` writer - SCU_EN_SMT_IO146"]
pub type Scuensmtio146W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO146` reader - SCU_EN_HV_IO146"]
pub type Scuenhvio146R = crate::BitReader;
#[doc = "Field `SCUENHVIO146` writer - SCU_EN_HV_IO146"]
pub type Scuenhvio146W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO147` reader - SCU_DIS_PD_IO147"]
pub type Scudispdio147R = crate::BitReader;
#[doc = "Field `SCUDISPDIO147` writer - SCU_DIS_PD_IO147"]
pub type Scudispdio147W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO147` reader - SCU_DIS_PU_IO147"]
pub type Scudispuio147R = crate::BitReader;
#[doc = "Field `SCUDISPUIO147` writer - SCU_DIS_PU_IO147"]
pub type Scudispuio147W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO147` reader - SCU_DRV_IO147"]
pub type Scudrvio147R = crate::FieldReader;
#[doc = "Field `SCUDRVIO147` writer - SCU_DRV_IO147"]
pub type Scudrvio147W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO147` reader - SCU_EN_SMT_IO147"]
pub type Scuensmtio147R = crate::BitReader;
#[doc = "Field `SCUENSMTIO147` writer - SCU_EN_SMT_IO147"]
pub type Scuensmtio147W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO147` reader - SCU_EN_HV_IO147"]
pub type Scuenhvio147R = crate::BitReader;
#[doc = "Field `SCUENHVIO147` writer - SCU_EN_HV_IO147"]
pub type Scuenhvio147W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO146"]
    #[inline(always)]
    pub fn scudispdio146(&self) -> Scudispdio146R {
        Scudispdio146R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO146"]
    #[inline(always)]
    pub fn scudispuio146(&self) -> Scudispuio146R {
        Scudispuio146R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO146"]
    #[inline(always)]
    pub fn scudrvio146(&self) -> Scudrvio146R {
        Scudrvio146R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO146"]
    #[inline(always)]
    pub fn scuensmtio146(&self) -> Scuensmtio146R {
        Scuensmtio146R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO146"]
    #[inline(always)]
    pub fn scuenhvio146(&self) -> Scuenhvio146R {
        Scuenhvio146R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO147"]
    #[inline(always)]
    pub fn scudispdio147(&self) -> Scudispdio147R {
        Scudispdio147R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO147"]
    #[inline(always)]
    pub fn scudispuio147(&self) -> Scudispuio147R {
        Scudispuio147R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO147"]
    #[inline(always)]
    pub fn scudrvio147(&self) -> Scudrvio147R {
        Scudrvio147R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO147"]
    #[inline(always)]
    pub fn scuensmtio147(&self) -> Scuensmtio147R {
        Scuensmtio147R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO147"]
    #[inline(always)]
    pub fn scuenhvio147(&self) -> Scuenhvio147R {
        Scuenhvio147R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO146"]
    #[inline(always)]
    pub fn scudispdio146(&mut self) -> Scudispdio146W<Scu5a4Spec> {
        Scudispdio146W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO146"]
    #[inline(always)]
    pub fn scudispuio146(&mut self) -> Scudispuio146W<Scu5a4Spec> {
        Scudispuio146W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO146"]
    #[inline(always)]
    pub fn scudrvio146(&mut self) -> Scudrvio146W<Scu5a4Spec> {
        Scudrvio146W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO146"]
    #[inline(always)]
    pub fn scuensmtio146(&mut self) -> Scuensmtio146W<Scu5a4Spec> {
        Scuensmtio146W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO146"]
    #[inline(always)]
    pub fn scuenhvio146(&mut self) -> Scuenhvio146W<Scu5a4Spec> {
        Scuenhvio146W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO147"]
    #[inline(always)]
    pub fn scudispdio147(&mut self) -> Scudispdio147W<Scu5a4Spec> {
        Scudispdio147W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO147"]
    #[inline(always)]
    pub fn scudispuio147(&mut self) -> Scudispuio147W<Scu5a4Spec> {
        Scudispuio147W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO147"]
    #[inline(always)]
    pub fn scudrvio147(&mut self) -> Scudrvio147W<Scu5a4Spec> {
        Scudrvio147W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO147"]
    #[inline(always)]
    pub fn scuensmtio147(&mut self) -> Scuensmtio147W<Scu5a4Spec> {
        Scuensmtio147W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO147"]
    #[inline(always)]
    pub fn scuenhvio147(&mut self) -> Scuenhvio147W<Scu5a4Spec> {
        Scuenhvio147W::new(self, 25)
    }
}
#[doc = "IO Control \\#74\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5a4Spec;
impl crate::RegisterSpec for Scu5a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5a4::R`](R) reader structure"]
impl crate::Readable for Scu5a4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5a4::W`](W) writer structure"]
impl crate::Writable for Scu5a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5A4 to value 0x0204_0204"]
impl crate::Resettable for Scu5a4Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
