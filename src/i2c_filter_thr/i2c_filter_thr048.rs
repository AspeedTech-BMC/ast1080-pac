#[doc = "Register `I2C_FILTER_THR048` reader"]
pub type R = crate::R<I2cFilterThr048Spec>;
#[doc = "Register `I2C_FILTER_THR048` writer"]
pub type W = crate::W<I2cFilterThr048Spec>;
#[doc = "Field `MAP2` reader - MAP2"]
pub type Map2R = crate::FieldReader<u32>;
#[doc = "Field `MAP2` writer - MAP2"]
pub type Map2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAP2"]
    #[inline(always)]
    pub fn map2(&self) -> Map2R {
        Map2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAP2"]
    #[inline(always)]
    pub fn map2(&mut self) -> Map2W<I2cFilterThr048Spec> {
        Map2W::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_MAP2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr048Spec;
impl crate::RegisterSpec for I2cFilterThr048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr048::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr048Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr048::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR048 to value 0"]
impl crate::Resettable for I2cFilterThr048Spec {}
