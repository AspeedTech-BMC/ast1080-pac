#[doc = "Register `I2C9C` reader"]
pub type R = crate::R<I2c9cSpec>;
#[doc = "Register `I2C9C` writer"]
pub type W = crate::W<I2c9cSpec>;
#[doc = "Field `DEBTH` reader - DEB_TH"]
pub type DebthR = crate::FieldReader;
#[doc = "Field `DEBTH` writer - DEB_TH"]
pub type DebthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPTH` reader - SP_TH"]
pub type SpthR = crate::FieldReader;
#[doc = "Field `SPTH` writer - SP_TH"]
pub type SpthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DEB_TH"]
    #[inline(always)]
    pub fn debth(&self) -> DebthR {
        DebthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SP_TH"]
    #[inline(always)]
    pub fn spth(&self) -> SpthR {
        SpthR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DEB_TH"]
    #[inline(always)]
    pub fn debth(&mut self) -> DebthW<I2c9cSpec> {
        DebthW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SP_TH"]
    #[inline(always)]
    pub fn spth(&mut self) -> SpthW<I2c9cSpec> {
        SpthW::new(self, 8)
    }
}
#[doc = "I2CC\\_MISC2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c9c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c9c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c9cSpec;
impl crate::RegisterSpec for I2c9cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c9c::R`](R) reader structure"]
impl crate::Readable for I2c9cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c9c::W`](W) writer structure"]
impl crate::Writable for I2c9cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C9C to value 0x1020"]
impl crate::Resettable for I2c9cSpec {
    const RESET_VALUE: u32 = 0x1020;
}
