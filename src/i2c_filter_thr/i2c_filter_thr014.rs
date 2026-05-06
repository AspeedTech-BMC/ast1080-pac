#[doc = "Register `I2C_FILTER_THR014` reader"]
pub type R = crate::R<I2cFilterThr014Spec>;
#[doc = "Register `I2C_FILTER_THR014` writer"]
pub type W = crate::W<I2cFilterThr014Spec>;
#[doc = "Field `INTEN` reader - INTEN"]
pub type IntenR = crate::FieldReader;
#[doc = "Field `INTEN` writer - INTEN"]
pub type IntenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<I2cFilterThr014Spec> {
        IntenW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr014Spec;
impl crate::RegisterSpec for I2cFilterThr014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr014::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr014Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr014::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR014 to value 0"]
impl crate::Resettable for I2cFilterThr014Spec {}
