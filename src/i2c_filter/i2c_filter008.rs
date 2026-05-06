#[doc = "Register `I2C_FILTER008` reader"]
pub type R = crate::R<I2cFilter008Spec>;
#[doc = "Register `I2C_FILTER008` writer"]
pub type W = crate::W<I2cFilter008Spec>;
#[doc = "Field `TOPIRQEN` reader - TOP_IRQEN"]
pub type TopirqenR = crate::FieldReader<u16>;
#[doc = "Field `TOPIRQEN` writer - TOP_IRQEN"]
pub type TopirqenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TOP_IRQEN"]
    #[inline(always)]
    pub fn topirqen(&self) -> TopirqenR {
        TopirqenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TOP_IRQEN"]
    #[inline(always)]
    pub fn topirqen(&mut self) -> TopirqenW<I2cFilter008Spec> {
        TopirqenW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_IRQEN\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilter008Spec;
impl crate::RegisterSpec for I2cFilter008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter008::R`](R) reader structure"]
impl crate::Readable for I2cFilter008Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter008::W`](W) writer structure"]
impl crate::Writable for I2cFilter008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER008 to value 0"]
impl crate::Resettable for I2cFilter008Spec {}
