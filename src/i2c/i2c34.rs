#[doc = "Register `I2C34` reader"]
pub type R = crate::R<I2c34Spec>;
#[doc = "Register `I2C34` writer"]
pub type W = crate::W<I2c34Spec>;
#[doc = "Field `MRXA` reader - MRXA"]
pub type MrxaR = crate::FieldReader<u32>;
#[doc = "Field `MRXA` writer - MRXA"]
pub type MrxaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MRXA"]
    #[inline(always)]
    pub fn mrxa(&self) -> MrxaR {
        MrxaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MRXA"]
    #[inline(always)]
    pub fn mrxa(&mut self) -> MrxaW<I2c34Spec> {
        MrxaW::new(self, 0)
    }
}
#[doc = "Master DMA Mode Rx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c34Spec;
impl crate::RegisterSpec for I2c34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c34::R`](R) reader structure"]
impl crate::Readable for I2c34Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c34::W`](W) writer structure"]
impl crate::Writable for I2c34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C34 to value 0"]
impl crate::Resettable for I2c34Spec {}
