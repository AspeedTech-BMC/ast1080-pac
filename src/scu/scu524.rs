#[doc = "Register `SCU524` reader"]
pub type R = crate::R<Scu524Spec>;
#[doc = "Register `SCU524` writer"]
pub type W = crate::W<Scu524Spec>;
#[doc = "Field `SCUDISPDIO082` reader - SCU_DIS_PD_IO082"]
pub type Scudispdio082R = crate::BitReader;
#[doc = "Field `SCUDISPDIO082` writer - SCU_DIS_PD_IO082"]
pub type Scudispdio082W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO082` reader - SCU_DIS_PU_IO082"]
pub type Scudispuio082R = crate::BitReader;
#[doc = "Field `SCUDISPUIO082` writer - SCU_DIS_PU_IO082"]
pub type Scudispuio082W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO082` reader - SCU_DRV_IO082"]
pub type Scudrvio082R = crate::FieldReader;
#[doc = "Field `SCUDRVIO082` writer - SCU_DRV_IO082"]
pub type Scudrvio082W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO082` reader - SCU_EN_SMT_IO082"]
pub type Scuensmtio082R = crate::BitReader;
#[doc = "Field `SCUENSMTIO082` writer - SCU_EN_SMT_IO082"]
pub type Scuensmtio082W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO082` reader - SCU_EN_HV_IO082"]
pub type Scuenhvio082R = crate::BitReader;
#[doc = "Field `SCUENHVIO082` writer - SCU_EN_HV_IO082"]
pub type Scuenhvio082W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO083` reader - SCU_DIS_PD_IO083"]
pub type Scudispdio083R = crate::BitReader;
#[doc = "Field `SCUDISPDIO083` writer - SCU_DIS_PD_IO083"]
pub type Scudispdio083W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO083` reader - SCU_DIS_PU_IO083"]
pub type Scudispuio083R = crate::BitReader;
#[doc = "Field `SCUDISPUIO083` writer - SCU_DIS_PU_IO083"]
pub type Scudispuio083W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO083` reader - SCU_DRV_IO083"]
pub type Scudrvio083R = crate::FieldReader;
#[doc = "Field `SCUDRVIO083` writer - SCU_DRV_IO083"]
pub type Scudrvio083W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO083` reader - SCU_EN_SMT_IO083"]
pub type Scuensmtio083R = crate::BitReader;
#[doc = "Field `SCUENSMTIO083` writer - SCU_EN_SMT_IO083"]
pub type Scuensmtio083W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO083` reader - SCU_EN_HV_IO083"]
pub type Scuenhvio083R = crate::BitReader;
#[doc = "Field `SCUENHVIO083` writer - SCU_EN_HV_IO083"]
pub type Scuenhvio083W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO082"]
    #[inline(always)]
    pub fn scudispdio082(&self) -> Scudispdio082R {
        Scudispdio082R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO082"]
    #[inline(always)]
    pub fn scudispuio082(&self) -> Scudispuio082R {
        Scudispuio082R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO082"]
    #[inline(always)]
    pub fn scudrvio082(&self) -> Scudrvio082R {
        Scudrvio082R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO082"]
    #[inline(always)]
    pub fn scuensmtio082(&self) -> Scuensmtio082R {
        Scuensmtio082R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO082"]
    #[inline(always)]
    pub fn scuenhvio082(&self) -> Scuenhvio082R {
        Scuenhvio082R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO083"]
    #[inline(always)]
    pub fn scudispdio083(&self) -> Scudispdio083R {
        Scudispdio083R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO083"]
    #[inline(always)]
    pub fn scudispuio083(&self) -> Scudispuio083R {
        Scudispuio083R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO083"]
    #[inline(always)]
    pub fn scudrvio083(&self) -> Scudrvio083R {
        Scudrvio083R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO083"]
    #[inline(always)]
    pub fn scuensmtio083(&self) -> Scuensmtio083R {
        Scuensmtio083R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO083"]
    #[inline(always)]
    pub fn scuenhvio083(&self) -> Scuenhvio083R {
        Scuenhvio083R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO082"]
    #[inline(always)]
    pub fn scudispdio082(&mut self) -> Scudispdio082W<Scu524Spec> {
        Scudispdio082W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO082"]
    #[inline(always)]
    pub fn scudispuio082(&mut self) -> Scudispuio082W<Scu524Spec> {
        Scudispuio082W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO082"]
    #[inline(always)]
    pub fn scudrvio082(&mut self) -> Scudrvio082W<Scu524Spec> {
        Scudrvio082W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO082"]
    #[inline(always)]
    pub fn scuensmtio082(&mut self) -> Scuensmtio082W<Scu524Spec> {
        Scuensmtio082W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO082"]
    #[inline(always)]
    pub fn scuenhvio082(&mut self) -> Scuenhvio082W<Scu524Spec> {
        Scuenhvio082W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO083"]
    #[inline(always)]
    pub fn scudispdio083(&mut self) -> Scudispdio083W<Scu524Spec> {
        Scudispdio083W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO083"]
    #[inline(always)]
    pub fn scudispuio083(&mut self) -> Scudispuio083W<Scu524Spec> {
        Scudispuio083W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO083"]
    #[inline(always)]
    pub fn scudrvio083(&mut self) -> Scudrvio083W<Scu524Spec> {
        Scudrvio083W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO083"]
    #[inline(always)]
    pub fn scuensmtio083(&mut self) -> Scuensmtio083W<Scu524Spec> {
        Scuensmtio083W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO083"]
    #[inline(always)]
    pub fn scuenhvio083(&mut self) -> Scuenhvio083W<Scu524Spec> {
        Scuenhvio083W::new(self, 25)
    }
}
#[doc = "IO Control \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`scu524::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu524::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu524Spec;
impl crate::RegisterSpec for Scu524Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu524::R`](R) reader structure"]
impl crate::Readable for Scu524Spec {}
#[doc = "`write(|w| ..)` method takes [`scu524::W`](W) writer structure"]
impl crate::Writable for Scu524Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU524 to value 0x0204_0204"]
impl crate::Resettable for Scu524Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
