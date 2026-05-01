#[doc = "Register `SCU114` reader"]
pub type R = crate::R<Scu114Spec>;
#[doc = "Register `SCU114` writer"]
pub type W = crate::W<Scu114Spec>;
#[doc = "Field `SCUMCURVLOCK` reader - SCU_MCU_RV_LOCK"]
pub type ScumcurvlockR = crate::BitReader;
#[doc = "Field `SCUMCURVLOCK` writer - SCU_MCU_RV_LOCK"]
pub type ScumcurvlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUMCURV` reader - SCU_MCU_RV"]
pub type ScumcurvR = crate::FieldReader<u32>;
#[doc = "Field `SCUMCURV` writer - SCU_MCU_RV"]
pub type ScumcurvW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - SCU_MCU_RV_LOCK"]
    #[inline(always)]
    pub fn scumcurvlock(&self) -> ScumcurvlockR {
        ScumcurvlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:31 - SCU_MCU_RV"]
    #[inline(always)]
    pub fn scumcurv(&self) -> ScumcurvR {
        ScumcurvR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_MCU_RV_LOCK"]
    #[inline(always)]
    pub fn scumcurvlock(&mut self) -> ScumcurvlockW<Scu114Spec> {
        ScumcurvlockW::new(self, 0)
    }
    #[doc = "Bits 2:31 - SCU_MCU_RV"]
    #[inline(always)]
    pub fn scumcurv(&mut self) -> ScumcurvW<Scu114Spec> {
        ScumcurvW::new(self, 2)
    }
}
#[doc = "MCU Reset Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`scu114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu114Spec;
impl crate::RegisterSpec for Scu114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu114::R`](R) reader structure"]
impl crate::Readable for Scu114Spec {}
#[doc = "`write(|w| ..)` method takes [`scu114::W`](W) writer structure"]
impl crate::Writable for Scu114Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU114 to value 0x14b0_0000"]
impl crate::Resettable for Scu114Spec {
    const RESET_VALUE: u32 = 0x14b0_0000;
}
