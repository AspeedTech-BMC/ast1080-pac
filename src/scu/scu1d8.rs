#[doc = "Register `SCU1D8` reader"]
pub type R = crate::R<Scu1d8Spec>;
#[doc = "Register `SCU1D8` writer"]
pub type W = crate::W<Scu1d8Spec>;
#[doc = "Field `SCUSCRATCH23` reader - SCU_SCRATCH_23"]
pub type Scuscratch23R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH23` writer - SCU_SCRATCH_23"]
pub type Scuscratch23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_23"]
    #[inline(always)]
    pub fn scuscratch23(&self) -> Scuscratch23R {
        Scuscratch23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_23"]
    #[inline(always)]
    pub fn scuscratch23(&mut self) -> Scuscratch23W<Scu1d8Spec> {
        Scuscratch23W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1d8Spec;
impl crate::RegisterSpec for Scu1d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1d8::R`](R) reader structure"]
impl crate::Readable for Scu1d8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1d8::W`](W) writer structure"]
impl crate::Writable for Scu1d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1D8 to value 0"]
impl crate::Resettable for Scu1d8Spec {}
