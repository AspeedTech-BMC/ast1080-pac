#[doc = "Register `SCU160` reader"]
pub type R = crate::R<Scu160Spec>;
#[doc = "Register `SCU160` writer"]
pub type W = crate::W<Scu160Spec>;
#[doc = "Field `SCUCPTRAMCIGENERICOUTPUT1` reader - SCU_CPTRA_MCI_GENERIC_OUTPUT_1"]
pub type Scucptramcigenericoutput1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_CPTRA_MCI_GENERIC_OUTPUT_1"]
    #[inline(always)]
    pub fn scucptramcigenericoutput1(&self) -> Scucptramcigenericoutput1R {
        Scucptramcigenericoutput1R::new(self.bits)
    }
}
impl W {}
#[doc = "Caliptra Config Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu160Spec;
impl crate::RegisterSpec for Scu160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu160::R`](R) reader structure"]
impl crate::Readable for Scu160Spec {}
#[doc = "`write(|w| ..)` method takes [`scu160::W`](W) writer structure"]
impl crate::Writable for Scu160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU160 to value 0"]
impl crate::Resettable for Scu160Spec {}
