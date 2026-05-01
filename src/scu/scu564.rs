#[doc = "Register `SCU564` reader"]
pub type R = crate::R<Scu564Spec>;
#[doc = "Register `SCU564` writer"]
pub type W = crate::W<Scu564Spec>;
#[doc = "Field `SCUDISPDIO114` reader - SCU_DIS_PD_IO114"]
pub type Scudispdio114R = crate::BitReader;
#[doc = "Field `SCUDISPDIO114` writer - SCU_DIS_PD_IO114"]
pub type Scudispdio114W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO114` reader - SCU_DIS_PU_IO114"]
pub type Scudispuio114R = crate::BitReader;
#[doc = "Field `SCUDISPUIO114` writer - SCU_DIS_PU_IO114"]
pub type Scudispuio114W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO114` reader - SCU_DRV_IO114"]
pub type Scudrvio114R = crate::FieldReader;
#[doc = "Field `SCUDRVIO114` writer - SCU_DRV_IO114"]
pub type Scudrvio114W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO114` reader - SCU_EN_SMT_IO114"]
pub type Scuensmtio114R = crate::BitReader;
#[doc = "Field `SCUENSMTIO114` writer - SCU_EN_SMT_IO114"]
pub type Scuensmtio114W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO114` reader - SCU_EN_HV_IO114"]
pub type Scuenhvio114R = crate::BitReader;
#[doc = "Field `SCUENHVIO114` writer - SCU_EN_HV_IO114"]
pub type Scuenhvio114W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO115` reader - SCU_DIS_PD_IO115"]
pub type Scudispdio115R = crate::BitReader;
#[doc = "Field `SCUDISPDIO115` writer - SCU_DIS_PD_IO115"]
pub type Scudispdio115W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO115` reader - SCU_DIS_PU_IO115"]
pub type Scudispuio115R = crate::BitReader;
#[doc = "Field `SCUDISPUIO115` writer - SCU_DIS_PU_IO115"]
pub type Scudispuio115W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO115` reader - SCU_DRV_IO115"]
pub type Scudrvio115R = crate::FieldReader;
#[doc = "Field `SCUDRVIO115` writer - SCU_DRV_IO115"]
pub type Scudrvio115W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO115` reader - SCU_EN_SMT_IO115"]
pub type Scuensmtio115R = crate::BitReader;
#[doc = "Field `SCUENSMTIO115` writer - SCU_EN_SMT_IO115"]
pub type Scuensmtio115W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO115` reader - SCU_EN_HV_IO115"]
pub type Scuenhvio115R = crate::BitReader;
#[doc = "Field `SCUENHVIO115` writer - SCU_EN_HV_IO115"]
pub type Scuenhvio115W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO114"]
    #[inline(always)]
    pub fn scudispdio114(&self) -> Scudispdio114R {
        Scudispdio114R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO114"]
    #[inline(always)]
    pub fn scudispuio114(&self) -> Scudispuio114R {
        Scudispuio114R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO114"]
    #[inline(always)]
    pub fn scudrvio114(&self) -> Scudrvio114R {
        Scudrvio114R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO114"]
    #[inline(always)]
    pub fn scuensmtio114(&self) -> Scuensmtio114R {
        Scuensmtio114R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO114"]
    #[inline(always)]
    pub fn scuenhvio114(&self) -> Scuenhvio114R {
        Scuenhvio114R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO115"]
    #[inline(always)]
    pub fn scudispdio115(&self) -> Scudispdio115R {
        Scudispdio115R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO115"]
    #[inline(always)]
    pub fn scudispuio115(&self) -> Scudispuio115R {
        Scudispuio115R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO115"]
    #[inline(always)]
    pub fn scudrvio115(&self) -> Scudrvio115R {
        Scudrvio115R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO115"]
    #[inline(always)]
    pub fn scuensmtio115(&self) -> Scuensmtio115R {
        Scuensmtio115R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO115"]
    #[inline(always)]
    pub fn scuenhvio115(&self) -> Scuenhvio115R {
        Scuenhvio115R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO114"]
    #[inline(always)]
    pub fn scudispdio114(&mut self) -> Scudispdio114W<Scu564Spec> {
        Scudispdio114W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO114"]
    #[inline(always)]
    pub fn scudispuio114(&mut self) -> Scudispuio114W<Scu564Spec> {
        Scudispuio114W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO114"]
    #[inline(always)]
    pub fn scudrvio114(&mut self) -> Scudrvio114W<Scu564Spec> {
        Scudrvio114W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO114"]
    #[inline(always)]
    pub fn scuensmtio114(&mut self) -> Scuensmtio114W<Scu564Spec> {
        Scuensmtio114W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO114"]
    #[inline(always)]
    pub fn scuenhvio114(&mut self) -> Scuenhvio114W<Scu564Spec> {
        Scuenhvio114W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO115"]
    #[inline(always)]
    pub fn scudispdio115(&mut self) -> Scudispdio115W<Scu564Spec> {
        Scudispdio115W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO115"]
    #[inline(always)]
    pub fn scudispuio115(&mut self) -> Scudispuio115W<Scu564Spec> {
        Scudispuio115W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO115"]
    #[inline(always)]
    pub fn scudrvio115(&mut self) -> Scudrvio115W<Scu564Spec> {
        Scudrvio115W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO115"]
    #[inline(always)]
    pub fn scuensmtio115(&mut self) -> Scuensmtio115W<Scu564Spec> {
        Scuensmtio115W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO115"]
    #[inline(always)]
    pub fn scuenhvio115(&mut self) -> Scuenhvio115W<Scu564Spec> {
        Scuenhvio115W::new(self, 25)
    }
}
#[doc = "IO Control \\#58\n\nYou can [`read`](crate::Reg::read) this register and get [`scu564::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu564::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu564Spec;
impl crate::RegisterSpec for Scu564Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu564::R`](R) reader structure"]
impl crate::Readable for Scu564Spec {}
#[doc = "`write(|w| ..)` method takes [`scu564::W`](W) writer structure"]
impl crate::Writable for Scu564Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU564 to value 0x0204_0204"]
impl crate::Resettable for Scu564Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
