#[doc = "Register `SCU5B8` reader"]
pub type R = crate::R<Scu5b8Spec>;
#[doc = "Register `SCU5B8` writer"]
pub type W = crate::W<Scu5b8Spec>;
#[doc = "Field `SCUDISPDIO156` reader - SCU_DIS_PD_IO156"]
pub type Scudispdio156R = crate::BitReader;
#[doc = "Field `SCUDISPDIO156` writer - SCU_DIS_PD_IO156"]
pub type Scudispdio156W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO156` reader - SCU_DIS_PU_IO156"]
pub type Scudispuio156R = crate::BitReader;
#[doc = "Field `SCUDISPUIO156` writer - SCU_DIS_PU_IO156"]
pub type Scudispuio156W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO156` reader - SCU_DRV_IO156"]
pub type Scudrvio156R = crate::FieldReader;
#[doc = "Field `SCUDRVIO156` writer - SCU_DRV_IO156"]
pub type Scudrvio156W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO156` reader - SCU_EN_SMT_IO156"]
pub type Scuensmtio156R = crate::BitReader;
#[doc = "Field `SCUENSMTIO156` writer - SCU_EN_SMT_IO156"]
pub type Scuensmtio156W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO156` reader - SCU_EN_HV_IO156"]
pub type Scuenhvio156R = crate::BitReader;
#[doc = "Field `SCUENHVIO156` writer - SCU_EN_HV_IO156"]
pub type Scuenhvio156W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO157` reader - SCU_DIS_PD_IO157"]
pub type Scudispdio157R = crate::BitReader;
#[doc = "Field `SCUDISPDIO157` writer - SCU_DIS_PD_IO157"]
pub type Scudispdio157W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO157` reader - SCU_DIS_PU_IO157"]
pub type Scudispuio157R = crate::BitReader;
#[doc = "Field `SCUDISPUIO157` writer - SCU_DIS_PU_IO157"]
pub type Scudispuio157W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO157` reader - SCU_DRV_IO157"]
pub type Scudrvio157R = crate::FieldReader;
#[doc = "Field `SCUDRVIO157` writer - SCU_DRV_IO157"]
pub type Scudrvio157W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO157` reader - SCU_EN_SMT_IO157"]
pub type Scuensmtio157R = crate::BitReader;
#[doc = "Field `SCUENSMTIO157` writer - SCU_EN_SMT_IO157"]
pub type Scuensmtio157W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO157` reader - SCU_EN_HV_IO157"]
pub type Scuenhvio157R = crate::BitReader;
#[doc = "Field `SCUENHVIO157` writer - SCU_EN_HV_IO157"]
pub type Scuenhvio157W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO156"]
    #[inline(always)]
    pub fn scudispdio156(&self) -> Scudispdio156R {
        Scudispdio156R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO156"]
    #[inline(always)]
    pub fn scudispuio156(&self) -> Scudispuio156R {
        Scudispuio156R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO156"]
    #[inline(always)]
    pub fn scudrvio156(&self) -> Scudrvio156R {
        Scudrvio156R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO156"]
    #[inline(always)]
    pub fn scuensmtio156(&self) -> Scuensmtio156R {
        Scuensmtio156R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO156"]
    #[inline(always)]
    pub fn scuenhvio156(&self) -> Scuenhvio156R {
        Scuenhvio156R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO157"]
    #[inline(always)]
    pub fn scudispdio157(&self) -> Scudispdio157R {
        Scudispdio157R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO157"]
    #[inline(always)]
    pub fn scudispuio157(&self) -> Scudispuio157R {
        Scudispuio157R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO157"]
    #[inline(always)]
    pub fn scudrvio157(&self) -> Scudrvio157R {
        Scudrvio157R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO157"]
    #[inline(always)]
    pub fn scuensmtio157(&self) -> Scuensmtio157R {
        Scuensmtio157R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO157"]
    #[inline(always)]
    pub fn scuenhvio157(&self) -> Scuenhvio157R {
        Scuenhvio157R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO156"]
    #[inline(always)]
    pub fn scudispdio156(&mut self) -> Scudispdio156W<Scu5b8Spec> {
        Scudispdio156W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO156"]
    #[inline(always)]
    pub fn scudispuio156(&mut self) -> Scudispuio156W<Scu5b8Spec> {
        Scudispuio156W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO156"]
    #[inline(always)]
    pub fn scudrvio156(&mut self) -> Scudrvio156W<Scu5b8Spec> {
        Scudrvio156W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO156"]
    #[inline(always)]
    pub fn scuensmtio156(&mut self) -> Scuensmtio156W<Scu5b8Spec> {
        Scuensmtio156W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO156"]
    #[inline(always)]
    pub fn scuenhvio156(&mut self) -> Scuenhvio156W<Scu5b8Spec> {
        Scuenhvio156W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO157"]
    #[inline(always)]
    pub fn scudispdio157(&mut self) -> Scudispdio157W<Scu5b8Spec> {
        Scudispdio157W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO157"]
    #[inline(always)]
    pub fn scudispuio157(&mut self) -> Scudispuio157W<Scu5b8Spec> {
        Scudispuio157W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO157"]
    #[inline(always)]
    pub fn scudrvio157(&mut self) -> Scudrvio157W<Scu5b8Spec> {
        Scudrvio157W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO157"]
    #[inline(always)]
    pub fn scuensmtio157(&mut self) -> Scuensmtio157W<Scu5b8Spec> {
        Scuensmtio157W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO157"]
    #[inline(always)]
    pub fn scuenhvio157(&mut self) -> Scuenhvio157W<Scu5b8Spec> {
        Scuenhvio157W::new(self, 25)
    }
}
#[doc = "IO Control \\#79\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5b8Spec;
impl crate::RegisterSpec for Scu5b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5b8::R`](R) reader structure"]
impl crate::Readable for Scu5b8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5b8::W`](W) writer structure"]
impl crate::Writable for Scu5b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5B8 to value 0x0201_0201"]
impl crate::Resettable for Scu5b8Spec {
    const RESET_VALUE: u32 = 0x0201_0201;
}
