#[doc = "Register `ADC13C` reader"]
pub type R = crate::R<Adc13cSpec>;
#[doc = "Register `ADC13C` writer"]
pub type W = crate::W<Adc13cSpec>;
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
    pub fn lower_bound(&mut self) -> LowerBoundW<Adc13cSpec> {
        LowerBoundW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Upper bound"]
    #[inline(always)]
    pub fn upper_bound(&mut self) -> UpperBoundW<Adc13cSpec> {
        UpperBoundW::new(self, 16)
    }
}
#[doc = "Upper and Lower bound of Channel 11\n\nYou can [`read`](crate::Reg::read) this register and get [`adc13c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc13c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc13cSpec;
impl crate::RegisterSpec for Adc13cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc13c::R`](R) reader structure"]
impl crate::Readable for Adc13cSpec {}
#[doc = "`write(|w| ..)` method takes [`adc13c::W`](W) writer structure"]
impl crate::Writable for Adc13cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC13C to value 0"]
impl crate::Resettable for Adc13cSpec {}
