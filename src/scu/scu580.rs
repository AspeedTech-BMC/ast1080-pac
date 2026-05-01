#[doc = "Register `SCU580` reader"]
pub type R = crate::R<Scu580Spec>;
#[doc = "Register `SCU580` writer"]
pub type W = crate::W<Scu580Spec>;
#[doc = "Field `SCUDISPDIO128` reader - SCU_DIS_PD_IO128"]
pub type Scudispdio128R = crate::BitReader;
#[doc = "Field `SCUDISPDIO128` writer - SCU_DIS_PD_IO128"]
pub type Scudispdio128W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO128` reader - SCU_DIS_PU_IO128"]
pub type Scudispuio128R = crate::BitReader;
#[doc = "Field `SCUDISPUIO128` writer - SCU_DIS_PU_IO128"]
pub type Scudispuio128W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO128` reader - SCU_DRV_IO128"]
pub type Scudrvio128R = crate::FieldReader;
#[doc = "Field `SCUDRVIO128` writer - SCU_DRV_IO128"]
pub type Scudrvio128W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO128` reader - SCU_EN_SMT_IO128"]
pub type Scuensmtio128R = crate::BitReader;
#[doc = "Field `SCUENSMTIO128` writer - SCU_EN_SMT_IO128"]
pub type Scuensmtio128W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO128` reader - SCU_EN_HV_IO128"]
pub type Scuenhvio128R = crate::BitReader;
#[doc = "Field `SCUENHVIO128` writer - SCU_EN_HV_IO128"]
pub type Scuenhvio128W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO129` reader - SCU_DIS_PD_IO129"]
pub type Scudispdio129R = crate::BitReader;
#[doc = "Field `SCUDISPDIO129` writer - SCU_DIS_PD_IO129"]
pub type Scudispdio129W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO129` reader - SCU_DIS_PU_IO129"]
pub type Scudispuio129R = crate::BitReader;
#[doc = "Field `SCUDISPUIO129` writer - SCU_DIS_PU_IO129"]
pub type Scudispuio129W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO129` reader - SCU_DRV_IO129"]
pub type Scudrvio129R = crate::FieldReader;
#[doc = "Field `SCUDRVIO129` writer - SCU_DRV_IO129"]
pub type Scudrvio129W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO129` reader - SCU_EN_SMT_IO129"]
pub type Scuensmtio129R = crate::BitReader;
#[doc = "Field `SCUENSMTIO129` writer - SCU_EN_SMT_IO129"]
pub type Scuensmtio129W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO129` reader - SCU_EN_HV_IO129"]
pub type Scuenhvio129R = crate::BitReader;
#[doc = "Field `SCUENHVIO129` writer - SCU_EN_HV_IO129"]
pub type Scuenhvio129W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO128"]
    #[inline(always)]
    pub fn scudispdio128(&self) -> Scudispdio128R {
        Scudispdio128R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO128"]
    #[inline(always)]
    pub fn scudispuio128(&self) -> Scudispuio128R {
        Scudispuio128R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO128"]
    #[inline(always)]
    pub fn scudrvio128(&self) -> Scudrvio128R {
        Scudrvio128R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO128"]
    #[inline(always)]
    pub fn scuensmtio128(&self) -> Scuensmtio128R {
        Scuensmtio128R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO128"]
    #[inline(always)]
    pub fn scuenhvio128(&self) -> Scuenhvio128R {
        Scuenhvio128R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO129"]
    #[inline(always)]
    pub fn scudispdio129(&self) -> Scudispdio129R {
        Scudispdio129R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO129"]
    #[inline(always)]
    pub fn scudispuio129(&self) -> Scudispuio129R {
        Scudispuio129R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO129"]
    #[inline(always)]
    pub fn scudrvio129(&self) -> Scudrvio129R {
        Scudrvio129R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO129"]
    #[inline(always)]
    pub fn scuensmtio129(&self) -> Scuensmtio129R {
        Scuensmtio129R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO129"]
    #[inline(always)]
    pub fn scuenhvio129(&self) -> Scuenhvio129R {
        Scuenhvio129R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO128"]
    #[inline(always)]
    pub fn scudispdio128(&mut self) -> Scudispdio128W<Scu580Spec> {
        Scudispdio128W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO128"]
    #[inline(always)]
    pub fn scudispuio128(&mut self) -> Scudispuio128W<Scu580Spec> {
        Scudispuio128W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO128"]
    #[inline(always)]
    pub fn scudrvio128(&mut self) -> Scudrvio128W<Scu580Spec> {
        Scudrvio128W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO128"]
    #[inline(always)]
    pub fn scuensmtio128(&mut self) -> Scuensmtio128W<Scu580Spec> {
        Scuensmtio128W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO128"]
    #[inline(always)]
    pub fn scuenhvio128(&mut self) -> Scuenhvio128W<Scu580Spec> {
        Scuenhvio128W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO129"]
    #[inline(always)]
    pub fn scudispdio129(&mut self) -> Scudispdio129W<Scu580Spec> {
        Scudispdio129W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO129"]
    #[inline(always)]
    pub fn scudispuio129(&mut self) -> Scudispuio129W<Scu580Spec> {
        Scudispuio129W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO129"]
    #[inline(always)]
    pub fn scudrvio129(&mut self) -> Scudrvio129W<Scu580Spec> {
        Scudrvio129W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO129"]
    #[inline(always)]
    pub fn scuensmtio129(&mut self) -> Scuensmtio129W<Scu580Spec> {
        Scuensmtio129W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO129"]
    #[inline(always)]
    pub fn scuenhvio129(&mut self) -> Scuenhvio129W<Scu580Spec> {
        Scuenhvio129W::new(self, 25)
    }
}
#[doc = "IO Control \\#65\n\nYou can [`read`](crate::Reg::read) this register and get [`scu580::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu580::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu580Spec;
impl crate::RegisterSpec for Scu580Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu580::R`](R) reader structure"]
impl crate::Readable for Scu580Spec {}
#[doc = "`write(|w| ..)` method takes [`scu580::W`](W) writer structure"]
impl crate::Writable for Scu580Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU580 to value 0x0204_0204"]
impl crate::Resettable for Scu580Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
