#[doc = "Register `VIC030` reader"]
pub type R = crate::R<Vic030Spec>;
#[doc = "Register `VIC030` writer"]
pub type W = crate::W<Vic030Spec>;
#[doc = "Field `VICENPSPSWINTTOMCU` reader - VIC_EN_PSP_SW_INT_TO_MCU"]
pub type VicenpspswinttomcuR = crate::FieldReader;
#[doc = "Field `VICENPSPSWINTTOMCU` writer - VIC_EN_PSP_SW_INT_TO_MCU"]
pub type VicenpspswinttomcuW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VICENPSPRSTINTTOMCU` reader - VIC_EN_PSP_RST_INT_TO_MCU"]
pub type VicenpsprstinttomcuR = crate::BitReader;
#[doc = "Field `VICENPSPRSTINTTOMCU` writer - VIC_EN_PSP_RST_INT_TO_MCU"]
pub type VicenpsprstinttomcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VICENCaliptraSSMCUSWINTTOMCU` reader - VIC_EN_Caliptra SSMCU_SW_INT_TO_MCU"]
pub type VicencaliptraSsmcuswinttomcuR = crate::FieldReader;
#[doc = "Field `VICENCaliptraSSMCUSWINTTOMCU` writer - VIC_EN_Caliptra SSMCU_SW_INT_TO_MCU"]
pub type VicencaliptraSsmcuswinttomcuW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VICENCaliptraSSMCURSTINTTOMCU` reader - VIC_EN_Caliptra SSMCU_RST_INT_TO_MCU"]
pub type VicencaliptraSsmcurstinttomcuR = crate::BitReader;
#[doc = "Field `VICENCaliptraSSMCURSTINTTOMCU` writer - VIC_EN_Caliptra SSMCU_RST_INT_TO_MCU"]
pub type VicencaliptraSsmcurstinttomcuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - VIC_EN_PSP_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicenpspswinttomcu(&self) -> VicenpspswinttomcuR {
        VicenpspswinttomcuR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - VIC_EN_PSP_RST_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicenpsprstinttomcu(&self) -> VicenpsprstinttomcuR {
        VicenpsprstinttomcuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - VIC_EN_Caliptra SSMCU_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicencaliptra_ssmcuswinttomcu(&self) -> VicencaliptraSsmcuswinttomcuR {
        VicencaliptraSsmcuswinttomcuR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - VIC_EN_Caliptra SSMCU_RST_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicencaliptra_ssmcurstinttomcu(&self) -> VicencaliptraSsmcurstinttomcuR {
        VicencaliptraSsmcurstinttomcuR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - VIC_EN_PSP_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicenpspswinttomcu(&mut self) -> VicenpspswinttomcuW<Vic030Spec> {
        VicenpspswinttomcuW::new(self, 0)
    }
    #[doc = "Bit 7 - VIC_EN_PSP_RST_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicenpsprstinttomcu(&mut self) -> VicenpsprstinttomcuW<Vic030Spec> {
        VicenpsprstinttomcuW::new(self, 7)
    }
    #[doc = "Bits 8:14 - VIC_EN_Caliptra SSMCU_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicencaliptra_ssmcuswinttomcu(&mut self) -> VicencaliptraSsmcuswinttomcuW<Vic030Spec> {
        VicencaliptraSsmcuswinttomcuW::new(self, 8)
    }
    #[doc = "Bit 15 - VIC_EN_Caliptra SSMCU_RST_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicencaliptra_ssmcurstinttomcu(&mut self) -> VicencaliptraSsmcurstinttomcuW<Vic030Spec> {
        VicencaliptraSsmcurstinttomcuW::new(self, 15)
    }
}
#[doc = "MCU Software Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vic030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic030Spec;
impl crate::RegisterSpec for Vic030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic030::R`](R) reader structure"]
impl crate::Readable for Vic030Spec {}
#[doc = "`write(|w| ..)` method takes [`vic030::W`](W) writer structure"]
impl crate::Writable for Vic030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC030 to value 0"]
impl crate::Resettable for Vic030Spec {}
