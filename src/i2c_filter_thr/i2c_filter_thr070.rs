#[doc = "Register `I2C_FILTER_THR070` reader"]
pub type R = crate::R<I2cFilterThr070Spec>;
#[doc = "Register `I2C_FILTER_THR070` writer"]
pub type W = crate::W<I2cFilterThr070Spec>;
#[doc = "Field `ADDRHI` reader - ADDR_HI"]
pub type AddrhiR = crate::FieldReader;
#[doc = "Field `ADDRHI` writer - ADDR_HI"]
pub type AddrhiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ADDR_HI"]
    #[inline(always)]
    pub fn addrhi(&self) -> AddrhiR {
        AddrhiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADDR_HI"]
    #[inline(always)]
    pub fn addrhi(&mut self) -> AddrhiW<I2cFilterThr070Spec> {
        AddrhiW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THR0\\_ADR\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr070Spec;
impl crate::RegisterSpec for I2cFilterThr070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr070::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr070Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr070::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR070 to value 0"]
impl crate::Resettable for I2cFilterThr070Spec {}
