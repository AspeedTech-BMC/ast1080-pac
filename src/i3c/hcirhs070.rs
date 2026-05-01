#[doc = "Register `HCIRHS070` reader"]
pub type R = crate::R<Hcirhs070Spec>;
#[doc = "Register `HCIRHS070` writer"]
pub type W = crate::W<Hcirhs070Spec>;
#[doc = "Field `REGIBISTATUSRINGBASELO` reader - REG_IBI_STATUS_RING_BASE_LO"]
pub type RegibistatusringbaseloR = crate::FieldReader<u32>;
#[doc = "Field `REGIBISTATUSRINGBASELO` writer - REG_IBI_STATUS_RING_BASE_LO"]
pub type RegibistatusringbaseloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_IBI_STATUS_RING_BASE_LO"]
    #[inline(always)]
    pub fn regibistatusringbaselo(&self) -> RegibistatusringbaseloR {
        RegibistatusringbaseloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_IBI_STATUS_RING_BASE_LO"]
    #[inline(always)]
    pub fn regibistatusringbaselo(&mut self) -> RegibistatusringbaseloW<Hcirhs070Spec> {
        RegibistatusringbaseloW::new(self, 0)
    }
}
#[doc = "RH\\_IBI\\_STATUS\\_RING\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs070Spec;
impl crate::RegisterSpec for Hcirhs070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs070::R`](R) reader structure"]
impl crate::Readable for Hcirhs070Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs070::W`](W) writer structure"]
impl crate::Writable for Hcirhs070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS070 to value 0"]
impl crate::Resettable for Hcirhs070Spec {}
