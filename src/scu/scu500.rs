#[doc = "Register `SCU500` reader"]
pub type R = crate::R<Scu500Spec>;
#[doc = "Register `SCU500` writer"]
pub type W = crate::W<Scu500Spec>;
#[doc = "Field `SCUDISPDIO064` reader - SCU_DIS_PD_IO064"]
pub type Scudispdio064R = crate::BitReader;
#[doc = "Field `SCUDISPDIO064` writer - SCU_DIS_PD_IO064"]
pub type Scudispdio064W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO064` reader - SCU_DIS_PU_IO064"]
pub type Scudispuio064R = crate::BitReader;
#[doc = "Field `SCUDISPUIO064` writer - SCU_DIS_PU_IO064"]
pub type Scudispuio064W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO064` reader - SCU_DRV_IO064"]
pub type Scudrvio064R = crate::FieldReader;
#[doc = "Field `SCUDRVIO064` writer - SCU_DRV_IO064"]
pub type Scudrvio064W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO064` reader - SCU_EN_SMT_IO064"]
pub type Scuensmtio064R = crate::BitReader;
#[doc = "Field `SCUENSMTIO064` writer - SCU_EN_SMT_IO064"]
pub type Scuensmtio064W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO064` reader - SCU_EN_HV_IO064"]
pub type Scuenhvio064R = crate::BitReader;
#[doc = "Field `SCUENHVIO064` writer - SCU_EN_HV_IO064"]
pub type Scuenhvio064W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO065` reader - SCU_DIS_PD_IO065"]
pub type Scudispdio065R = crate::BitReader;
#[doc = "Field `SCUDISPDIO065` writer - SCU_DIS_PD_IO065"]
pub type Scudispdio065W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO065` reader - SCU_DIS_PU_IO065"]
pub type Scudispuio065R = crate::BitReader;
#[doc = "Field `SCUDISPUIO065` writer - SCU_DIS_PU_IO065"]
pub type Scudispuio065W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO065` reader - SCU_DRV_IO065"]
pub type Scudrvio065R = crate::FieldReader;
#[doc = "Field `SCUDRVIO065` writer - SCU_DRV_IO065"]
pub type Scudrvio065W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO065` reader - SCU_EN_SMT_IO065"]
pub type Scuensmtio065R = crate::BitReader;
#[doc = "Field `SCUENSMTIO065` writer - SCU_EN_SMT_IO065"]
pub type Scuensmtio065W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO065` reader - SCU_EN_HV_IO065"]
pub type Scuenhvio065R = crate::BitReader;
#[doc = "Field `SCUENHVIO065` writer - SCU_EN_HV_IO065"]
pub type Scuenhvio065W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO064"]
    #[inline(always)]
    pub fn scudispdio064(&self) -> Scudispdio064R {
        Scudispdio064R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO064"]
    #[inline(always)]
    pub fn scudispuio064(&self) -> Scudispuio064R {
        Scudispuio064R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO064"]
    #[inline(always)]
    pub fn scudrvio064(&self) -> Scudrvio064R {
        Scudrvio064R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO064"]
    #[inline(always)]
    pub fn scuensmtio064(&self) -> Scuensmtio064R {
        Scuensmtio064R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO064"]
    #[inline(always)]
    pub fn scuenhvio064(&self) -> Scuenhvio064R {
        Scuenhvio064R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO065"]
    #[inline(always)]
    pub fn scudispdio065(&self) -> Scudispdio065R {
        Scudispdio065R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO065"]
    #[inline(always)]
    pub fn scudispuio065(&self) -> Scudispuio065R {
        Scudispuio065R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO065"]
    #[inline(always)]
    pub fn scudrvio065(&self) -> Scudrvio065R {
        Scudrvio065R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO065"]
    #[inline(always)]
    pub fn scuensmtio065(&self) -> Scuensmtio065R {
        Scuensmtio065R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO065"]
    #[inline(always)]
    pub fn scuenhvio065(&self) -> Scuenhvio065R {
        Scuenhvio065R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO064"]
    #[inline(always)]
    pub fn scudispdio064(&mut self) -> Scudispdio064W<Scu500Spec> {
        Scudispdio064W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO064"]
    #[inline(always)]
    pub fn scudispuio064(&mut self) -> Scudispuio064W<Scu500Spec> {
        Scudispuio064W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO064"]
    #[inline(always)]
    pub fn scudrvio064(&mut self) -> Scudrvio064W<Scu500Spec> {
        Scudrvio064W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO064"]
    #[inline(always)]
    pub fn scuensmtio064(&mut self) -> Scuensmtio064W<Scu500Spec> {
        Scuensmtio064W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO064"]
    #[inline(always)]
    pub fn scuenhvio064(&mut self) -> Scuenhvio064W<Scu500Spec> {
        Scuenhvio064W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO065"]
    #[inline(always)]
    pub fn scudispdio065(&mut self) -> Scudispdio065W<Scu500Spec> {
        Scudispdio065W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO065"]
    #[inline(always)]
    pub fn scudispuio065(&mut self) -> Scudispuio065W<Scu500Spec> {
        Scudispuio065W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO065"]
    #[inline(always)]
    pub fn scudrvio065(&mut self) -> Scudrvio065W<Scu500Spec> {
        Scudrvio065W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO065"]
    #[inline(always)]
    pub fn scuensmtio065(&mut self) -> Scuensmtio065W<Scu500Spec> {
        Scuensmtio065W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO065"]
    #[inline(always)]
    pub fn scuenhvio065(&mut self) -> Scuenhvio065W<Scu500Spec> {
        Scuenhvio065W::new(self, 25)
    }
}
#[doc = "IO Control \\#33\n\nYou can [`read`](crate::Reg::read) this register and get [`scu500::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu500::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu500Spec;
impl crate::RegisterSpec for Scu500Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu500::R`](R) reader structure"]
impl crate::Readable for Scu500Spec {}
#[doc = "`write(|w| ..)` method takes [`scu500::W`](W) writer structure"]
impl crate::Writable for Scu500Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU500 to value 0x0204_0204"]
impl crate::Resettable for Scu500Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
