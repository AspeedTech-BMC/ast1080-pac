#[doc = "Register `SCU4DC` reader"]
pub type R = crate::R<Scu4dcSpec>;
#[doc = "Register `SCU4DC` writer"]
pub type W = crate::W<Scu4dcSpec>;
#[doc = "Field `SCUDISPDIO046` reader - SCU_DIS_PD_IO046"]
pub type Scudispdio046R = crate::BitReader;
#[doc = "Field `SCUDISPDIO046` writer - SCU_DIS_PD_IO046"]
pub type Scudispdio046W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO046` reader - SCU_DIS_PU_IO046"]
pub type Scudispuio046R = crate::BitReader;
#[doc = "Field `SCUDISPUIO046` writer - SCU_DIS_PU_IO046"]
pub type Scudispuio046W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO046` reader - SCU_DRV_IO046"]
pub type Scudrvio046R = crate::FieldReader;
#[doc = "Field `SCUDRVIO046` writer - SCU_DRV_IO046"]
pub type Scudrvio046W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO046` reader - SCU_EN_SMT_IO046"]
pub type Scuensmtio046R = crate::BitReader;
#[doc = "Field `SCUENSMTIO046` writer - SCU_EN_SMT_IO046"]
pub type Scuensmtio046W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO046` reader - SCU_EN_HV_IO046"]
pub type Scuenhvio046R = crate::BitReader;
#[doc = "Field `SCUENHVIO046` writer - SCU_EN_HV_IO046"]
pub type Scuenhvio046W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO047` reader - SCU_DIS_PD_IO047"]
pub type Scudispdio047R = crate::BitReader;
#[doc = "Field `SCUDISPDIO047` writer - SCU_DIS_PD_IO047"]
pub type Scudispdio047W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO047` reader - SCU_DIS_PU_IO047"]
pub type Scudispuio047R = crate::BitReader;
#[doc = "Field `SCUDISPUIO047` writer - SCU_DIS_PU_IO047"]
pub type Scudispuio047W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO047` reader - SCU_DRV_IO047"]
pub type Scudrvio047R = crate::FieldReader;
#[doc = "Field `SCUDRVIO047` writer - SCU_DRV_IO047"]
pub type Scudrvio047W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO047` reader - SCU_EN_SMT_IO047"]
pub type Scuensmtio047R = crate::BitReader;
#[doc = "Field `SCUENSMTIO047` writer - SCU_EN_SMT_IO047"]
pub type Scuensmtio047W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO047` reader - SCU_EN_HV_IO047"]
pub type Scuenhvio047R = crate::BitReader;
#[doc = "Field `SCUENHVIO047` writer - SCU_EN_HV_IO047"]
pub type Scuenhvio047W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO046"]
    #[inline(always)]
    pub fn scudispdio046(&self) -> Scudispdio046R {
        Scudispdio046R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO046"]
    #[inline(always)]
    pub fn scudispuio046(&self) -> Scudispuio046R {
        Scudispuio046R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO046"]
    #[inline(always)]
    pub fn scudrvio046(&self) -> Scudrvio046R {
        Scudrvio046R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO046"]
    #[inline(always)]
    pub fn scuensmtio046(&self) -> Scuensmtio046R {
        Scuensmtio046R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO046"]
    #[inline(always)]
    pub fn scuenhvio046(&self) -> Scuenhvio046R {
        Scuenhvio046R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO047"]
    #[inline(always)]
    pub fn scudispdio047(&self) -> Scudispdio047R {
        Scudispdio047R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO047"]
    #[inline(always)]
    pub fn scudispuio047(&self) -> Scudispuio047R {
        Scudispuio047R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO047"]
    #[inline(always)]
    pub fn scudrvio047(&self) -> Scudrvio047R {
        Scudrvio047R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO047"]
    #[inline(always)]
    pub fn scuensmtio047(&self) -> Scuensmtio047R {
        Scuensmtio047R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO047"]
    #[inline(always)]
    pub fn scuenhvio047(&self) -> Scuenhvio047R {
        Scuenhvio047R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO046"]
    #[inline(always)]
    pub fn scudispdio046(&mut self) -> Scudispdio046W<Scu4dcSpec> {
        Scudispdio046W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO046"]
    #[inline(always)]
    pub fn scudispuio046(&mut self) -> Scudispuio046W<Scu4dcSpec> {
        Scudispuio046W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO046"]
    #[inline(always)]
    pub fn scudrvio046(&mut self) -> Scudrvio046W<Scu4dcSpec> {
        Scudrvio046W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO046"]
    #[inline(always)]
    pub fn scuensmtio046(&mut self) -> Scuensmtio046W<Scu4dcSpec> {
        Scuensmtio046W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO046"]
    #[inline(always)]
    pub fn scuenhvio046(&mut self) -> Scuenhvio046W<Scu4dcSpec> {
        Scuenhvio046W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO047"]
    #[inline(always)]
    pub fn scudispdio047(&mut self) -> Scudispdio047W<Scu4dcSpec> {
        Scudispdio047W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO047"]
    #[inline(always)]
    pub fn scudispuio047(&mut self) -> Scudispuio047W<Scu4dcSpec> {
        Scudispuio047W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO047"]
    #[inline(always)]
    pub fn scudrvio047(&mut self) -> Scudrvio047W<Scu4dcSpec> {
        Scudrvio047W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO047"]
    #[inline(always)]
    pub fn scuensmtio047(&mut self) -> Scuensmtio047W<Scu4dcSpec> {
        Scuensmtio047W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO047"]
    #[inline(always)]
    pub fn scuenhvio047(&mut self) -> Scuenhvio047W<Scu4dcSpec> {
        Scuenhvio047W::new(self, 25)
    }
}
#[doc = "IO Control \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4dcSpec;
impl crate::RegisterSpec for Scu4dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4dc::R`](R) reader structure"]
impl crate::Readable for Scu4dcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu4dc::W`](W) writer structure"]
impl crate::Writable for Scu4dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4DC to value 0x0204_0204"]
impl crate::Resettable for Scu4dcSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
