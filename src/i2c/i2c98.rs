#[doc = "Register `I2C98` reader"]
pub type R = crate::R<I2c98Spec>;
#[doc = "Register `I2C98` writer"]
pub type W = crate::W<I2c98Spec>;
#[doc = "Field `MCMDQ` reader - MCMDQ"]
pub type McmdqR = crate::FieldReader<u32>;
#[doc = "Field `MCMDQ` writer - MCMDQ"]
pub type McmdqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MCMDQ"]
    #[inline(always)]
    pub fn mcmdq(&self) -> McmdqR {
        McmdqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MCMDQ"]
    #[inline(always)]
    pub fn mcmdq(&mut self) -> McmdqW<I2c98Spec> {
        McmdqW::new(self, 0)
    }
}
#[doc = "I2CM\\_MCMDQ\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c98Spec;
impl crate::RegisterSpec for I2c98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c98::R`](R) reader structure"]
impl crate::Readable for I2c98Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c98::W`](W) writer structure"]
impl crate::Writable for I2c98Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C98 to value 0"]
impl crate::Resettable for I2c98Spec {}
