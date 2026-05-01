#[doc = "Register `I2C24` reader"]
pub type R = crate::R<I2c24Spec>;
#[doc = "Register `I2C24` writer"]
pub type W = crate::W<I2c24Spec>;
#[doc = "Field `SIRQSTA` reader - SIRQSTA"]
pub type SirqstaR = crate::FieldReader<u32>;
#[doc = "Field `SIRQSTA` writer - SIRQSTA"]
pub type SirqstaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SIRQSTA"]
    #[inline(always)]
    pub fn sirqsta(&self) -> SirqstaR {
        SirqstaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SIRQSTA"]
    #[inline(always)]
    pub fn sirqsta(&mut self) -> SirqstaW<I2c24Spec> {
        SirqstaW::new(self, 0)
    }
}
#[doc = "Slave Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c24Spec;
impl crate::RegisterSpec for I2c24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c24::R`](R) reader structure"]
impl crate::Readable for I2c24Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c24::W`](W) writer structure"]
impl crate::Writable for I2c24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C24 to value 0"]
impl crate::Resettable for I2c24Spec {}
