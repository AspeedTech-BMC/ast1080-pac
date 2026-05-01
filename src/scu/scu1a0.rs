#[doc = "Register `SCU1A0` reader"]
pub type R = crate::R<Scu1a0Spec>;
#[doc = "Register `SCU1A0` writer"]
pub type W = crate::W<Scu1a0Spec>;
#[doc = "Field `SCUSCRATCH9` reader - SCU_SCRATCH_9"]
pub type Scuscratch9R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH9` writer - SCU_SCRATCH_9"]
pub type Scuscratch9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_9"]
    #[inline(always)]
    pub fn scuscratch9(&self) -> Scuscratch9R {
        Scuscratch9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_9"]
    #[inline(always)]
    pub fn scuscratch9(&mut self) -> Scuscratch9W<Scu1a0Spec> {
        Scuscratch9W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1a0Spec;
impl crate::RegisterSpec for Scu1a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1a0::R`](R) reader structure"]
impl crate::Readable for Scu1a0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1a0::W`](W) writer structure"]
impl crate::Writable for Scu1a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1A0 to value 0"]
impl crate::Resettable for Scu1a0Spec {}
