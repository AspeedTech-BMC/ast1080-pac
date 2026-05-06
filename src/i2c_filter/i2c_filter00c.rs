#[doc = "Register `I2C_FILTER00C` reader"]
pub type R = crate::R<I2cFilter00cSpec>;
#[doc = "Register `I2C_FILTER00C` writer"]
pub type W = crate::W<I2cFilter00cSpec>;
#[doc = "Field `TOPIRQSTA` reader - TOP_IRQSTA"]
pub type TopirqstaR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TOP_IRQSTA"]
    #[inline(always)]
    pub fn topirqsta(&self) -> TopirqstaR {
        TopirqstaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "I2CFLT\\_IRQSTA\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilter00cSpec;
impl crate::RegisterSpec for I2cFilter00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter00c::R`](R) reader structure"]
impl crate::Readable for I2cFilter00cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter00c::W`](W) writer structure"]
impl crate::Writable for I2cFilter00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER00C to value 0"]
impl crate::Resettable for I2cFilter00cSpec {}
