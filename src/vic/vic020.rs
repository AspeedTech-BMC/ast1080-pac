#[doc = "Register `VIC020` reader"]
pub type R = crate::R<Vic020Spec>;
#[doc = "Register `VIC020` writer"]
pub type W = crate::W<Vic020Spec>;
#[doc = "Field `VICENPSPSWINTTOSSMCU` reader - VIC_EN_PSP_SW_INT_TO_SSMCU"]
pub type VicenpspswinttossmcuR = crate::FieldReader;
#[doc = "Field `VICENPSPSWINTTOSSMCU` writer - VIC_EN_PSP_SW_INT_TO_SSMCU"]
pub type VicenpspswinttossmcuW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VICENPSPRSTINTTOSSMCU` reader - VIC_EN_PSP_RST_INT_TO_SSMCU"]
pub type VicenpsprstinttossmcuR = crate::BitReader;
#[doc = "Field `VICENPSPRSTINTTOSSMCU` writer - VIC_EN_PSP_RST_INT_TO_SSMCU"]
pub type VicenpsprstinttossmcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VICENMCUSWINTTOSSMCU` reader - VIC_EN_MCU_SW_INT_TO_SSMCU"]
pub type VicenmcuswinttossmcuR = crate::FieldReader;
#[doc = "Field `VICENMCUSWINTTOSSMCU` writer - VIC_EN_MCU_SW_INT_TO_SSMCU"]
pub type VicenmcuswinttossmcuW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VICENMCURSTINTTOSSMCU` reader - VIC_EN_MCU_RST_INT_TO_SSMCU"]
pub type VicenmcurstinttossmcuR = crate::BitReader;
#[doc = "Field `VICENMCURSTINTTOSSMCU` writer - VIC_EN_MCU_RST_INT_TO_SSMCU"]
pub type VicenmcurstinttossmcuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - VIC_EN_PSP_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicenpspswinttossmcu(&self) -> VicenpspswinttossmcuR {
        VicenpspswinttossmcuR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - VIC_EN_PSP_RST_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicenpsprstinttossmcu(&self) -> VicenpsprstinttossmcuR {
        VicenpsprstinttossmcuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - VIC_EN_MCU_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicenmcuswinttossmcu(&self) -> VicenmcuswinttossmcuR {
        VicenmcuswinttossmcuR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - VIC_EN_MCU_RST_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicenmcurstinttossmcu(&self) -> VicenmcurstinttossmcuR {
        VicenmcurstinttossmcuR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - VIC_EN_PSP_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicenpspswinttossmcu(&mut self) -> VicenpspswinttossmcuW<Vic020Spec> {
        VicenpspswinttossmcuW::new(self, 0)
    }
    #[doc = "Bit 7 - VIC_EN_PSP_RST_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicenpsprstinttossmcu(&mut self) -> VicenpsprstinttossmcuW<Vic020Spec> {
        VicenpsprstinttossmcuW::new(self, 7)
    }
    #[doc = "Bits 8:14 - VIC_EN_MCU_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicenmcuswinttossmcu(&mut self) -> VicenmcuswinttossmcuW<Vic020Spec> {
        VicenmcuswinttossmcuW::new(self, 8)
    }
    #[doc = "Bit 15 - VIC_EN_MCU_RST_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicenmcurstinttossmcu(&mut self) -> VicenmcurstinttossmcuW<Vic020Spec> {
        VicenmcurstinttossmcuW::new(self, 15)
    }
}
#[doc = "Caliptra SSMCU Software Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vic020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic020Spec;
impl crate::RegisterSpec for Vic020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic020::R`](R) reader structure"]
impl crate::Readable for Vic020Spec {}
#[doc = "`write(|w| ..)` method takes [`vic020::W`](W) writer structure"]
impl crate::Writable for Vic020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC020 to value 0"]
impl crate::Resettable for Vic020Spec {}
