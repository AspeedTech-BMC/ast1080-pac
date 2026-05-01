#[doc = "Register `I2C84` reader"]
pub type R = crate::R<I2c84Spec>;
#[doc = "Register `I2C84` writer"]
pub type W = crate::W<I2c84Spec>;
#[doc = "Field `SBYTELOG` reader - SBYTE_LOG"]
pub type SbytelogR = crate::FieldReader<u32>;
#[doc = "Field `SBYTELOG` writer - SBYTE_LOG"]
pub type SbytelogW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SBYTE_LOG"]
    #[inline(always)]
    pub fn sbytelog(&self) -> SbytelogR {
        SbytelogR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SBYTE_LOG"]
    #[inline(always)]
    pub fn sbytelog(&mut self) -> SbytelogW<I2c84Spec> {
        SbytelogW::new(self, 0)
    }
}
#[doc = "Recorder information for Byte transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c84::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c84::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c84Spec;
impl crate::RegisterSpec for I2c84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c84::R`](R) reader structure"]
impl crate::Readable for I2c84Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c84::W`](W) writer structure"]
impl crate::Writable for I2c84Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C84 to value 0"]
impl crate::Resettable for I2c84Spec {}
