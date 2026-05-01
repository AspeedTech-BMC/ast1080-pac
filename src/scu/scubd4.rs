#[doc = "Register `SCUBD4` reader"]
pub type R = crate::R<Scubd4Spec>;
#[doc = "Register `SCUBD4` writer"]
pub type W = crate::W<Scubd4Spec>;
#[doc = "Field `SCUHWPUF21` reader - SCU_HW_PUF_21"]
pub type Scuhwpuf21R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_21"]
    #[inline(always)]
    pub fn scuhwpuf21(&self) -> Scuhwpuf21R {
        Scuhwpuf21R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`scubd4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubd4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubd4Spec;
impl crate::RegisterSpec for Scubd4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubd4::R`](R) reader structure"]
impl crate::Readable for Scubd4Spec {}
#[doc = "`write(|w| ..)` method takes [`scubd4::W`](W) writer structure"]
impl crate::Writable for Scubd4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBD4 to value 0"]
impl crate::Resettable for Scubd4Spec {}
