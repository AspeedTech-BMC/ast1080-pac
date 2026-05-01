#[doc = "Register `SCU1C4` reader"]
pub type R = crate::R<Scu1c4Spec>;
#[doc = "Register `SCU1C4` writer"]
pub type W = crate::W<Scu1c4Spec>;
#[doc = "Field `SCUSCRATCH18` reader - SCU_SCRATCH_18"]
pub type Scuscratch18R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH18` writer - SCU_SCRATCH_18"]
pub type Scuscratch18W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_18"]
    #[inline(always)]
    pub fn scuscratch18(&self) -> Scuscratch18R {
        Scuscratch18R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_18"]
    #[inline(always)]
    pub fn scuscratch18(&mut self) -> Scuscratch18W<Scu1c4Spec> {
        Scuscratch18W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_18\n\nYou can [`read`](crate::Reg::read) this register and get [`scu1c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu1c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu1c4Spec;
impl crate::RegisterSpec for Scu1c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu1c4::R`](R) reader structure"]
impl crate::Readable for Scu1c4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu1c4::W`](W) writer structure"]
impl crate::Writable for Scu1c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU1C4 to value 0"]
impl crate::Resettable for Scu1c4Spec {}
