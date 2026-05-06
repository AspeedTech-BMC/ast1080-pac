#[doc = "Register `I2C_FILTER_THR018` reader"]
pub type R = crate::R<I2cFilterThr018Spec>;
#[doc = "Register `I2C_FILTER_THR018` writer"]
pub type W = crate::W<I2cFilterThr018Spec>;
#[doc = "Field `INTS` reader - INTS"]
pub type IntsR = crate::FieldReader;
#[doc = "Field `INTS` writer - INTS"]
pub type IntsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - INTS"]
    #[inline(always)]
    pub fn ints(&self) -> IntsR {
        IntsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - INTS"]
    #[inline(always)]
    pub fn ints(&mut self) -> IntsW<I2cFilterThr018Spec> {
        IntsW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_INTS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr018Spec;
impl crate::RegisterSpec for I2cFilterThr018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr018::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr018Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr018::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR018 to value 0"]
impl crate::Resettable for I2cFilterThr018Spec {}
