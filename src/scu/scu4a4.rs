#[doc = "Register `SCU4A4` reader"]
pub type R = crate::R<Scu4a4Spec>;
#[doc = "Register `SCU4A4` writer"]
pub type W = crate::W<Scu4a4Spec>;
#[doc = "Field `SCUDISPDIO018` reader - SCU_DIS_PD_IO018"]
pub type Scudispdio018R = crate::BitReader;
#[doc = "Field `SCUDISPDIO018` writer - SCU_DIS_PD_IO018"]
pub type Scudispdio018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO018` reader - SCU_DIS_PU_IO018"]
pub type Scudispuio018R = crate::BitReader;
#[doc = "Field `SCUDISPUIO018` writer - SCU_DIS_PU_IO018"]
pub type Scudispuio018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO018` reader - SCU_DRV_IO018"]
pub type Scudrvio018R = crate::FieldReader;
#[doc = "Field `SCUDRVIO018` writer - SCU_DRV_IO018"]
pub type Scudrvio018W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO018` reader - SCU_EN_SMT_IO018"]
pub type Scuensmtio018R = crate::BitReader;
#[doc = "Field `SCUENSMTIO018` writer - SCU_EN_SMT_IO018"]
pub type Scuensmtio018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO018` reader - SCU_EN_HV_IO018"]
pub type Scuenhvio018R = crate::BitReader;
#[doc = "Field `SCUENHVIO018` writer - SCU_EN_HV_IO018"]
pub type Scuenhvio018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO019` reader - SCU_DIS_PD_IO019"]
pub type Scudispdio019R = crate::BitReader;
#[doc = "Field `SCUDISPDIO019` writer - SCU_DIS_PD_IO019"]
pub type Scudispdio019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO019` reader - SCU_DIS_PU_IO019"]
pub type Scudispuio019R = crate::BitReader;
#[doc = "Field `SCUDISPUIO019` writer - SCU_DIS_PU_IO019"]
pub type Scudispuio019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO019` reader - SCU_DRV_IO019"]
pub type Scudrvio019R = crate::FieldReader;
#[doc = "Field `SCUDRVIO019` writer - SCU_DRV_IO019"]
pub type Scudrvio019W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO019` reader - SCU_EN_SMT_IO019"]
pub type Scuensmtio019R = crate::BitReader;
#[doc = "Field `SCUENSMTIO019` writer - SCU_EN_SMT_IO019"]
pub type Scuensmtio019W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO019` reader - SCU_EN_HV_IO019"]
pub type Scuenhvio019R = crate::BitReader;
#[doc = "Field `SCUENHVIO019` writer - SCU_EN_HV_IO019"]
pub type Scuenhvio019W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO018"]
    #[inline(always)]
    pub fn scudispdio018(&self) -> Scudispdio018R {
        Scudispdio018R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO018"]
    #[inline(always)]
    pub fn scudispuio018(&self) -> Scudispuio018R {
        Scudispuio018R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO018"]
    #[inline(always)]
    pub fn scudrvio018(&self) -> Scudrvio018R {
        Scudrvio018R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO018"]
    #[inline(always)]
    pub fn scuensmtio018(&self) -> Scuensmtio018R {
        Scuensmtio018R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO018"]
    #[inline(always)]
    pub fn scuenhvio018(&self) -> Scuenhvio018R {
        Scuenhvio018R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO019"]
    #[inline(always)]
    pub fn scudispdio019(&self) -> Scudispdio019R {
        Scudispdio019R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO019"]
    #[inline(always)]
    pub fn scudispuio019(&self) -> Scudispuio019R {
        Scudispuio019R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO019"]
    #[inline(always)]
    pub fn scudrvio019(&self) -> Scudrvio019R {
        Scudrvio019R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO019"]
    #[inline(always)]
    pub fn scuensmtio019(&self) -> Scuensmtio019R {
        Scuensmtio019R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO019"]
    #[inline(always)]
    pub fn scuenhvio019(&self) -> Scuenhvio019R {
        Scuenhvio019R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO018"]
    #[inline(always)]
    pub fn scudispdio018(&mut self) -> Scudispdio018W<Scu4a4Spec> {
        Scudispdio018W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO018"]
    #[inline(always)]
    pub fn scudispuio018(&mut self) -> Scudispuio018W<Scu4a4Spec> {
        Scudispuio018W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO018"]
    #[inline(always)]
    pub fn scudrvio018(&mut self) -> Scudrvio018W<Scu4a4Spec> {
        Scudrvio018W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO018"]
    #[inline(always)]
    pub fn scuensmtio018(&mut self) -> Scuensmtio018W<Scu4a4Spec> {
        Scuensmtio018W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO018"]
    #[inline(always)]
    pub fn scuenhvio018(&mut self) -> Scuenhvio018W<Scu4a4Spec> {
        Scuenhvio018W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO019"]
    #[inline(always)]
    pub fn scudispdio019(&mut self) -> Scudispdio019W<Scu4a4Spec> {
        Scudispdio019W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO019"]
    #[inline(always)]
    pub fn scudispuio019(&mut self) -> Scudispuio019W<Scu4a4Spec> {
        Scudispuio019W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO019"]
    #[inline(always)]
    pub fn scudrvio019(&mut self) -> Scudrvio019W<Scu4a4Spec> {
        Scudrvio019W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO019"]
    #[inline(always)]
    pub fn scuensmtio019(&mut self) -> Scuensmtio019W<Scu4a4Spec> {
        Scuensmtio019W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO019"]
    #[inline(always)]
    pub fn scuenhvio019(&mut self) -> Scuenhvio019W<Scu4a4Spec> {
        Scuenhvio019W::new(self, 25)
    }
}
#[doc = "IO Control \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4a4Spec;
impl crate::RegisterSpec for Scu4a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4a4::R`](R) reader structure"]
impl crate::Readable for Scu4a4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4a4::W`](W) writer structure"]
impl crate::Writable for Scu4a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4A4 to value 0x0204_0204"]
impl crate::Resettable for Scu4a4Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
