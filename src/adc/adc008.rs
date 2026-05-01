#[doc = "Register `ADC008` reader"]
pub type R = crate::R<Adc008Spec>;
#[doc = "Register `ADC008` writer"]
pub type W = crate::W<Adc008Spec>;
#[doc = "Field `DivisorOfADCClkUsedForVGADet` reader - Divisor of ADC clock used for VGA detection"]
pub type DivisorOfAdcclkUsedForVgadetR = crate::FieldReader<u16>;
#[doc = "Field `DivisorOfADCClkUsedForVGADet` writer - Divisor of ADC clock used for VGA detection"]
pub type DivisorOfAdcclkUsedForVgadetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADCVGADetectEnbl` reader - ADC VGA detect enable"]
pub type AdcvgadetectEnblR = crate::BitReader;
#[doc = "Field `ADCVGADetectEnbl` writer - ADC VGA detect enable"]
pub type AdcvgadetectEnblW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Divisor of ADC clock used for VGA detection"]
    #[inline(always)]
    pub fn divisor_of_adcclk_used_for_vgadet(&self) -> DivisorOfAdcclkUsedForVgadetR {
        DivisorOfAdcclkUsedForVgadetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - ADC VGA detect enable"]
    #[inline(always)]
    pub fn adcvgadetect_enbl(&self) -> AdcvgadetectEnblR {
        AdcvgadetectEnblR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Divisor of ADC clock used for VGA detection"]
    #[inline(always)]
    pub fn divisor_of_adcclk_used_for_vgadet(
        &mut self,
    ) -> DivisorOfAdcclkUsedForVgadetW<Adc008Spec> {
        DivisorOfAdcclkUsedForVgadetW::new(self, 0)
    }
    #[doc = "Bit 16 - ADC VGA detect enable"]
    #[inline(always)]
    pub fn adcvgadetect_enbl(&mut self) -> AdcvgadetectEnblW<Adc008Spec> {
        AdcvgadetectEnblW::new(self, 16)
    }
}
#[doc = "ADC VGA Detect Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc008Spec;
impl crate::RegisterSpec for Adc008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc008::R`](R) reader structure"]
impl crate::Readable for Adc008Spec {}
#[doc = "`write(|w| ..)` method takes [`adc008::W`](W) writer structure"]
impl crate::Writable for Adc008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC008 to value 0x0f"]
impl crate::Resettable for Adc008Spec {
    const RESET_VALUE: u32 = 0x0f;
}
