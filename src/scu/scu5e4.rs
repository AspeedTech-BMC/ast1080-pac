#[doc = "Register `SCU5E4` reader"]
pub type R = crate::R<Scu5e4Spec>;
#[doc = "Register `SCU5E4` writer"]
pub type W = crate::W<Scu5e4Spec>;
#[doc = "Field `SCUDISPDIO178` reader - SCU_DIS_PD_IO178"]
pub type Scudispdio178R = crate::BitReader;
#[doc = "Field `SCUDISPDIO178` writer - SCU_DIS_PD_IO178"]
pub type Scudispdio178W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO178` reader - SCU_DIS_PU_IO178"]
pub type Scudispuio178R = crate::BitReader;
#[doc = "Field `SCUDISPUIO178` writer - SCU_DIS_PU_IO178"]
pub type Scudispuio178W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO178` reader - SCU_DRV_IO178"]
pub type Scudrvio178R = crate::FieldReader;
#[doc = "Field `SCUDRVIO178` writer - SCU_DRV_IO178"]
pub type Scudrvio178W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO178` reader - SCU_EN_SMT_IO178"]
pub type Scuensmtio178R = crate::BitReader;
#[doc = "Field `SCUENSMTIO178` writer - SCU_EN_SMT_IO178"]
pub type Scuensmtio178W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO178` reader - SCU_EN_HV_IO178"]
pub type Scuenhvio178R = crate::BitReader;
#[doc = "Field `SCUENHVIO178` writer - SCU_EN_HV_IO178"]
pub type Scuenhvio178W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO179` reader - SCU_DIS_PD_IO179"]
pub type Scudispdio179R = crate::BitReader;
#[doc = "Field `SCUDISPDIO179` writer - SCU_DIS_PD_IO179"]
pub type Scudispdio179W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO179` reader - SCU_DIS_PU_IO179"]
pub type Scudispuio179R = crate::BitReader;
#[doc = "Field `SCUDISPUIO179` writer - SCU_DIS_PU_IO179"]
pub type Scudispuio179W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO179` reader - SCU_DRV_IO179"]
pub type Scudrvio179R = crate::FieldReader;
#[doc = "Field `SCUDRVIO179` writer - SCU_DRV_IO179"]
pub type Scudrvio179W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO179` reader - SCU_EN_SMT_IO179"]
pub type Scuensmtio179R = crate::BitReader;
#[doc = "Field `SCUENSMTIO179` writer - SCU_EN_SMT_IO179"]
pub type Scuensmtio179W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO179` reader - SCU_EN_HV_IO179"]
pub type Scuenhvio179R = crate::BitReader;
#[doc = "Field `SCUENHVIO179` writer - SCU_EN_HV_IO179"]
pub type Scuenhvio179W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO178"]
    #[inline(always)]
    pub fn scudispdio178(&self) -> Scudispdio178R {
        Scudispdio178R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO178"]
    #[inline(always)]
    pub fn scudispuio178(&self) -> Scudispuio178R {
        Scudispuio178R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO178"]
    #[inline(always)]
    pub fn scudrvio178(&self) -> Scudrvio178R {
        Scudrvio178R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO178"]
    #[inline(always)]
    pub fn scuensmtio178(&self) -> Scuensmtio178R {
        Scuensmtio178R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO178"]
    #[inline(always)]
    pub fn scuenhvio178(&self) -> Scuenhvio178R {
        Scuenhvio178R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO179"]
    #[inline(always)]
    pub fn scudispdio179(&self) -> Scudispdio179R {
        Scudispdio179R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO179"]
    #[inline(always)]
    pub fn scudispuio179(&self) -> Scudispuio179R {
        Scudispuio179R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO179"]
    #[inline(always)]
    pub fn scudrvio179(&self) -> Scudrvio179R {
        Scudrvio179R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO179"]
    #[inline(always)]
    pub fn scuensmtio179(&self) -> Scuensmtio179R {
        Scuensmtio179R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO179"]
    #[inline(always)]
    pub fn scuenhvio179(&self) -> Scuenhvio179R {
        Scuenhvio179R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO178"]
    #[inline(always)]
    pub fn scudispdio178(&mut self) -> Scudispdio178W<Scu5e4Spec> {
        Scudispdio178W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO178"]
    #[inline(always)]
    pub fn scudispuio178(&mut self) -> Scudispuio178W<Scu5e4Spec> {
        Scudispuio178W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO178"]
    #[inline(always)]
    pub fn scudrvio178(&mut self) -> Scudrvio178W<Scu5e4Spec> {
        Scudrvio178W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO178"]
    #[inline(always)]
    pub fn scuensmtio178(&mut self) -> Scuensmtio178W<Scu5e4Spec> {
        Scuensmtio178W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO178"]
    #[inline(always)]
    pub fn scuenhvio178(&mut self) -> Scuenhvio178W<Scu5e4Spec> {
        Scuenhvio178W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO179"]
    #[inline(always)]
    pub fn scudispdio179(&mut self) -> Scudispdio179W<Scu5e4Spec> {
        Scudispdio179W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO179"]
    #[inline(always)]
    pub fn scudispuio179(&mut self) -> Scudispuio179W<Scu5e4Spec> {
        Scudispuio179W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO179"]
    #[inline(always)]
    pub fn scudrvio179(&mut self) -> Scudrvio179W<Scu5e4Spec> {
        Scudrvio179W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO179"]
    #[inline(always)]
    pub fn scuensmtio179(&mut self) -> Scuensmtio179W<Scu5e4Spec> {
        Scuensmtio179W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO179"]
    #[inline(always)]
    pub fn scuenhvio179(&mut self) -> Scuenhvio179W<Scu5e4Spec> {
        Scuenhvio179W::new(self, 25)
    }
}
#[doc = "IO Control \\#90\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5e4Spec;
impl crate::RegisterSpec for Scu5e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5e4::R`](R) reader structure"]
impl crate::Readable for Scu5e4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu5e4::W`](W) writer structure"]
impl crate::Writable for Scu5e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5E4 to value 0x0204_0204"]
impl crate::Resettable for Scu5e4Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
