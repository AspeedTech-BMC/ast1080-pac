#[doc = "Register `SCU588` reader"]
pub type R = crate::R<Scu588Spec>;
#[doc = "Register `SCU588` writer"]
pub type W = crate::W<Scu588Spec>;
#[doc = "Field `SCUDISPDIO132` reader - SCU_DIS_PD_IO132"]
pub type Scudispdio132R = crate::BitReader;
#[doc = "Field `SCUDISPDIO132` writer - SCU_DIS_PD_IO132"]
pub type Scudispdio132W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO132` reader - SCU_DIS_PU_IO132"]
pub type Scudispuio132R = crate::BitReader;
#[doc = "Field `SCUDISPUIO132` writer - SCU_DIS_PU_IO132"]
pub type Scudispuio132W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO132` reader - SCU_DRV_IO132"]
pub type Scudrvio132R = crate::FieldReader;
#[doc = "Field `SCUDRVIO132` writer - SCU_DRV_IO132"]
pub type Scudrvio132W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO132` reader - SCU_EN_SMT_IO132"]
pub type Scuensmtio132R = crate::BitReader;
#[doc = "Field `SCUENSMTIO132` writer - SCU_EN_SMT_IO132"]
pub type Scuensmtio132W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO132` reader - SCU_EN_HV_IO132"]
pub type Scuenhvio132R = crate::BitReader;
#[doc = "Field `SCUENHVIO132` writer - SCU_EN_HV_IO132"]
pub type Scuenhvio132W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO133` reader - SCU_DIS_PD_IO133"]
pub type Scudispdio133R = crate::BitReader;
#[doc = "Field `SCUDISPDIO133` writer - SCU_DIS_PD_IO133"]
pub type Scudispdio133W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO133` reader - SCU_DIS_PU_IO133"]
pub type Scudispuio133R = crate::BitReader;
#[doc = "Field `SCUDISPUIO133` writer - SCU_DIS_PU_IO133"]
pub type Scudispuio133W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO133` reader - SCU_DRV_IO133"]
pub type Scudrvio133R = crate::FieldReader;
#[doc = "Field `SCUDRVIO133` writer - SCU_DRV_IO133"]
pub type Scudrvio133W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO133` reader - SCU_EN_SMT_IO133"]
pub type Scuensmtio133R = crate::BitReader;
#[doc = "Field `SCUENSMTIO133` writer - SCU_EN_SMT_IO133"]
pub type Scuensmtio133W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO133` reader - SCU_EN_HV_IO133"]
pub type Scuenhvio133R = crate::BitReader;
#[doc = "Field `SCUENHVIO133` writer - SCU_EN_HV_IO133"]
pub type Scuenhvio133W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO132"]
    #[inline(always)]
    pub fn scudispdio132(&self) -> Scudispdio132R {
        Scudispdio132R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO132"]
    #[inline(always)]
    pub fn scudispuio132(&self) -> Scudispuio132R {
        Scudispuio132R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO132"]
    #[inline(always)]
    pub fn scudrvio132(&self) -> Scudrvio132R {
        Scudrvio132R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO132"]
    #[inline(always)]
    pub fn scuensmtio132(&self) -> Scuensmtio132R {
        Scuensmtio132R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO132"]
    #[inline(always)]
    pub fn scuenhvio132(&self) -> Scuenhvio132R {
        Scuenhvio132R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO133"]
    #[inline(always)]
    pub fn scudispdio133(&self) -> Scudispdio133R {
        Scudispdio133R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO133"]
    #[inline(always)]
    pub fn scudispuio133(&self) -> Scudispuio133R {
        Scudispuio133R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO133"]
    #[inline(always)]
    pub fn scudrvio133(&self) -> Scudrvio133R {
        Scudrvio133R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO133"]
    #[inline(always)]
    pub fn scuensmtio133(&self) -> Scuensmtio133R {
        Scuensmtio133R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO133"]
    #[inline(always)]
    pub fn scuenhvio133(&self) -> Scuenhvio133R {
        Scuenhvio133R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO132"]
    #[inline(always)]
    pub fn scudispdio132(&mut self) -> Scudispdio132W<Scu588Spec> {
        Scudispdio132W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO132"]
    #[inline(always)]
    pub fn scudispuio132(&mut self) -> Scudispuio132W<Scu588Spec> {
        Scudispuio132W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO132"]
    #[inline(always)]
    pub fn scudrvio132(&mut self) -> Scudrvio132W<Scu588Spec> {
        Scudrvio132W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO132"]
    #[inline(always)]
    pub fn scuensmtio132(&mut self) -> Scuensmtio132W<Scu588Spec> {
        Scuensmtio132W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO132"]
    #[inline(always)]
    pub fn scuenhvio132(&mut self) -> Scuenhvio132W<Scu588Spec> {
        Scuenhvio132W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO133"]
    #[inline(always)]
    pub fn scudispdio133(&mut self) -> Scudispdio133W<Scu588Spec> {
        Scudispdio133W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO133"]
    #[inline(always)]
    pub fn scudispuio133(&mut self) -> Scudispuio133W<Scu588Spec> {
        Scudispuio133W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO133"]
    #[inline(always)]
    pub fn scudrvio133(&mut self) -> Scudrvio133W<Scu588Spec> {
        Scudrvio133W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO133"]
    #[inline(always)]
    pub fn scuensmtio133(&mut self) -> Scuensmtio133W<Scu588Spec> {
        Scuensmtio133W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO133"]
    #[inline(always)]
    pub fn scuenhvio133(&mut self) -> Scuenhvio133W<Scu588Spec> {
        Scuenhvio133W::new(self, 25)
    }
}
#[doc = "IO Control \\#67\n\nYou can [`read`](crate::Reg::read) this register and get [`scu588::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu588::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu588Spec;
impl crate::RegisterSpec for Scu588Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu588::R`](R) reader structure"]
impl crate::Readable for Scu588Spec {}
#[doc = "`write(|w| ..)` method takes [`scu588::W`](W) writer structure"]
impl crate::Writable for Scu588Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU588 to value 0x0204_0204"]
impl crate::Resettable for Scu588Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
