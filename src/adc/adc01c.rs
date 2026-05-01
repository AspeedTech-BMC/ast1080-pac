#[doc = "Register `ADC01C` reader"]
pub type R = crate::R<Adc01cSpec>;
#[doc = "Register `ADC01C` writer"]
pub type W = crate::W<Adc01cSpec>;
#[doc = "Field `DataOfChannel6` reader - Data of Channel 6"]
pub type DataOfChannel6R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DataOfChannel7` reader - Data of Channel 7"]
pub type DataOfChannel7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data of Channel 6"]
    #[inline(always)]
    pub fn data_of_channel6(&self) -> DataOfChannel6R {
        DataOfChannel6R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Data of Channel 7"]
    #[inline(always)]
    pub fn data_of_channel7(&self) -> DataOfChannel7R {
        DataOfChannel7R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {}
#[doc = "Data of Channel 7 and 6\n\nYou can [`read`](crate::Reg::read) this register and get [`adc01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc01cSpec;
impl crate::RegisterSpec for Adc01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc01c::R`](R) reader structure"]
impl crate::Readable for Adc01cSpec {}
#[doc = "`write(|w| ..)` method takes [`adc01c::W`](W) writer structure"]
impl crate::Writable for Adc01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC01C to value 0"]
impl crate::Resettable for Adc01cSpec {}
