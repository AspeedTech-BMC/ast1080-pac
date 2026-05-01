#[doc = "Register `SCUBC8` reader"]
pub type R = crate::R<Scubc8Spec>;
#[doc = "Register `SCUBC8` writer"]
pub type W = crate::W<Scubc8Spec>;
#[doc = "Field `SCUHWPUF18` reader - SCU_HW_PUF_18"]
pub type Scuhwpuf18R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_18"]
    #[inline(always)]
    pub fn scuhwpuf18(&self) -> Scuhwpuf18R {
        Scuhwpuf18R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`scubc8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubc8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubc8Spec;
impl crate::RegisterSpec for Scubc8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubc8::R`](R) reader structure"]
impl crate::Readable for Scubc8Spec {}
#[doc = "`write(|w| ..)` method takes [`scubc8::W`](W) writer structure"]
impl crate::Writable for Scubc8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBC8 to value 0"]
impl crate::Resettable for Scubc8Spec {}
