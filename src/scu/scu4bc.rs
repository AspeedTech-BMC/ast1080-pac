#[doc = "Register `SCU4BC` reader"]
pub type R = crate::R<Scu4bcSpec>;
#[doc = "Register `SCU4BC` writer"]
pub type W = crate::W<Scu4bcSpec>;
#[doc = "Field `SCUDISPDIO030` reader - SCU_DIS_PD_IO030"]
pub type Scudispdio030R = crate::BitReader;
#[doc = "Field `SCUDISPDIO030` writer - SCU_DIS_PD_IO030"]
pub type Scudispdio030W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO030` reader - SCU_DIS_PU_IO030"]
pub type Scudispuio030R = crate::BitReader;
#[doc = "Field `SCUDISPUIO030` writer - SCU_DIS_PU_IO030"]
pub type Scudispuio030W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO030` reader - SCU_DRV_IO030"]
pub type Scudrvio030R = crate::FieldReader;
#[doc = "Field `SCUDRVIO030` writer - SCU_DRV_IO030"]
pub type Scudrvio030W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO030` reader - SCU_EN_SMT_IO030"]
pub type Scuensmtio030R = crate::BitReader;
#[doc = "Field `SCUENSMTIO030` writer - SCU_EN_SMT_IO030"]
pub type Scuensmtio030W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO030` reader - SCU_EN_HV_IO030"]
pub type Scuenhvio030R = crate::BitReader;
#[doc = "Field `SCUENHVIO030` writer - SCU_EN_HV_IO030"]
pub type Scuenhvio030W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO031` reader - SCU_DIS_PD_IO031"]
pub type Scudispdio031R = crate::BitReader;
#[doc = "Field `SCUDISPDIO031` writer - SCU_DIS_PD_IO031"]
pub type Scudispdio031W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO031` reader - SCU_DIS_PU_IO031"]
pub type Scudispuio031R = crate::BitReader;
#[doc = "Field `SCUDISPUIO031` writer - SCU_DIS_PU_IO031"]
pub type Scudispuio031W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO031` reader - SCU_DRV_IO031"]
pub type Scudrvio031R = crate::FieldReader;
#[doc = "Field `SCUDRVIO031` writer - SCU_DRV_IO031"]
pub type Scudrvio031W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO031` reader - SCU_EN_SMT_IO031"]
pub type Scuensmtio031R = crate::BitReader;
#[doc = "Field `SCUENSMTIO031` writer - SCU_EN_SMT_IO031"]
pub type Scuensmtio031W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO031` reader - SCU_EN_HV_IO031"]
pub type Scuenhvio031R = crate::BitReader;
#[doc = "Field `SCUENHVIO031` writer - SCU_EN_HV_IO031"]
pub type Scuenhvio031W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO030"]
    #[inline(always)]
    pub fn scudispdio030(&self) -> Scudispdio030R {
        Scudispdio030R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO030"]
    #[inline(always)]
    pub fn scudispuio030(&self) -> Scudispuio030R {
        Scudispuio030R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO030"]
    #[inline(always)]
    pub fn scudrvio030(&self) -> Scudrvio030R {
        Scudrvio030R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO030"]
    #[inline(always)]
    pub fn scuensmtio030(&self) -> Scuensmtio030R {
        Scuensmtio030R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO030"]
    #[inline(always)]
    pub fn scuenhvio030(&self) -> Scuenhvio030R {
        Scuenhvio030R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO031"]
    #[inline(always)]
    pub fn scudispdio031(&self) -> Scudispdio031R {
        Scudispdio031R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO031"]
    #[inline(always)]
    pub fn scudispuio031(&self) -> Scudispuio031R {
        Scudispuio031R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO031"]
    #[inline(always)]
    pub fn scudrvio031(&self) -> Scudrvio031R {
        Scudrvio031R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO031"]
    #[inline(always)]
    pub fn scuensmtio031(&self) -> Scuensmtio031R {
        Scuensmtio031R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO031"]
    #[inline(always)]
    pub fn scuenhvio031(&self) -> Scuenhvio031R {
        Scuenhvio031R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO030"]
    #[inline(always)]
    pub fn scudispdio030(&mut self) -> Scudispdio030W<Scu4bcSpec> {
        Scudispdio030W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO030"]
    #[inline(always)]
    pub fn scudispuio030(&mut self) -> Scudispuio030W<Scu4bcSpec> {
        Scudispuio030W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO030"]
    #[inline(always)]
    pub fn scudrvio030(&mut self) -> Scudrvio030W<Scu4bcSpec> {
        Scudrvio030W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO030"]
    #[inline(always)]
    pub fn scuensmtio030(&mut self) -> Scuensmtio030W<Scu4bcSpec> {
        Scuensmtio030W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO030"]
    #[inline(always)]
    pub fn scuenhvio030(&mut self) -> Scuenhvio030W<Scu4bcSpec> {
        Scuenhvio030W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO031"]
    #[inline(always)]
    pub fn scudispdio031(&mut self) -> Scudispdio031W<Scu4bcSpec> {
        Scudispdio031W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO031"]
    #[inline(always)]
    pub fn scudispuio031(&mut self) -> Scudispuio031W<Scu4bcSpec> {
        Scudispuio031W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO031"]
    #[inline(always)]
    pub fn scudrvio031(&mut self) -> Scudrvio031W<Scu4bcSpec> {
        Scudrvio031W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO031"]
    #[inline(always)]
    pub fn scuensmtio031(&mut self) -> Scuensmtio031W<Scu4bcSpec> {
        Scuensmtio031W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO031"]
    #[inline(always)]
    pub fn scuenhvio031(&mut self) -> Scuenhvio031W<Scu4bcSpec> {
        Scuenhvio031W::new(self, 25)
    }
}
#[doc = "IO Control \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4bcSpec;
impl crate::RegisterSpec for Scu4bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4bc::R`](R) reader structure"]
impl crate::Readable for Scu4bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu4bc::W`](W) writer structure"]
impl crate::Writable for Scu4bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4BC to value 0x0204_0204"]
impl crate::Resettable for Scu4bcSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
