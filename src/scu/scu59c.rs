#[doc = "Register `SCU59C` reader"]
pub type R = crate::R<Scu59cSpec>;
#[doc = "Register `SCU59C` writer"]
pub type W = crate::W<Scu59cSpec>;
#[doc = "Field `SCUDISPDIO142` reader - SCU_DIS_PD_IO142"]
pub type Scudispdio142R = crate::BitReader;
#[doc = "Field `SCUDISPDIO142` writer - SCU_DIS_PD_IO142"]
pub type Scudispdio142W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO142` reader - SCU_DIS_PU_IO142"]
pub type Scudispuio142R = crate::BitReader;
#[doc = "Field `SCUDISPUIO142` writer - SCU_DIS_PU_IO142"]
pub type Scudispuio142W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO142` reader - SCU_DRV_IO142"]
pub type Scudrvio142R = crate::FieldReader;
#[doc = "Field `SCUDRVIO142` writer - SCU_DRV_IO142"]
pub type Scudrvio142W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO142` reader - SCU_EN_SMT_IO142"]
pub type Scuensmtio142R = crate::BitReader;
#[doc = "Field `SCUENSMTIO142` writer - SCU_EN_SMT_IO142"]
pub type Scuensmtio142W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO142` reader - SCU_EN_HV_IO142"]
pub type Scuenhvio142R = crate::BitReader;
#[doc = "Field `SCUENHVIO142` writer - SCU_EN_HV_IO142"]
pub type Scuenhvio142W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO143` reader - SCU_DIS_PD_IO143"]
pub type Scudispdio143R = crate::BitReader;
#[doc = "Field `SCUDISPDIO143` writer - SCU_DIS_PD_IO143"]
pub type Scudispdio143W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO143` reader - SCU_DIS_PU_IO143"]
pub type Scudispuio143R = crate::BitReader;
#[doc = "Field `SCUDISPUIO143` writer - SCU_DIS_PU_IO143"]
pub type Scudispuio143W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO143` reader - SCU_DRV_IO143"]
pub type Scudrvio143R = crate::FieldReader;
#[doc = "Field `SCUDRVIO143` writer - SCU_DRV_IO143"]
pub type Scudrvio143W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO143` reader - SCU_EN_SMT_IO143"]
pub type Scuensmtio143R = crate::BitReader;
#[doc = "Field `SCUENSMTIO143` writer - SCU_EN_SMT_IO143"]
pub type Scuensmtio143W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO143` reader - SCU_EN_HV_IO143"]
pub type Scuenhvio143R = crate::BitReader;
#[doc = "Field `SCUENHVIO143` writer - SCU_EN_HV_IO143"]
pub type Scuenhvio143W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO142"]
    #[inline(always)]
    pub fn scudispdio142(&self) -> Scudispdio142R {
        Scudispdio142R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO142"]
    #[inline(always)]
    pub fn scudispuio142(&self) -> Scudispuio142R {
        Scudispuio142R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO142"]
    #[inline(always)]
    pub fn scudrvio142(&self) -> Scudrvio142R {
        Scudrvio142R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO142"]
    #[inline(always)]
    pub fn scuensmtio142(&self) -> Scuensmtio142R {
        Scuensmtio142R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO142"]
    #[inline(always)]
    pub fn scuenhvio142(&self) -> Scuenhvio142R {
        Scuenhvio142R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO143"]
    #[inline(always)]
    pub fn scudispdio143(&self) -> Scudispdio143R {
        Scudispdio143R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO143"]
    #[inline(always)]
    pub fn scudispuio143(&self) -> Scudispuio143R {
        Scudispuio143R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO143"]
    #[inline(always)]
    pub fn scudrvio143(&self) -> Scudrvio143R {
        Scudrvio143R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO143"]
    #[inline(always)]
    pub fn scuensmtio143(&self) -> Scuensmtio143R {
        Scuensmtio143R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO143"]
    #[inline(always)]
    pub fn scuenhvio143(&self) -> Scuenhvio143R {
        Scuenhvio143R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO142"]
    #[inline(always)]
    pub fn scudispdio142(&mut self) -> Scudispdio142W<Scu59cSpec> {
        Scudispdio142W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO142"]
    #[inline(always)]
    pub fn scudispuio142(&mut self) -> Scudispuio142W<Scu59cSpec> {
        Scudispuio142W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO142"]
    #[inline(always)]
    pub fn scudrvio142(&mut self) -> Scudrvio142W<Scu59cSpec> {
        Scudrvio142W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO142"]
    #[inline(always)]
    pub fn scuensmtio142(&mut self) -> Scuensmtio142W<Scu59cSpec> {
        Scuensmtio142W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO142"]
    #[inline(always)]
    pub fn scuenhvio142(&mut self) -> Scuenhvio142W<Scu59cSpec> {
        Scuenhvio142W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO143"]
    #[inline(always)]
    pub fn scudispdio143(&mut self) -> Scudispdio143W<Scu59cSpec> {
        Scudispdio143W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO143"]
    #[inline(always)]
    pub fn scudispuio143(&mut self) -> Scudispuio143W<Scu59cSpec> {
        Scudispuio143W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO143"]
    #[inline(always)]
    pub fn scudrvio143(&mut self) -> Scudrvio143W<Scu59cSpec> {
        Scudrvio143W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO143"]
    #[inline(always)]
    pub fn scuensmtio143(&mut self) -> Scuensmtio143W<Scu59cSpec> {
        Scuensmtio143W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO143"]
    #[inline(always)]
    pub fn scuenhvio143(&mut self) -> Scuenhvio143W<Scu59cSpec> {
        Scuenhvio143W::new(self, 25)
    }
}
#[doc = "IO Control \\#72\n\nYou can [`read`](crate::Reg::read) this register and get [`scu59c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu59c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu59cSpec;
impl crate::RegisterSpec for Scu59cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu59c::R`](R) reader structure"]
impl crate::Readable for Scu59cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu59c::W`](W) writer structure"]
impl crate::Writable for Scu59cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU59C to value 0x0204_0204"]
impl crate::Resettable for Scu59cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
