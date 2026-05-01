#[doc = "Register `SCU538` reader"]
pub type R = crate::R<Scu538Spec>;
#[doc = "Register `SCU538` writer"]
pub type W = crate::W<Scu538Spec>;
#[doc = "Field `SCUDISPDIO092` reader - SCU_DIS_PD_IO092"]
pub type Scudispdio092R = crate::BitReader;
#[doc = "Field `SCUDISPDIO092` writer - SCU_DIS_PD_IO092"]
pub type Scudispdio092W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO092` reader - SCU_DIS_PU_IO092"]
pub type Scudispuio092R = crate::BitReader;
#[doc = "Field `SCUDISPUIO092` writer - SCU_DIS_PU_IO092"]
pub type Scudispuio092W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO092` reader - SCU_DRV_IO092"]
pub type Scudrvio092R = crate::FieldReader;
#[doc = "Field `SCUDRVIO092` writer - SCU_DRV_IO092"]
pub type Scudrvio092W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO092` reader - SCU_EN_SMT_IO092"]
pub type Scuensmtio092R = crate::BitReader;
#[doc = "Field `SCUENSMTIO092` writer - SCU_EN_SMT_IO092"]
pub type Scuensmtio092W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO092` reader - SCU_EN_HV_IO092"]
pub type Scuenhvio092R = crate::BitReader;
#[doc = "Field `SCUENHVIO092` writer - SCU_EN_HV_IO092"]
pub type Scuenhvio092W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO093` reader - SCU_DIS_PD_IO093"]
pub type Scudispdio093R = crate::BitReader;
#[doc = "Field `SCUDISPDIO093` writer - SCU_DIS_PD_IO093"]
pub type Scudispdio093W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO093` reader - SCU_DIS_PU_IO093"]
pub type Scudispuio093R = crate::BitReader;
#[doc = "Field `SCUDISPUIO093` writer - SCU_DIS_PU_IO093"]
pub type Scudispuio093W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO093` reader - SCU_DRV_IO093"]
pub type Scudrvio093R = crate::FieldReader;
#[doc = "Field `SCUDRVIO093` writer - SCU_DRV_IO093"]
pub type Scudrvio093W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO093` reader - SCU_EN_SMT_IO093"]
pub type Scuensmtio093R = crate::BitReader;
#[doc = "Field `SCUENSMTIO093` writer - SCU_EN_SMT_IO093"]
pub type Scuensmtio093W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO093` reader - SCU_EN_HV_IO093"]
pub type Scuenhvio093R = crate::BitReader;
#[doc = "Field `SCUENHVIO093` writer - SCU_EN_HV_IO093"]
pub type Scuenhvio093W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO092"]
    #[inline(always)]
    pub fn scudispdio092(&self) -> Scudispdio092R {
        Scudispdio092R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO092"]
    #[inline(always)]
    pub fn scudispuio092(&self) -> Scudispuio092R {
        Scudispuio092R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO092"]
    #[inline(always)]
    pub fn scudrvio092(&self) -> Scudrvio092R {
        Scudrvio092R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO092"]
    #[inline(always)]
    pub fn scuensmtio092(&self) -> Scuensmtio092R {
        Scuensmtio092R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO092"]
    #[inline(always)]
    pub fn scuenhvio092(&self) -> Scuenhvio092R {
        Scuenhvio092R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO093"]
    #[inline(always)]
    pub fn scudispdio093(&self) -> Scudispdio093R {
        Scudispdio093R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO093"]
    #[inline(always)]
    pub fn scudispuio093(&self) -> Scudispuio093R {
        Scudispuio093R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO093"]
    #[inline(always)]
    pub fn scudrvio093(&self) -> Scudrvio093R {
        Scudrvio093R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO093"]
    #[inline(always)]
    pub fn scuensmtio093(&self) -> Scuensmtio093R {
        Scuensmtio093R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO093"]
    #[inline(always)]
    pub fn scuenhvio093(&self) -> Scuenhvio093R {
        Scuenhvio093R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO092"]
    #[inline(always)]
    pub fn scudispdio092(&mut self) -> Scudispdio092W<Scu538Spec> {
        Scudispdio092W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO092"]
    #[inline(always)]
    pub fn scudispuio092(&mut self) -> Scudispuio092W<Scu538Spec> {
        Scudispuio092W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO092"]
    #[inline(always)]
    pub fn scudrvio092(&mut self) -> Scudrvio092W<Scu538Spec> {
        Scudrvio092W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO092"]
    #[inline(always)]
    pub fn scuensmtio092(&mut self) -> Scuensmtio092W<Scu538Spec> {
        Scuensmtio092W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO092"]
    #[inline(always)]
    pub fn scuenhvio092(&mut self) -> Scuenhvio092W<Scu538Spec> {
        Scuenhvio092W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO093"]
    #[inline(always)]
    pub fn scudispdio093(&mut self) -> Scudispdio093W<Scu538Spec> {
        Scudispdio093W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO093"]
    #[inline(always)]
    pub fn scudispuio093(&mut self) -> Scudispuio093W<Scu538Spec> {
        Scudispuio093W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO093"]
    #[inline(always)]
    pub fn scudrvio093(&mut self) -> Scudrvio093W<Scu538Spec> {
        Scudrvio093W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO093"]
    #[inline(always)]
    pub fn scuensmtio093(&mut self) -> Scuensmtio093W<Scu538Spec> {
        Scuensmtio093W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO093"]
    #[inline(always)]
    pub fn scuenhvio093(&mut self) -> Scuenhvio093W<Scu538Spec> {
        Scuenhvio093W::new(self, 25)
    }
}
#[doc = "IO Control \\#47\n\nYou can [`read`](crate::Reg::read) this register and get [`scu538::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu538::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu538Spec;
impl crate::RegisterSpec for Scu538Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu538::R`](R) reader structure"]
impl crate::Readable for Scu538Spec {}
#[doc = "`write(|w| ..)` method takes [`scu538::W`](W) writer structure"]
impl crate::Writable for Scu538Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU538 to value 0x0204_0204"]
impl crate::Resettable for Scu538Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
