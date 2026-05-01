#[doc = "Register `I2C68` reader"]
pub type R = crate::R<I2c68Spec>;
#[doc = "Register `I2C68` writer"]
pub type W = crate::W<I2c68Spec>;
#[doc = "Field `STXAHI` reader - STXA_HI"]
pub type StxahiR = crate::FieldReader;
#[doc = "Field `STXAHI` writer - STXA_HI"]
pub type StxahiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - STXA_HI"]
    #[inline(always)]
    pub fn stxahi(&self) -> StxahiR {
        StxahiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STXA_HI"]
    #[inline(always)]
    pub fn stxahi(&mut self) -> StxahiW<I2c68Spec> {
        StxahiW::new(self, 0)
    }
}
#[doc = "Slave DMA Mode Tx Buffer Base Address\\[39:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c68::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c68::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c68Spec;
impl crate::RegisterSpec for I2c68Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c68::R`](R) reader structure"]
impl crate::Readable for I2c68Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c68::W`](W) writer structure"]
impl crate::Writable for I2c68Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C68 to value 0"]
impl crate::Resettable for I2c68Spec {}
