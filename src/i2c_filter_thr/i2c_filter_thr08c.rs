#[doc = "Register `I2C_FILTER_THR08C` reader"]
pub type R = crate::R<I2cFilterThr08cSpec>;
#[doc = "Register `I2C_FILTER_THR08C` writer"]
pub type W = crate::W<I2cFilterThr08cSpec>;
#[doc = "Field `ELOG03` reader - ELOG03"]
pub type Elog03R = crate::FieldReader<u32>;
#[doc = "Field `ELOG03` writer - ELOG03"]
pub type Elog03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG03"]
    #[inline(always)]
    pub fn elog03(&self) -> Elog03R {
        Elog03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG03"]
    #[inline(always)]
    pub fn elog03(&mut self) -> Elog03W<I2cFilterThr08cSpec> {
        Elog03W::new(self, 0)
    }
}
#[doc = "I2CF\\_ELOG03\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr08cSpec;
impl crate::RegisterSpec for I2cFilterThr08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr08c::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr08cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr08c::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR08C to value 0"]
impl crate::Resettable for I2cFilterThr08cSpec {}
