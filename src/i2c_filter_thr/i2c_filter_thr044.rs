#[doc = "Register `I2C_FILTER_THR044` reader"]
pub type R = crate::R<I2cFilterThr044Spec>;
#[doc = "Register `I2C_FILTER_THR044` writer"]
pub type W = crate::W<I2cFilterThr044Spec>;
#[doc = "Field `MAP1` reader - MAP1"]
pub type Map1R = crate::FieldReader<u32>;
#[doc = "Field `MAP1` writer - MAP1"]
pub type Map1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAP1"]
    #[inline(always)]
    pub fn map1(&self) -> Map1R {
        Map1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAP1"]
    #[inline(always)]
    pub fn map1(&mut self) -> Map1W<I2cFilterThr044Spec> {
        Map1W::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_MAP1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr044Spec;
impl crate::RegisterSpec for I2cFilterThr044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr044::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr044Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr044::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR044 to value 0"]
impl crate::Resettable for I2cFilterThr044Spec {}
