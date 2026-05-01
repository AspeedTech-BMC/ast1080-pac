#[doc = "Register `SCU598` reader"]
pub type R = crate::R<Scu598Spec>;
#[doc = "Register `SCU598` writer"]
pub type W = crate::W<Scu598Spec>;
#[doc = "Field `SCUDISPDIO140` reader - SCU_DIS_PD_IO140"]
pub type Scudispdio140R = crate::BitReader;
#[doc = "Field `SCUDISPDIO140` writer - SCU_DIS_PD_IO140"]
pub type Scudispdio140W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO140` reader - SCU_DIS_PU_IO140"]
pub type Scudispuio140R = crate::BitReader;
#[doc = "Field `SCUDISPUIO140` writer - SCU_DIS_PU_IO140"]
pub type Scudispuio140W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO140` reader - SCU_DRV_IO140"]
pub type Scudrvio140R = crate::FieldReader;
#[doc = "Field `SCUDRVIO140` writer - SCU_DRV_IO140"]
pub type Scudrvio140W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO140` reader - SCU_EN_SMT_IO140"]
pub type Scuensmtio140R = crate::BitReader;
#[doc = "Field `SCUENSMTIO140` writer - SCU_EN_SMT_IO140"]
pub type Scuensmtio140W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO140` reader - SCU_EN_HV_IO140"]
pub type Scuenhvio140R = crate::BitReader;
#[doc = "Field `SCUENHVIO140` writer - SCU_EN_HV_IO140"]
pub type Scuenhvio140W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO141` reader - SCU_DIS_PD_IO141"]
pub type Scudispdio141R = crate::BitReader;
#[doc = "Field `SCUDISPDIO141` writer - SCU_DIS_PD_IO141"]
pub type Scudispdio141W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO141` reader - SCU_DIS_PU_IO141"]
pub type Scudispuio141R = crate::BitReader;
#[doc = "Field `SCUDISPUIO141` writer - SCU_DIS_PU_IO141"]
pub type Scudispuio141W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO141` reader - SCU_DRV_IO141"]
pub type Scudrvio141R = crate::FieldReader;
#[doc = "Field `SCUDRVIO141` writer - SCU_DRV_IO141"]
pub type Scudrvio141W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO141` reader - SCU_EN_SMT_IO141"]
pub type Scuensmtio141R = crate::BitReader;
#[doc = "Field `SCUENSMTIO141` writer - SCU_EN_SMT_IO141"]
pub type Scuensmtio141W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO141` reader - SCU_EN_HV_IO141"]
pub type Scuenhvio141R = crate::BitReader;
#[doc = "Field `SCUENHVIO141` writer - SCU_EN_HV_IO141"]
pub type Scuenhvio141W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO140"]
    #[inline(always)]
    pub fn scudispdio140(&self) -> Scudispdio140R {
        Scudispdio140R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO140"]
    #[inline(always)]
    pub fn scudispuio140(&self) -> Scudispuio140R {
        Scudispuio140R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO140"]
    #[inline(always)]
    pub fn scudrvio140(&self) -> Scudrvio140R {
        Scudrvio140R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO140"]
    #[inline(always)]
    pub fn scuensmtio140(&self) -> Scuensmtio140R {
        Scuensmtio140R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO140"]
    #[inline(always)]
    pub fn scuenhvio140(&self) -> Scuenhvio140R {
        Scuenhvio140R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO141"]
    #[inline(always)]
    pub fn scudispdio141(&self) -> Scudispdio141R {
        Scudispdio141R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO141"]
    #[inline(always)]
    pub fn scudispuio141(&self) -> Scudispuio141R {
        Scudispuio141R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO141"]
    #[inline(always)]
    pub fn scudrvio141(&self) -> Scudrvio141R {
        Scudrvio141R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO141"]
    #[inline(always)]
    pub fn scuensmtio141(&self) -> Scuensmtio141R {
        Scuensmtio141R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO141"]
    #[inline(always)]
    pub fn scuenhvio141(&self) -> Scuenhvio141R {
        Scuenhvio141R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO140"]
    #[inline(always)]
    pub fn scudispdio140(&mut self) -> Scudispdio140W<Scu598Spec> {
        Scudispdio140W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO140"]
    #[inline(always)]
    pub fn scudispuio140(&mut self) -> Scudispuio140W<Scu598Spec> {
        Scudispuio140W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO140"]
    #[inline(always)]
    pub fn scudrvio140(&mut self) -> Scudrvio140W<Scu598Spec> {
        Scudrvio140W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO140"]
    #[inline(always)]
    pub fn scuensmtio140(&mut self) -> Scuensmtio140W<Scu598Spec> {
        Scuensmtio140W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO140"]
    #[inline(always)]
    pub fn scuenhvio140(&mut self) -> Scuenhvio140W<Scu598Spec> {
        Scuenhvio140W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO141"]
    #[inline(always)]
    pub fn scudispdio141(&mut self) -> Scudispdio141W<Scu598Spec> {
        Scudispdio141W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO141"]
    #[inline(always)]
    pub fn scudispuio141(&mut self) -> Scudispuio141W<Scu598Spec> {
        Scudispuio141W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO141"]
    #[inline(always)]
    pub fn scudrvio141(&mut self) -> Scudrvio141W<Scu598Spec> {
        Scudrvio141W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO141"]
    #[inline(always)]
    pub fn scuensmtio141(&mut self) -> Scuensmtio141W<Scu598Spec> {
        Scuensmtio141W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO141"]
    #[inline(always)]
    pub fn scuenhvio141(&mut self) -> Scuenhvio141W<Scu598Spec> {
        Scuenhvio141W::new(self, 25)
    }
}
#[doc = "IO Control \\#71\n\nYou can [`read`](crate::Reg::read) this register and get [`scu598::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu598::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu598Spec;
impl crate::RegisterSpec for Scu598Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu598::R`](R) reader structure"]
impl crate::Readable for Scu598Spec {}
#[doc = "`write(|w| ..)` method takes [`scu598::W`](W) writer structure"]
impl crate::Writable for Scu598Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU598 to value 0x0204_0204"]
impl crate::Resettable for Scu598Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
