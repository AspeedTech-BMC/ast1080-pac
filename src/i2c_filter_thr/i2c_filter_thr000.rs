#[doc = "Register `I2C_FILTER_THR000` reader"]
pub type R = crate::R<I2cFilterThr000Spec>;
#[doc = "Register `I2C_FILTER_THR000` writer"]
pub type W = crate::W<I2cFilterThr000Spec>;
#[doc = "Field `RST` reader - RST"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - RST"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RST"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RST"]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<I2cFilterThr000Spec> {
        RstW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_RST\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr000Spec;
impl crate::RegisterSpec for I2cFilterThr000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr000::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr000Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr000::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR000 to value 0"]
impl crate::Resettable for I2cFilterThr000Spec {}
