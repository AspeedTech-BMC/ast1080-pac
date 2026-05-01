#[doc = "Register `SCUBE8` reader"]
pub type R = crate::R<Scube8Spec>;
#[doc = "Register `SCUBE8` writer"]
pub type W = crate::W<Scube8Spec>;
#[doc = "Field `SCUHWPUF26` reader - SCU_HW_PUF_26"]
pub type Scuhwpuf26R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_26"]
    #[inline(always)]
    pub fn scuhwpuf26(&self) -> Scuhwpuf26R {
        Scuhwpuf26R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`scube8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scube8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scube8Spec;
impl crate::RegisterSpec for Scube8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scube8::R`](R) reader structure"]
impl crate::Readable for Scube8Spec {}
#[doc = "`write(|w| ..)` method takes [`scube8::W`](W) writer structure"]
impl crate::Writable for Scube8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBE8 to value 0"]
impl crate::Resettable for Scube8Spec {}
