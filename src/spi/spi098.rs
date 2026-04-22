#[doc = "Register `SPI098` reader"]
pub type R = crate::R<Spi098Spec>;
#[doc = "Register `SPI098` writer"]
pub type W = crate::W<Spi098Spec>;
#[doc = "Field `CSET1DIDLY0` reader - CSET1_DIDLY0"]
pub type Cset1didly0R = crate::FieldReader;
#[doc = "Field `CSET1DIDLY0` writer - CSET1_DIDLY0"]
pub type Cset1didly0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET1DIDLY1` reader - CSET1_DIDLY1"]
pub type Cset1didly1R = crate::FieldReader;
#[doc = "Field `CSET1DIDLY1` writer - CSET1_DIDLY1"]
pub type Cset1didly1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET1DIDLY2` reader - CSET1_DIDLY2"]
pub type Cset1didly2R = crate::FieldReader;
#[doc = "Field `CSET1DIDLY2` writer - CSET1_DIDLY2"]
pub type Cset1didly2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET1DIDLY3` reader - CSET1_DIDLY3"]
pub type Cset1didly3R = crate::FieldReader;
#[doc = "Field `CSET1DIDLY3` writer - CSET1_DIDLY3"]
pub type Cset1didly3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CSET1_DIDLY0"]
    #[inline(always)]
    pub fn cset1didly0(&self) -> Cset1didly0R {
        Cset1didly0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CSET1_DIDLY1"]
    #[inline(always)]
    pub fn cset1didly1(&self) -> Cset1didly1R {
        Cset1didly1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CSET1_DIDLY2"]
    #[inline(always)]
    pub fn cset1didly2(&self) -> Cset1didly2R {
        Cset1didly2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CSET1_DIDLY3"]
    #[inline(always)]
    pub fn cset1didly3(&self) -> Cset1didly3R {
        Cset1didly3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CSET1_DIDLY0"]
    #[inline(always)]
    pub fn cset1didly0(&mut self) -> Cset1didly0W<Spi098Spec> {
        Cset1didly0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - CSET1_DIDLY1"]
    #[inline(always)]
    pub fn cset1didly1(&mut self) -> Cset1didly1W<Spi098Spec> {
        Cset1didly1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - CSET1_DIDLY2"]
    #[inline(always)]
    pub fn cset1didly2(&mut self) -> Cset1didly2W<Spi098Spec> {
        Cset1didly2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CSET1_DIDLY3"]
    #[inline(always)]
    pub fn cset1didly3(&mut self) -> Cset1didly3W<Spi098Spec> {
        Cset1didly3W::new(self, 24)
    }
}
#[doc = "CE1 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi098Spec;
impl crate::RegisterSpec for Spi098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi098::R`](R) reader structure"]
impl crate::Readable for Spi098Spec {}
#[doc = "`write(|w| ..)` method takes [`spi098::W`](W) writer structure"]
impl crate::Writable for Spi098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI098 to value 0"]
impl crate::Resettable for Spi098Spec {}
