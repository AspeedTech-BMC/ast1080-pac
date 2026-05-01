#[doc = "Register `SCU49C` reader"]
pub type R = crate::R<Scu49cSpec>;
#[doc = "Register `SCU49C` writer"]
pub type W = crate::W<Scu49cSpec>;
#[doc = "Field `SCUDISPDIO014` reader - SCU_DIS_PD_IO014"]
pub type Scudispdio014R = crate::BitReader;
#[doc = "Field `SCUDISPDIO014` writer - SCU_DIS_PD_IO014"]
pub type Scudispdio014W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO014` reader - SCU_DIS_PU_IO014"]
pub type Scudispuio014R = crate::BitReader;
#[doc = "Field `SCUDISPUIO014` writer - SCU_DIS_PU_IO014"]
pub type Scudispuio014W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO014` reader - SCU_DRV_IO014"]
pub type Scudrvio014R = crate::FieldReader;
#[doc = "Field `SCUDRVIO014` writer - SCU_DRV_IO014"]
pub type Scudrvio014W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO014` reader - SCU_EN_SMT_IO014"]
pub type Scuensmtio014R = crate::BitReader;
#[doc = "Field `SCUENSMTIO014` writer - SCU_EN_SMT_IO014"]
pub type Scuensmtio014W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO014` reader - SCU_EN_HV_IO014"]
pub type Scuenhvio014R = crate::BitReader;
#[doc = "Field `SCUENHVIO014` writer - SCU_EN_HV_IO014"]
pub type Scuenhvio014W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO015` reader - SCU_DIS_PD_IO015"]
pub type Scudispdio015R = crate::BitReader;
#[doc = "Field `SCUDISPDIO015` writer - SCU_DIS_PD_IO015"]
pub type Scudispdio015W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO015` reader - SCU_DIS_PU_IO015"]
pub type Scudispuio015R = crate::BitReader;
#[doc = "Field `SCUDISPUIO015` writer - SCU_DIS_PU_IO015"]
pub type Scudispuio015W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO015` reader - SCU_DRV_IO015"]
pub type Scudrvio015R = crate::FieldReader;
#[doc = "Field `SCUDRVIO015` writer - SCU_DRV_IO015"]
pub type Scudrvio015W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO015` reader - SCU_EN_SMT_IO015"]
pub type Scuensmtio015R = crate::BitReader;
#[doc = "Field `SCUENSMTIO015` writer - SCU_EN_SMT_IO015"]
pub type Scuensmtio015W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO015` reader - SCU_EN_HV_IO015"]
pub type Scuenhvio015R = crate::BitReader;
#[doc = "Field `SCUENHVIO015` writer - SCU_EN_HV_IO015"]
pub type Scuenhvio015W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO014"]
    #[inline(always)]
    pub fn scudispdio014(&self) -> Scudispdio014R {
        Scudispdio014R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO014"]
    #[inline(always)]
    pub fn scudispuio014(&self) -> Scudispuio014R {
        Scudispuio014R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO014"]
    #[inline(always)]
    pub fn scudrvio014(&self) -> Scudrvio014R {
        Scudrvio014R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO014"]
    #[inline(always)]
    pub fn scuensmtio014(&self) -> Scuensmtio014R {
        Scuensmtio014R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO014"]
    #[inline(always)]
    pub fn scuenhvio014(&self) -> Scuenhvio014R {
        Scuenhvio014R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO015"]
    #[inline(always)]
    pub fn scudispdio015(&self) -> Scudispdio015R {
        Scudispdio015R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO015"]
    #[inline(always)]
    pub fn scudispuio015(&self) -> Scudispuio015R {
        Scudispuio015R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO015"]
    #[inline(always)]
    pub fn scudrvio015(&self) -> Scudrvio015R {
        Scudrvio015R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO015"]
    #[inline(always)]
    pub fn scuensmtio015(&self) -> Scuensmtio015R {
        Scuensmtio015R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO015"]
    #[inline(always)]
    pub fn scuenhvio015(&self) -> Scuenhvio015R {
        Scuenhvio015R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO014"]
    #[inline(always)]
    pub fn scudispdio014(&mut self) -> Scudispdio014W<Scu49cSpec> {
        Scudispdio014W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO014"]
    #[inline(always)]
    pub fn scudispuio014(&mut self) -> Scudispuio014W<Scu49cSpec> {
        Scudispuio014W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO014"]
    #[inline(always)]
    pub fn scudrvio014(&mut self) -> Scudrvio014W<Scu49cSpec> {
        Scudrvio014W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO014"]
    #[inline(always)]
    pub fn scuensmtio014(&mut self) -> Scuensmtio014W<Scu49cSpec> {
        Scuensmtio014W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO014"]
    #[inline(always)]
    pub fn scuenhvio014(&mut self) -> Scuenhvio014W<Scu49cSpec> {
        Scuenhvio014W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO015"]
    #[inline(always)]
    pub fn scudispdio015(&mut self) -> Scudispdio015W<Scu49cSpec> {
        Scudispdio015W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO015"]
    #[inline(always)]
    pub fn scudispuio015(&mut self) -> Scudispuio015W<Scu49cSpec> {
        Scudispuio015W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO015"]
    #[inline(always)]
    pub fn scudrvio015(&mut self) -> Scudrvio015W<Scu49cSpec> {
        Scudrvio015W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO015"]
    #[inline(always)]
    pub fn scuensmtio015(&mut self) -> Scuensmtio015W<Scu49cSpec> {
        Scuensmtio015W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO015"]
    #[inline(always)]
    pub fn scuenhvio015(&mut self) -> Scuenhvio015W<Scu49cSpec> {
        Scuenhvio015W::new(self, 25)
    }
}
#[doc = "IO Control \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu49c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu49c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu49cSpec;
impl crate::RegisterSpec for Scu49cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu49c::R`](R) reader structure"]
impl crate::Readable for Scu49cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu49c::W`](W) writer structure"]
impl crate::Writable for Scu49cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU49C to value 0x0204_0204"]
impl crate::Resettable for Scu49cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
