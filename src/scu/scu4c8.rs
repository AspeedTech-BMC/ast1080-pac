#[doc = "Register `SCU4C8` reader"]
pub type R = crate::R<Scu4c8Spec>;
#[doc = "Register `SCU4C8` writer"]
pub type W = crate::W<Scu4c8Spec>;
#[doc = "Field `SCUDISPDIO036` reader - SCU_DIS_PD_IO036"]
pub type Scudispdio036R = crate::BitReader;
#[doc = "Field `SCUDISPDIO036` writer - SCU_DIS_PD_IO036"]
pub type Scudispdio036W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO036` reader - SCU_DIS_PU_IO036"]
pub type Scudispuio036R = crate::BitReader;
#[doc = "Field `SCUDISPUIO036` writer - SCU_DIS_PU_IO036"]
pub type Scudispuio036W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO036` reader - SCU_DRV_IO036"]
pub type Scudrvio036R = crate::FieldReader;
#[doc = "Field `SCUDRVIO036` writer - SCU_DRV_IO036"]
pub type Scudrvio036W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO036` reader - SCU_EN_SMT_IO036"]
pub type Scuensmtio036R = crate::BitReader;
#[doc = "Field `SCUENSMTIO036` writer - SCU_EN_SMT_IO036"]
pub type Scuensmtio036W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO036` reader - SCU_EN_HV_IO036"]
pub type Scuenhvio036R = crate::BitReader;
#[doc = "Field `SCUENHVIO036` writer - SCU_EN_HV_IO036"]
pub type Scuenhvio036W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO037` reader - SCU_DIS_PD_IO037"]
pub type Scudispdio037R = crate::BitReader;
#[doc = "Field `SCUDISPDIO037` writer - SCU_DIS_PD_IO037"]
pub type Scudispdio037W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO037` reader - SCU_DIS_PU_IO037"]
pub type Scudispuio037R = crate::BitReader;
#[doc = "Field `SCUDISPUIO037` writer - SCU_DIS_PU_IO037"]
pub type Scudispuio037W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO037` reader - SCU_DRV_IO037"]
pub type Scudrvio037R = crate::FieldReader;
#[doc = "Field `SCUDRVIO037` writer - SCU_DRV_IO037"]
pub type Scudrvio037W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO037` reader - SCU_EN_SMT_IO037"]
pub type Scuensmtio037R = crate::BitReader;
#[doc = "Field `SCUENSMTIO037` writer - SCU_EN_SMT_IO037"]
pub type Scuensmtio037W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO037` reader - SCU_EN_HV_IO037"]
pub type Scuenhvio037R = crate::BitReader;
#[doc = "Field `SCUENHVIO037` writer - SCU_EN_HV_IO037"]
pub type Scuenhvio037W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO036"]
    #[inline(always)]
    pub fn scudispdio036(&self) -> Scudispdio036R {
        Scudispdio036R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO036"]
    #[inline(always)]
    pub fn scudispuio036(&self) -> Scudispuio036R {
        Scudispuio036R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO036"]
    #[inline(always)]
    pub fn scudrvio036(&self) -> Scudrvio036R {
        Scudrvio036R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO036"]
    #[inline(always)]
    pub fn scuensmtio036(&self) -> Scuensmtio036R {
        Scuensmtio036R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO036"]
    #[inline(always)]
    pub fn scuenhvio036(&self) -> Scuenhvio036R {
        Scuenhvio036R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO037"]
    #[inline(always)]
    pub fn scudispdio037(&self) -> Scudispdio037R {
        Scudispdio037R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO037"]
    #[inline(always)]
    pub fn scudispuio037(&self) -> Scudispuio037R {
        Scudispuio037R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO037"]
    #[inline(always)]
    pub fn scudrvio037(&self) -> Scudrvio037R {
        Scudrvio037R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO037"]
    #[inline(always)]
    pub fn scuensmtio037(&self) -> Scuensmtio037R {
        Scuensmtio037R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO037"]
    #[inline(always)]
    pub fn scuenhvio037(&self) -> Scuenhvio037R {
        Scuenhvio037R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO036"]
    #[inline(always)]
    pub fn scudispdio036(&mut self) -> Scudispdio036W<Scu4c8Spec> {
        Scudispdio036W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO036"]
    #[inline(always)]
    pub fn scudispuio036(&mut self) -> Scudispuio036W<Scu4c8Spec> {
        Scudispuio036W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO036"]
    #[inline(always)]
    pub fn scudrvio036(&mut self) -> Scudrvio036W<Scu4c8Spec> {
        Scudrvio036W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO036"]
    #[inline(always)]
    pub fn scuensmtio036(&mut self) -> Scuensmtio036W<Scu4c8Spec> {
        Scuensmtio036W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO036"]
    #[inline(always)]
    pub fn scuenhvio036(&mut self) -> Scuenhvio036W<Scu4c8Spec> {
        Scuenhvio036W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO037"]
    #[inline(always)]
    pub fn scudispdio037(&mut self) -> Scudispdio037W<Scu4c8Spec> {
        Scudispdio037W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO037"]
    #[inline(always)]
    pub fn scudispuio037(&mut self) -> Scudispuio037W<Scu4c8Spec> {
        Scudispuio037W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO037"]
    #[inline(always)]
    pub fn scudrvio037(&mut self) -> Scudrvio037W<Scu4c8Spec> {
        Scudrvio037W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO037"]
    #[inline(always)]
    pub fn scuensmtio037(&mut self) -> Scuensmtio037W<Scu4c8Spec> {
        Scuensmtio037W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO037"]
    #[inline(always)]
    pub fn scuenhvio037(&mut self) -> Scuenhvio037W<Scu4c8Spec> {
        Scuenhvio037W::new(self, 25)
    }
}
#[doc = "IO Control \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4c8Spec;
impl crate::RegisterSpec for Scu4c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4c8::R`](R) reader structure"]
impl crate::Readable for Scu4c8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4c8::W`](W) writer structure"]
impl crate::Writable for Scu4c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4C8 to value 0x0204_0204"]
impl crate::Resettable for Scu4c8Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
