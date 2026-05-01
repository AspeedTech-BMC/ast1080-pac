#[doc = "Register `SCU158` reader"]
pub type R = crate::R<Scu158Spec>;
#[doc = "Register `SCU158` writer"]
pub type W = crate::W<Scu158Spec>;
#[doc = "Field `SCUCPTRAMCIGENERICINPUT1` reader - SCU_CPTRA_MCI_GENERIC_INPUT_1"]
pub type Scucptramcigenericinput1R = crate::FieldReader<u32>;
#[doc = "Field `SCUCPTRAMCIGENERICINPUT1` writer - SCU_CPTRA_MCI_GENERIC_INPUT_1"]
pub type Scucptramcigenericinput1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_CPTRA_MCI_GENERIC_INPUT_1"]
    #[inline(always)]
    pub fn scucptramcigenericinput1(&self) -> Scucptramcigenericinput1R {
        Scucptramcigenericinput1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_CPTRA_MCI_GENERIC_INPUT_1"]
    #[inline(always)]
    pub fn scucptramcigenericinput1(&mut self) -> Scucptramcigenericinput1W<Scu158Spec> {
        Scucptramcigenericinput1W::new(self, 0)
    }
}
#[doc = "Caliptra Config Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu158::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu158::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu158Spec;
impl crate::RegisterSpec for Scu158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu158::R`](R) reader structure"]
impl crate::Readable for Scu158Spec {}
#[doc = "`write(|w| ..)` method takes [`scu158::W`](W) writer structure"]
impl crate::Writable for Scu158Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU158 to value 0"]
impl crate::Resettable for Scu158Spec {}
