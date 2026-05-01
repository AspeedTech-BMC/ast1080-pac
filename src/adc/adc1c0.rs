#[doc = "Register `ADC1C0` reader"]
pub type R = crate::R<Adc1c0Spec>;
#[doc = "Register `ADC1C0` writer"]
pub type W = crate::W<Adc1c0Spec>;
#[doc = "Field `INTSource` reader - Interrupt Source."]
pub type IntsourceR = crate::FieldReader;
#[doc = "Field `INTSource` writer - Interrupt Source."]
pub type IntsourceW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Source."]
    #[inline(always)]
    pub fn intsource(&self) -> IntsourceR {
        IntsourceR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Source."]
    #[inline(always)]
    pub fn intsource(&mut self) -> IntsourceW<Adc1c0Spec> {
        IntsourceW::new(self, 0)
    }
}
#[doc = "Interrupt Source\n\nYou can [`read`](crate::Reg::read) this register and get [`adc1c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1c0Spec;
impl crate::RegisterSpec for Adc1c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc1c0::R`](R) reader structure"]
impl crate::Readable for Adc1c0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc1c0::W`](W) writer structure"]
impl crate::Writable for Adc1c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC1C0 to value 0"]
impl crate::Resettable for Adc1c0Spec {}
