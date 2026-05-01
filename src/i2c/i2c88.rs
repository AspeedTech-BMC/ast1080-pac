#[doc = "Register `I2C88` reader"]
pub type R = crate::R<I2c88Spec>;
#[doc = "Register `I2C88` writer"]
pub type W = crate::W<I2c88Spec>;
#[doc = "Field `MIRQLOG` reader - MIRQ_LOG"]
pub type MirqlogR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - MIRQ_LOG"]
    #[inline(always)]
    pub fn mirqlog(&self) -> MirqlogR {
        MirqlogR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "I2CC\\_MIRQ\\_LOG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c88::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c88::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c88Spec;
impl crate::RegisterSpec for I2c88Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c88::R`](R) reader structure"]
impl crate::Readable for I2c88Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c88::W`](W) writer structure"]
impl crate::Writable for I2c88Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C88 to value 0"]
impl crate::Resettable for I2c88Spec {}
