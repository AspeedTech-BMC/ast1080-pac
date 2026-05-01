#[doc = "Register `SCUBB0` reader"]
pub type R = crate::R<Scubb0Spec>;
#[doc = "Register `SCUBB0` writer"]
pub type W = crate::W<Scubb0Spec>;
#[doc = "Field `SCUHWPUF12` reader - SCU_HW_PUF_12"]
pub type Scuhwpuf12R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_12"]
    #[inline(always)]
    pub fn scuhwpuf12(&self) -> Scuhwpuf12R {
        Scuhwpuf12R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`scubb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubb0Spec;
impl crate::RegisterSpec for Scubb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubb0::R`](R) reader structure"]
impl crate::Readable for Scubb0Spec {}
#[doc = "`write(|w| ..)` method takes [`scubb0::W`](W) writer structure"]
impl crate::Writable for Scubb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBB0 to value 0"]
impl crate::Resettable for Scubb0Spec {}
