#[doc = "Register `SCU198` reader"]
pub type R = crate::R<Scu198Spec>;
#[doc = "Register `SCU198` writer"]
pub type W = crate::W<Scu198Spec>;
#[doc = "Field `SCUSCRATCH7` reader - SCU_SCRATCH_7"]
pub type Scuscratch7R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH7` writer - SCU_SCRATCH_7"]
pub type Scuscratch7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_7"]
    #[inline(always)]
    pub fn scuscratch7(&self) -> Scuscratch7R {
        Scuscratch7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_7"]
    #[inline(always)]
    pub fn scuscratch7(&mut self) -> Scuscratch7W<Scu198Spec> {
        Scuscratch7W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu198::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu198::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu198Spec;
impl crate::RegisterSpec for Scu198Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu198::R`](R) reader structure"]
impl crate::Readable for Scu198Spec {}
#[doc = "`write(|w| ..)` method takes [`scu198::W`](W) writer structure"]
impl crate::Writable for Scu198Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU198 to value 0"]
impl crate::Resettable for Scu198Spec {}
