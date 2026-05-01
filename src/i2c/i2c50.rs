#[doc = "Register `I2C50` reader"]
pub type R = crate::R<I2c50Spec>;
#[doc = "Register `I2C50` writer"]
pub type W = crate::W<I2c50Spec>;
#[doc = "Field `DMACFG` reader - DMA_CFG"]
pub type DmacfgR = crate::FieldReader<u32>;
#[doc = "Field `DMACFG` writer - DMA_CFG"]
pub type DmacfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA_CFG"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DmacfgR {
        DmacfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA_CFG"]
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DmacfgW<I2c50Spec> {
        DmacfgW::new(self, 0)
    }
}
#[doc = "I2CC\\_DMA\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c50Spec;
impl crate::RegisterSpec for I2c50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c50::R`](R) reader structure"]
impl crate::Readable for I2c50Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c50::W`](W) writer structure"]
impl crate::Writable for I2c50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C50 to value 0x0404"]
impl crate::Resettable for I2c50Spec {
    const RESET_VALUE: u32 = 0x0404;
}
