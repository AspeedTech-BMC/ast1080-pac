#[doc = "Register `SCU5A8` reader"]
pub type R = crate::R<Scu5a8Spec>;
#[doc = "Register `SCU5A8` writer"]
pub type W = crate::W<Scu5a8Spec>;
#[doc = "Field `SCUDISPDIO148` reader - SCU_DIS_PD_IO148"]
pub type Scudispdio148R = crate::BitReader;
#[doc = "Field `SCUDISPDIO148` writer - SCU_DIS_PD_IO148"]
pub type Scudispdio148W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO148` reader - SCU_DIS_PU_IO148"]
pub type Scudispuio148R = crate::BitReader;
#[doc = "Field `SCUDISPUIO148` writer - SCU_DIS_PU_IO148"]
pub type Scudispuio148W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO148` reader - SCU_DRV_IO148"]
pub type Scudrvio148R = crate::FieldReader;
#[doc = "Field `SCUDRVIO148` writer - SCU_DRV_IO148"]
pub type Scudrvio148W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO148` reader - SCU_EN_SMT_IO148"]
pub type Scuensmtio148R = crate::BitReader;
#[doc = "Field `SCUENSMTIO148` writer - SCU_EN_SMT_IO148"]
pub type Scuensmtio148W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO148` reader - SCU_EN_HV_IO148"]
pub type Scuenhvio148R = crate::BitReader;
#[doc = "Field `SCUENHVIO148` writer - SCU_EN_HV_IO148"]
pub type Scuenhvio148W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO149` reader - SCU_DIS_PD_IO149"]
pub type Scudispdio149R = crate::BitReader;
#[doc = "Field `SCUDISPDIO149` writer - SCU_DIS_PD_IO149"]
pub type Scudispdio149W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO149` reader - SCU_DIS_PU_IO149"]
pub type Scudispuio149R = crate::BitReader;
#[doc = "Field `SCUDISPUIO149` writer - SCU_DIS_PU_IO149"]
pub type Scudispuio149W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO149` reader - SCU_DRV_IO149"]
pub type Scudrvio149R = crate::FieldReader;
#[doc = "Field `SCUDRVIO149` writer - SCU_DRV_IO149"]
pub type Scudrvio149W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO149` reader - SCU_EN_SMT_IO149"]
pub type Scuensmtio149R = crate::BitReader;
#[doc = "Field `SCUENSMTIO149` writer - SCU_EN_SMT_IO149"]
pub type Scuensmtio149W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO149` reader - SCU_EN_HV_IO149"]
pub type Scuenhvio149R = crate::BitReader;
#[doc = "Field `SCUENHVIO149` writer - SCU_EN_HV_IO149"]
pub type Scuenhvio149W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO148"]
    #[inline(always)]
    pub fn scudispdio148(&self) -> Scudispdio148R {
        Scudispdio148R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO148"]
    #[inline(always)]
    pub fn scudispuio148(&self) -> Scudispuio148R {
        Scudispuio148R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO148"]
    #[inline(always)]
    pub fn scudrvio148(&self) -> Scudrvio148R {
        Scudrvio148R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO148"]
    #[inline(always)]
    pub fn scuensmtio148(&self) -> Scuensmtio148R {
        Scuensmtio148R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO148"]
    #[inline(always)]
    pub fn scuenhvio148(&self) -> Scuenhvio148R {
        Scuenhvio148R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO149"]
    #[inline(always)]
    pub fn scudispdio149(&self) -> Scudispdio149R {
        Scudispdio149R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO149"]
    #[inline(always)]
    pub fn scudispuio149(&self) -> Scudispuio149R {
        Scudispuio149R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO149"]
    #[inline(always)]
    pub fn scudrvio149(&self) -> Scudrvio149R {
        Scudrvio149R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO149"]
    #[inline(always)]
    pub fn scuensmtio149(&self) -> Scuensmtio149R {
        Scuensmtio149R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO149"]
    #[inline(always)]
    pub fn scuenhvio149(&self) -> Scuenhvio149R {
        Scuenhvio149R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO148"]
    #[inline(always)]
    pub fn scudispdio148(&mut self) -> Scudispdio148W<Scu5a8Spec> {
        Scudispdio148W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO148"]
    #[inline(always)]
    pub fn scudispuio148(&mut self) -> Scudispuio148W<Scu5a8Spec> {
        Scudispuio148W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO148"]
    #[inline(always)]
    pub fn scudrvio148(&mut self) -> Scudrvio148W<Scu5a8Spec> {
        Scudrvio148W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO148"]
    #[inline(always)]
    pub fn scuensmtio148(&mut self) -> Scuensmtio148W<Scu5a8Spec> {
        Scuensmtio148W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO148"]
    #[inline(always)]
    pub fn scuenhvio148(&mut self) -> Scuenhvio148W<Scu5a8Spec> {
        Scuenhvio148W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO149"]
    #[inline(always)]
    pub fn scudispdio149(&mut self) -> Scudispdio149W<Scu5a8Spec> {
        Scudispdio149W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO149"]
    #[inline(always)]
    pub fn scudispuio149(&mut self) -> Scudispuio149W<Scu5a8Spec> {
        Scudispuio149W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO149"]
    #[inline(always)]
    pub fn scudrvio149(&mut self) -> Scudrvio149W<Scu5a8Spec> {
        Scudrvio149W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO149"]
    #[inline(always)]
    pub fn scuensmtio149(&mut self) -> Scuensmtio149W<Scu5a8Spec> {
        Scuensmtio149W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO149"]
    #[inline(always)]
    pub fn scuenhvio149(&mut self) -> Scuenhvio149W<Scu5a8Spec> {
        Scuenhvio149W::new(self, 25)
    }
}
#[doc = "IO Control \\#75\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5a8Spec;
impl crate::RegisterSpec for Scu5a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5a8::R`](R) reader structure"]
impl crate::Readable for Scu5a8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5a8::W`](W) writer structure"]
impl crate::Writable for Scu5a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5A8 to value 0x0201_0204"]
impl crate::Resettable for Scu5a8Spec {
    const RESET_VALUE: u32 = 0x0201_0204;
}
