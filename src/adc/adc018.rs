#[doc = "Register `ADC018` reader"]
pub type R = crate::R<Adc018Spec>;
#[doc = "Register `ADC018` writer"]
pub type W = crate::W<Adc018Spec>;
#[doc = "Field `DataOfChannel4` reader - Data of Channel 4"]
pub type DataOfChannel4R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DataOfChannel5` reader - Data of Channel 5"]
pub type DataOfChannel5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data of Channel 4"]
    #[inline(always)]
    pub fn data_of_channel4(&self) -> DataOfChannel4R {
        DataOfChannel4R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Data of Channel 5"]
    #[inline(always)]
    pub fn data_of_channel5(&self) -> DataOfChannel5R {
        DataOfChannel5R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {}
#[doc = "Data of Channel 5 and 4\n\nYou can [`read`](crate::Reg::read) this register and get [`adc018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc018Spec;
impl crate::RegisterSpec for Adc018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc018::R`](R) reader structure"]
impl crate::Readable for Adc018Spec {}
#[doc = "`write(|w| ..)` method takes [`adc018::W`](W) writer structure"]
impl crate::Writable for Adc018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC018 to value 0"]
impl crate::Resettable for Adc018Spec {}
