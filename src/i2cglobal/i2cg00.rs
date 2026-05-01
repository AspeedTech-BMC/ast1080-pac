#[doc = "Register `I2CG00` reader"]
pub type R = crate::R<I2cg00Spec>;
#[doc = "Register `I2CG00` writer"]
pub type W = crate::W<I2cg00Spec>;
#[doc = "Field `I2CMIRQ` reader - I2C_MIRQ"]
pub type I2cmirqR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - I2C_MIRQ"]
    #[inline(always)]
    pub fn i2cmirq(&self) -> I2cmirqR {
        I2cmirqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Device Master Mode Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cg00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cg00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cg00Spec;
impl crate::RegisterSpec for I2cg00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cg00::R`](R) reader structure"]
impl crate::Readable for I2cg00Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cg00::W`](W) writer structure"]
impl crate::Writable for I2cg00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CG00 to value 0"]
impl crate::Resettable for I2cg00Spec {}
