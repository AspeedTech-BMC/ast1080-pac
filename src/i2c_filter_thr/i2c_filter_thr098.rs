#[doc = "Register `I2C_FILTER_THR098` reader"]
pub type R = crate::R<I2cFilterThr098Spec>;
#[doc = "Register `I2C_FILTER_THR098` writer"]
pub type W = crate::W<I2cFilterThr098Spec>;
#[doc = "Field `ELOG06` reader - ELOG06"]
pub type Elog06R = crate::FieldReader<u32>;
#[doc = "Field `ELOG06` writer - ELOG06"]
pub type Elog06W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG06"]
    #[inline(always)]
    pub fn elog06(&self) -> Elog06R {
        Elog06R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG06"]
    #[inline(always)]
    pub fn elog06(&mut self) -> Elog06W<I2cFilterThr098Spec> {
        Elog06W::new(self, 0)
    }
}
#[doc = "I2CF\\_ELOG06\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr098Spec;
impl crate::RegisterSpec for I2cFilterThr098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr098::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr098Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr098::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR098 to value 0"]
impl crate::Resettable for I2cFilterThr098Spec {}
