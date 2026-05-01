#[doc = "Register `SCUBD8` reader"]
pub type R = crate::R<Scubd8Spec>;
#[doc = "Register `SCUBD8` writer"]
pub type W = crate::W<Scubd8Spec>;
#[doc = "Field `SCUHWPUF22` reader - SCU_HW_PUF_22"]
pub type Scuhwpuf22R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_22"]
    #[inline(always)]
    pub fn scuhwpuf22(&self) -> Scuhwpuf22R {
        Scuhwpuf22R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`scubd8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubd8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scubd8Spec;
impl crate::RegisterSpec for Scubd8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubd8::R`](R) reader structure"]
impl crate::Readable for Scubd8Spec {}
#[doc = "`write(|w| ..)` method takes [`scubd8::W`](W) writer structure"]
impl crate::Writable for Scubd8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBD8 to value 0"]
impl crate::Resettable for Scubd8Spec {}
