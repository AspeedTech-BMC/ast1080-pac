#[doc = "Register `I2C_FILTER_THR040` reader"]
pub type R = crate::R<I2cFilterThr040Spec>;
#[doc = "Register `I2C_FILTER_THR040` writer"]
pub type W = crate::W<I2cFilterThr040Spec>;
#[doc = "Field `MAP0` reader - MAP0"]
pub type Map0R = crate::FieldReader<u32>;
#[doc = "Field `MAP0` writer - MAP0"]
pub type Map0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAP0"]
    #[inline(always)]
    pub fn map0(&self) -> Map0R {
        Map0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAP0"]
    #[inline(always)]
    pub fn map0(&mut self) -> Map0W<I2cFilterThr040Spec> {
        Map0W::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_MAP0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr040Spec;
impl crate::RegisterSpec for I2cFilterThr040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr040::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr040Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr040::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR040 to value 0"]
impl crate::Resettable for I2cFilterThr040Spec {}
