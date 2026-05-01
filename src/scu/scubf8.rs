#[doc = "Register `SCUBF8` reader"]
pub type R = crate::R<Scubf8Spec>;
#[doc = "Register `SCUBF8` writer"]
pub type W = crate::W<Scubf8Spec>;
#[doc = "Field `SCUHWPUF30` reader - SCU_HW_PUF_30"]
pub type Scuhwpuf30R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_30"]
    #[inline(always)]
    pub fn scuhwpuf30(&self) -> Scuhwpuf30R {
        Scuhwpuf30R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`scubf8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubf8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubf8Spec;
impl crate::RegisterSpec for Scubf8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubf8::R`](R) reader structure"]
impl crate::Readable for Scubf8Spec {}
#[doc = "`write(|w| ..)` method takes [`scubf8::W`](W) writer structure"]
impl crate::Writable for Scubf8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBF8 to value 0"]
impl crate::Resettable for Scubf8Spec {}
