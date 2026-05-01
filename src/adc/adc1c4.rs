#[doc = "Register `ADC1C4` reader"]
pub type R = crate::R<Adc1c4Spec>;
#[doc = "Register `ADC1C4` writer"]
pub type W = crate::W<Adc1c4Spec>;
#[doc = "Field `TrimmingValue` reader - Trimming value."]
pub type TrimmingValueR = crate::FieldReader;
#[doc = "Field `TrimmingValue` writer - Trimming value."]
pub type TrimmingValueW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `CompensatingValue` reader - Compensating value."]
pub type CompensatingValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Trimming value."]
    #[inline(always)]
    pub fn trimming_value(&self) -> TrimmingValueR {
        TrimmingValueR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - Compensating value."]
    #[inline(always)]
    pub fn compensating_value(&self) -> CompensatingValueR {
        CompensatingValueR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trimming value."]
    #[inline(always)]
    pub fn trimming_value(&mut self) -> TrimmingValueW<Adc1c4Spec> {
        TrimmingValueW::new(self, 0)
    }
}
#[doc = "Compensating and Trimming\n\nYou can [`read`](crate::Reg::read) this register and get [`adc1c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc1c4Spec;
impl crate::RegisterSpec for Adc1c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc1c4::R`](R) reader structure"]
impl crate::Readable for Adc1c4Spec {}
#[doc = "`write(|w| ..)` method takes [`adc1c4::W`](W) writer structure"]
impl crate::Writable for Adc1c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC1C4 to value 0"]
impl crate::Resettable for Adc1c4Spec {}
