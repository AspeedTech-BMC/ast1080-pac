#[doc = "Register `SCU4B4` reader"]
pub type R = crate::R<Scu4b4Spec>;
#[doc = "Register `SCU4B4` writer"]
pub type W = crate::W<Scu4b4Spec>;
#[doc = "Field `SCUDISPDIO026` reader - SCU_DIS_PD_IO026"]
pub type Scudispdio026R = crate::BitReader;
#[doc = "Field `SCUDISPDIO026` writer - SCU_DIS_PD_IO026"]
pub type Scudispdio026W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO026` reader - SCU_DIS_PU_IO026"]
pub type Scudispuio026R = crate::BitReader;
#[doc = "Field `SCUDISPUIO026` writer - SCU_DIS_PU_IO026"]
pub type Scudispuio026W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO026` reader - SCU_DRV_IO026"]
pub type Scudrvio026R = crate::FieldReader;
#[doc = "Field `SCUDRVIO026` writer - SCU_DRV_IO026"]
pub type Scudrvio026W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO026` reader - SCU_EN_SMT_IO026"]
pub type Scuensmtio026R = crate::BitReader;
#[doc = "Field `SCUENSMTIO026` writer - SCU_EN_SMT_IO026"]
pub type Scuensmtio026W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO026` reader - SCU_EN_HV_IO026"]
pub type Scuenhvio026R = crate::BitReader;
#[doc = "Field `SCUENHVIO026` writer - SCU_EN_HV_IO026"]
pub type Scuenhvio026W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO027` reader - SCU_DIS_PD_IO027"]
pub type Scudispdio027R = crate::BitReader;
#[doc = "Field `SCUDISPDIO027` writer - SCU_DIS_PD_IO027"]
pub type Scudispdio027W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO027` reader - SCU_DIS_PU_IO027"]
pub type Scudispuio027R = crate::BitReader;
#[doc = "Field `SCUDISPUIO027` writer - SCU_DIS_PU_IO027"]
pub type Scudispuio027W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO027` reader - SCU_DRV_IO027"]
pub type Scudrvio027R = crate::FieldReader;
#[doc = "Field `SCUDRVIO027` writer - SCU_DRV_IO027"]
pub type Scudrvio027W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO027` reader - SCU_EN_SMT_IO027"]
pub type Scuensmtio027R = crate::BitReader;
#[doc = "Field `SCUENSMTIO027` writer - SCU_EN_SMT_IO027"]
pub type Scuensmtio027W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO027` reader - SCU_EN_HV_IO027"]
pub type Scuenhvio027R = crate::BitReader;
#[doc = "Field `SCUENHVIO027` writer - SCU_EN_HV_IO027"]
pub type Scuenhvio027W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO026"]
    #[inline(always)]
    pub fn scudispdio026(&self) -> Scudispdio026R {
        Scudispdio026R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO026"]
    #[inline(always)]
    pub fn scudispuio026(&self) -> Scudispuio026R {
        Scudispuio026R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO026"]
    #[inline(always)]
    pub fn scudrvio026(&self) -> Scudrvio026R {
        Scudrvio026R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO026"]
    #[inline(always)]
    pub fn scuensmtio026(&self) -> Scuensmtio026R {
        Scuensmtio026R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO026"]
    #[inline(always)]
    pub fn scuenhvio026(&self) -> Scuenhvio026R {
        Scuenhvio026R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO027"]
    #[inline(always)]
    pub fn scudispdio027(&self) -> Scudispdio027R {
        Scudispdio027R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO027"]
    #[inline(always)]
    pub fn scudispuio027(&self) -> Scudispuio027R {
        Scudispuio027R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO027"]
    #[inline(always)]
    pub fn scudrvio027(&self) -> Scudrvio027R {
        Scudrvio027R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO027"]
    #[inline(always)]
    pub fn scuensmtio027(&self) -> Scuensmtio027R {
        Scuensmtio027R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO027"]
    #[inline(always)]
    pub fn scuenhvio027(&self) -> Scuenhvio027R {
        Scuenhvio027R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO026"]
    #[inline(always)]
    pub fn scudispdio026(&mut self) -> Scudispdio026W<Scu4b4Spec> {
        Scudispdio026W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO026"]
    #[inline(always)]
    pub fn scudispuio026(&mut self) -> Scudispuio026W<Scu4b4Spec> {
        Scudispuio026W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO026"]
    #[inline(always)]
    pub fn scudrvio026(&mut self) -> Scudrvio026W<Scu4b4Spec> {
        Scudrvio026W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO026"]
    #[inline(always)]
    pub fn scuensmtio026(&mut self) -> Scuensmtio026W<Scu4b4Spec> {
        Scuensmtio026W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO026"]
    #[inline(always)]
    pub fn scuenhvio026(&mut self) -> Scuenhvio026W<Scu4b4Spec> {
        Scuenhvio026W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO027"]
    #[inline(always)]
    pub fn scudispdio027(&mut self) -> Scudispdio027W<Scu4b4Spec> {
        Scudispdio027W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO027"]
    #[inline(always)]
    pub fn scudispuio027(&mut self) -> Scudispuio027W<Scu4b4Spec> {
        Scudispuio027W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO027"]
    #[inline(always)]
    pub fn scudrvio027(&mut self) -> Scudrvio027W<Scu4b4Spec> {
        Scudrvio027W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO027"]
    #[inline(always)]
    pub fn scuensmtio027(&mut self) -> Scuensmtio027W<Scu4b4Spec> {
        Scuensmtio027W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO027"]
    #[inline(always)]
    pub fn scuenhvio027(&mut self) -> Scuenhvio027W<Scu4b4Spec> {
        Scuenhvio027W::new(self, 25)
    }
}
#[doc = "IO Control \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4b4Spec;
impl crate::RegisterSpec for Scu4b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4b4::R`](R) reader structure"]
impl crate::Readable for Scu4b4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4b4::W`](W) writer structure"]
impl crate::Writable for Scu4b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4B4 to value 0x0204_0204"]
impl crate::Resettable for Scu4b4Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
