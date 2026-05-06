#[doc = "Register `I2C_FILTER_THR090` reader"]
pub type R = crate::R<I2cFilterThr090Spec>;
#[doc = "Register `I2C_FILTER_THR090` writer"]
pub type W = crate::W<I2cFilterThr090Spec>;
#[doc = "Field `ELOG04` reader - ELOG04"]
pub type Elog04R = crate::FieldReader<u32>;
#[doc = "Field `ELOG04` writer - ELOG04"]
pub type Elog04W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG04"]
    #[inline(always)]
    pub fn elog04(&self) -> Elog04R {
        Elog04R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG04"]
    #[inline(always)]
    pub fn elog04(&mut self) -> Elog04W<I2cFilterThr090Spec> {
        Elog04W::new(self, 0)
    }
}
#[doc = "I2CF\\_ELOG04\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr090Spec;
impl crate::RegisterSpec for I2cFilterThr090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr090::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr090Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr090::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR090 to value 0"]
impl crate::Resettable for I2cFilterThr090Spec {}
