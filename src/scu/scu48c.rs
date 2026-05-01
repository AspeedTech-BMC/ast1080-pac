#[doc = "Register `SCU48C` reader"]
pub type R = crate::R<Scu48cSpec>;
#[doc = "Register `SCU48C` writer"]
pub type W = crate::W<Scu48cSpec>;
#[doc = "Field `SCUDISPDIO006` reader - SCU_DIS_PD_IO006"]
pub type Scudispdio006R = crate::BitReader;
#[doc = "Field `SCUDISPDIO006` writer - SCU_DIS_PD_IO006"]
pub type Scudispdio006W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO006` reader - SCU_DIS_PU_IO006"]
pub type Scudispuio006R = crate::BitReader;
#[doc = "Field `SCUDISPUIO006` writer - SCU_DIS_PU_IO006"]
pub type Scudispuio006W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO006` reader - SCU_DRV_IO006"]
pub type Scudrvio006R = crate::FieldReader;
#[doc = "Field `SCUDRVIO006` writer - SCU_DRV_IO006"]
pub type Scudrvio006W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO006` reader - SCU_EN_SMT_IO006"]
pub type Scuensmtio006R = crate::BitReader;
#[doc = "Field `SCUENSMTIO006` writer - SCU_EN_SMT_IO006"]
pub type Scuensmtio006W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO006` reader - SCU_EN_HV_IO006"]
pub type Scuenhvio006R = crate::BitReader;
#[doc = "Field `SCUENHVIO006` writer - SCU_EN_HV_IO006"]
pub type Scuenhvio006W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO007` reader - SCU_DIS_PD_IO007"]
pub type Scudispdio007R = crate::BitReader;
#[doc = "Field `SCUDISPDIO007` writer - SCU_DIS_PD_IO007"]
pub type Scudispdio007W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO007` reader - SCU_DIS_PU_IO007"]
pub type Scudispuio007R = crate::BitReader;
#[doc = "Field `SCUDISPUIO007` writer - SCU_DIS_PU_IO007"]
pub type Scudispuio007W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO007` reader - SCU_DRV_IO007"]
pub type Scudrvio007R = crate::FieldReader;
#[doc = "Field `SCUDRVIO007` writer - SCU_DRV_IO007"]
pub type Scudrvio007W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO007` reader - SCU_EN_SMT_IO007"]
pub type Scuensmtio007R = crate::BitReader;
#[doc = "Field `SCUENSMTIO007` writer - SCU_EN_SMT_IO007"]
pub type Scuensmtio007W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO007` reader - SCU_EN_HV_IO007"]
pub type Scuenhvio007R = crate::BitReader;
#[doc = "Field `SCUENHVIO007` writer - SCU_EN_HV_IO007"]
pub type Scuenhvio007W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO006"]
    #[inline(always)]
    pub fn scudispdio006(&self) -> Scudispdio006R {
        Scudispdio006R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO006"]
    #[inline(always)]
    pub fn scudispuio006(&self) -> Scudispuio006R {
        Scudispuio006R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO006"]
    #[inline(always)]
    pub fn scudrvio006(&self) -> Scudrvio006R {
        Scudrvio006R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO006"]
    #[inline(always)]
    pub fn scuensmtio006(&self) -> Scuensmtio006R {
        Scuensmtio006R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO006"]
    #[inline(always)]
    pub fn scuenhvio006(&self) -> Scuenhvio006R {
        Scuenhvio006R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO007"]
    #[inline(always)]
    pub fn scudispdio007(&self) -> Scudispdio007R {
        Scudispdio007R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO007"]
    #[inline(always)]
    pub fn scudispuio007(&self) -> Scudispuio007R {
        Scudispuio007R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO007"]
    #[inline(always)]
    pub fn scudrvio007(&self) -> Scudrvio007R {
        Scudrvio007R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO007"]
    #[inline(always)]
    pub fn scuensmtio007(&self) -> Scuensmtio007R {
        Scuensmtio007R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO007"]
    #[inline(always)]
    pub fn scuenhvio007(&self) -> Scuenhvio007R {
        Scuenhvio007R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO006"]
    #[inline(always)]
    pub fn scudispdio006(&mut self) -> Scudispdio006W<Scu48cSpec> {
        Scudispdio006W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO006"]
    #[inline(always)]
    pub fn scudispuio006(&mut self) -> Scudispuio006W<Scu48cSpec> {
        Scudispuio006W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO006"]
    #[inline(always)]
    pub fn scudrvio006(&mut self) -> Scudrvio006W<Scu48cSpec> {
        Scudrvio006W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO006"]
    #[inline(always)]
    pub fn scuensmtio006(&mut self) -> Scuensmtio006W<Scu48cSpec> {
        Scuensmtio006W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO006"]
    #[inline(always)]
    pub fn scuenhvio006(&mut self) -> Scuenhvio006W<Scu48cSpec> {
        Scuenhvio006W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO007"]
    #[inline(always)]
    pub fn scudispdio007(&mut self) -> Scudispdio007W<Scu48cSpec> {
        Scudispdio007W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO007"]
    #[inline(always)]
    pub fn scudispuio007(&mut self) -> Scudispuio007W<Scu48cSpec> {
        Scudispuio007W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO007"]
    #[inline(always)]
    pub fn scudrvio007(&mut self) -> Scudrvio007W<Scu48cSpec> {
        Scudrvio007W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO007"]
    #[inline(always)]
    pub fn scuensmtio007(&mut self) -> Scuensmtio007W<Scu48cSpec> {
        Scuensmtio007W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO007"]
    #[inline(always)]
    pub fn scuenhvio007(&mut self) -> Scuenhvio007W<Scu48cSpec> {
        Scuenhvio007W::new(self, 25)
    }
}
#[doc = "IO Control \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu48c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu48c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu48cSpec;
impl crate::RegisterSpec for Scu48cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu48c::R`](R) reader structure"]
impl crate::Readable for Scu48cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu48c::W`](W) writer structure"]
impl crate::Writable for Scu48cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU48C to value 0x0204_0204"]
impl crate::Resettable for Scu48cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
