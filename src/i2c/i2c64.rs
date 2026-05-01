#[doc = "Register `I2C64` reader"]
pub type R = crate::R<I2c64Spec>;
#[doc = "Register `I2C64` writer"]
pub type W = crate::W<I2c64Spec>;
#[doc = "Field `MRXAHI` reader - MRXA_HI"]
pub type MrxahiR = crate::FieldReader;
#[doc = "Field `MRXAHI` writer - MRXA_HI"]
pub type MrxahiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MRXA_HI"]
    #[inline(always)]
    pub fn mrxahi(&self) -> MrxahiR {
        MrxahiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MRXA_HI"]
    #[inline(always)]
    pub fn mrxahi(&mut self) -> MrxahiW<I2c64Spec> {
        MrxahiW::new(self, 0)
    }
}
#[doc = "Master DMA Mode Rx Buffer Base Address\\[39:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c64Spec;
impl crate::RegisterSpec for I2c64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c64::R`](R) reader structure"]
impl crate::Readable for I2c64Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c64::W`](W) writer structure"]
impl crate::Writable for I2c64Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C64 to value 0"]
impl crate::Resettable for I2c64Spec {}
