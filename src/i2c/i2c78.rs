#[doc = "Register `I2C78` reader"]
pub type R = crate::R<I2c78Spec>;
#[doc = "Register `I2C78` writer"]
pub type W = crate::W<I2c78Spec>;
#[doc = "Field `SERIALSTATUS` reader - SERIAL_STATUS"]
pub type SerialstatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SERIAL_STATUS"]
    #[inline(always)]
    pub fn serialstatus(&self) -> SerialstatusR {
        SerialstatusR::new(self.bits)
    }
}
impl W {}
#[doc = "Debug information for device\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c78::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c78::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c78Spec;
impl crate::RegisterSpec for I2c78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c78::R`](R) reader structure"]
impl crate::Readable for I2c78Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c78::W`](W) writer structure"]
impl crate::Writable for I2c78Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C78 to value 0"]
impl crate::Resettable for I2c78Spec {}
