#[doc = "Register `SCUBA0` reader"]
pub type R = crate::R<Scuba0Spec>;
#[doc = "Register `SCUBA0` writer"]
pub type W = crate::W<Scuba0Spec>;
#[doc = "Field `SCUHWPUF8` reader - SCU_HW_PUF_8"]
pub type Scuhwpuf8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_8"]
    #[inline(always)]
    pub fn scuhwpuf8(&self) -> Scuhwpuf8R {
        Scuhwpuf8R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scuba0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuba0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuba0Spec;
impl crate::RegisterSpec for Scuba0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuba0::R`](R) reader structure"]
impl crate::Readable for Scuba0Spec {}
#[doc = "`write(|w| ..)` method takes [`scuba0::W`](W) writer structure"]
impl crate::Writable for Scuba0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBA0 to value 0"]
impl crate::Resettable for Scuba0Spec {}
