#[doc = "Register `SCU560` reader"]
pub type R = crate::R<Scu560Spec>;
#[doc = "Register `SCU560` writer"]
pub type W = crate::W<Scu560Spec>;
#[doc = "Field `SCUDISPDIO112` reader - SCU_DIS_PD_IO112"]
pub type Scudispdio112R = crate::BitReader;
#[doc = "Field `SCUDISPDIO112` writer - SCU_DIS_PD_IO112"]
pub type Scudispdio112W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO112` reader - SCU_DIS_PU_IO112"]
pub type Scudispuio112R = crate::BitReader;
#[doc = "Field `SCUDISPUIO112` writer - SCU_DIS_PU_IO112"]
pub type Scudispuio112W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO112` reader - SCU_DRV_IO112"]
pub type Scudrvio112R = crate::FieldReader;
#[doc = "Field `SCUDRVIO112` writer - SCU_DRV_IO112"]
pub type Scudrvio112W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO112` reader - SCU_EN_SMT_IO112"]
pub type Scuensmtio112R = crate::BitReader;
#[doc = "Field `SCUENSMTIO112` writer - SCU_EN_SMT_IO112"]
pub type Scuensmtio112W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO112` reader - SCU_EN_HV_IO112"]
pub type Scuenhvio112R = crate::BitReader;
#[doc = "Field `SCUENHVIO112` writer - SCU_EN_HV_IO112"]
pub type Scuenhvio112W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO113` reader - SCU_DIS_PD_IO113"]
pub type Scudispdio113R = crate::BitReader;
#[doc = "Field `SCUDISPDIO113` writer - SCU_DIS_PD_IO113"]
pub type Scudispdio113W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO113` reader - SCU_DIS_PU_IO113"]
pub type Scudispuio113R = crate::BitReader;
#[doc = "Field `SCUDISPUIO113` writer - SCU_DIS_PU_IO113"]
pub type Scudispuio113W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO113` reader - SCU_DRV_IO113"]
pub type Scudrvio113R = crate::FieldReader;
#[doc = "Field `SCUDRVIO113` writer - SCU_DRV_IO113"]
pub type Scudrvio113W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO113` reader - SCU_EN_SMT_IO113"]
pub type Scuensmtio113R = crate::BitReader;
#[doc = "Field `SCUENSMTIO113` writer - SCU_EN_SMT_IO113"]
pub type Scuensmtio113W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO113` reader - SCU_EN_HV_IO113"]
pub type Scuenhvio113R = crate::BitReader;
#[doc = "Field `SCUENHVIO113` writer - SCU_EN_HV_IO113"]
pub type Scuenhvio113W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO112"]
    #[inline(always)]
    pub fn scudispdio112(&self) -> Scudispdio112R {
        Scudispdio112R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO112"]
    #[inline(always)]
    pub fn scudispuio112(&self) -> Scudispuio112R {
        Scudispuio112R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO112"]
    #[inline(always)]
    pub fn scudrvio112(&self) -> Scudrvio112R {
        Scudrvio112R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO112"]
    #[inline(always)]
    pub fn scuensmtio112(&self) -> Scuensmtio112R {
        Scuensmtio112R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO112"]
    #[inline(always)]
    pub fn scuenhvio112(&self) -> Scuenhvio112R {
        Scuenhvio112R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO113"]
    #[inline(always)]
    pub fn scudispdio113(&self) -> Scudispdio113R {
        Scudispdio113R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO113"]
    #[inline(always)]
    pub fn scudispuio113(&self) -> Scudispuio113R {
        Scudispuio113R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO113"]
    #[inline(always)]
    pub fn scudrvio113(&self) -> Scudrvio113R {
        Scudrvio113R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO113"]
    #[inline(always)]
    pub fn scuensmtio113(&self) -> Scuensmtio113R {
        Scuensmtio113R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO113"]
    #[inline(always)]
    pub fn scuenhvio113(&self) -> Scuenhvio113R {
        Scuenhvio113R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO112"]
    #[inline(always)]
    pub fn scudispdio112(&mut self) -> Scudispdio112W<Scu560Spec> {
        Scudispdio112W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO112"]
    #[inline(always)]
    pub fn scudispuio112(&mut self) -> Scudispuio112W<Scu560Spec> {
        Scudispuio112W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO112"]
    #[inline(always)]
    pub fn scudrvio112(&mut self) -> Scudrvio112W<Scu560Spec> {
        Scudrvio112W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO112"]
    #[inline(always)]
    pub fn scuensmtio112(&mut self) -> Scuensmtio112W<Scu560Spec> {
        Scuensmtio112W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO112"]
    #[inline(always)]
    pub fn scuenhvio112(&mut self) -> Scuenhvio112W<Scu560Spec> {
        Scuenhvio112W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO113"]
    #[inline(always)]
    pub fn scudispdio113(&mut self) -> Scudispdio113W<Scu560Spec> {
        Scudispdio113W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO113"]
    #[inline(always)]
    pub fn scudispuio113(&mut self) -> Scudispuio113W<Scu560Spec> {
        Scudispuio113W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO113"]
    #[inline(always)]
    pub fn scudrvio113(&mut self) -> Scudrvio113W<Scu560Spec> {
        Scudrvio113W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO113"]
    #[inline(always)]
    pub fn scuensmtio113(&mut self) -> Scuensmtio113W<Scu560Spec> {
        Scuensmtio113W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO113"]
    #[inline(always)]
    pub fn scuenhvio113(&mut self) -> Scuenhvio113W<Scu560Spec> {
        Scuenhvio113W::new(self, 25)
    }
}
#[doc = "IO Control \\#57\n\nYou can [`read`](crate::Reg::read) this register and get [`scu560::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu560::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu560Spec;
impl crate::RegisterSpec for Scu560Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu560::R`](R) reader structure"]
impl crate::Readable for Scu560Spec {}
#[doc = "`write(|w| ..)` method takes [`scu560::W`](W) writer structure"]
impl crate::Writable for Scu560Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU560 to value 0x0204_0204"]
impl crate::Resettable for Scu560Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
