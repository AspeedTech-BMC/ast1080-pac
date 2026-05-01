#[doc = "Register `SCU180` reader"]
pub type R = crate::R<Scu180Spec>;
#[doc = "Register `SCU180` writer"]
pub type W = crate::W<Scu180Spec>;
#[doc = "Field `SCUSCRATCH1` reader - SCU_SCRATCH_1"]
pub type Scuscratch1R = crate::FieldReader<u32>;
#[doc = "Field `SCUSCRATCH1` writer - SCU_SCRATCH_1"]
pub type Scuscratch1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SCRATCH_1"]
    #[inline(always)]
    pub fn scuscratch1(&self) -> Scuscratch1R {
        Scuscratch1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_SCRATCH_1"]
    #[inline(always)]
    pub fn scuscratch1(&mut self) -> Scuscratch1W<Scu180Spec> {
        Scuscratch1W::new(self, 0)
    }
}
#[doc = "SCU\\_CPU\\_SCRATCH\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu180::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu180::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu180Spec;
impl crate::RegisterSpec for Scu180Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu180::R`](R) reader structure"]
impl crate::Readable for Scu180Spec {}
#[doc = "`write(|w| ..)` method takes [`scu180::W`](W) writer structure"]
impl crate::Writable for Scu180Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU180 to value 0"]
impl crate::Resettable for Scu180Spec {}
