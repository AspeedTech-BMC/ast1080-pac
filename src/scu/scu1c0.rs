#[doc = "Register `SCU1C0` reader"]
pub type R = crate::R<Scu1c0Spec>;
#[doc = "Register `SCU1C0` writer"]
pub type W = crate::W<Scu1c0Spec>;
#[doc = "Field `SCUSCRATCH17` reader - SCU_SCRATCH_17"]
pub type Scuscratch17R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH17` writer - SCU_SCRATCH_17"]
pub type Scuscratch17W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_17"]
    #[inline(always)]
    pub fn scuscratch17(&self) -> Scuscratch17R {
        Scuscratch17R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_17"]
    #[inline(always)]
    pub fn scuscratch17(&mut self) -> Scuscratch17W<Scu1c0Spec> {
        Scuscratch17W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_17\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1c0Spec;
impl crate::RegisterSpec for Scu1c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1c0::R`](R) reader structure"]
impl crate::Readable for Scu1c0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1c0::W`](W) writer structure"]
impl crate::Writable for Scu1c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1C0 to value 0"]
impl crate::Resettable for Scu1c0Spec {}
