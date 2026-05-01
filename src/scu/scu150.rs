#[doc = "Register `SCU150` reader"]
pub type R = crate::R<Scu150Spec>;
#[doc = "Register `SCU150` writer"]
pub type W = crate::W<Scu150Spec>;
#[doc = "Field `SCUCPTRACOREGENERICINPUT1` reader - SCU_CPTRA_CORE_GENERIC_INPUT_1"]
pub type Scucptracoregenericinput1R = crate::FieldReader<u32>;
#[doc = "Field `SCUCPTRACOREGENERICINPUT1` writer - SCU_CPTRA_CORE_GENERIC_INPUT_1"]
pub type Scucptracoregenericinput1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_CPTRA_CORE_GENERIC_INPUT_1"]
    #[inline(always)]
    pub fn scucptracoregenericinput1(&self) -> Scucptracoregenericinput1R {
        Scucptracoregenericinput1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_CPTRA_CORE_GENERIC_INPUT_1"]
    #[inline(always)]
    pub fn scucptracoregenericinput1(&mut self) -> Scucptracoregenericinput1W<Scu150Spec> {
        Scucptracoregenericinput1W::new(self, 0)
    }
}
#[doc = "Caliptra Config Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu150::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu150::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu150Spec;
impl crate::RegisterSpec for Scu150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu150::R`](R) reader structure"]
impl crate::Readable for Scu150Spec {}
#[doc = "`write(|w| ..)` method takes [`scu150::W`](W) writer structure"]
impl crate::Writable for Scu150Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU150 to value 0"]
impl crate::Resettable for Scu150Spec {}
