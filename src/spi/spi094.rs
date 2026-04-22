#[doc = "Register `SPI094` reader"]
pub type R = crate::R<Spi094Spec>;
#[doc = "Register `SPI094` writer"]
pub type W = crate::W<Spi094Spec>;
#[doc = "Field `CSET0DIDLY0` reader - CSET0_DIDLY0"]
pub type Cset0didly0R = crate::FieldReader;
#[doc = "Field `CSET0DIDLY0` writer - CSET0_DIDLY0"]
pub type Cset0didly0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET0DIDLY1` reader - CSET0_DIDLY1"]
pub type Cset0didly1R = crate::FieldReader;
#[doc = "Field `CSET0DIDLY1` writer - CSET0_DIDLY1"]
pub type Cset0didly1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET0DIDLY2` reader - CSET0_DIDLY2"]
pub type Cset0didly2R = crate::FieldReader;
#[doc = "Field `CSET0DIDLY2` writer - CSET0_DIDLY2"]
pub type Cset0didly2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET0DIDLY3` reader - CSET0_DIDLY3"]
pub type Cset0didly3R = crate::FieldReader;
#[doc = "Field `CSET0DIDLY3` writer - CSET0_DIDLY3"]
pub type Cset0didly3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CSET0_DIDLY0"]
    #[inline(always)]
    pub fn cset0didly0(&self) -> Cset0didly0R {
        Cset0didly0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CSET0_DIDLY1"]
    #[inline(always)]
    pub fn cset0didly1(&self) -> Cset0didly1R {
        Cset0didly1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CSET0_DIDLY2"]
    #[inline(always)]
    pub fn cset0didly2(&self) -> Cset0didly2R {
        Cset0didly2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CSET0_DIDLY3"]
    #[inline(always)]
    pub fn cset0didly3(&self) -> Cset0didly3R {
        Cset0didly3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CSET0_DIDLY0"]
    #[inline(always)]
    pub fn cset0didly0(&mut self) -> Cset0didly0W<Spi094Spec> {
        Cset0didly0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - CSET0_DIDLY1"]
    #[inline(always)]
    pub fn cset0didly1(&mut self) -> Cset0didly1W<Spi094Spec> {
        Cset0didly1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - CSET0_DIDLY2"]
    #[inline(always)]
    pub fn cset0didly2(&mut self) -> Cset0didly2W<Spi094Spec> {
        Cset0didly2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CSET0_DIDLY3"]
    #[inline(always)]
    pub fn cset0didly3(&mut self) -> Cset0didly3W<Spi094Spec> {
        Cset0didly3W::new(self, 24)
    }
}
#[doc = "CE0 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi094Spec;
impl crate::RegisterSpec for Spi094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi094::R`](R) reader structure"]
impl crate::Readable for Spi094Spec {}
#[doc = "`write(|w| ..)` method takes [`spi094::W`](W) writer structure"]
impl crate::Writable for Spi094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI094 to value 0"]
impl crate::Resettable for Spi094Spec {}
