#[doc = "Register `SCU924` reader"]
pub type R = crate::R<Scu924Spec>;
#[doc = "Register `SCU924` writer"]
pub type W = crate::W<Scu924Spec>;
#[doc = "Field `SCUPSPMAPTCMSIZE` reader - SCU_PSP_MAP_TCM_SIZE"]
pub type ScupspmaptcmsizeR = crate::FieldReader<u32>;
#[doc = "Field `SCUPSPMAPTCMSIZE` writer - SCU_PSP_MAP_TCM_SIZE"]
pub type ScupspmaptcmsizeW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_TCM_SIZE"]
    #[inline(always)]
    pub fn scupspmaptcmsize(&self) -> ScupspmaptcmsizeR {
        ScupspmaptcmsizeR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_TCM_SIZE"]
    #[inline(always)]
    pub fn scupspmaptcmsize(&mut self) -> ScupspmaptcmsizeW<Scu924Spec> {
        ScupspmaptcmsizeW::new(self, 0)
    }
}
#[doc = "\\PSP\\ Service Processor REMAP Size Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu924::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu924::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu924Spec;
impl crate::RegisterSpec for Scu924Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu924::R`](R) reader structure"]
impl crate::Readable for Scu924Spec {}
#[doc = "`write(|w| ..)` method takes [`scu924::W`](W) writer structure"]
impl crate::Writable for Scu924Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU924 to value 0"]
impl crate::Resettable for Scu924Spec {}
