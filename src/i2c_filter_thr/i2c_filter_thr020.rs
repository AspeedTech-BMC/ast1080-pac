#[doc = "Register `I2C_FILTER_THR020` reader"]
pub type R = crate::R<I2cFilterThr020Spec>;
#[doc = "Register `I2C_FILTER_THR020` writer"]
pub type W = crate::W<I2cFilterThr020Spec>;
#[doc = "Field `STATUS` reader - STATUS"]
pub type StatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - STATUS"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl W {}
#[doc = "I2CFLT\\_THR0\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr020Spec;
impl crate::RegisterSpec for I2cFilterThr020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr020::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr020Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr020::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR020 to value 0"]
impl crate::Resettable for I2cFilterThr020Spec {}
