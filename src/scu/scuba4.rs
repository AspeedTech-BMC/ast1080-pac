#[doc = "Register `SCUBA4` reader"]
pub type R = crate::R<Scuba4Spec>;
#[doc = "Register `SCUBA4` writer"]
pub type W = crate::W<Scuba4Spec>;
#[doc = "Field `SCUHWPUF9` reader - SCU_HW_PUF_9"]
pub type Scuhwpuf9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_9"]
    #[inline(always)]
    pub fn scuhwpuf9(&self) -> Scuhwpuf9R {
        Scuhwpuf9R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`scuba4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuba4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuba4Spec;
impl crate::RegisterSpec for Scuba4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuba4::R`](R) reader structure"]
impl crate::Readable for Scuba4Spec {}
#[doc = "`write(|w| ..)` method takes [`scuba4::W`](W) writer structure"]
impl crate::Writable for Scuba4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBA4 to value 0"]
impl crate::Resettable for Scuba4Spec {}
