#[doc = "Register `SCU5BC` reader"]
pub type R = crate::R<Scu5bcSpec>;
#[doc = "Register `SCU5BC` writer"]
pub type W = crate::W<Scu5bcSpec>;
#[doc = "Field `SCUDISPDIO158` reader - SCU_DIS_PD_IO158"]
pub type Scudispdio158R = crate::BitReader;
#[doc = "Field `SCUDISPDIO158` writer - SCU_DIS_PD_IO158"]
pub type Scudispdio158W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO158` reader - SCU_DIS_PU_IO158"]
pub type Scudispuio158R = crate::BitReader;
#[doc = "Field `SCUDISPUIO158` writer - SCU_DIS_PU_IO158"]
pub type Scudispuio158W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO158` reader - SCU_DRV_IO158"]
pub type Scudrvio158R = crate::FieldReader;
#[doc = "Field `SCUDRVIO158` writer - SCU_DRV_IO158"]
pub type Scudrvio158W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO158` reader - SCU_EN_SMT_IO158"]
pub type Scuensmtio158R = crate::BitReader;
#[doc = "Field `SCUENSMTIO158` writer - SCU_EN_SMT_IO158"]
pub type Scuensmtio158W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO158` reader - SCU_EN_HV_IO158"]
pub type Scuenhvio158R = crate::BitReader;
#[doc = "Field `SCUENHVIO158` writer - SCU_EN_HV_IO158"]
pub type Scuenhvio158W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO159` reader - SCU_DIS_PD_IO159"]
pub type Scudispdio159R = crate::BitReader;
#[doc = "Field `SCUDISPDIO159` writer - SCU_DIS_PD_IO159"]
pub type Scudispdio159W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO159` reader - SCU_DIS_PU_IO159"]
pub type Scudispuio159R = crate::BitReader;
#[doc = "Field `SCUDISPUIO159` writer - SCU_DIS_PU_IO159"]
pub type Scudispuio159W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO159` reader - SCU_DRV_IO159"]
pub type Scudrvio159R = crate::FieldReader;
#[doc = "Field `SCUDRVIO159` writer - SCU_DRV_IO159"]
pub type Scudrvio159W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO159` reader - SCU_EN_SMT_IO159"]
pub type Scuensmtio159R = crate::BitReader;
#[doc = "Field `SCUENSMTIO159` writer - SCU_EN_SMT_IO159"]
pub type Scuensmtio159W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO159` reader - SCU_EN_HV_IO159"]
pub type Scuenhvio159R = crate::BitReader;
#[doc = "Field `SCUENHVIO159` writer - SCU_EN_HV_IO159"]
pub type Scuenhvio159W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO158"]
    #[inline(always)]
    pub fn scudispdio158(&self) -> Scudispdio158R {
        Scudispdio158R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO158"]
    #[inline(always)]
    pub fn scudispuio158(&self) -> Scudispuio158R {
        Scudispuio158R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO158"]
    #[inline(always)]
    pub fn scudrvio158(&self) -> Scudrvio158R {
        Scudrvio158R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO158"]
    #[inline(always)]
    pub fn scuensmtio158(&self) -> Scuensmtio158R {
        Scuensmtio158R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO158"]
    #[inline(always)]
    pub fn scuenhvio158(&self) -> Scuenhvio158R {
        Scuenhvio158R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO159"]
    #[inline(always)]
    pub fn scudispdio159(&self) -> Scudispdio159R {
        Scudispdio159R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO159"]
    #[inline(always)]
    pub fn scudispuio159(&self) -> Scudispuio159R {
        Scudispuio159R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO159"]
    #[inline(always)]
    pub fn scudrvio159(&self) -> Scudrvio159R {
        Scudrvio159R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO159"]
    #[inline(always)]
    pub fn scuensmtio159(&self) -> Scuensmtio159R {
        Scuensmtio159R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO159"]
    #[inline(always)]
    pub fn scuenhvio159(&self) -> Scuenhvio159R {
        Scuenhvio159R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO158"]
    #[inline(always)]
    pub fn scudispdio158(&mut self) -> Scudispdio158W<Scu5bcSpec> {
        Scudispdio158W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO158"]
    #[inline(always)]
    pub fn scudispuio158(&mut self) -> Scudispuio158W<Scu5bcSpec> {
        Scudispuio158W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO158"]
    #[inline(always)]
    pub fn scudrvio158(&mut self) -> Scudrvio158W<Scu5bcSpec> {
        Scudrvio158W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO158"]
    #[inline(always)]
    pub fn scuensmtio158(&mut self) -> Scuensmtio158W<Scu5bcSpec> {
        Scuensmtio158W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO158"]
    #[inline(always)]
    pub fn scuenhvio158(&mut self) -> Scuenhvio158W<Scu5bcSpec> {
        Scuenhvio158W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO159"]
    #[inline(always)]
    pub fn scudispdio159(&mut self) -> Scudispdio159W<Scu5bcSpec> {
        Scudispdio159W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO159"]
    #[inline(always)]
    pub fn scudispuio159(&mut self) -> Scudispuio159W<Scu5bcSpec> {
        Scudispuio159W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO159"]
    #[inline(always)]
    pub fn scudrvio159(&mut self) -> Scudrvio159W<Scu5bcSpec> {
        Scudrvio159W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO159"]
    #[inline(always)]
    pub fn scuensmtio159(&mut self) -> Scuensmtio159W<Scu5bcSpec> {
        Scuensmtio159W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO159"]
    #[inline(always)]
    pub fn scuenhvio159(&mut self) -> Scuenhvio159W<Scu5bcSpec> {
        Scuenhvio159W::new(self, 25)
    }
}
#[doc = "IO Control \\#80\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5bcSpec;
impl crate::RegisterSpec for Scu5bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5bc::R`](R) reader structure"]
impl crate::Readable for Scu5bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu5bc::W`](W) writer structure"]
impl crate::Writable for Scu5bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5BC to value 0x0201_0201"]
impl crate::Resettable for Scu5bcSpec {
    const RESET_VALUE: u32 = 0x0201_0201;
}
