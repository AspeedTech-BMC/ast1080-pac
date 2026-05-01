#[doc = "Register `I2C30` reader"]
pub type R = crate::R<I2c30Spec>;
#[doc = "Register `I2C30` writer"]
pub type W = crate::W<I2c30Spec>;
#[doc = "Field `MTXA` reader - MTXA"]
pub type MtxaR = crate::FieldReader<u32>;
#[doc = "Field `MTXA` writer - MTXA"]
pub type MtxaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MTXA"]
    #[inline(always)]
    pub fn mtxa(&self) -> MtxaR {
        MtxaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MTXA"]
    #[inline(always)]
    pub fn mtxa(&mut self) -> MtxaW<I2c30Spec> {
        MtxaW::new(self, 0)
    }
}
#[doc = "Master DMA Mode Tx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c30Spec;
impl crate::RegisterSpec for I2c30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c30::R`](R) reader structure"]
impl crate::Readable for I2c30Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c30::W`](W) writer structure"]
impl crate::Writable for I2c30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C30 to value 0"]
impl crate::Resettable for I2c30Spec {}
