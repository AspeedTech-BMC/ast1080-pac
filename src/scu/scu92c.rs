#[doc = "Register `SCU92C` reader"]
pub type R = crate::R<Scu92cSpec>;
#[doc = "Register `SCU92C` writer"]
pub type W = crate::W<Scu92cSpec>;
#[doc = "Field `SCUPSPMAPAHBSIZE` reader - SCU_PSP_MAP_AHB_SIZE"]
pub type ScupspmapahbsizeR = crate::FieldReader<u32>;
#[doc = "Field `SCUPSPMAPAHBSIZE` writer - SCU_PSP_MAP_AHB_SIZE"]
pub type ScupspmapahbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_AHB_SIZE"]
    #[inline(always)]
    pub fn scupspmapahbsize(&self) -> ScupspmapahbsizeR {
        ScupspmapahbsizeR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_AHB_SIZE"]
    #[inline(always)]
    pub fn scupspmapahbsize(&mut self) -> ScupspmapahbsizeW<Scu92cSpec> {
        ScupspmapahbsizeW::new(self, 0)
    }
}
#[doc = "\\PSP\\ Service Processor REMAP Size Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu92c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu92c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu92cSpec;
impl crate::RegisterSpec for Scu92cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu92c::R`](R) reader structure"]
impl crate::Readable for Scu92cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu92c::W`](W) writer structure"]
impl crate::Writable for Scu92cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU92C to value 0"]
impl crate::Resettable for Scu92cSpec {}
