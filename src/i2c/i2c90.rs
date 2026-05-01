#[doc = "Register `I2C90` reader"]
pub type R = crate::R<I2c90Spec>;
#[doc = "Register `I2C90` writer"]
pub type W = crate::W<I2c90Spec>;
#[doc = "Field `WLOCK` reader - WLOCK"]
pub type WlockR = crate::FieldReader<u32>;
#[doc = "Field `WLOCK` writer - WLOCK"]
pub type WlockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK"]
    #[inline(always)]
    pub fn wlock(&self) -> WlockR {
        WlockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK"]
    #[inline(always)]
    pub fn wlock(&mut self) -> WlockW<I2c90Spec> {
        WlockW::new(self, 0)
    }
}
#[doc = "I2CC\\_WLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c90::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c90::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c90Spec;
impl crate::RegisterSpec for I2c90Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c90::R`](R) reader structure"]
impl crate::Readable for I2c90Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c90::W`](W) writer structure"]
impl crate::Writable for I2c90Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C90 to value 0"]
impl crate::Resettable for I2c90Spec {}
