#[doc = "Register `SCU1C8` reader"]
pub type R = crate::R<Scu1c8Spec>;
#[doc = "Register `SCU1C8` writer"]
pub type W = crate::W<Scu1c8Spec>;
#[doc = "Field `SCUSCRATCH19` reader - SCU_SCRATCH_19"]
pub type Scuscratch19R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH19` writer - SCU_SCRATCH_19"]
pub type Scuscratch19W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_19"]
    #[inline(always)]
    pub fn scuscratch19(&self) -> Scuscratch19R {
        Scuscratch19R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_19"]
    #[inline(always)]
    pub fn scuscratch19(&mut self) -> Scuscratch19W<Scu1c8Spec> {
        Scuscratch19W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1c8Spec;
impl crate::RegisterSpec for Scu1c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1c8::R`](R) reader structure"]
impl crate::Readable for Scu1c8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1c8::W`](W) writer structure"]
impl crate::Writable for Scu1c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1C8 to value 0"]
impl crate::Resettable for Scu1c8Spec {}
