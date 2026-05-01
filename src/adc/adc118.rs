#[doc = "Register `ADC118` reader"]
pub type R = crate::R<Adc118Spec>;
#[doc = "Register `ADC118` writer"]
pub type W = crate::W<Adc118Spec>;
#[doc = "Field `DataOfChannel12` reader - Data of Channel 12"]
pub type DataOfChannel12R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DataOfChannel13` reader - Data of Channel 13"]
pub type DataOfChannel13R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data of Channel 12"]
    #[inline(always)]
    pub fn data_of_channel12(&self) -> DataOfChannel12R {
        DataOfChannel12R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Data of Channel 13"]
    #[inline(always)]
    pub fn data_of_channel13(&self) -> DataOfChannel13R {
        DataOfChannel13R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {}
#[doc = "Data of Channel 13 and 12\n\nYou can [`read`](crate::Reg::read) this register and get [`adc118::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc118::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc118Spec;
impl crate::RegisterSpec for Adc118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc118::R`](R) reader structure"]
impl crate::Readable for Adc118Spec {}
#[doc = "`write(|w| ..)` method takes [`adc118::W`](W) writer structure"]
impl crate::Writable for Adc118Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC118 to value 0"]
impl crate::Resettable for Adc118Spec {}
