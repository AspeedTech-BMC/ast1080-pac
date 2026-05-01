#[doc = "Register `SCUBD0` reader"]
pub type R = crate::R<Scubd0Spec>;
#[doc = "Register `SCUBD0` writer"]
pub type W = crate::W<Scubd0Spec>;
#[doc = "Field `SCUHWPUF20` reader - SCU_HW_PUF_20"]
pub type Scuhwpuf20R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_20"]
    #[inline(always)]
    pub fn scuhwpuf20(&self) -> Scuhwpuf20R {
        Scuhwpuf20R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`scubd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubd0Spec;
impl crate::RegisterSpec for Scubd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubd0::R`](R) reader structure"]
impl crate::Readable for Scubd0Spec {}
#[doc = "`write(|w| ..)` method takes [`scubd0::W`](W) writer structure"]
impl crate::Writable for Scubd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBD0 to value 0"]
impl crate::Resettable for Scubd0Spec {}
