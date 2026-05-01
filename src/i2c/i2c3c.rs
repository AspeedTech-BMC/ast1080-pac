#[doc = "Register `I2C3C` reader"]
pub type R = crate::R<I2c3cSpec>;
#[doc = "Register `I2C3C` writer"]
pub type W = crate::W<I2c3cSpec>;
#[doc = "Field `SRXA` reader - SRXA"]
pub type SrxaR = crate::FieldReader<u32>;
#[doc = "Field `SRXA` writer - SRXA"]
pub type SrxaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SRXA"]
    #[inline(always)]
    pub fn srxa(&self) -> SrxaR {
        SrxaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SRXA"]
    #[inline(always)]
    pub fn srxa(&mut self) -> SrxaW<I2c3cSpec> {
        SrxaW::new(self, 0)
    }
}
#[doc = "Slave DMA Mode Rx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c3cSpec;
impl crate::RegisterSpec for I2c3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c3c::R`](R) reader structure"]
impl crate::Readable for I2c3cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c3c::W`](W) writer structure"]
impl crate::Writable for I2c3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C3C to value 0"]
impl crate::Resettable for I2c3cSpec {}
