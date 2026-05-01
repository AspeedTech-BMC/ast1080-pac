#[doc = "Register `ADC104` reader"]
pub type R = crate::R<Adc104Spec>;
#[doc = "Register `ADC104` writer"]
pub type W = crate::W<Adc104Spec>;
#[doc = "Field `INTSts` reader - Interrupt status"]
pub type IntstsR = crate::FieldReader;
#[doc = "Field `INTSts` writer - Interrupt status"]
pub type IntstsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `INTEnbl` reader - Interrupt enable"]
pub type IntenblR = crate::FieldReader;
#[doc = "Field `INTEnbl` writer - Interrupt enable"]
pub type IntenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt status"]
    #[inline(always)]
    pub fn intsts(&self) -> IntstsR {
        IntstsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl(&self) -> IntenblR {
        IntenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt status"]
    #[inline(always)]
    pub fn intsts(&mut self) -> IntstsW<Adc104Spec> {
        IntstsW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl(&mut self) -> IntenblW<Adc104Spec> {
        IntenblW::new(self, 16)
    }
}
#[doc = "Interrupt Enable and Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`adc104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc104Spec;
impl crate::RegisterSpec for Adc104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc104::R`](R) reader structure"]
impl crate::Readable for Adc104Spec {}
#[doc = "`write(|w| ..)` method takes [`adc104::W`](W) writer structure"]
impl crate::Writable for Adc104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC104 to value 0"]
impl crate::Resettable for Adc104Spec {}
