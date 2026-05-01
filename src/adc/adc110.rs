#[doc = "Register `ADC110` reader"]
pub type R = crate::R<Adc110Spec>;
#[doc = "Register `ADC110` writer"]
pub type W = crate::W<Adc110Spec>;
#[doc = "Field `DataOfChannel8` reader - Data of Channel 8"]
pub type DataOfChannel8R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DataOfChannel9` reader - Data of Channel 9"]
pub type DataOfChannel9R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data of Channel 8"]
    #[inline(always)]
    pub fn data_of_channel8(&self) -> DataOfChannel8R {
        DataOfChannel8R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Data of Channel 9"]
    #[inline(always)]
    pub fn data_of_channel9(&self) -> DataOfChannel9R {
        DataOfChannel9R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {}
#[doc = "Data of Channel 9 and 8\n\nYou can [`read`](crate::Reg::read) this register and get [`adc110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc110Spec;
impl crate::RegisterSpec for Adc110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc110::R`](R) reader structure"]
impl crate::Readable for Adc110Spec {}
#[doc = "`write(|w| ..)` method takes [`adc110::W`](W) writer structure"]
impl crate::Writable for Adc110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC110 to value 0"]
impl crate::Resettable for Adc110Spec {}
