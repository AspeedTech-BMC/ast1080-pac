#[doc = "Register `I2C8C` reader"]
pub type R = crate::R<I2c8cSpec>;
#[doc = "Register `I2C8C` writer"]
pub type W = crate::W<I2c8cSpec>;
#[doc = "Field `SIRQLOG` reader - SIRQ_LOG"]
pub type SirqlogR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - SIRQ_LOG"]
    #[inline(always)]
    pub fn sirqlog(&self) -> SirqlogR {
        SirqlogR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "I2CC\\_SIRQ\\_LOG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c8c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c8c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c8cSpec;
impl crate::RegisterSpec for I2c8cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c8c::R`](R) reader structure"]
impl crate::Readable for I2c8cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c8c::W`](W) writer structure"]
impl crate::Writable for I2c8cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C8C to value 0"]
impl crate::Resettable for I2c8cSpec {}
