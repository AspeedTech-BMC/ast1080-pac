#[doc = "Register `SCU570` reader"]
pub type R = crate::R<Scu570Spec>;
#[doc = "Register `SCU570` writer"]
pub type W = crate::W<Scu570Spec>;
#[doc = "Field `SCUDISPDIO120` reader - SCU_DIS_PD_IO120"]
pub type Scudispdio120R = crate::BitReader;
#[doc = "Field `SCUDISPDIO120` writer - SCU_DIS_PD_IO120"]
pub type Scudispdio120W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO120` reader - SCU_DIS_PU_IO120"]
pub type Scudispuio120R = crate::BitReader;
#[doc = "Field `SCUDISPUIO120` writer - SCU_DIS_PU_IO120"]
pub type Scudispuio120W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO120` reader - SCU_DRV_IO120"]
pub type Scudrvio120R = crate::FieldReader;
#[doc = "Field `SCUDRVIO120` writer - SCU_DRV_IO120"]
pub type Scudrvio120W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO120` reader - SCU_EN_SMT_IO120"]
pub type Scuensmtio120R = crate::BitReader;
#[doc = "Field `SCUENSMTIO120` writer - SCU_EN_SMT_IO120"]
pub type Scuensmtio120W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO120` reader - SCU_EN_HV_IO120"]
pub type Scuenhvio120R = crate::BitReader;
#[doc = "Field `SCUENHVIO120` writer - SCU_EN_HV_IO120"]
pub type Scuenhvio120W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO121` reader - SCU_DIS_PD_IO121"]
pub type Scudispdio121R = crate::BitReader;
#[doc = "Field `SCUDISPDIO121` writer - SCU_DIS_PD_IO121"]
pub type Scudispdio121W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO121` reader - SCU_DIS_PU_IO121"]
pub type Scudispuio121R = crate::BitReader;
#[doc = "Field `SCUDISPUIO121` writer - SCU_DIS_PU_IO121"]
pub type Scudispuio121W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO121` reader - SCU_DRV_IO121"]
pub type Scudrvio121R = crate::FieldReader;
#[doc = "Field `SCUDRVIO121` writer - SCU_DRV_IO121"]
pub type Scudrvio121W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO121` reader - SCU_EN_SMT_IO121"]
pub type Scuensmtio121R = crate::BitReader;
#[doc = "Field `SCUENSMTIO121` writer - SCU_EN_SMT_IO121"]
pub type Scuensmtio121W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO121` reader - SCU_EN_HV_IO121"]
pub type Scuenhvio121R = crate::BitReader;
#[doc = "Field `SCUENHVIO121` writer - SCU_EN_HV_IO121"]
pub type Scuenhvio121W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO120"]
    #[inline(always)]
    pub fn scudispdio120(&self) -> Scudispdio120R {
        Scudispdio120R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO120"]
    #[inline(always)]
    pub fn scudispuio120(&self) -> Scudispuio120R {
        Scudispuio120R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO120"]
    #[inline(always)]
    pub fn scudrvio120(&self) -> Scudrvio120R {
        Scudrvio120R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO120"]
    #[inline(always)]
    pub fn scuensmtio120(&self) -> Scuensmtio120R {
        Scuensmtio120R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO120"]
    #[inline(always)]
    pub fn scuenhvio120(&self) -> Scuenhvio120R {
        Scuenhvio120R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO121"]
    #[inline(always)]
    pub fn scudispdio121(&self) -> Scudispdio121R {
        Scudispdio121R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO121"]
    #[inline(always)]
    pub fn scudispuio121(&self) -> Scudispuio121R {
        Scudispuio121R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO121"]
    #[inline(always)]
    pub fn scudrvio121(&self) -> Scudrvio121R {
        Scudrvio121R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO121"]
    #[inline(always)]
    pub fn scuensmtio121(&self) -> Scuensmtio121R {
        Scuensmtio121R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO121"]
    #[inline(always)]
    pub fn scuenhvio121(&self) -> Scuenhvio121R {
        Scuenhvio121R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO120"]
    #[inline(always)]
    pub fn scudispdio120(&mut self) -> Scudispdio120W<Scu570Spec> {
        Scudispdio120W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO120"]
    #[inline(always)]
    pub fn scudispuio120(&mut self) -> Scudispuio120W<Scu570Spec> {
        Scudispuio120W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO120"]
    #[inline(always)]
    pub fn scudrvio120(&mut self) -> Scudrvio120W<Scu570Spec> {
        Scudrvio120W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO120"]
    #[inline(always)]
    pub fn scuensmtio120(&mut self) -> Scuensmtio120W<Scu570Spec> {
        Scuensmtio120W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO120"]
    #[inline(always)]
    pub fn scuenhvio120(&mut self) -> Scuenhvio120W<Scu570Spec> {
        Scuenhvio120W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO121"]
    #[inline(always)]
    pub fn scudispdio121(&mut self) -> Scudispdio121W<Scu570Spec> {
        Scudispdio121W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO121"]
    #[inline(always)]
    pub fn scudispuio121(&mut self) -> Scudispuio121W<Scu570Spec> {
        Scudispuio121W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO121"]
    #[inline(always)]
    pub fn scudrvio121(&mut self) -> Scudrvio121W<Scu570Spec> {
        Scudrvio121W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO121"]
    #[inline(always)]
    pub fn scuensmtio121(&mut self) -> Scuensmtio121W<Scu570Spec> {
        Scuensmtio121W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO121"]
    #[inline(always)]
    pub fn scuenhvio121(&mut self) -> Scuenhvio121W<Scu570Spec> {
        Scuenhvio121W::new(self, 25)
    }
}
#[doc = "IO Control \\#61\n\nYou can [`read`](crate::Reg::read) this register and get [`scu570::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu570::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu570Spec;
impl crate::RegisterSpec for Scu570Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu570::R`](R) reader structure"]
impl crate::Readable for Scu570Spec {}
#[doc = "`write(|w| ..)` method takes [`scu570::W`](W) writer structure"]
impl crate::Writable for Scu570Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU570 to value 0x0204_0204"]
impl crate::Resettable for Scu570Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
