#[doc = "Register `SPI0A0` reader"]
pub type R = crate::R<Spi0a0Spec>;
#[doc = "Register `SPI0A0` writer"]
pub type W = crate::W<Spi0a0Spec>;
#[doc = "Field `CSET3DIDLY0` reader - CSET3_DIDLY0"]
pub type Cset3didly0R = crate::FieldReader;
#[doc = "Field `CSET3DIDLY0` writer - CSET3_DIDLY0"]
pub type Cset3didly0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET3DIDLY1` reader - CSET3_DIDLY1"]
pub type Cset3didly1R = crate::FieldReader;
#[doc = "Field `CSET3DIDLY1` writer - CSET3_DIDLY1"]
pub type Cset3didly1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET3DIDLY2` reader - CSET3_DIDLY2"]
pub type Cset3didly2R = crate::FieldReader;
#[doc = "Field `CSET3DIDLY2` writer - CSET3_DIDLY2"]
pub type Cset3didly2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET3DIDLY3` reader - CSET3_DIDLY3"]
pub type Cset3didly3R = crate::FieldReader;
#[doc = "Field `CSET3DIDLY3` writer - CSET3_DIDLY3"]
pub type Cset3didly3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CSET3_DIDLY0"]
    #[inline(always)]
    pub fn cset3didly0(&self) -> Cset3didly0R {
        Cset3didly0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CSET3_DIDLY1"]
    #[inline(always)]
    pub fn cset3didly1(&self) -> Cset3didly1R {
        Cset3didly1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CSET3_DIDLY2"]
    #[inline(always)]
    pub fn cset3didly2(&self) -> Cset3didly2R {
        Cset3didly2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CSET3_DIDLY3"]
    #[inline(always)]
    pub fn cset3didly3(&self) -> Cset3didly3R {
        Cset3didly3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CSET3_DIDLY0"]
    #[inline(always)]
    pub fn cset3didly0(&mut self) -> Cset3didly0W<Spi0a0Spec> {
        Cset3didly0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - CSET3_DIDLY1"]
    #[inline(always)]
    pub fn cset3didly1(&mut self) -> Cset3didly1W<Spi0a0Spec> {
        Cset3didly1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - CSET3_DIDLY2"]
    #[inline(always)]
    pub fn cset3didly2(&mut self) -> Cset3didly2W<Spi0a0Spec> {
        Cset3didly2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CSET3_DIDLY3"]
    #[inline(always)]
    pub fn cset3didly3(&mut self) -> Cset3didly3W<Spi0a0Spec> {
        Cset3didly3W::new(self, 24)
    }
}
#[doc = "CE3 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0a0Spec;
impl crate::RegisterSpec for Spi0a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0a0::R`](R) reader structure"]
impl crate::Readable for Spi0a0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi0a0::W`](W) writer structure"]
impl crate::Writable for Spi0a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0A0 to value 0"]
impl crate::Resettable for Spi0a0Spec {}
