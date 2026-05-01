#[doc = "Register `SCU590` reader"]
pub type R = crate::R<Scu590Spec>;
#[doc = "Register `SCU590` writer"]
pub type W = crate::W<Scu590Spec>;
#[doc = "Field `SCUDISPDIO136` reader - SCU_DIS_PD_IO136"]
pub type Scudispdio136R = crate::BitReader;
#[doc = "Field `SCUDISPDIO136` writer - SCU_DIS_PD_IO136"]
pub type Scudispdio136W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO136` reader - SCU_DIS_PU_IO136"]
pub type Scudispuio136R = crate::BitReader;
#[doc = "Field `SCUDISPUIO136` writer - SCU_DIS_PU_IO136"]
pub type Scudispuio136W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO136` reader - SCU_DRV_IO136"]
pub type Scudrvio136R = crate::FieldReader;
#[doc = "Field `SCUDRVIO136` writer - SCU_DRV_IO136"]
pub type Scudrvio136W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO136` reader - SCU_EN_SMT_IO136"]
pub type Scuensmtio136R = crate::BitReader;
#[doc = "Field `SCUENSMTIO136` writer - SCU_EN_SMT_IO136"]
pub type Scuensmtio136W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO136` reader - SCU_EN_HV_IO136"]
pub type Scuenhvio136R = crate::BitReader;
#[doc = "Field `SCUENHVIO136` writer - SCU_EN_HV_IO136"]
pub type Scuenhvio136W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO137` reader - SCU_DIS_PD_IO137"]
pub type Scudispdio137R = crate::BitReader;
#[doc = "Field `SCUDISPDIO137` writer - SCU_DIS_PD_IO137"]
pub type Scudispdio137W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO137` reader - SCU_DIS_PU_IO137"]
pub type Scudispuio137R = crate::BitReader;
#[doc = "Field `SCUDISPUIO137` writer - SCU_DIS_PU_IO137"]
pub type Scudispuio137W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO137` reader - SCU_DRV_IO137"]
pub type Scudrvio137R = crate::FieldReader;
#[doc = "Field `SCUDRVIO137` writer - SCU_DRV_IO137"]
pub type Scudrvio137W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO137` reader - SCU_EN_SMT_IO137"]
pub type Scuensmtio137R = crate::BitReader;
#[doc = "Field `SCUENSMTIO137` writer - SCU_EN_SMT_IO137"]
pub type Scuensmtio137W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO137` reader - SCU_EN_HV_IO137"]
pub type Scuenhvio137R = crate::BitReader;
#[doc = "Field `SCUENHVIO137` writer - SCU_EN_HV_IO137"]
pub type Scuenhvio137W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO136"]
    #[inline(always)]
    pub fn scudispdio136(&self) -> Scudispdio136R {
        Scudispdio136R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO136"]
    #[inline(always)]
    pub fn scudispuio136(&self) -> Scudispuio136R {
        Scudispuio136R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO136"]
    #[inline(always)]
    pub fn scudrvio136(&self) -> Scudrvio136R {
        Scudrvio136R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO136"]
    #[inline(always)]
    pub fn scuensmtio136(&self) -> Scuensmtio136R {
        Scuensmtio136R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO136"]
    #[inline(always)]
    pub fn scuenhvio136(&self) -> Scuenhvio136R {
        Scuenhvio136R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO137"]
    #[inline(always)]
    pub fn scudispdio137(&self) -> Scudispdio137R {
        Scudispdio137R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO137"]
    #[inline(always)]
    pub fn scudispuio137(&self) -> Scudispuio137R {
        Scudispuio137R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO137"]
    #[inline(always)]
    pub fn scudrvio137(&self) -> Scudrvio137R {
        Scudrvio137R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO137"]
    #[inline(always)]
    pub fn scuensmtio137(&self) -> Scuensmtio137R {
        Scuensmtio137R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO137"]
    #[inline(always)]
    pub fn scuenhvio137(&self) -> Scuenhvio137R {
        Scuenhvio137R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO136"]
    #[inline(always)]
    pub fn scudispdio136(&mut self) -> Scudispdio136W<Scu590Spec> {
        Scudispdio136W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO136"]
    #[inline(always)]
    pub fn scudispuio136(&mut self) -> Scudispuio136W<Scu590Spec> {
        Scudispuio136W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO136"]
    #[inline(always)]
    pub fn scudrvio136(&mut self) -> Scudrvio136W<Scu590Spec> {
        Scudrvio136W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO136"]
    #[inline(always)]
    pub fn scuensmtio136(&mut self) -> Scuensmtio136W<Scu590Spec> {
        Scuensmtio136W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO136"]
    #[inline(always)]
    pub fn scuenhvio136(&mut self) -> Scuenhvio136W<Scu590Spec> {
        Scuenhvio136W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO137"]
    #[inline(always)]
    pub fn scudispdio137(&mut self) -> Scudispdio137W<Scu590Spec> {
        Scudispdio137W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO137"]
    #[inline(always)]
    pub fn scudispuio137(&mut self) -> Scudispuio137W<Scu590Spec> {
        Scudispuio137W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO137"]
    #[inline(always)]
    pub fn scudrvio137(&mut self) -> Scudrvio137W<Scu590Spec> {
        Scudrvio137W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO137"]
    #[inline(always)]
    pub fn scuensmtio137(&mut self) -> Scuensmtio137W<Scu590Spec> {
        Scuensmtio137W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO137"]
    #[inline(always)]
    pub fn scuenhvio137(&mut self) -> Scuenhvio137W<Scu590Spec> {
        Scuenhvio137W::new(self, 25)
    }
}
#[doc = "IO Control \\#69\n\nYou can [`read`](crate::Reg::read) this register and get [`scu590::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu590::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu590Spec;
impl crate::RegisterSpec for Scu590Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu590::R`](R) reader structure"]
impl crate::Readable for Scu590Spec {}
#[doc = "`write(|w| ..)` method takes [`scu590::W`](W) writer structure"]
impl crate::Writable for Scu590Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU590 to value 0x0204_0204"]
impl crate::Resettable for Scu590Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
