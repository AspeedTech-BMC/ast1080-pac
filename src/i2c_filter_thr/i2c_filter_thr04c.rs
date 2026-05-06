#[doc = "Register `I2C_FILTER_THR04C` reader"]
pub type R = crate::R<I2cFilterThr04cSpec>;
#[doc = "Register `I2C_FILTER_THR04C` writer"]
pub type W = crate::W<I2cFilterThr04cSpec>;
#[doc = "Field `MAP3` reader - MAP3"]
pub type Map3R = crate::FieldReader<u32>;
#[doc = "Field `MAP3` writer - MAP3"]
pub type Map3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAP3"]
    #[inline(always)]
    pub fn map3(&self) -> Map3R {
        Map3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAP3"]
    #[inline(always)]
    pub fn map3(&mut self) -> Map3W<I2cFilterThr04cSpec> {
        Map3W::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_MAP3\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr04cSpec;
impl crate::RegisterSpec for I2cFilterThr04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr04c::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr04cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr04c::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR04C to value 0"]
impl crate::Resettable for I2cFilterThr04cSpec {}
