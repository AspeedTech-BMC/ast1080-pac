#[doc = "Register `SCUE50` reader"]
pub type R = crate::R<Scue50Spec>;
#[doc = "Register `SCUE50` writer"]
pub type W = crate::W<Scue50Spec>;
#[doc = "Field `SCUREGLOCKA00` reader - SCU_REG_LOCK_A00"]
pub type Scureglocka00R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA00` writer - SCU_REG_LOCK_A00"]
pub type Scureglocka00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA08` reader - SCU_REG_LOCK_A08"]
pub type Scureglocka08R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA08` writer - SCU_REG_LOCK_A08"]
pub type Scureglocka08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA0C` reader - SCU_REG_LOCK_A0C"]
pub type Scureglocka0cR = crate::BitReader;
#[doc = "Field `SCUREGLOCKA0C` writer - SCU_REG_LOCK_A0C"]
pub type Scureglocka0cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA30` reader - SCU_REG_LOCK_A30"]
pub type Scureglocka30R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA30` writer - SCU_REG_LOCK_A30"]
pub type Scureglocka30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA34` reader - SCU_REG_LOCK_A34"]
pub type Scureglocka34R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA34` writer - SCU_REG_LOCK_A34"]
pub type Scureglocka34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA38` reader - SCU_REG_LOCK_A38"]
pub type Scureglocka38R = crate::BitReader;
#[doc = "Field `SCUREGLOCKA38` writer - SCU_REG_LOCK_A38"]
pub type Scureglocka38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCKA3C` reader - SCU_REG_LOCK_A3C"]
pub type Scureglocka3cR = crate::BitReader;
#[doc = "Field `SCUREGLOCKA3C` writer - SCU_REG_LOCK_A3C"]
pub type Scureglocka3cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_A00"]
    #[inline(always)]
    pub fn scureglocka00(&self) -> Scureglocka00R {
        Scureglocka00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_A08"]
    #[inline(always)]
    pub fn scureglocka08(&self) -> Scureglocka08R {
        Scureglocka08R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_A0C"]
    #[inline(always)]
    pub fn scureglocka0c(&self) -> Scureglocka0cR {
        Scureglocka0cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_A30"]
    #[inline(always)]
    pub fn scureglocka30(&self) -> Scureglocka30R {
        Scureglocka30R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_A34"]
    #[inline(always)]
    pub fn scureglocka34(&self) -> Scureglocka34R {
        Scureglocka34R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_A38"]
    #[inline(always)]
    pub fn scureglocka38(&self) -> Scureglocka38R {
        Scureglocka38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_A3C"]
    #[inline(always)]
    pub fn scureglocka3c(&self) -> Scureglocka3cR {
        Scureglocka3cR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_A00"]
    #[inline(always)]
    pub fn scureglocka00(&mut self) -> Scureglocka00W<Scue50Spec> {
        Scureglocka00W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_A08"]
    #[inline(always)]
    pub fn scureglocka08(&mut self) -> Scureglocka08W<Scue50Spec> {
        Scureglocka08W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_A0C"]
    #[inline(always)]
    pub fn scureglocka0c(&mut self) -> Scureglocka0cW<Scue50Spec> {
        Scureglocka0cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_A30"]
    #[inline(always)]
    pub fn scureglocka30(&mut self) -> Scureglocka30W<Scue50Spec> {
        Scureglocka30W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_A34"]
    #[inline(always)]
    pub fn scureglocka34(&mut self) -> Scureglocka34W<Scue50Spec> {
        Scureglocka34W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_A38"]
    #[inline(always)]
    pub fn scureglocka38(&mut self) -> Scureglocka38W<Scue50Spec> {
        Scureglocka38W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_A3C"]
    #[inline(always)]
    pub fn scureglocka3c(&mut self) -> Scureglocka3cW<Scue50Spec> {
        Scureglocka3cW::new(self, 7)
    }
}
#[doc = "Write Protection 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue50Spec;
impl crate::RegisterSpec for Scue50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue50::R`](R) reader structure"]
impl crate::Readable for Scue50Spec {}
#[doc = "`write(|w| ..)` method takes [`scue50::W`](W) writer structure"]
impl crate::Writable for Scue50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE50 to value 0"]
impl crate::Resettable for Scue50Spec {}
