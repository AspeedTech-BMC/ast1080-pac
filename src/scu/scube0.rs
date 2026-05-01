#[doc = "Register `SCUBE0` reader"]
pub type R = crate::R<Scube0Spec>;
#[doc = "Register `SCUBE0` writer"]
pub type W = crate::W<Scube0Spec>;
#[doc = "Field `SCUHWPUF24` reader - SCU_HW_PUF_24"]
pub type Scuhwpuf24R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_24"]
    #[inline(always)]
    pub fn scuhwpuf24(&self) -> Scuhwpuf24R {
        Scuhwpuf24R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`scube0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scube0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scube0Spec;
impl crate::RegisterSpec for Scube0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scube0::R`](R) reader structure"]
impl crate::Readable for Scube0Spec {}
#[doc = "`write(|w| ..)` method takes [`scube0::W`](W) writer structure"]
impl crate::Writable for Scube0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBE0 to value 0"]
impl crate::Resettable for Scube0Spec {}
