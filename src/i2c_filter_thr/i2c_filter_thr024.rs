#[doc = "Register `I2C_FILTER_THR024` reader"]
pub type R = crate::R<I2cFilterThr024Spec>;
#[doc = "Register `I2C_FILTER_THR024` writer"]
pub type W = crate::W<I2cFilterThr024Spec>;
#[doc = "Field `RXSEQVAL` reader - RX_SEQ_VAL"]
pub type RxseqvalR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RX_SEQ_VAL"]
    #[inline(always)]
    pub fn rxseqval(&self) -> RxseqvalR {
        RxseqvalR::new(self.bits)
    }
}
impl W {}
#[doc = "I2CFLT\\_THR0\\_SEQ\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr024Spec;
impl crate::RegisterSpec for I2cFilterThr024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr024::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr024Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr024::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR024 to value 0"]
impl crate::Resettable for I2cFilterThr024Spec {}
