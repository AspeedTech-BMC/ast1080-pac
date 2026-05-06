#[doc = "Register `I2CGLOBAL004` reader"]
pub type R = crate::R<I2cglobal004Spec>;
#[doc = "Register `I2CGLOBAL004` writer"]
pub type W = crate::W<I2cglobal004Spec>;
#[doc = "Field `I2CSIRQ` reader - I2C_SIRQ"]
pub type I2csirqR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - I2C_SIRQ"]
    #[inline(always)]
    pub fn i2csirq(&self) -> I2csirqR {
        I2csirqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Device Slave Mode Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cglobal004Spec;
impl crate::RegisterSpec for I2cglobal004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cglobal004::R`](R) reader structure"]
impl crate::Readable for I2cglobal004Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cglobal004::W`](W) writer structure"]
impl crate::Writable for I2cglobal004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CGLOBAL004 to value 0"]
impl crate::Resettable for I2cglobal004Spec {}
