#[doc = "Register `SCU930` reader"]
pub type R = crate::R<Scu930Spec>;
#[doc = "Register `SCU930` writer"]
pub type W = crate::W<Scu930Spec>;
#[doc = "Field `SCUPSPMAPMEMSTART` reader - SCU_PSP_MAP_MEM_START"]
pub type ScupspmapmemstartR = crate::FieldReader<u32>;
#[doc = "Field `SCUPSPMAPMEMSTART` writer - SCU_PSP_MAP_MEM_START"]
pub type ScupspmapmemstartW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_MEM_START"]
    #[inline(always)]
    pub fn scupspmapmemstart(&self) -> ScupspmapmemstartR {
        ScupspmapmemstartR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - SCU_PSP_MAP_MEM_START"]
    #[inline(always)]
    pub fn scupspmapmemstart(&mut self) -> ScupspmapmemstartW<Scu930Spec> {
        ScupspmapmemstartW::new(self, 0)
    }
}
#[doc = "\\PSP\\ Service Processor REMAP Base Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu930::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu930::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu930Spec;
impl crate::RegisterSpec for Scu930Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu930::R`](R) reader structure"]
impl crate::Readable for Scu930Spec {}
#[doc = "`write(|w| ..)` method takes [`scu930::W`](W) writer structure"]
impl crate::Writable for Scu930Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU930 to value 0"]
impl crate::Resettable for Scu930Spec {}
