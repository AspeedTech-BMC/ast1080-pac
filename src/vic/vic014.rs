#[doc = "Register `VIC014` reader"]
pub type R = crate::R<Vic014Spec>;
#[doc = "Register `VIC014` writer"]
pub type W = crate::W<Vic014Spec>;
#[doc = "Field `VICCaliptraSSMCUSWINTTOPSP` reader - VIC_Caliptra SSMCU_SW_INT_TO_PSP"]
pub type ViccaliptraSsmcuswinttopspR = crate::FieldReader;
#[doc = "Field `VICCaliptraSSMCURSTINTTOPSP` reader - VIC_Caliptra SSMCU_RST_INT_TO_PSP"]
pub type ViccaliptraSsmcurstinttopspR = crate::BitReader;
#[doc = "Field `VICMCUSWINTTOPSP` reader - VIC_MCU_SW_INT_TO_PSP"]
pub type VicmcuswinttopspR = crate::FieldReader;
#[doc = "Field `VICMCURSTINTTOPSP` reader - VIC_MCU_RST_INT_TO_PSP"]
pub type VicmcurstinttopspR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - VIC_Caliptra SSMCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn viccaliptra_ssmcuswinttopsp(&self) -> ViccaliptraSsmcuswinttopspR {
        ViccaliptraSsmcuswinttopspR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - VIC_Caliptra SSMCU_RST_INT_TO_PSP"]
    #[inline(always)]
    pub fn viccaliptra_ssmcurstinttopsp(&self) -> ViccaliptraSsmcurstinttopspR {
        ViccaliptraSsmcurstinttopspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - VIC_MCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicmcuswinttopsp(&self) -> VicmcuswinttopspR {
        VicmcuswinttopspR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - VIC_MCU_RST_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicmcurstinttopsp(&self) -> VicmcurstinttopspR {
        VicmcurstinttopspR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "PSP Software Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`vic014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic014Spec;
impl crate::RegisterSpec for Vic014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic014::R`](R) reader structure"]
impl crate::Readable for Vic014Spec {}
#[doc = "`write(|w| ..)` method takes [`vic014::W`](W) writer structure"]
impl crate::Writable for Vic014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC014 to value 0"]
impl crate::Resettable for Vic014Spec {}
