#[doc = "Register `ADC11C` reader"]
pub type R = crate::R<Adc11cSpec>;
#[doc = "Register `ADC11C` writer"]
pub type W = crate::W<Adc11cSpec>;
#[doc = "Field `DataOfChannel14` reader - Data of Channel 14"]
pub type DataOfChannel14R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DataOfChannel15` reader - Data of Channel 15"]
pub type DataOfChannel15R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data of Channel 14"]
    #[inline(always)]
    pub fn data_of_channel14(&self) -> DataOfChannel14R {
        DataOfChannel14R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Data of Channel 15"]
    #[inline(always)]
    pub fn data_of_channel15(&self) -> DataOfChannel15R {
        DataOfChannel15R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {}
#[doc = "Data of Channel 15 and 14\n\nYou can [`read`](crate::Reg::read) this register and get [`adc11c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc11c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc11cSpec;
impl crate::RegisterSpec for Adc11cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc11c::R`](R) reader structure"]
impl crate::Readable for Adc11cSpec {}
#[doc = "`write(|w| ..)` method takes [`adc11c::W`](W) writer structure"]
impl crate::Writable for Adc11cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC11C to value 0"]
impl crate::Resettable for Adc11cSpec {}
