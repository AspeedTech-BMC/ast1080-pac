#[doc = "Register `SCU1F8` reader"]
pub type R = crate::R<Scu1f8Spec>;
#[doc = "Register `SCU1F8` writer"]
pub type W = crate::W<Scu1f8Spec>;
#[doc = "Field `SCUSCRATCH31` reader - SCU_SCRATCH_31"]
pub type Scuscratch31R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH31` writer - SCU_SCRATCH_31"]
pub type Scuscratch31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_31"]
    #[inline(always)]
    pub fn scuscratch31(&self) -> Scuscratch31R {
        Scuscratch31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_31"]
    #[inline(always)]
    pub fn scuscratch31(&mut self) -> Scuscratch31W<Scu1f8Spec> {
        Scuscratch31W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_31\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1f8Spec;
impl crate::RegisterSpec for Scu1f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1f8::R`](R) reader structure"]
impl crate::Readable for Scu1f8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1f8::W`](W) writer structure"]
impl crate::Writable for Scu1f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1F8 to value 0"]
impl crate::Resettable for Scu1f8Spec {}
