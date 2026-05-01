#[doc = "Register `SCUF50` reader"]
pub type R = crate::R<Scuf50Spec>;
#[doc = "Register `SCUF50` writer"]
pub type W = crate::W<Scuf50Spec>;
#[doc = "Field `SCUREGRSTA00` reader - SCU_REG_RST_A00"]
pub type Scuregrsta00R = crate::BitReader;
#[doc = "Field `SCUREGRSTA00` writer - SCU_REG_RST_A00"]
pub type Scuregrsta00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUREGRSTA08` reader - SCU_REG_RST_A08"]
pub type Scuregrsta08R = crate::BitReader;
#[doc = "Field `SCUREGRSTA08` writer - SCU_REG_RST_A08"]
pub type Scuregrsta08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRSTA0C` reader - SCU_REG_RST_A0C"]
pub type Scuregrsta0cR = crate::BitReader;
#[doc = "Field `SCUREGRSTA0C` writer - SCU_REG_RST_A0C"]
pub type Scuregrsta0cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRSTA30` reader - SCU_REG_RST_A30"]
pub type Scuregrsta30R = crate::BitReader;
#[doc = "Field `SCUREGRSTA30` writer - SCU_REG_RST_A30"]
pub type Scuregrsta30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRSTA34` reader - SCU_REG_RST_A34"]
pub type Scuregrsta34R = crate::BitReader;
#[doc = "Field `SCUREGRSTA34` writer - SCU_REG_RST_A34"]
pub type Scuregrsta34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRSTA38` reader - SCU_REG_RST_A38"]
pub type Scuregrsta38R = crate::BitReader;
#[doc = "Field `SCUREGRSTA38` writer - SCU_REG_RST_A38"]
pub type Scuregrsta38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRSTA3C` reader - SCU_REG_RST_A3C"]
pub type Scuregrsta3cR = crate::BitReader;
#[doc = "Field `SCUREGRSTA3C` writer - SCU_REG_RST_A3C"]
pub type Scuregrsta3cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_A00"]
    #[inline(always)]
    pub fn scuregrsta00(&self) -> Scuregrsta00R {
        Scuregrsta00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_RST_A08"]
    #[inline(always)]
    pub fn scuregrsta08(&self) -> Scuregrsta08R {
        Scuregrsta08R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_RST_A0C"]
    #[inline(always)]
    pub fn scuregrsta0c(&self) -> Scuregrsta0cR {
        Scuregrsta0cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_RST_A30"]
    #[inline(always)]
    pub fn scuregrsta30(&self) -> Scuregrsta30R {
        Scuregrsta30R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_RST_A34"]
    #[inline(always)]
    pub fn scuregrsta34(&self) -> Scuregrsta34R {
        Scuregrsta34R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_RST_A38"]
    #[inline(always)]
    pub fn scuregrsta38(&self) -> Scuregrsta38R {
        Scuregrsta38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_RST_A3C"]
    #[inline(always)]
    pub fn scuregrsta3c(&self) -> Scuregrsta3cR {
        Scuregrsta3cR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_A00"]
    #[inline(always)]
    pub fn scuregrsta00(&mut self) -> Scuregrsta00W<Scuf50Spec> {
        Scuregrsta00W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_REG_RST_A08"]
    #[inline(always)]
    pub fn scuregrsta08(&mut self) -> Scuregrsta08W<Scuf50Spec> {
        Scuregrsta08W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_RST_A0C"]
    #[inline(always)]
    pub fn scuregrsta0c(&mut self) -> Scuregrsta0cW<Scuf50Spec> {
        Scuregrsta0cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_RST_A30"]
    #[inline(always)]
    pub fn scuregrsta30(&mut self) -> Scuregrsta30W<Scuf50Spec> {
        Scuregrsta30W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_RST_A34"]
    #[inline(always)]
    pub fn scuregrsta34(&mut self) -> Scuregrsta34W<Scuf50Spec> {
        Scuregrsta34W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_RST_A38"]
    #[inline(always)]
    pub fn scuregrsta38(&mut self) -> Scuregrsta38W<Scuf50Spec> {
        Scuregrsta38W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_RST_A3C"]
    #[inline(always)]
    pub fn scuregrsta3c(&mut self) -> Scuregrsta3cW<Scuf50Spec> {
        Scuregrsta3cW::new(self, 7)
    }
}
#[doc = "Reset Control 21 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf50Spec;
impl crate::RegisterSpec for Scuf50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf50::R`](R) reader structure"]
impl crate::Readable for Scuf50Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf50::W`](W) writer structure"]
impl crate::Writable for Scuf50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF50 to value 0"]
impl crate::Resettable for Scuf50Spec {}
