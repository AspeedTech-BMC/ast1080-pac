#[doc = "Register `SCU4EC` reader"]
pub type R = crate::R<Scu4ecSpec>;
#[doc = "Register `SCU4EC` writer"]
pub type W = crate::W<Scu4ecSpec>;
#[doc = "Field `SCUDISPDIO054` reader - SCU_DIS_PD_IO054"]
pub type Scudispdio054R = crate::BitReader;
#[doc = "Field `SCUDISPDIO054` writer - SCU_DIS_PD_IO054"]
pub type Scudispdio054W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO054` reader - SCU_DIS_PU_IO054"]
pub type Scudispuio054R = crate::BitReader;
#[doc = "Field `SCUDISPUIO054` writer - SCU_DIS_PU_IO054"]
pub type Scudispuio054W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO054` reader - SCU_DRV_IO054"]
pub type Scudrvio054R = crate::FieldReader;
#[doc = "Field `SCUDRVIO054` writer - SCU_DRV_IO054"]
pub type Scudrvio054W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO054` reader - SCU_EN_SMT_IO054"]
pub type Scuensmtio054R = crate::BitReader;
#[doc = "Field `SCUENSMTIO054` writer - SCU_EN_SMT_IO054"]
pub type Scuensmtio054W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO054` reader - SCU_EN_HV_IO054"]
pub type Scuenhvio054R = crate::BitReader;
#[doc = "Field `SCUENHVIO054` writer - SCU_EN_HV_IO054"]
pub type Scuenhvio054W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO055` reader - SCU_DIS_PD_IO055"]
pub type Scudispdio055R = crate::BitReader;
#[doc = "Field `SCUDISPDIO055` writer - SCU_DIS_PD_IO055"]
pub type Scudispdio055W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO055` reader - SCU_DIS_PU_IO055"]
pub type Scudispuio055R = crate::BitReader;
#[doc = "Field `SCUDISPUIO055` writer - SCU_DIS_PU_IO055"]
pub type Scudispuio055W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO055` reader - SCU_DRV_IO055"]
pub type Scudrvio055R = crate::FieldReader;
#[doc = "Field `SCUDRVIO055` writer - SCU_DRV_IO055"]
pub type Scudrvio055W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO055` reader - SCU_EN_SMT_IO055"]
pub type Scuensmtio055R = crate::BitReader;
#[doc = "Field `SCUENSMTIO055` writer - SCU_EN_SMT_IO055"]
pub type Scuensmtio055W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO055` reader - SCU_EN_HV_IO055"]
pub type Scuenhvio055R = crate::BitReader;
#[doc = "Field `SCUENHVIO055` writer - SCU_EN_HV_IO055"]
pub type Scuenhvio055W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO054"]
    #[inline(always)]
    pub fn scudispdio054(&self) -> Scudispdio054R {
        Scudispdio054R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO054"]
    #[inline(always)]
    pub fn scudispuio054(&self) -> Scudispuio054R {
        Scudispuio054R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO054"]
    #[inline(always)]
    pub fn scudrvio054(&self) -> Scudrvio054R {
        Scudrvio054R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO054"]
    #[inline(always)]
    pub fn scuensmtio054(&self) -> Scuensmtio054R {
        Scuensmtio054R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO054"]
    #[inline(always)]
    pub fn scuenhvio054(&self) -> Scuenhvio054R {
        Scuenhvio054R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO055"]
    #[inline(always)]
    pub fn scudispdio055(&self) -> Scudispdio055R {
        Scudispdio055R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO055"]
    #[inline(always)]
    pub fn scudispuio055(&self) -> Scudispuio055R {
        Scudispuio055R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO055"]
    #[inline(always)]
    pub fn scudrvio055(&self) -> Scudrvio055R {
        Scudrvio055R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO055"]
    #[inline(always)]
    pub fn scuensmtio055(&self) -> Scuensmtio055R {
        Scuensmtio055R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO055"]
    #[inline(always)]
    pub fn scuenhvio055(&self) -> Scuenhvio055R {
        Scuenhvio055R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO054"]
    #[inline(always)]
    pub fn scudispdio054(&mut self) -> Scudispdio054W<Scu4ecSpec> {
        Scudispdio054W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO054"]
    #[inline(always)]
    pub fn scudispuio054(&mut self) -> Scudispuio054W<Scu4ecSpec> {
        Scudispuio054W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO054"]
    #[inline(always)]
    pub fn scudrvio054(&mut self) -> Scudrvio054W<Scu4ecSpec> {
        Scudrvio054W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO054"]
    #[inline(always)]
    pub fn scuensmtio054(&mut self) -> Scuensmtio054W<Scu4ecSpec> {
        Scuensmtio054W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO054"]
    #[inline(always)]
    pub fn scuenhvio054(&mut self) -> Scuenhvio054W<Scu4ecSpec> {
        Scuenhvio054W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO055"]
    #[inline(always)]
    pub fn scudispdio055(&mut self) -> Scudispdio055W<Scu4ecSpec> {
        Scudispdio055W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO055"]
    #[inline(always)]
    pub fn scudispuio055(&mut self) -> Scudispuio055W<Scu4ecSpec> {
        Scudispuio055W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO055"]
    #[inline(always)]
    pub fn scudrvio055(&mut self) -> Scudrvio055W<Scu4ecSpec> {
        Scudrvio055W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO055"]
    #[inline(always)]
    pub fn scuensmtio055(&mut self) -> Scuensmtio055W<Scu4ecSpec> {
        Scuensmtio055W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO055"]
    #[inline(always)]
    pub fn scuenhvio055(&mut self) -> Scuenhvio055W<Scu4ecSpec> {
        Scuenhvio055W::new(self, 25)
    }
}
#[doc = "IO Control \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4ecSpec;
impl crate::RegisterSpec for Scu4ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4ec::R`](R) reader structure"]
impl crate::Readable for Scu4ecSpec {}
#[doc = "`write(|w| ..)` method takes [`scu4ec::W`](W) writer structure"]
impl crate::Writable for Scu4ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4EC to value 0x0204_0204"]
impl crate::Resettable for Scu4ecSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
