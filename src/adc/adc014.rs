#[doc = "Register `ADC014` reader"]
pub type R = crate::R<Adc014Spec>;
#[doc = "Register `ADC014` writer"]
pub type W = crate::W<Adc014Spec>;
#[doc = "Field `DataOfChannel2` reader - Data of Channel 2"]
pub type DataOfChannel2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DataOfChannel3` reader - Data of Channel 3"]
pub type DataOfChannel3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data of Channel 2"]
    #[inline(always)]
    pub fn data_of_channel2(&self) -> DataOfChannel2R {
        DataOfChannel2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Data of Channel 3"]
    #[inline(always)]
    pub fn data_of_channel3(&self) -> DataOfChannel3R {
        DataOfChannel3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {}
#[doc = "Data of Channel 3 and 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc014Spec;
impl crate::RegisterSpec for Adc014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc014::R`](R) reader structure"]
impl crate::Readable for Adc014Spec {}
#[doc = "`write(|w| ..)` method takes [`adc014::W`](W) writer structure"]
impl crate::Writable for Adc014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC014 to value 0"]
impl crate::Resettable for Adc014Spec {}
