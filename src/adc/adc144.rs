#[doc = "Register `ADC144` reader"]
pub type R = crate::R<Adc144Spec>;
#[doc = "Register `ADC144` writer"]
pub type W = crate::W<Adc144Spec>;
#[doc = "Field `LowerBound` reader - Lower bound"]
pub type LowerBoundR = crate::FieldReader<u16>;
#[doc = "Field `LowerBound` writer - Lower bound"]
pub type LowerBoundW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `UpperBound` reader - Upper bound"]
pub type UpperBoundR = crate::FieldReader<u16>;
#[doc = "Field `UpperBound` writer - Upper bound"]
pub type UpperBoundW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Lower bound"]
    #[inline(always)]
    pub fn lower_bound(&self) -> LowerBoundR {
        LowerBoundR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Upper bound"]
    #[inline(always)]
    pub fn upper_bound(&self) -> UpperBoundR {
        UpperBoundR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lower bound"]
    #[inline(always)]
    pub fn lower_bound(&mut self) -> LowerBoundW<Adc144Spec> {
        LowerBoundW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Upper bound"]
    #[inline(always)]
    pub fn upper_bound(&mut self) -> UpperBoundW<Adc144Spec> {
        UpperBoundW::new(self, 16)
    }
}
#[doc = "Upper and Lower bound of Channel 13\n\nYou can [`read`](crate::Reg::read) this register and get [`adc144::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc144::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc144Spec;
impl crate::RegisterSpec for Adc144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc144::R`](R) reader structure"]
impl crate::Readable for Adc144Spec {}
#[doc = "`write(|w| ..)` method takes [`adc144::W`](W) writer structure"]
impl crate::Writable for Adc144Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC144 to value 0"]
impl crate::Resettable for Adc144Spec {}
