#[doc = "Register `HCIRHS078` reader"]
pub type R = crate::R<Hcirhs078Spec>;
#[doc = "Register `HCIRHS078` writer"]
pub type W = crate::W<Hcirhs078Spec>;
#[doc = "Field `REGIBIDATARINGBASELO` reader - REG_IBI_DATA_RING_BASE_LO"]
pub type RegibidataringbaseloR = crate::FieldReader<u32>;
#[doc = "Field `REGIBIDATARINGBASELO` writer - REG_IBI_DATA_RING_BASE_LO"]
pub type RegibidataringbaseloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_IBI_DATA_RING_BASE_LO"]
    #[inline(always)]
    pub fn regibidataringbaselo(&self) -> RegibidataringbaseloR {
        RegibidataringbaseloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_IBI_DATA_RING_BASE_LO"]
    #[inline(always)]
    pub fn regibidataringbaselo(&mut self) -> RegibidataringbaseloW<Hcirhs078Spec> {
        RegibidataringbaseloW::new(self, 0)
    }
}
#[doc = "RH\\_IBI\\_DATA\\_RING\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs078Spec;
impl crate::RegisterSpec for Hcirhs078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs078::R`](R) reader structure"]
impl crate::Readable for Hcirhs078Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs078::W`](W) writer structure"]
impl crate::Writable for Hcirhs078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS078 to value 0"]
impl crate::Resettable for Hcirhs078Spec {}
