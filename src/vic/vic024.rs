#[doc = "Register `VIC024` reader"]
pub type R = crate::R<Vic024Spec>;
#[doc = "Register `VIC024` writer"]
pub type W = crate::W<Vic024Spec>;
#[doc = "Field `VICPSPSWINTTOSSMCU` reader - VIC_PSP_SW_INT_TO_SSMCU"]
pub type VicpspswinttossmcuR = crate::FieldReader;
#[doc = "Field `VICPSPRSTINTTOSSMCU` reader - VIC_PSP_RST_INT_TO_SSMCU"]
pub type VicpsprstinttossmcuR = crate::BitReader;
#[doc = "Field `VICMCUSWINTTOSSMCU` reader - VIC_MCU_SW_INT_TO_SSMCU"]
pub type VicmcuswinttossmcuR = crate::FieldReader;
#[doc = "Field `VICMCURSTINTTOSSMCU` reader - VIC_MCU_RST_INT_TO_SSMCU"]
pub type VicmcurstinttossmcuR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - VIC_PSP_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicpspswinttossmcu(&self) -> VicpspswinttossmcuR {
        VicpspswinttossmcuR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - VIC_PSP_RST_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicpsprstinttossmcu(&self) -> VicpsprstinttossmcuR {
        VicpsprstinttossmcuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - VIC_MCU_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicmcuswinttossmcu(&self) -> VicmcuswinttossmcuR {
        VicmcuswinttossmcuR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - VIC_MCU_RST_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicmcurstinttossmcu(&self) -> VicmcurstinttossmcuR {
        VicmcurstinttossmcuR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "Caliptra SSMCU Software Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`vic024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic024Spec;
impl crate::RegisterSpec for Vic024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic024::R`](R) reader structure"]
impl crate::Readable for Vic024Spec {}
#[doc = "`write(|w| ..)` method takes [`vic024::W`](W) writer structure"]
impl crate::Writable for Vic024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC024 to value 0"]
impl crate::Resettable for Vic024Spec {}
