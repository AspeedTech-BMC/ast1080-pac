#[doc = "Register `SCU57C` reader"]
pub type R = crate::R<Scu57cSpec>;
#[doc = "Register `SCU57C` writer"]
pub type W = crate::W<Scu57cSpec>;
#[doc = "Field `SCUDISPDIO126` reader - SCU_DIS_PD_IO126"]
pub type Scudispdio126R = crate::BitReader;
#[doc = "Field `SCUDISPDIO126` writer - SCU_DIS_PD_IO126"]
pub type Scudispdio126W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO126` reader - SCU_DIS_PU_IO126"]
pub type Scudispuio126R = crate::BitReader;
#[doc = "Field `SCUDISPUIO126` writer - SCU_DIS_PU_IO126"]
pub type Scudispuio126W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO126` reader - SCU_DRV_IO126"]
pub type Scudrvio126R = crate::FieldReader;
#[doc = "Field `SCUDRVIO126` writer - SCU_DRV_IO126"]
pub type Scudrvio126W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO126` reader - SCU_EN_SMT_IO126"]
pub type Scuensmtio126R = crate::BitReader;
#[doc = "Field `SCUENSMTIO126` writer - SCU_EN_SMT_IO126"]
pub type Scuensmtio126W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO126` reader - SCU_EN_HV_IO126"]
pub type Scuenhvio126R = crate::BitReader;
#[doc = "Field `SCUENHVIO126` writer - SCU_EN_HV_IO126"]
pub type Scuenhvio126W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO127` reader - SCU_DIS_PD_IO127"]
pub type Scudispdio127R = crate::BitReader;
#[doc = "Field `SCUDISPDIO127` writer - SCU_DIS_PD_IO127"]
pub type Scudispdio127W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO127` reader - SCU_DIS_PU_IO127"]
pub type Scudispuio127R = crate::BitReader;
#[doc = "Field `SCUDISPUIO127` writer - SCU_DIS_PU_IO127"]
pub type Scudispuio127W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO127` reader - SCU_DRV_IO127"]
pub type Scudrvio127R = crate::FieldReader;
#[doc = "Field `SCUDRVIO127` writer - SCU_DRV_IO127"]
pub type Scudrvio127W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO127` reader - SCU_EN_SMT_IO127"]
pub type Scuensmtio127R = crate::BitReader;
#[doc = "Field `SCUENSMTIO127` writer - SCU_EN_SMT_IO127"]
pub type Scuensmtio127W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO127` reader - SCU_EN_HV_IO127"]
pub type Scuenhvio127R = crate::BitReader;
#[doc = "Field `SCUENHVIO127` writer - SCU_EN_HV_IO127"]
pub type Scuenhvio127W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO126"]
    #[inline(always)]
    pub fn scudispdio126(&self) -> Scudispdio126R {
        Scudispdio126R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO126"]
    #[inline(always)]
    pub fn scudispuio126(&self) -> Scudispuio126R {
        Scudispuio126R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO126"]
    #[inline(always)]
    pub fn scudrvio126(&self) -> Scudrvio126R {
        Scudrvio126R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO126"]
    #[inline(always)]
    pub fn scuensmtio126(&self) -> Scuensmtio126R {
        Scuensmtio126R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO126"]
    #[inline(always)]
    pub fn scuenhvio126(&self) -> Scuenhvio126R {
        Scuenhvio126R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO127"]
    #[inline(always)]
    pub fn scudispdio127(&self) -> Scudispdio127R {
        Scudispdio127R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO127"]
    #[inline(always)]
    pub fn scudispuio127(&self) -> Scudispuio127R {
        Scudispuio127R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO127"]
    #[inline(always)]
    pub fn scudrvio127(&self) -> Scudrvio127R {
        Scudrvio127R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO127"]
    #[inline(always)]
    pub fn scuensmtio127(&self) -> Scuensmtio127R {
        Scuensmtio127R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO127"]
    #[inline(always)]
    pub fn scuenhvio127(&self) -> Scuenhvio127R {
        Scuenhvio127R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO126"]
    #[inline(always)]
    pub fn scudispdio126(&mut self) -> Scudispdio126W<Scu57cSpec> {
        Scudispdio126W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO126"]
    #[inline(always)]
    pub fn scudispuio126(&mut self) -> Scudispuio126W<Scu57cSpec> {
        Scudispuio126W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO126"]
    #[inline(always)]
    pub fn scudrvio126(&mut self) -> Scudrvio126W<Scu57cSpec> {
        Scudrvio126W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO126"]
    #[inline(always)]
    pub fn scuensmtio126(&mut self) -> Scuensmtio126W<Scu57cSpec> {
        Scuensmtio126W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO126"]
    #[inline(always)]
    pub fn scuenhvio126(&mut self) -> Scuenhvio126W<Scu57cSpec> {
        Scuenhvio126W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO127"]
    #[inline(always)]
    pub fn scudispdio127(&mut self) -> Scudispdio127W<Scu57cSpec> {
        Scudispdio127W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO127"]
    #[inline(always)]
    pub fn scudispuio127(&mut self) -> Scudispuio127W<Scu57cSpec> {
        Scudispuio127W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO127"]
    #[inline(always)]
    pub fn scudrvio127(&mut self) -> Scudrvio127W<Scu57cSpec> {
        Scudrvio127W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO127"]
    #[inline(always)]
    pub fn scuensmtio127(&mut self) -> Scuensmtio127W<Scu57cSpec> {
        Scuensmtio127W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO127"]
    #[inline(always)]
    pub fn scuenhvio127(&mut self) -> Scuenhvio127W<Scu57cSpec> {
        Scuenhvio127W::new(self, 25)
    }
}
#[doc = "IO Control \\#64\n\nYou can [`read`](crate::Reg::read) this register and get [`scu57c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu57c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu57cSpec;
impl crate::RegisterSpec for Scu57cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu57c::R`](R) reader structure"]
impl crate::Readable for Scu57cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu57c::W`](W) writer structure"]
impl crate::Writable for Scu57cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU57C to value 0x0204_0204"]
impl crate::Resettable for Scu57cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
