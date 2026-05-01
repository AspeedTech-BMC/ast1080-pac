#[doc = "Register `SCU490` reader"]
pub type R = crate::R<Scu490Spec>;
#[doc = "Register `SCU490` writer"]
pub type W = crate::W<Scu490Spec>;
#[doc = "Field `SCUDISPDIO008` reader - SCU_DIS_PD_IO008"]
pub type Scudispdio008R = crate::BitReader;
#[doc = "Field `SCUDISPDIO008` writer - SCU_DIS_PD_IO008"]
pub type Scudispdio008W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO008` reader - SCU_DIS_PU_IO008"]
pub type Scudispuio008R = crate::BitReader;
#[doc = "Field `SCUDISPUIO008` writer - SCU_DIS_PU_IO008"]
pub type Scudispuio008W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO008` reader - SCU_DRV_IO008"]
pub type Scudrvio008R = crate::FieldReader;
#[doc = "Field `SCUDRVIO008` writer - SCU_DRV_IO008"]
pub type Scudrvio008W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO008` reader - SCU_EN_SMT_IO008"]
pub type Scuensmtio008R = crate::BitReader;
#[doc = "Field `SCUENSMTIO008` writer - SCU_EN_SMT_IO008"]
pub type Scuensmtio008W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO008` reader - SCU_EN_HV_IO008"]
pub type Scuenhvio008R = crate::BitReader;
#[doc = "Field `SCUENHVIO008` writer - SCU_EN_HV_IO008"]
pub type Scuenhvio008W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO009` reader - SCU_DIS_PD_IO009"]
pub type Scudispdio009R = crate::BitReader;
#[doc = "Field `SCUDISPDIO009` writer - SCU_DIS_PD_IO009"]
pub type Scudispdio009W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO009` reader - SCU_DIS_PU_IO009"]
pub type Scudispuio009R = crate::BitReader;
#[doc = "Field `SCUDISPUIO009` writer - SCU_DIS_PU_IO009"]
pub type Scudispuio009W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO009` reader - SCU_DRV_IO009"]
pub type Scudrvio009R = crate::FieldReader;
#[doc = "Field `SCUDRVIO009` writer - SCU_DRV_IO009"]
pub type Scudrvio009W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO009` reader - SCU_EN_SMT_IO009"]
pub type Scuensmtio009R = crate::BitReader;
#[doc = "Field `SCUENSMTIO009` writer - SCU_EN_SMT_IO009"]
pub type Scuensmtio009W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO009` reader - SCU_EN_HV_IO009"]
pub type Scuenhvio009R = crate::BitReader;
#[doc = "Field `SCUENHVIO009` writer - SCU_EN_HV_IO009"]
pub type Scuenhvio009W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO008"]
    #[inline(always)]
    pub fn scudispdio008(&self) -> Scudispdio008R {
        Scudispdio008R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO008"]
    #[inline(always)]
    pub fn scudispuio008(&self) -> Scudispuio008R {
        Scudispuio008R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO008"]
    #[inline(always)]
    pub fn scudrvio008(&self) -> Scudrvio008R {
        Scudrvio008R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO008"]
    #[inline(always)]
    pub fn scuensmtio008(&self) -> Scuensmtio008R {
        Scuensmtio008R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO008"]
    #[inline(always)]
    pub fn scuenhvio008(&self) -> Scuenhvio008R {
        Scuenhvio008R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO009"]
    #[inline(always)]
    pub fn scudispdio009(&self) -> Scudispdio009R {
        Scudispdio009R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO009"]
    #[inline(always)]
    pub fn scudispuio009(&self) -> Scudispuio009R {
        Scudispuio009R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO009"]
    #[inline(always)]
    pub fn scudrvio009(&self) -> Scudrvio009R {
        Scudrvio009R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO009"]
    #[inline(always)]
    pub fn scuensmtio009(&self) -> Scuensmtio009R {
        Scuensmtio009R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO009"]
    #[inline(always)]
    pub fn scuenhvio009(&self) -> Scuenhvio009R {
        Scuenhvio009R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO008"]
    #[inline(always)]
    pub fn scudispdio008(&mut self) -> Scudispdio008W<Scu490Spec> {
        Scudispdio008W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO008"]
    #[inline(always)]
    pub fn scudispuio008(&mut self) -> Scudispuio008W<Scu490Spec> {
        Scudispuio008W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO008"]
    #[inline(always)]
    pub fn scudrvio008(&mut self) -> Scudrvio008W<Scu490Spec> {
        Scudrvio008W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO008"]
    #[inline(always)]
    pub fn scuensmtio008(&mut self) -> Scuensmtio008W<Scu490Spec> {
        Scuensmtio008W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO008"]
    #[inline(always)]
    pub fn scuenhvio008(&mut self) -> Scuenhvio008W<Scu490Spec> {
        Scuenhvio008W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO009"]
    #[inline(always)]
    pub fn scudispdio009(&mut self) -> Scudispdio009W<Scu490Spec> {
        Scudispdio009W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO009"]
    #[inline(always)]
    pub fn scudispuio009(&mut self) -> Scudispuio009W<Scu490Spec> {
        Scudispuio009W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO009"]
    #[inline(always)]
    pub fn scudrvio009(&mut self) -> Scudrvio009W<Scu490Spec> {
        Scudrvio009W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO009"]
    #[inline(always)]
    pub fn scuensmtio009(&mut self) -> Scuensmtio009W<Scu490Spec> {
        Scuensmtio009W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO009"]
    #[inline(always)]
    pub fn scuenhvio009(&mut self) -> Scuenhvio009W<Scu490Spec> {
        Scuenhvio009W::new(self, 25)
    }
}
#[doc = "IO Control \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu490::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu490::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu490Spec;
impl crate::RegisterSpec for Scu490Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu490::R`](R) reader structure"]
impl crate::Readable for Scu490Spec {}
#[doc = "`write(|w| ..)` method takes [`scu490::W`](W) writer structure"]
impl crate::Writable for Scu490Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU490 to value 0x0204_0204"]
impl crate::Resettable for Scu490Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
