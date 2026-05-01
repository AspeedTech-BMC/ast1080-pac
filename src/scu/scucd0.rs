#[doc = "Register `SCUCD0` reader"]
pub type R = crate::R<Scucd0Spec>;
#[doc = "Register `SCUCD0` writer"]
pub type W = crate::W<Scucd0Spec>;
#[doc = "Field `SCUREGSEC2A00` reader - SCU_REG_SEC2_A00"]
pub type Scuregsec2a00R = crate::BitReader;
#[doc = "Field `SCUREGSEC2A00` writer - SCU_REG_SEC2_A00"]
pub type Scuregsec2a00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUREGSEC2A08` reader - SCU_REG_SEC2_A08"]
pub type Scuregsec2a08R = crate::BitReader;
#[doc = "Field `SCUREGSEC2A08` writer - SCU_REG_SEC2_A08"]
pub type Scuregsec2a08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2A0C` reader - SCU_REG_SEC2_A0C"]
pub type Scuregsec2a0cR = crate::BitReader;
#[doc = "Field `SCUREGSEC2A0C` writer - SCU_REG_SEC2_A0C"]
pub type Scuregsec2a0cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2A30` reader - SCU_REG_SEC2_A30"]
pub type Scuregsec2a30R = crate::BitReader;
#[doc = "Field `SCUREGSEC2A30` writer - SCU_REG_SEC2_A30"]
pub type Scuregsec2a30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2A34` reader - SCU_REG_SEC2_A34"]
pub type Scuregsec2a34R = crate::BitReader;
#[doc = "Field `SCUREGSEC2A34` writer - SCU_REG_SEC2_A34"]
pub type Scuregsec2a34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2A38` reader - SCU_REG_SEC2_A38"]
pub type Scuregsec2a38R = crate::BitReader;
#[doc = "Field `SCUREGSEC2A38` writer - SCU_REG_SEC2_A38"]
pub type Scuregsec2a38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2A3C` reader - SCU_REG_SEC2_A3C"]
pub type Scuregsec2a3cR = crate::BitReader;
#[doc = "Field `SCUREGSEC2A3C` writer - SCU_REG_SEC2_A3C"]
pub type Scuregsec2a3cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC2_A00"]
    #[inline(always)]
    pub fn scuregsec2a00(&self) -> Scuregsec2a00R {
        Scuregsec2a00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC2_A08"]
    #[inline(always)]
    pub fn scuregsec2a08(&self) -> Scuregsec2a08R {
        Scuregsec2a08R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_SEC2_A0C"]
    #[inline(always)]
    pub fn scuregsec2a0c(&self) -> Scuregsec2a0cR {
        Scuregsec2a0cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_SEC2_A30"]
    #[inline(always)]
    pub fn scuregsec2a30(&self) -> Scuregsec2a30R {
        Scuregsec2a30R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_SEC2_A34"]
    #[inline(always)]
    pub fn scuregsec2a34(&self) -> Scuregsec2a34R {
        Scuregsec2a34R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_SEC2_A38"]
    #[inline(always)]
    pub fn scuregsec2a38(&self) -> Scuregsec2a38R {
        Scuregsec2a38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_SEC2_A3C"]
    #[inline(always)]
    pub fn scuregsec2a3c(&self) -> Scuregsec2a3cR {
        Scuregsec2a3cR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC2_A00"]
    #[inline(always)]
    pub fn scuregsec2a00(&mut self) -> Scuregsec2a00W<Scucd0Spec> {
        Scuregsec2a00W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC2_A08"]
    #[inline(always)]
    pub fn scuregsec2a08(&mut self) -> Scuregsec2a08W<Scucd0Spec> {
        Scuregsec2a08W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_SEC2_A0C"]
    #[inline(always)]
    pub fn scuregsec2a0c(&mut self) -> Scuregsec2a0cW<Scucd0Spec> {
        Scuregsec2a0cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_SEC2_A30"]
    #[inline(always)]
    pub fn scuregsec2a30(&mut self) -> Scuregsec2a30W<Scucd0Spec> {
        Scuregsec2a30W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_SEC2_A34"]
    #[inline(always)]
    pub fn scuregsec2a34(&mut self) -> Scuregsec2a34W<Scucd0Spec> {
        Scuregsec2a34W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_SEC2_A38"]
    #[inline(always)]
    pub fn scuregsec2a38(&mut self) -> Scuregsec2a38W<Scucd0Spec> {
        Scuregsec2a38W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_SEC2_A3C"]
    #[inline(always)]
    pub fn scuregsec2a3c(&mut self) -> Scuregsec2a3cW<Scucd0Spec> {
        Scuregsec2a3cW::new(self, 7)
    }
}
#[doc = "Secure2 Control 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scucd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scucd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scucd0Spec;
impl crate::RegisterSpec for Scucd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scucd0::R`](R) reader structure"]
impl crate::Readable for Scucd0Spec {}
#[doc = "`write(|w| ..)` method takes [`scucd0::W`](W) writer structure"]
impl crate::Writable for Scucd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUCD0 to value 0"]
impl crate::Resettable for Scucd0Spec {}
