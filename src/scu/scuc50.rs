#[doc = "Register `SCUC50` reader"]
pub type R = crate::R<Scuc50Spec>;
#[doc = "Register `SCUC50` writer"]
pub type W = crate::W<Scuc50Spec>;
#[doc = "Field `SCUREGSEC1A00` reader - SCU_REG_SEC1_A00"]
pub type Scuregsec1a00R = crate::BitReader;
#[doc = "Field `SCUREGSEC1A00` writer - SCU_REG_SEC1_A00"]
pub type Scuregsec1a00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUREGSEC1A08` reader - SCU_REG_SEC1_A08"]
pub type Scuregsec1a08R = crate::BitReader;
#[doc = "Field `SCUREGSEC1A08` writer - SCU_REG_SEC1_A08"]
pub type Scuregsec1a08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1A0C` reader - SCU_REG_SEC1_A0C"]
pub type Scuregsec1a0cR = crate::BitReader;
#[doc = "Field `SCUREGSEC1A0C` writer - SCU_REG_SEC1_A0C"]
pub type Scuregsec1a0cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1A30` reader - SCU_REG_SEC1_A30"]
pub type Scuregsec1a30R = crate::BitReader;
#[doc = "Field `SCUREGSEC1A30` writer - SCU_REG_SEC1_A30"]
pub type Scuregsec1a30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1A34` reader - SCU_REG_SEC1_A34"]
pub type Scuregsec1a34R = crate::BitReader;
#[doc = "Field `SCUREGSEC1A34` writer - SCU_REG_SEC1_A34"]
pub type Scuregsec1a34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1A38` reader - SCU_REG_SEC1_A38"]
pub type Scuregsec1a38R = crate::BitReader;
#[doc = "Field `SCUREGSEC1A38` writer - SCU_REG_SEC1_A38"]
pub type Scuregsec1a38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1A3C` reader - SCU_REG_SEC1_A3C"]
pub type Scuregsec1a3cR = crate::BitReader;
#[doc = "Field `SCUREGSEC1A3C` writer - SCU_REG_SEC1_A3C"]
pub type Scuregsec1a3cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC1_A00"]
    #[inline(always)]
    pub fn scuregsec1a00(&self) -> Scuregsec1a00R {
        Scuregsec1a00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC1_A08"]
    #[inline(always)]
    pub fn scuregsec1a08(&self) -> Scuregsec1a08R {
        Scuregsec1a08R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_SEC1_A0C"]
    #[inline(always)]
    pub fn scuregsec1a0c(&self) -> Scuregsec1a0cR {
        Scuregsec1a0cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_SEC1_A30"]
    #[inline(always)]
    pub fn scuregsec1a30(&self) -> Scuregsec1a30R {
        Scuregsec1a30R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_SEC1_A34"]
    #[inline(always)]
    pub fn scuregsec1a34(&self) -> Scuregsec1a34R {
        Scuregsec1a34R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_SEC1_A38"]
    #[inline(always)]
    pub fn scuregsec1a38(&self) -> Scuregsec1a38R {
        Scuregsec1a38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_SEC1_A3C"]
    #[inline(always)]
    pub fn scuregsec1a3c(&self) -> Scuregsec1a3cR {
        Scuregsec1a3cR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC1_A00"]
    #[inline(always)]
    pub fn scuregsec1a00(&mut self) -> Scuregsec1a00W<Scuc50Spec> {
        Scuregsec1a00W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC1_A08"]
    #[inline(always)]
    pub fn scuregsec1a08(&mut self) -> Scuregsec1a08W<Scuc50Spec> {
        Scuregsec1a08W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_SEC1_A0C"]
    #[inline(always)]
    pub fn scuregsec1a0c(&mut self) -> Scuregsec1a0cW<Scuc50Spec> {
        Scuregsec1a0cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_SEC1_A30"]
    #[inline(always)]
    pub fn scuregsec1a30(&mut self) -> Scuregsec1a30W<Scuc50Spec> {
        Scuregsec1a30W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_SEC1_A34"]
    #[inline(always)]
    pub fn scuregsec1a34(&mut self) -> Scuregsec1a34W<Scuc50Spec> {
        Scuregsec1a34W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_SEC1_A38"]
    #[inline(always)]
    pub fn scuregsec1a38(&mut self) -> Scuregsec1a38W<Scuc50Spec> {
        Scuregsec1a38W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_SEC1_A3C"]
    #[inline(always)]
    pub fn scuregsec1a3c(&mut self) -> Scuregsec1a3cW<Scuc50Spec> {
        Scuregsec1a3cW::new(self, 7)
    }
}
#[doc = "Secure1 Control 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc50Spec;
impl crate::RegisterSpec for Scuc50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc50::R`](R) reader structure"]
impl crate::Readable for Scuc50Spec {}
#[doc = "`write(|w| ..)` method takes [`scuc50::W`](W) writer structure"]
impl crate::Writable for Scuc50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC50 to value 0"]
impl crate::Resettable for Scuc50Spec {}
