#[doc = "Register `VIC034` reader"]
pub type R = crate::R<Vic034Spec>;
#[doc = "Register `VIC034` writer"]
pub type W = crate::W<Vic034Spec>;
#[doc = "Field `VICPSPSWINTTOMCU` reader - VIC_PSP_SW_INT_TO_MCU"]
pub type VicpspswinttomcuR = crate::FieldReader;
#[doc = "Field `VICPSPRSTINTTOMCU` reader - VIC_PSP_RST_INT_TO_MCU"]
pub type VicpsprstinttomcuR = crate::BitReader;
#[doc = "Field `VICCaliptraSSMCUSWINTTOMCU` reader - VIC_Caliptra SSMCU_SW_INT_TO_MCU"]
pub type ViccaliptraSsmcuswinttomcuR = crate::FieldReader;
#[doc = "Field `VICCaliptraSSMCURSTINTTOMCU` reader - VIC_Caliptra SSMCU_RST_INT_TO_MCU"]
pub type ViccaliptraSsmcurstinttomcuR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - VIC_PSP_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicpspswinttomcu(&self) -> VicpspswinttomcuR {
        VicpspswinttomcuR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - VIC_PSP_RST_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicpsprstinttomcu(&self) -> VicpsprstinttomcuR {
        VicpsprstinttomcuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - VIC_Caliptra SSMCU_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn viccaliptra_ssmcuswinttomcu(&self) -> ViccaliptraSsmcuswinttomcuR {
        ViccaliptraSsmcuswinttomcuR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - VIC_Caliptra SSMCU_RST_INT_TO_MCU"]
    #[inline(always)]
    pub fn viccaliptra_ssmcurstinttomcu(&self) -> ViccaliptraSsmcurstinttomcuR {
        ViccaliptraSsmcurstinttomcuR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "MCU Software Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`vic034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic034Spec;
impl crate::RegisterSpec for Vic034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic034::R`](R) reader structure"]
impl crate::Readable for Vic034Spec {}
#[doc = "`write(|w| ..)` method takes [`vic034::W`](W) writer structure"]
impl crate::Writable for Vic034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC034 to value 0"]
impl crate::Resettable for Vic034Spec {}
