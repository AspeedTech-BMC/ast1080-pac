#[doc = "Register `SCUC34` reader"]
pub type R = crate::R<Scuc34Spec>;
#[doc = "Register `SCUC34` writer"]
pub type W = crate::W<Scuc34Spec>;
#[doc = "Field `SCUREGSEC1680` reader - SCU_REG_SEC1_680"]
pub type Scuregsec1680R = crate::BitReader;
#[doc = "Field `SCUREGSEC1680` writer - SCU_REG_SEC1_680"]
pub type Scuregsec1680W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1684` reader - SCU_REG_SEC1_684"]
pub type Scuregsec1684R = crate::BitReader;
#[doc = "Field `SCUREGSEC1684` writer - SCU_REG_SEC1_684"]
pub type Scuregsec1684W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1688` reader - SCU_REG_SEC1_688"]
pub type Scuregsec1688R = crate::BitReader;
#[doc = "Field `SCUREGSEC1688` writer - SCU_REG_SEC1_688"]
pub type Scuregsec1688W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC168C` reader - SCU_REG_SEC1_68C"]
pub type Scuregsec168cR = crate::BitReader;
#[doc = "Field `SCUREGSEC168C` writer - SCU_REG_SEC1_68C"]
pub type Scuregsec168cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC16FC` reader - SCU_REG_SEC1_6FC"]
pub type Scuregsec16fcR = crate::BitReader;
#[doc = "Field `SCUREGSEC16FC` writer - SCU_REG_SEC1_6FC"]
pub type Scuregsec16fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC1_680"]
    #[inline(always)]
    pub fn scuregsec1680(&self) -> Scuregsec1680R {
        Scuregsec1680R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC1_684"]
    #[inline(always)]
    pub fn scuregsec1684(&self) -> Scuregsec1684R {
        Scuregsec1684R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC1_688"]
    #[inline(always)]
    pub fn scuregsec1688(&self) -> Scuregsec1688R {
        Scuregsec1688R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_SEC1_68C"]
    #[inline(always)]
    pub fn scuregsec168c(&self) -> Scuregsec168cR {
        Scuregsec168cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_SEC1_6FC"]
    #[inline(always)]
    pub fn scuregsec16fc(&self) -> Scuregsec16fcR {
        Scuregsec16fcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC1_680"]
    #[inline(always)]
    pub fn scuregsec1680(&mut self) -> Scuregsec1680W<Scuc34Spec> {
        Scuregsec1680W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC1_684"]
    #[inline(always)]
    pub fn scuregsec1684(&mut self) -> Scuregsec1684W<Scuc34Spec> {
        Scuregsec1684W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_SEC1_688"]
    #[inline(always)]
    pub fn scuregsec1688(&mut self) -> Scuregsec1688W<Scuc34Spec> {
        Scuregsec1688W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_SEC1_68C"]
    #[inline(always)]
    pub fn scuregsec168c(&mut self) -> Scuregsec168cW<Scuc34Spec> {
        Scuregsec168cW::new(self, 3)
    }
    #[doc = "Bit 31 - SCU_REG_SEC1_6FC"]
    #[inline(always)]
    pub fn scuregsec16fc(&mut self) -> Scuregsec16fcW<Scuc34Spec> {
        Scuregsec16fcW::new(self, 31)
    }
}
#[doc = "Secure1 Control 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc34Spec;
impl crate::RegisterSpec for Scuc34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc34::R`](R) reader structure"]
impl crate::Readable for Scuc34Spec {}
#[doc = "`write(|w| ..)` method takes [`scuc34::W`](W) writer structure"]
impl crate::Writable for Scuc34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC34 to value 0"]
impl crate::Resettable for Scuc34Spec {}
