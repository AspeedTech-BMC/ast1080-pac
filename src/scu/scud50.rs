#[doc = "Register `SCUD50` reader"]
pub type R = crate::R<Scud50Spec>;
#[doc = "Register `SCUD50` writer"]
pub type W = crate::W<Scud50Spec>;
#[doc = "Field `SCUREGSEC3A00` reader - SCU_REG_SEC3_A00"]
pub type Scuregsec3a00R = crate::BitReader;
#[doc = "Field `SCUREGSEC3A00` writer - SCU_REG_SEC3_A00"]
pub type Scuregsec3a00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUREGSEC3A08` reader - SCU_REG_SEC3_A08"]
pub type Scuregsec3a08R = crate::BitReader;
#[doc = "Field `SCUREGSEC3A08` writer - SCU_REG_SEC3_A08"]
pub type Scuregsec3a08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3A0C` reader - SCU_REG_SEC3_A0C"]
pub type Scuregsec3a0cR = crate::BitReader;
#[doc = "Field `SCUREGSEC3A0C` writer - SCU_REG_SEC3_A0C"]
pub type Scuregsec3a0cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3A30` reader - SCU_REG_SEC3_A30"]
pub type Scuregsec3a30R = crate::BitReader;
#[doc = "Field `SCUREGSEC3A30` writer - SCU_REG_SEC3_A30"]
pub type Scuregsec3a30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3A34` reader - SCU_REG_SEC3_A34"]
pub type Scuregsec3a34R = crate::BitReader;
#[doc = "Field `SCUREGSEC3A34` writer - SCU_REG_SEC3_A34"]
pub type Scuregsec3a34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3A38` reader - SCU_REG_SEC3_A38"]
pub type Scuregsec3a38R = crate::BitReader;
#[doc = "Field `SCUREGSEC3A38` writer - SCU_REG_SEC3_A38"]
pub type Scuregsec3a38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3A3C` reader - SCU_REG_SEC3_A3C"]
pub type Scuregsec3a3cR = crate::BitReader;
#[doc = "Field `SCUREGSEC3A3C` writer - SCU_REG_SEC3_A3C"]
pub type Scuregsec3a3cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC3_A00"]
    #[inline(always)]
    pub fn scuregsec3a00(&self) -> Scuregsec3a00R {
        Scuregsec3a00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC3_A08"]
    #[inline(always)]
    pub fn scuregsec3a08(&self) -> Scuregsec3a08R {
        Scuregsec3a08R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_SEC3_A0C"]
    #[inline(always)]
    pub fn scuregsec3a0c(&self) -> Scuregsec3a0cR {
        Scuregsec3a0cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_SEC3_A30"]
    #[inline(always)]
    pub fn scuregsec3a30(&self) -> Scuregsec3a30R {
        Scuregsec3a30R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_SEC3_A34"]
    #[inline(always)]
    pub fn scuregsec3a34(&self) -> Scuregsec3a34R {
        Scuregsec3a34R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_SEC3_A38"]
    #[inline(always)]
    pub fn scuregsec3a38(&self) -> Scuregsec3a38R {
        Scuregsec3a38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_SEC3_A3C"]
    #[inline(always)]
    pub fn scuregsec3a3c(&self) -> Scuregsec3a3cR {
        Scuregsec3a3cR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC3_A00"]
    #[inline(always)]
    pub fn scuregsec3a00(&mut self) -> Scuregsec3a00W<Scud50Spec> {
        Scuregsec3a00W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC3_A08"]
    #[inline(always)]
    pub fn scuregsec3a08(&mut self) -> Scuregsec3a08W<Scud50Spec> {
        Scuregsec3a08W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_SEC3_A0C"]
    #[inline(always)]
    pub fn scuregsec3a0c(&mut self) -> Scuregsec3a0cW<Scud50Spec> {
        Scuregsec3a0cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_SEC3_A30"]
    #[inline(always)]
    pub fn scuregsec3a30(&mut self) -> Scuregsec3a30W<Scud50Spec> {
        Scuregsec3a30W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_SEC3_A34"]
    #[inline(always)]
    pub fn scuregsec3a34(&mut self) -> Scuregsec3a34W<Scud50Spec> {
        Scuregsec3a34W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_SEC3_A38"]
    #[inline(always)]
    pub fn scuregsec3a38(&mut self) -> Scuregsec3a38W<Scud50Spec> {
        Scuregsec3a38W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_SEC3_A3C"]
    #[inline(always)]
    pub fn scuregsec3a3c(&mut self) -> Scuregsec3a3cW<Scud50Spec> {
        Scuregsec3a3cW::new(self, 7)
    }
}
#[doc = "Secure3 Control 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scud50Spec;
impl crate::RegisterSpec for Scud50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scud50::R`](R) reader structure"]
impl crate::Readable for Scud50Spec {}
#[doc = "`write(|w| ..)` method takes [`scud50::W`](W) writer structure"]
impl crate::Writable for Scud50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUD50 to value 0"]
impl crate::Resettable for Scud50Spec {}
