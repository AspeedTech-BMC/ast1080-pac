#[doc = "Register `VIC028` reader"]
pub type R = crate::R<Vic028Spec>;
#[doc = "Register `VIC028` writer"]
pub type W = crate::W<Vic028Spec>;
#[doc = "Field `VICCaliptraSSMCUSWINTTOPSP` reader - VIC_Caliptra SSMCU_SW_INT_TO_PSP"]
pub type ViccaliptraSsmcuswinttopspR = crate::FieldReader;
#[doc = "Field `VICCaliptraSSMCUSWINTTOPSP` writer - VIC_Caliptra SSMCU_SW_INT_TO_PSP"]
pub type ViccaliptraSsmcuswinttopspW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `VICCaliptraSSMCUSWINTTOMCU` reader - VIC_Caliptra SSMCU_SW_INT_TO_MCU"]
pub type ViccaliptraSsmcuswinttomcuR = crate::FieldReader;
#[doc = "Field `VICCaliptraSSMCUSWINTTOMCU` writer - VIC_Caliptra SSMCU_SW_INT_TO_MCU"]
pub type ViccaliptraSsmcuswinttomcuW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - VIC_Caliptra SSMCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn viccaliptra_ssmcuswinttopsp(&self) -> ViccaliptraSsmcuswinttopspR {
        ViccaliptraSsmcuswinttopspR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - VIC_Caliptra SSMCU_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn viccaliptra_ssmcuswinttomcu(&self) -> ViccaliptraSsmcuswinttomcuR {
        ViccaliptraSsmcuswinttomcuR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - VIC_Caliptra SSMCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn viccaliptra_ssmcuswinttopsp(&mut self) -> ViccaliptraSsmcuswinttopspW<Vic028Spec> {
        ViccaliptraSsmcuswinttopspW::new(self, 0)
    }
    #[doc = "Bits 8:14 - VIC_Caliptra SSMCU_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn viccaliptra_ssmcuswinttomcu(&mut self) -> ViccaliptraSsmcuswinttomcuW<Vic028Spec> {
        ViccaliptraSsmcuswinttomcuW::new(self, 8)
    }
}
#[doc = "Caliptra SSMCU Software Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`vic028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic028Spec;
impl crate::RegisterSpec for Vic028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic028::R`](R) reader structure"]
impl crate::Readable for Vic028Spec {}
#[doc = "`write(|w| ..)` method takes [`vic028::W`](W) writer structure"]
impl crate::Writable for Vic028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC028 to value 0"]
impl crate::Resettable for Vic028Spec {}
