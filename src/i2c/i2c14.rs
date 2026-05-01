#[doc = "Register `I2C14` reader"]
pub type R = crate::R<I2c14Spec>;
#[doc = "Register `I2C14` writer"]
pub type W = crate::W<I2c14Spec>;
#[doc = "Field `MIRQSTA` reader - MIRQSTA"]
pub type MirqstaR = crate::FieldReader<u32>;
#[doc = "Field `MIRQSTA` writer - MIRQSTA"]
pub type MirqstaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MIRQSTA"]
    #[inline(always)]
    pub fn mirqsta(&self) -> MirqstaR {
        MirqstaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MIRQSTA"]
    #[inline(always)]
    pub fn mirqsta(&mut self) -> MirqstaW<I2c14Spec> {
        MirqstaW::new(self, 0)
    }
}
#[doc = "Master Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c14Spec;
impl crate::RegisterSpec for I2c14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c14::R`](R) reader structure"]
impl crate::Readable for I2c14Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c14::W`](W) writer structure"]
impl crate::Writable for I2c14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C14 to value 0"]
impl crate::Resettable for I2c14Spec {}
