#[doc = "Register `I2C_FILTER_THR09C` reader"]
pub type R = crate::R<I2cFilterThr09cSpec>;
#[doc = "Register `I2C_FILTER_THR09C` writer"]
pub type W = crate::W<I2cFilterThr09cSpec>;
#[doc = "Field `ELOG07` reader - ELOG07"]
pub type Elog07R = crate::FieldReader<u32>;
#[doc = "Field `ELOG07` writer - ELOG07"]
pub type Elog07W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG07"]
    #[inline(always)]
    pub fn elog07(&self) -> Elog07R {
        Elog07R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG07"]
    #[inline(always)]
    pub fn elog07(&mut self) -> Elog07W<I2cFilterThr09cSpec> {
        Elog07W::new(self, 0)
    }
}
#[doc = "I2CF\\_ELOG07\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr09cSpec;
impl crate::RegisterSpec for I2cFilterThr09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr09c::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr09cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr09c::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR09C to value 0"]
impl crate::Resettable for I2cFilterThr09cSpec {}
