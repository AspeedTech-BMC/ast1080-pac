#[doc = "Register `SCU190` reader"]
pub type R = crate::R<Scu190Spec>;
#[doc = "Register `SCU190` writer"]
pub type W = crate::W<Scu190Spec>;
#[doc = "Field `SCUSCRATCH5` reader - SCU_SCRATCH_5"]
pub type Scuscratch5R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH5` writer - SCU_SCRATCH_5"]
pub type Scuscratch5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_5"]
    #[inline(always)]
    pub fn scuscratch5(&self) -> Scuscratch5R {
        Scuscratch5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_5"]
    #[inline(always)]
    pub fn scuscratch5(&mut self) -> Scuscratch5W<Scu190Spec> {
        Scuscratch5W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu190::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu190::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu190Spec;
impl crate::RegisterSpec for Scu190Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu190::R`](R) reader structure"]
impl crate::Readable for Scu190Spec {}
#[doc = "`write(|w| ..)` method takes [`scu190::W`](W) writer structure"]
impl crate::Writable for Scu190Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU190 to value 0"]
impl crate::Resettable for Scu190Spec {}
