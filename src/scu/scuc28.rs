#[doc = "Register `SCUC28` reader"]
pub type R = crate::R<Scuc28Spec>;
#[doc = "Register `SCUC28` writer"]
pub type W = crate::W<Scuc28Spec>;
#[doc = "Field `SCUREGSEC1540` reader - SCU_REG_SEC1_540"]
pub type Scuregsec1540R = crate::BitReader;
#[doc = "Field `SCUREGSEC1540` writer - SCU_REG_SEC1_540"]
pub type Scuregsec1540W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1544` reader - SCU_REG_SEC1_544"]
pub type Scuregsec1544R = crate::BitReader;
#[doc = "Field `SCUREGSEC1544` writer - SCU_REG_SEC1_544"]
pub type Scuregsec1544W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1548` reader - SCU_REG_SEC1_548"]
pub type Scuregsec1548R = crate::BitReader;
#[doc = "Field `SCUREGSEC1548` writer - SCU_REG_SEC1_548"]
pub type Scuregsec1548W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC154C` reader - SCU_REG_SEC1_54C"]
pub type Scuregsec154cR = crate::BitReader;
#[doc = "Field `SCUREGSEC154C` writer - SCU_REG_SEC1_54C"]
pub type Scuregsec154cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC157C` reader - SCU_REG_SEC1_57C"]
pub type Scuregsec157cR = crate::BitReader;
#[doc = "Field `SCUREGSEC157C` writer - SCU_REG_SEC1_57C"]
pub type Scuregsec157cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - SCU_REG_SEC1_540"]
    #[inline(always)]
    pub fn scuregsec1540(&self) -> Scuregsec1540R {
        Scuregsec1540R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_SEC1_544"]
    #[inline(always)]
    pub fn scuregsec1544(&self) -> Scuregsec1544R {
        Scuregsec1544R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_SEC1_548"]
    #[inline(always)]
    pub fn scuregsec1548(&self) -> Scuregsec1548R {
        Scuregsec1548R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_SEC1_54C"]
    #[inline(always)]
    pub fn scuregsec154c(&self) -> Scuregsec154cR {
        Scuregsec154cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_SEC1_57C"]
    #[inline(always)]
    pub fn scuregsec157c(&self) -> Scuregsec157cR {
        Scuregsec157cR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - SCU_REG_SEC1_540"]
    #[inline(always)]
    pub fn scuregsec1540(&mut self) -> Scuregsec1540W<Scuc28Spec> {
        Scuregsec1540W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_SEC1_544"]
    #[inline(always)]
    pub fn scuregsec1544(&mut self) -> Scuregsec1544W<Scuc28Spec> {
        Scuregsec1544W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_SEC1_548"]
    #[inline(always)]
    pub fn scuregsec1548(&mut self) -> Scuregsec1548W<Scuc28Spec> {
        Scuregsec1548W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_SEC1_54C"]
    #[inline(always)]
    pub fn scuregsec154c(&mut self) -> Scuregsec154cW<Scuc28Spec> {
        Scuregsec154cW::new(self, 19)
    }
    #[doc = "Bit 31 - SCU_REG_SEC1_57C"]
    #[inline(always)]
    pub fn scuregsec157c(&mut self) -> Scuregsec157cW<Scuc28Spec> {
        Scuregsec157cW::new(self, 31)
    }
}
#[doc = "Secure1 Control 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc28Spec;
impl crate::RegisterSpec for Scuc28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc28::R`](R) reader structure"]
impl crate::Readable for Scuc28Spec {}
#[doc = "`write(|w| ..)` method takes [`scuc28::W`](W) writer structure"]
impl crate::Writable for Scuc28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC28 to value 0"]
impl crate::Resettable for Scuc28Spec {}
