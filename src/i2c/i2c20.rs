#[doc = "Register `I2C20` reader"]
pub type R = crate::R<I2c20Spec>;
#[doc = "Register `I2C20` writer"]
pub type W = crate::W<I2c20Spec>;
#[doc = "Field `SIRQEN` reader - SIRQEN"]
pub type SirqenR = crate::FieldReader<u32>;
#[doc = "Field `SIRQEN` writer - SIRQEN"]
pub type SirqenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SIRQEN"]
    #[inline(always)]
    pub fn sirqen(&self) -> SirqenR {
        SirqenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SIRQEN"]
    #[inline(always)]
    pub fn sirqen(&mut self) -> SirqenW<I2c20Spec> {
        SirqenW::new(self, 0)
    }
}
#[doc = "Slave Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c20Spec;
impl crate::RegisterSpec for I2c20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c20::R`](R) reader structure"]
impl crate::Readable for I2c20Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c20::W`](W) writer structure"]
impl crate::Writable for I2c20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C20 to value 0"]
impl crate::Resettable for I2c20Spec {}
