#[doc = "Register `I2C_FILTER_THR088` reader"]
pub type R = crate::R<I2cFilterThr088Spec>;
#[doc = "Register `I2C_FILTER_THR088` writer"]
pub type W = crate::W<I2cFilterThr088Spec>;
#[doc = "Field `ELOG02` reader - ELOG02"]
pub type Elog02R = crate::FieldReader<u32>;
#[doc = "Field `ELOG02` writer - ELOG02"]
pub type Elog02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG02"]
    #[inline(always)]
    pub fn elog02(&self) -> Elog02R {
        Elog02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG02"]
    #[inline(always)]
    pub fn elog02(&mut self) -> Elog02W<I2cFilterThr088Spec> {
        Elog02W::new(self, 0)
    }
}
#[doc = "I2CF\\_ELOG02\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr088Spec;
impl crate::RegisterSpec for I2cFilterThr088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr088::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr088Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr088::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR088 to value 0"]
impl crate::Resettable for I2cFilterThr088Spec {}
