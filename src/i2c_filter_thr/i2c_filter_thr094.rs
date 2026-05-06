#[doc = "Register `I2C_FILTER_THR094` reader"]
pub type R = crate::R<I2cFilterThr094Spec>;
#[doc = "Register `I2C_FILTER_THR094` writer"]
pub type W = crate::W<I2cFilterThr094Spec>;
#[doc = "Field `ELOG05` reader - ELOG05"]
pub type Elog05R = crate::FieldReader<u32>;
#[doc = "Field `ELOG05` writer - ELOG05"]
pub type Elog05W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG05"]
    #[inline(always)]
    pub fn elog05(&self) -> Elog05R {
        Elog05R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG05"]
    #[inline(always)]
    pub fn elog05(&mut self) -> Elog05W<I2cFilterThr094Spec> {
        Elog05W::new(self, 0)
    }
}
#[doc = "I2CF\\_ELOG05\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr094Spec;
impl crate::RegisterSpec for I2cFilterThr094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr094::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr094Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr094::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR094 to value 0"]
impl crate::Resettable for I2cFilterThr094Spec {}
