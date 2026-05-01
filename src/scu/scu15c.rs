#[doc = "Register `SCU15C` reader"]
pub type R = crate::R<Scu15cSpec>;
#[doc = "Register `SCU15C` writer"]
pub type W = crate::W<Scu15cSpec>;
#[doc = "Field `SCUCPTRAMCIGENERICOUTPUT0` reader - SCU_CPTRA_MCI_GENERIC_OUTPUT_0"]
pub type Scucptramcigenericoutput0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_CPTRA_MCI_GENERIC_OUTPUT_0"]
    #[inline(always)]
    pub fn scucptramcigenericoutput0(&self) -> Scucptramcigenericoutput0R {
        Scucptramcigenericoutput0R::new(self.bits)
    }
}
impl W {}
#[doc = "Caliptra Config Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu15c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu15c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu15cSpec;
impl crate::RegisterSpec for Scu15cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu15c::R`](R) reader structure"]
impl crate::Readable for Scu15cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu15c::W`](W) writer structure"]
impl crate::Writable for Scu15cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU15C to value 0"]
impl crate::Resettable for Scu15cSpec {}
