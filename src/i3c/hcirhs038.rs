#[doc = "Register `HCIRHS038` reader"]
pub type R = crate::R<Hcirhs038Spec>;
#[doc = "Register `HCIRHS038` writer"]
pub type W = crate::W<Hcirhs038Spec>;
#[doc = "Field `REGCHUNKCOUNTER` reader - REG_CHUNK_COUNTER"]
pub type RegchunkcounterR = crate::FieldReader<u32>;
#[doc = "Field `REGCHUNKCOUNTER` writer - REG_CHUNK_COUNTER"]
pub type RegchunkcounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_CHUNK_COUNTER"]
    #[inline(always)]
    pub fn regchunkcounter(&self) -> RegchunkcounterR {
        RegchunkcounterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_CHUNK_COUNTER"]
    #[inline(always)]
    pub fn regchunkcounter(&mut self) -> RegchunkcounterW<Hcirhs038Spec> {
        RegchunkcounterW::new(self, 0)
    }
}
#[doc = "CHUNK\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs038Spec;
impl crate::RegisterSpec for Hcirhs038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs038::R`](R) reader structure"]
impl crate::Readable for Hcirhs038Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs038::W`](W) writer structure"]
impl crate::Writable for Hcirhs038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS038 to value 0"]
impl crate::Resettable for Hcirhs038Spec {}
