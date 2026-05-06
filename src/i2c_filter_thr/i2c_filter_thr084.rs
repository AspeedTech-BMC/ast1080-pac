#[doc = "Register `I2C_FILTER_THR084` reader"]
pub type R = crate::R<I2cFilterThr084Spec>;
#[doc = "Register `I2C_FILTER_THR084` writer"]
pub type W = crate::W<I2cFilterThr084Spec>;
#[doc = "Field `ELOG01` reader - ELOG01"]
pub type Elog01R = crate::FieldReader<u32>;
#[doc = "Field `ELOG01` writer - ELOG01"]
pub type Elog01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG01"]
    #[inline(always)]
    pub fn elog01(&self) -> Elog01R {
        Elog01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG01"]
    #[inline(always)]
    pub fn elog01(&mut self) -> Elog01W<I2cFilterThr084Spec> {
        Elog01W::new(self, 0)
    }
}
#[doc = "I2CF\\_ELOG01\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr084Spec;
impl crate::RegisterSpec for I2cFilterThr084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr084::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr084Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr084::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR084 to value 0"]
impl crate::Resettable for I2cFilterThr084Spec {}
