#[doc = "Register `I2C60` reader"]
pub type R = crate::R<I2c60Spec>;
#[doc = "Register `I2C60` writer"]
pub type W = crate::W<I2c60Spec>;
#[doc = "Field `MTXAHI` reader - MTXA_HI"]
pub type MtxahiR = crate::FieldReader;
#[doc = "Field `MTXAHI` writer - MTXA_HI"]
pub type MtxahiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MTXA_HI"]
    #[inline(always)]
    pub fn mtxahi(&self) -> MtxahiR {
        MtxahiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MTXA_HI"]
    #[inline(always)]
    pub fn mtxahi(&mut self) -> MtxahiW<I2c60Spec> {
        MtxahiW::new(self, 0)
    }
}
#[doc = "Master DMA Mode Tx Buffer Base Address\\[39:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c60Spec;
impl crate::RegisterSpec for I2c60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c60::R`](R) reader structure"]
impl crate::Readable for I2c60Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c60::W`](W) writer structure"]
impl crate::Writable for I2c60Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C60 to value 0"]
impl crate::Resettable for I2c60Spec {}
