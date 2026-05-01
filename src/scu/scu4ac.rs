#[doc = "Register `SCU4AC` reader"]
pub type R = crate::R<Scu4acSpec>;
#[doc = "Register `SCU4AC` writer"]
pub type W = crate::W<Scu4acSpec>;
#[doc = "Field `SCUDISPDIO022` reader - SCU_DIS_PD_IO022"]
pub type Scudispdio022R = crate::BitReader;
#[doc = "Field `SCUDISPDIO022` writer - SCU_DIS_PD_IO022"]
pub type Scudispdio022W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO022` reader - SCU_DIS_PU_IO022"]
pub type Scudispuio022R = crate::BitReader;
#[doc = "Field `SCUDISPUIO022` writer - SCU_DIS_PU_IO022"]
pub type Scudispuio022W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO022` reader - SCU_DRV_IO022"]
pub type Scudrvio022R = crate::FieldReader;
#[doc = "Field `SCUDRVIO022` writer - SCU_DRV_IO022"]
pub type Scudrvio022W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO022` reader - SCU_EN_SMT_IO022"]
pub type Scuensmtio022R = crate::BitReader;
#[doc = "Field `SCUENSMTIO022` writer - SCU_EN_SMT_IO022"]
pub type Scuensmtio022W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO022` reader - SCU_EN_HV_IO022"]
pub type Scuenhvio022R = crate::BitReader;
#[doc = "Field `SCUENHVIO022` writer - SCU_EN_HV_IO022"]
pub type Scuenhvio022W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO023` reader - SCU_DIS_PD_IO023"]
pub type Scudispdio023R = crate::BitReader;
#[doc = "Field `SCUDISPDIO023` writer - SCU_DIS_PD_IO023"]
pub type Scudispdio023W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO023` reader - SCU_DIS_PU_IO023"]
pub type Scudispuio023R = crate::BitReader;
#[doc = "Field `SCUDISPUIO023` writer - SCU_DIS_PU_IO023"]
pub type Scudispuio023W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO023` reader - SCU_DRV_IO023"]
pub type Scudrvio023R = crate::FieldReader;
#[doc = "Field `SCUDRVIO023` writer - SCU_DRV_IO023"]
pub type Scudrvio023W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO023` reader - SCU_EN_SMT_IO023"]
pub type Scuensmtio023R = crate::BitReader;
#[doc = "Field `SCUENSMTIO023` writer - SCU_EN_SMT_IO023"]
pub type Scuensmtio023W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO023` reader - SCU_EN_HV_IO023"]
pub type Scuenhvio023R = crate::BitReader;
#[doc = "Field `SCUENHVIO023` writer - SCU_EN_HV_IO023"]
pub type Scuenhvio023W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO022"]
    #[inline(always)]
    pub fn scudispdio022(&self) -> Scudispdio022R {
        Scudispdio022R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO022"]
    #[inline(always)]
    pub fn scudispuio022(&self) -> Scudispuio022R {
        Scudispuio022R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO022"]
    #[inline(always)]
    pub fn scudrvio022(&self) -> Scudrvio022R {
        Scudrvio022R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO022"]
    #[inline(always)]
    pub fn scuensmtio022(&self) -> Scuensmtio022R {
        Scuensmtio022R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO022"]
    #[inline(always)]
    pub fn scuenhvio022(&self) -> Scuenhvio022R {
        Scuenhvio022R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO023"]
    #[inline(always)]
    pub fn scudispdio023(&self) -> Scudispdio023R {
        Scudispdio023R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO023"]
    #[inline(always)]
    pub fn scudispuio023(&self) -> Scudispuio023R {
        Scudispuio023R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO023"]
    #[inline(always)]
    pub fn scudrvio023(&self) -> Scudrvio023R {
        Scudrvio023R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO023"]
    #[inline(always)]
    pub fn scuensmtio023(&self) -> Scuensmtio023R {
        Scuensmtio023R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO023"]
    #[inline(always)]
    pub fn scuenhvio023(&self) -> Scuenhvio023R {
        Scuenhvio023R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO022"]
    #[inline(always)]
    pub fn scudispdio022(&mut self) -> Scudispdio022W<Scu4acSpec> {
        Scudispdio022W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO022"]
    #[inline(always)]
    pub fn scudispuio022(&mut self) -> Scudispuio022W<Scu4acSpec> {
        Scudispuio022W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO022"]
    #[inline(always)]
    pub fn scudrvio022(&mut self) -> Scudrvio022W<Scu4acSpec> {
        Scudrvio022W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO022"]
    #[inline(always)]
    pub fn scuensmtio022(&mut self) -> Scuensmtio022W<Scu4acSpec> {
        Scuensmtio022W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO022"]
    #[inline(always)]
    pub fn scuenhvio022(&mut self) -> Scuenhvio022W<Scu4acSpec> {
        Scuenhvio022W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO023"]
    #[inline(always)]
    pub fn scudispdio023(&mut self) -> Scudispdio023W<Scu4acSpec> {
        Scudispdio023W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO023"]
    #[inline(always)]
    pub fn scudispuio023(&mut self) -> Scudispuio023W<Scu4acSpec> {
        Scudispuio023W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO023"]
    #[inline(always)]
    pub fn scudrvio023(&mut self) -> Scudrvio023W<Scu4acSpec> {
        Scudrvio023W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO023"]
    #[inline(always)]
    pub fn scuensmtio023(&mut self) -> Scuensmtio023W<Scu4acSpec> {
        Scuensmtio023W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO023"]
    #[inline(always)]
    pub fn scuenhvio023(&mut self) -> Scuenhvio023W<Scu4acSpec> {
        Scuenhvio023W::new(self, 25)
    }
}
#[doc = "IO Control \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4acSpec;
impl crate::RegisterSpec for Scu4acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4ac::R`](R) reader structure"]
impl crate::Readable for Scu4acSpec {}
#[doc = "`write(|w| ..)` method takes [`scu4ac::W`](W) writer structure"]
impl crate::Writable for Scu4acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4AC to value 0x0204_0204"]
impl crate::Resettable for Scu4acSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
