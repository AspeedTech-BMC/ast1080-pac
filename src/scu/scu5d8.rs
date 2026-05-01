#[doc = "Register `SCU5D8` reader"]
pub type R = crate::R<Scu5d8Spec>;
#[doc = "Register `SCU5D8` writer"]
pub type W = crate::W<Scu5d8Spec>;
#[doc = "Field `SCUDISPDIO172` reader - SCU_DIS_PD_IO172"]
pub type Scudispdio172R = crate::BitReader;
#[doc = "Field `SCUDISPDIO172` writer - SCU_DIS_PD_IO172"]
pub type Scudispdio172W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO172` reader - SCU_DIS_PU_IO172"]
pub type Scudispuio172R = crate::BitReader;
#[doc = "Field `SCUDISPUIO172` writer - SCU_DIS_PU_IO172"]
pub type Scudispuio172W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO172` reader - SCU_DRV_IO172"]
pub type Scudrvio172R = crate::FieldReader;
#[doc = "Field `SCUDRVIO172` writer - SCU_DRV_IO172"]
pub type Scudrvio172W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO172` reader - SCU_EN_SMT_IO172"]
pub type Scuensmtio172R = crate::BitReader;
#[doc = "Field `SCUENSMTIO172` writer - SCU_EN_SMT_IO172"]
pub type Scuensmtio172W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO172` reader - SCU_EN_HV_IO172"]
pub type Scuenhvio172R = crate::BitReader;
#[doc = "Field `SCUENHVIO172` writer - SCU_EN_HV_IO172"]
pub type Scuenhvio172W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO173` reader - SCU_DIS_PD_IO173"]
pub type Scudispdio173R = crate::BitReader;
#[doc = "Field `SCUDISPDIO173` writer - SCU_DIS_PD_IO173"]
pub type Scudispdio173W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO173` reader - SCU_DIS_PU_IO173"]
pub type Scudispuio173R = crate::BitReader;
#[doc = "Field `SCUDISPUIO173` writer - SCU_DIS_PU_IO173"]
pub type Scudispuio173W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO173` reader - SCU_DRV_IO173"]
pub type Scudrvio173R = crate::FieldReader;
#[doc = "Field `SCUDRVIO173` writer - SCU_DRV_IO173"]
pub type Scudrvio173W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO173` reader - SCU_EN_SMT_IO173"]
pub type Scuensmtio173R = crate::BitReader;
#[doc = "Field `SCUENSMTIO173` writer - SCU_EN_SMT_IO173"]
pub type Scuensmtio173W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO173` reader - SCU_EN_HV_IO173"]
pub type Scuenhvio173R = crate::BitReader;
#[doc = "Field `SCUENHVIO173` writer - SCU_EN_HV_IO173"]
pub type Scuenhvio173W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO172"]
    #[inline(always)]
    pub fn scudispdio172(&self) -> Scudispdio172R {
        Scudispdio172R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO172"]
    #[inline(always)]
    pub fn scudispuio172(&self) -> Scudispuio172R {
        Scudispuio172R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO172"]
    #[inline(always)]
    pub fn scudrvio172(&self) -> Scudrvio172R {
        Scudrvio172R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO172"]
    #[inline(always)]
    pub fn scuensmtio172(&self) -> Scuensmtio172R {
        Scuensmtio172R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO172"]
    #[inline(always)]
    pub fn scuenhvio172(&self) -> Scuenhvio172R {
        Scuenhvio172R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO173"]
    #[inline(always)]
    pub fn scudispdio173(&self) -> Scudispdio173R {
        Scudispdio173R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO173"]
    #[inline(always)]
    pub fn scudispuio173(&self) -> Scudispuio173R {
        Scudispuio173R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO173"]
    #[inline(always)]
    pub fn scudrvio173(&self) -> Scudrvio173R {
        Scudrvio173R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO173"]
    #[inline(always)]
    pub fn scuensmtio173(&self) -> Scuensmtio173R {
        Scuensmtio173R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO173"]
    #[inline(always)]
    pub fn scuenhvio173(&self) -> Scuenhvio173R {
        Scuenhvio173R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO172"]
    #[inline(always)]
    pub fn scudispdio172(&mut self) -> Scudispdio172W<Scu5d8Spec> {
        Scudispdio172W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO172"]
    #[inline(always)]
    pub fn scudispuio172(&mut self) -> Scudispuio172W<Scu5d8Spec> {
        Scudispuio172W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO172"]
    #[inline(always)]
    pub fn scudrvio172(&mut self) -> Scudrvio172W<Scu5d8Spec> {
        Scudrvio172W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO172"]
    #[inline(always)]
    pub fn scuensmtio172(&mut self) -> Scuensmtio172W<Scu5d8Spec> {
        Scuensmtio172W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO172"]
    #[inline(always)]
    pub fn scuenhvio172(&mut self) -> Scuenhvio172W<Scu5d8Spec> {
        Scuenhvio172W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO173"]
    #[inline(always)]
    pub fn scudispdio173(&mut self) -> Scudispdio173W<Scu5d8Spec> {
        Scudispdio173W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO173"]
    #[inline(always)]
    pub fn scudispuio173(&mut self) -> Scudispuio173W<Scu5d8Spec> {
        Scudispuio173W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO173"]
    #[inline(always)]
    pub fn scudrvio173(&mut self) -> Scudrvio173W<Scu5d8Spec> {
        Scudrvio173W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO173"]
    #[inline(always)]
    pub fn scuensmtio173(&mut self) -> Scuensmtio173W<Scu5d8Spec> {
        Scuensmtio173W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO173"]
    #[inline(always)]
    pub fn scuenhvio173(&mut self) -> Scuenhvio173W<Scu5d8Spec> {
        Scuenhvio173W::new(self, 25)
    }
}
#[doc = "IO Control \\#87\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5d8Spec;
impl crate::RegisterSpec for Scu5d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5d8::R`](R) reader structure"]
impl crate::Readable for Scu5d8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5d8::W`](W) writer structure"]
impl crate::Writable for Scu5d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5D8 to value 0x0204_0204"]
impl crate::Resettable for Scu5d8Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
