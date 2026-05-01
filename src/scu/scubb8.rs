#[doc = "Register `SCUBB8` reader"]
pub type R = crate::R<Scubb8Spec>;
#[doc = "Register `SCUBB8` writer"]
pub type W = crate::W<Scubb8Spec>;
#[doc = "Field `SCUHWPUF14` reader - SCU_HW_PUF_14"]
pub type Scuhwpuf14R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_14"]
    #[inline(always)]
    pub fn scuhwpuf14(&self) -> Scuhwpuf14R {
        Scuhwpuf14R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`scubb8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubb8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubb8Spec;
impl crate::RegisterSpec for Scubb8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubb8::R`](R) reader structure"]
impl crate::Readable for Scubb8Spec {}
#[doc = "`write(|w| ..)` method takes [`scubb8::W`](W) writer structure"]
impl crate::Writable for Scubb8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBB8 to value 0"]
impl crate::Resettable for Scubb8Spec {}
