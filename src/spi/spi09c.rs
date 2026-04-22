#[doc = "Register `SPI09C` reader"]
pub type R = crate::R<Spi09cSpec>;
#[doc = "Register `SPI09C` writer"]
pub type W = crate::W<Spi09cSpec>;
#[doc = "Field `CSET2DIDLY0` reader - CSET2_DIDLY0"]
pub type Cset2didly0R = crate::FieldReader;
#[doc = "Field `CSET2DIDLY0` writer - CSET2_DIDLY0"]
pub type Cset2didly0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET2DIDLY1` reader - CSET2_DIDLY1"]
pub type Cset2didly1R = crate::FieldReader;
#[doc = "Field `CSET2DIDLY1` writer - CSET2_DIDLY1"]
pub type Cset2didly1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET2DIDLY2` reader - CSET2_DIDLY2"]
pub type Cset2didly2R = crate::FieldReader;
#[doc = "Field `CSET2DIDLY2` writer - CSET2_DIDLY2"]
pub type Cset2didly2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET2DIDLY3` reader - CSET2_DIDLY3"]
pub type Cset2didly3R = crate::FieldReader;
#[doc = "Field `CSET2DIDLY3` writer - CSET2_DIDLY3"]
pub type Cset2didly3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CSET2_DIDLY0"]
    #[inline(always)]
    pub fn cset2didly0(&self) -> Cset2didly0R {
        Cset2didly0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CSET2_DIDLY1"]
    #[inline(always)]
    pub fn cset2didly1(&self) -> Cset2didly1R {
        Cset2didly1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CSET2_DIDLY2"]
    #[inline(always)]
    pub fn cset2didly2(&self) -> Cset2didly2R {
        Cset2didly2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CSET2_DIDLY3"]
    #[inline(always)]
    pub fn cset2didly3(&self) -> Cset2didly3R {
        Cset2didly3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CSET2_DIDLY0"]
    #[inline(always)]
    pub fn cset2didly0(&mut self) -> Cset2didly0W<Spi09cSpec> {
        Cset2didly0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - CSET2_DIDLY1"]
    #[inline(always)]
    pub fn cset2didly1(&mut self) -> Cset2didly1W<Spi09cSpec> {
        Cset2didly1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - CSET2_DIDLY2"]
    #[inline(always)]
    pub fn cset2didly2(&mut self) -> Cset2didly2W<Spi09cSpec> {
        Cset2didly2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CSET2_DIDLY3"]
    #[inline(always)]
    pub fn cset2didly3(&mut self) -> Cset2didly3W<Spi09cSpec> {
        Cset2didly3W::new(self, 24)
    }
}
#[doc = "CE2 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi09cSpec;
impl crate::RegisterSpec for Spi09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi09c::R`](R) reader structure"]
impl crate::Readable for Spi09cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi09c::W`](W) writer structure"]
impl crate::Writable for Spi09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI09C to value 0"]
impl crate::Resettable for Spi09cSpec {}
