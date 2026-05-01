#[doc = "Register `SCU920` reader"]
pub type R = crate::R<Scu920Spec>;
#[doc = "Register `SCU920` writer"]
pub type W = crate::W<Scu920Spec>;
#[doc = "Field `SCUPSPMAPTCMSTART` reader - SCU_PSP_MAP_TCM_START"]
pub type ScupspmaptcmstartR = crate::FieldReader<u32>;
#[doc = "Field `SCUPSPMAPTCMSTART` writer - SCU_PSP_MAP_TCM_START"]
pub type ScupspmaptcmstartW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_TCM_START"]
    #[inline(always)]
    pub fn scupspmaptcmstart(&self) -> ScupspmaptcmstartR {
        ScupspmaptcmstartR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_TCM_START"]
    #[inline(always)]
    pub fn scupspmaptcmstart(&mut self) -> ScupspmaptcmstartW<Scu920Spec> {
        ScupspmaptcmstartW::new(self, 0)
    }
}
#[doc = "\\PSP\\ Service Processor REMAP Base Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu920::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu920::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu920Spec;
impl crate::RegisterSpec for Scu920Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu920::R`](R) reader structure"]
impl crate::Readable for Scu920Spec {}
#[doc = "`write(|w| ..)` method takes [`scu920::W`](W) writer structure"]
impl crate::Writable for Scu920Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU920 to value 0"]
impl crate::Resettable for Scu920Spec {}
