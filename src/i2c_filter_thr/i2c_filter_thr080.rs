#[doc = "Register `I2C_FILTER_THR080` reader"]
pub type R = crate::R<I2cFilterThr080Spec>;
#[doc = "Register `I2C_FILTER_THR080` writer"]
pub type W = crate::W<I2cFilterThr080Spec>;
#[doc = "Field `ELOG00` reader - ELOG00"]
pub type Elog00R = crate::FieldReader<u32>;
#[doc = "Field `ELOG00` writer - ELOG00"]
pub type Elog00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG00"]
    #[inline(always)]
    pub fn elog00(&self) -> Elog00R {
        Elog00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG00"]
    #[inline(always)]
    pub fn elog00(&mut self) -> Elog00W<I2cFilterThr080Spec> {
        Elog00W::new(self, 0)
    }
}
#[doc = "I2CF\\_ELOG00\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr080Spec;
impl crate::RegisterSpec for I2cFilterThr080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr080::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr080Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr080::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR080 to value 0"]
impl crate::Resettable for I2cFilterThr080Spec {}
