#[doc = "Register `SCU148` reader"]
pub type R = crate::R<Scu148Spec>;
#[doc = "Register `SCU148` writer"]
pub type W = crate::W<Scu148Spec>;
#[doc = "Field `SCUCPTRASTRAPNUMDEBUG` reader - SCU_CPTRA_STRAP_NUM_DEBUG"]
pub type ScucptrastrapnumdebugR = crate::FieldReader<u32>;
#[doc = "Field `SCUCPTRASTRAPNUMDEBUG` writer - SCU_CPTRA_STRAP_NUM_DEBUG"]
pub type ScucptrastrapnumdebugW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_CPTRA_STRAP_NUM_DEBUG"]
    #[inline(always)]
    pub fn scucptrastrapnumdebug(&self) -> ScucptrastrapnumdebugR {
        ScucptrastrapnumdebugR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_CPTRA_STRAP_NUM_DEBUG"]
    #[inline(always)]
    pub fn scucptrastrapnumdebug(&mut self) -> ScucptrastrapnumdebugW<Scu148Spec> {
        ScucptrastrapnumdebugW::new(self, 0)
    }
}
#[doc = "Caliptra Config Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu148::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu148::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu148Spec;
impl crate::RegisterSpec for Scu148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu148::R`](R) reader structure"]
impl crate::Readable for Scu148Spec {}
#[doc = "`write(|w| ..)` method takes [`scu148::W`](W) writer structure"]
impl crate::Writable for Scu148Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU148 to value 0"]
impl crate::Resettable for Scu148Spec {}
