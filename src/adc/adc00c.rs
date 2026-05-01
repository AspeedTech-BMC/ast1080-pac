#[doc = "Register `ADC00C` reader"]
pub type R = crate::R<Adc00cSpec>;
#[doc = "Register `ADC00C` writer"]
pub type W = crate::W<Adc00cSpec>;
#[doc = "Field `DivisorOfADCClk` reader - Divisor of ADC clock"]
pub type DivisorOfAdcclkR = crate::FieldReader<u16>;
#[doc = "Field `DivisorOfADCClk` writer - Divisor of ADC clock"]
pub type DivisorOfAdcclkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Divisor of ADC clock"]
    #[inline(always)]
    pub fn divisor_of_adcclk(&self) -> DivisorOfAdcclkR {
        DivisorOfAdcclkR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Divisor of ADC clock"]
    #[inline(always)]
    pub fn divisor_of_adcclk(&mut self) -> DivisorOfAdcclkW<Adc00cSpec> {
        DivisorOfAdcclkW::new(self, 0)
    }
}
#[doc = "ADC Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc00cSpec;
impl crate::RegisterSpec for Adc00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc00c::R`](R) reader structure"]
impl crate::Readable for Adc00cSpec {}
#[doc = "`write(|w| ..)` method takes [`adc00c::W`](W) writer structure"]
impl crate::Writable for Adc00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC00C to value 0x0f"]
impl crate::Resettable for Adc00cSpec {
    const RESET_VALUE: u32 = 0x0f;
}
