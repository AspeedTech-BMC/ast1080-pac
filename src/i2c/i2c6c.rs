#[doc = "Register `I2C6C` reader"]
pub type R = crate::R<I2c6cSpec>;
#[doc = "Register `I2C6C` writer"]
pub type W = crate::W<I2c6cSpec>;
#[doc = "Field `SRXAHI` reader - SRXA_HI"]
pub type SrxahiR = crate::FieldReader;
#[doc = "Field `SRXAHI` writer - SRXA_HI"]
pub type SrxahiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRXA_HI"]
    #[inline(always)]
    pub fn srxahi(&self) -> SrxahiR {
        SrxahiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRXA_HI"]
    #[inline(always)]
    pub fn srxahi(&mut self) -> SrxahiW<I2c6cSpec> {
        SrxahiW::new(self, 0)
    }
}
#[doc = "Slave DMA Mode Rx Buffer Base Address\\[39:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c6c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c6c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c6cSpec;
impl crate::RegisterSpec for I2c6cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c6c::R`](R) reader structure"]
impl crate::Readable for I2c6cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c6c::W`](W) writer structure"]
impl crate::Writable for I2c6cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C6C to value 0"]
impl crate::Resettable for I2c6cSpec {}
