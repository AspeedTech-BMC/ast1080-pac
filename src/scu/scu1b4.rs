#[doc = "Register `SCU1B4` reader"]
pub type R = crate::R<Scu1b4Spec>;
#[doc = "Register `SCU1B4` writer"]
pub type W = crate::W<Scu1b4Spec>;
#[doc = "Field `SCUSCRATCH14` reader - SCU_SCRATCH_14"]
pub type Scuscratch14R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH14` writer - SCU_SCRATCH_14"]
pub type Scuscratch14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_14"]
    #[inline(always)]
    pub fn scuscratch14(&self) -> Scuscratch14R {
        Scuscratch14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_14"]
    #[inline(always)]
    pub fn scuscratch14(&mut self) -> Scuscratch14W<Scu1b4Spec> {
        Scuscratch14W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1b4Spec;
impl crate::RegisterSpec for Scu1b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1b4::R`](R) reader structure"]
impl crate::Readable for Scu1b4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1b4::W`](W) writer structure"]
impl crate::Writable for Scu1b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1B4 to value 0"]
impl crate::Resettable for Scu1b4Spec {}
