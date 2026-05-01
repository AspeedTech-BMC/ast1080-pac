#[doc = "Register `SCU154` reader"]
pub type R = crate::R<Scu154Spec>;
#[doc = "Register `SCU154` writer"]
pub type W = crate::W<Scu154Spec>;
#[doc = "Field `SCUCPTRAMCIGENERICINPUT0` reader - SCU_CPTRA_MCI_GENERIC_INPUT_0"]
pub type Scucptramcigenericinput0R = crate::FieldReader<u32>;
#[doc = "Field `SCUCPTRAMCIGENERICINPUT0` writer - SCU_CPTRA_MCI_GENERIC_INPUT_0"]
pub type Scucptramcigenericinput0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_CPTRA_MCI_GENERIC_INPUT_0"]
    #[inline(always)]
    pub fn scucptramcigenericinput0(&self) -> Scucptramcigenericinput0R {
        Scucptramcigenericinput0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_CPTRA_MCI_GENERIC_INPUT_0"]
    #[inline(always)]
    pub fn scucptramcigenericinput0(&mut self) -> Scucptramcigenericinput0W<Scu154Spec> {
        Scucptramcigenericinput0W::new(self, 0)
    }
}
#[doc = "Caliptra Config Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu154::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu154::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu154Spec;
impl crate::RegisterSpec for Scu154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu154::R`](R) reader structure"]
impl crate::Readable for Scu154Spec {}
#[doc = "`write(|w| ..)` method takes [`scu154::W`](W) writer structure"]
impl crate::Writable for Scu154Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU154 to value 0"]
impl crate::Resettable for Scu154Spec {}
