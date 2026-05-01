#[doc = "Register `ADC010` reader"]
pub type R = crate::R<Adc010Spec>;
#[doc = "Register `ADC010` writer"]
pub type W = crate::W<Adc010Spec>;
#[doc = "Field `DataOfChannel0` reader - Data of Channel 0"]
pub type DataOfChannel0R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DataOfChannel1` reader - Data of Channel 1"]
pub type DataOfChannel1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data of Channel 0"]
    #[inline(always)]
    pub fn data_of_channel0(&self) -> DataOfChannel0R {
        DataOfChannel0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Data of Channel 1"]
    #[inline(always)]
    pub fn data_of_channel1(&self) -> DataOfChannel1R {
        DataOfChannel1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {}
#[doc = "Data of Channel 1 and 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc010Spec;
impl crate::RegisterSpec for Adc010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc010::R`](R) reader structure"]
impl crate::Readable for Adc010Spec {}
#[doc = "`write(|w| ..)` method takes [`adc010::W`](W) writer structure"]
impl crate::Writable for Adc010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC010 to value 0"]
impl crate::Resettable for Adc010Spec {}
