#[doc = "Register `SCU928` reader"]
pub type R = crate::R<Scu928Spec>;
#[doc = "Register `SCU928` writer"]
pub type W = crate::W<Scu928Spec>;
#[doc = "Field `SCUPSPMAPAHBSTART` reader - SCU_PSP_MAP_AHB_START"]
pub type ScupspmapahbstartR = crate::FieldReader<u32>;
#[doc = "Field `SCUPSPMAPAHBSTART` writer - SCU_PSP_MAP_AHB_START"]
pub type ScupspmapahbstartW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_AHB_START"]
    #[inline(always)]
    pub fn scupspmapahbstart(&self) -> ScupspmapahbstartR {
        ScupspmapahbstartR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_AHB_START"]
    #[inline(always)]
    pub fn scupspmapahbstart(&mut self) -> ScupspmapahbstartW<Scu928Spec> {
        ScupspmapahbstartW::new(self, 0)
    }
}
#[doc = "\\PSP\\ Service Processor REMAP Base Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu928::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu928::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu928Spec;
impl crate::RegisterSpec for Scu928Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu928::R`](R) reader structure"]
impl crate::Readable for Scu928Spec {}
#[doc = "`write(|w| ..)` method takes [`scu928::W`](W) writer structure"]
impl crate::Writable for Scu928Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU928 to value 0"]
impl crate::Resettable for Scu928Spec {}
