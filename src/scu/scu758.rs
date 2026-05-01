#[doc = "Register `SCU758` reader"]
pub type R = crate::R<Scu758Spec>;
#[doc = "Register `SCU758` writer"]
pub type W = crate::W<Scu758Spec>;
#[doc = "Field `SCUMUXSEC1IO192` reader - SCU_MUX_SEC1_IO192"]
pub type Scumuxsec1io192R = crate::BitReader;
#[doc = "Field `SCUMUXSEC1IO192` writer - SCU_MUX_SEC1_IO192"]
pub type Scumuxsec1io192W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUMUXSEC1IO193` reader - SCU_MUX_SEC1_IO193"]
pub type Scumuxsec1io193R = crate::BitReader;
#[doc = "Field `SCUMUXSEC1IO193` writer - SCU_MUX_SEC1_IO193"]
pub type Scumuxsec1io193W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_MUX_SEC1_IO192"]
    #[inline(always)]
    pub fn scumuxsec1io192(&self) -> Scumuxsec1io192R {
        Scumuxsec1io192R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_MUX_SEC1_IO193"]
    #[inline(always)]
    pub fn scumuxsec1io193(&self) -> Scumuxsec1io193R {
        Scumuxsec1io193R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_MUX_SEC1_IO192"]
    #[inline(always)]
    pub fn scumuxsec1io192(&mut self) -> Scumuxsec1io192W<Scu758Spec> {
        Scumuxsec1io192W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_MUX_SEC1_IO193"]
    #[inline(always)]
    pub fn scumuxsec1io193(&mut self) -> Scumuxsec1io193W<Scu758Spec> {
        Scumuxsec1io193W::new(self, 1)
    }
}
#[doc = "IO Secure 1 Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu758::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu758::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu758Spec;
impl crate::RegisterSpec for Scu758Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu758::R`](R) reader structure"]
impl crate::Readable for Scu758Spec {}
#[doc = "`write(|w| ..)` method takes [`scu758::W`](W) writer structure"]
impl crate::Writable for Scu758Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU758 to value 0"]
impl crate::Resettable for Scu758Spec {}
