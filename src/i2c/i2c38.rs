#[doc = "Register `I2C38` reader"]
pub type R = crate::R<I2c38Spec>;
#[doc = "Register `I2C38` writer"]
pub type W = crate::W<I2c38Spec>;
#[doc = "Field `STXA` reader - STXA"]
pub type StxaR = crate::FieldReader<u32>;
#[doc = "Field `STXA` writer - STXA"]
pub type StxaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - STXA"]
    #[inline(always)]
    pub fn stxa(&self) -> StxaR {
        StxaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - STXA"]
    #[inline(always)]
    pub fn stxa(&mut self) -> StxaW<I2c38Spec> {
        StxaW::new(self, 0)
    }
}
#[doc = "Slave DMA Mode Tx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c38Spec;
impl crate::RegisterSpec for I2c38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c38::R`](R) reader structure"]
impl crate::Readable for I2c38Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c38::W`](W) writer structure"]
impl crate::Writable for I2c38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C38 to value 0"]
impl crate::Resettable for I2c38Spec {}
