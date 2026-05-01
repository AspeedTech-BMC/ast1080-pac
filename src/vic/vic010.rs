#[doc = "Register `VIC010` reader"]
pub type R = crate::R<Vic010Spec>;
#[doc = "Register `VIC010` writer"]
pub type W = crate::W<Vic010Spec>;
#[doc = "Field `VICENCaliptraSSMCUSWINTTOPSP` reader - VIC_EN_Caliptra SSMCU_SW_INT_TO_PSP"]
pub type VicencaliptraSsmcuswinttopspR = crate::FieldReader;
#[doc = "Field `VICENCaliptraSSMCUSWINTTOPSP` writer - VIC_EN_Caliptra SSMCU_SW_INT_TO_PSP"]
pub type VicencaliptraSsmcuswinttopspW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VICENCaliptraSSMCURSTINTTOPSP` reader - VIC_EN_Caliptra SSMCU_RST_INT_TO_PSP"]
pub type VicencaliptraSsmcurstinttopspR = crate::BitReader;
#[doc = "Field `VICENCaliptraSSMCURSTINTTOPSP` writer - VIC_EN_Caliptra SSMCU_RST_INT_TO_PSP"]
pub type VicencaliptraSsmcurstinttopspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VICENMCUSWINTTOPSP` reader - VIC_EN_MCU_SW_INT_TO_PSP"]
pub type VicenmcuswinttopspR = crate::FieldReader;
#[doc = "Field `VICENMCUSWINTTOPSP` writer - VIC_EN_MCU_SW_INT_TO_PSP"]
pub type VicenmcuswinttopspW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VICENMCURSTINTTOPSP` reader - VIC_EN_MCU_RST_INT_TO_PSP"]
pub type VicenmcurstinttopspR = crate::BitReader;
#[doc = "Field `VICENMCURSTINTTOPSP` writer - VIC_EN_MCU_RST_INT_TO_PSP"]
pub type VicenmcurstinttopspW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - VIC_EN_Caliptra SSMCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicencaliptra_ssmcuswinttopsp(&self) -> VicencaliptraSsmcuswinttopspR {
        VicencaliptraSsmcuswinttopspR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - VIC_EN_Caliptra SSMCU_RST_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicencaliptra_ssmcurstinttopsp(&self) -> VicencaliptraSsmcurstinttopspR {
        VicencaliptraSsmcurstinttopspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - VIC_EN_MCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicenmcuswinttopsp(&self) -> VicenmcuswinttopspR {
        VicenmcuswinttopspR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - VIC_EN_MCU_RST_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicenmcurstinttopsp(&self) -> VicenmcurstinttopspR {
        VicenmcurstinttopspR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - VIC_EN_Caliptra SSMCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicencaliptra_ssmcuswinttopsp(&mut self) -> VicencaliptraSsmcuswinttopspW<Vic010Spec> {
        VicencaliptraSsmcuswinttopspW::new(self, 0)
    }
    #[doc = "Bit 7 - VIC_EN_Caliptra SSMCU_RST_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicencaliptra_ssmcurstinttopsp(&mut self) -> VicencaliptraSsmcurstinttopspW<Vic010Spec> {
        VicencaliptraSsmcurstinttopspW::new(self, 7)
    }
    #[doc = "Bits 8:14 - VIC_EN_MCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicenmcuswinttopsp(&mut self) -> VicenmcuswinttopspW<Vic010Spec> {
        VicenmcuswinttopspW::new(self, 8)
    }
    #[doc = "Bit 15 - VIC_EN_MCU_RST_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicenmcurstinttopsp(&mut self) -> VicenmcurstinttopspW<Vic010Spec> {
        VicenmcurstinttopspW::new(self, 15)
    }
}
#[doc = "PSP Software Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vic010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic010Spec;
impl crate::RegisterSpec for Vic010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic010::R`](R) reader structure"]
impl crate::Readable for Vic010Spec {}
#[doc = "`write(|w| ..)` method takes [`vic010::W`](W) writer structure"]
impl crate::Writable for Vic010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC010 to value 0"]
impl crate::Resettable for Vic010Spec {}
