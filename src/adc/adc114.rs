#[doc = "Register `ADC114` reader"]
pub type R = crate::R<Adc114Spec>;
#[doc = "Register `ADC114` writer"]
pub type W = crate::W<Adc114Spec>;
#[doc = "Field `DataOfChannel10` reader - Data of Channel 10"]
pub type DataOfChannel10R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DataOfChannel11` reader - Data of Channel 11"]
pub type DataOfChannel11R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data of Channel 10"]
    #[inline(always)]
    pub fn data_of_channel10(&self) -> DataOfChannel10R {
        DataOfChannel10R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Data of Channel 11"]
    #[inline(always)]
    pub fn data_of_channel11(&self) -> DataOfChannel11R {
        DataOfChannel11R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {}
#[doc = "Data of Channel 11 and 10\n\nYou can [`read`](crate::Reg::read) this register and get [`adc114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc114Spec;
impl crate::RegisterSpec for Adc114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc114::R`](R) reader structure"]
impl crate::Readable for Adc114Spec {}
#[doc = "`write(|w| ..)` method takes [`adc114::W`](W) writer structure"]
impl crate::Writable for Adc114Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC114 to value 0"]
impl crate::Resettable for Adc114Spec {}
