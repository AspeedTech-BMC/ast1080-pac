#[doc = "Register `SCUBF0` reader"]
pub type R = crate::R<Scubf0Spec>;
#[doc = "Register `SCUBF0` writer"]
pub type W = crate::W<Scubf0Spec>;
#[doc = "Field `SCUHWPUF28` reader - SCU_HW_PUF_28"]
pub type Scuhwpuf28R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_28"]
    #[inline(always)]
    pub fn scuhwpuf28(&self) -> Scuhwpuf28R {
        Scuhwpuf28R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`scubf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubf0Spec;
impl crate::RegisterSpec for Scubf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubf0::R`](R) reader structure"]
impl crate::Readable for Scubf0Spec {}
#[doc = "`write(|w| ..)` method takes [`scubf0::W`](W) writer structure"]
impl crate::Writable for Scubf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBF0 to value 0"]
impl crate::Resettable for Scubf0Spec {}
