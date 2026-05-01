#[doc = "Register `SCU5F8` reader"]
pub type R = crate::R<Scu5f8Spec>;
#[doc = "Register `SCU5F8` writer"]
pub type W = crate::W<Scu5f8Spec>;
#[doc = "Field `SCUDISPDIO188` reader - SCU_DIS_PD_IO188"]
pub type Scudispdio188R = crate::BitReader;
#[doc = "Field `SCUDISPDIO188` writer - SCU_DIS_PD_IO188"]
pub type Scudispdio188W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO188` reader - SCU_DIS_PU_IO188"]
pub type Scudispuio188R = crate::BitReader;
#[doc = "Field `SCUDISPUIO188` writer - SCU_DIS_PU_IO188"]
pub type Scudispuio188W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO188` reader - SCU_DRV_IO188"]
pub type Scudrvio188R = crate::FieldReader;
#[doc = "Field `SCUDRVIO188` writer - SCU_DRV_IO188"]
pub type Scudrvio188W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO188` reader - SCU_EN_SMT_IO188"]
pub type Scuensmtio188R = crate::BitReader;
#[doc = "Field `SCUENSMTIO188` writer - SCU_EN_SMT_IO188"]
pub type Scuensmtio188W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO188` reader - SCU_EN_HV_IO188"]
pub type Scuenhvio188R = crate::BitReader;
#[doc = "Field `SCUENHVIO188` writer - SCU_EN_HV_IO188"]
pub type Scuenhvio188W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO189` reader - SCU_DIS_PD_IO189"]
pub type Scudispdio189R = crate::BitReader;
#[doc = "Field `SCUDISPDIO189` writer - SCU_DIS_PD_IO189"]
pub type Scudispdio189W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO189` reader - SCU_DIS_PU_IO189"]
pub type Scudispuio189R = crate::BitReader;
#[doc = "Field `SCUDISPUIO189` writer - SCU_DIS_PU_IO189"]
pub type Scudispuio189W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO189` reader - SCU_DRV_IO189"]
pub type Scudrvio189R = crate::FieldReader;
#[doc = "Field `SCUDRVIO189` writer - SCU_DRV_IO189"]
pub type Scudrvio189W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO189` reader - SCU_EN_SMT_IO189"]
pub type Scuensmtio189R = crate::BitReader;
#[doc = "Field `SCUENSMTIO189` writer - SCU_EN_SMT_IO189"]
pub type Scuensmtio189W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO189` reader - SCU_EN_HV_IO189"]
pub type Scuenhvio189R = crate::BitReader;
#[doc = "Field `SCUENHVIO189` writer - SCU_EN_HV_IO189"]
pub type Scuenhvio189W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO188"]
    #[inline(always)]
    pub fn scudispdio188(&self) -> Scudispdio188R {
        Scudispdio188R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO188"]
    #[inline(always)]
    pub fn scudispuio188(&self) -> Scudispuio188R {
        Scudispuio188R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO188"]
    #[inline(always)]
    pub fn scudrvio188(&self) -> Scudrvio188R {
        Scudrvio188R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO188"]
    #[inline(always)]
    pub fn scuensmtio188(&self) -> Scuensmtio188R {
        Scuensmtio188R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO188"]
    #[inline(always)]
    pub fn scuenhvio188(&self) -> Scuenhvio188R {
        Scuenhvio188R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO189"]
    #[inline(always)]
    pub fn scudispdio189(&self) -> Scudispdio189R {
        Scudispdio189R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO189"]
    #[inline(always)]
    pub fn scudispuio189(&self) -> Scudispuio189R {
        Scudispuio189R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO189"]
    #[inline(always)]
    pub fn scudrvio189(&self) -> Scudrvio189R {
        Scudrvio189R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO189"]
    #[inline(always)]
    pub fn scuensmtio189(&self) -> Scuensmtio189R {
        Scuensmtio189R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO189"]
    #[inline(always)]
    pub fn scuenhvio189(&self) -> Scuenhvio189R {
        Scuenhvio189R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO188"]
    #[inline(always)]
    pub fn scudispdio188(&mut self) -> Scudispdio188W<Scu5f8Spec> {
        Scudispdio188W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO188"]
    #[inline(always)]
    pub fn scudispuio188(&mut self) -> Scudispuio188W<Scu5f8Spec> {
        Scudispuio188W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO188"]
    #[inline(always)]
    pub fn scudrvio188(&mut self) -> Scudrvio188W<Scu5f8Spec> {
        Scudrvio188W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO188"]
    #[inline(always)]
    pub fn scuensmtio188(&mut self) -> Scuensmtio188W<Scu5f8Spec> {
        Scuensmtio188W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO188"]
    #[inline(always)]
    pub fn scuenhvio188(&mut self) -> Scuenhvio188W<Scu5f8Spec> {
        Scuenhvio188W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO189"]
    #[inline(always)]
    pub fn scudispdio189(&mut self) -> Scudispdio189W<Scu5f8Spec> {
        Scudispdio189W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO189"]
    #[inline(always)]
    pub fn scudispuio189(&mut self) -> Scudispuio189W<Scu5f8Spec> {
        Scudispuio189W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO189"]
    #[inline(always)]
    pub fn scudrvio189(&mut self) -> Scudrvio189W<Scu5f8Spec> {
        Scudrvio189W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO189"]
    #[inline(always)]
    pub fn scuensmtio189(&mut self) -> Scuensmtio189W<Scu5f8Spec> {
        Scuensmtio189W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO189"]
    #[inline(always)]
    pub fn scuenhvio189(&mut self) -> Scuenhvio189W<Scu5f8Spec> {
        Scuenhvio189W::new(self, 25)
    }
}
#[doc = "IO Control \\#95\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5f8Spec;
impl crate::RegisterSpec for Scu5f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5f8::R`](R) reader structure"]
impl crate::Readable for Scu5f8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5f8::W`](W) writer structure"]
impl crate::Writable for Scu5f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5F8 to value 0x0201_0201"]
impl crate::Resettable for Scu5f8Spec {
    const RESET_VALUE: u32 = 0x0201_0201;
}
