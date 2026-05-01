#[doc = "Register `HCIRHS074` reader"]
pub type R = crate::R<Hcirhs074Spec>;
#[doc = "Register `HCIRHS074` writer"]
pub type W = crate::W<Hcirhs074Spec>;
#[doc = "Field `REGIBISTATUSRINGBASEHI` reader - REG_IBI_STATUS_RING_BASE_HI"]
pub type RegibistatusringbasehiR = crate::FieldReader<u32>;
#[doc = "Field `REGIBISTATUSRINGBASEHI` writer - REG_IBI_STATUS_RING_BASE_HI"]
pub type RegibistatusringbasehiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_IBI_STATUS_RING_BASE_HI"]
    #[inline(always)]
    pub fn regibistatusringbasehi(&self) -> RegibistatusringbasehiR {
        RegibistatusringbasehiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_IBI_STATUS_RING_BASE_HI"]
    #[inline(always)]
    pub fn regibistatusringbasehi(&mut self) -> RegibistatusringbasehiW<Hcirhs074Spec> {
        RegibistatusringbasehiW::new(self, 0)
    }
}
#[doc = "RH\\_IBI\\_STATUS\\_RING\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs074Spec;
impl crate::RegisterSpec for Hcirhs074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs074::R`](R) reader structure"]
impl crate::Readable for Hcirhs074Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs074::W`](W) writer structure"]
impl crate::Writable for Hcirhs074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS074 to value 0"]
impl crate::Resettable for Hcirhs074Spec {}
