#[doc = "Register `SCUB20` reader"]
pub type R = crate::R<Scub20Spec>;
#[doc = "Register `SCUB20` writer"]
pub type W = crate::W<Scub20Spec>;
#[doc = "Field `SCUSWPUF8` reader - SCU_SW_PUF_8"]
pub type Scuswpuf8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_8"]
    #[inline(always)]
    pub fn scuswpuf8(&self) -> Scuswpuf8R {
        Scuswpuf8R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scub20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub20Spec;
impl crate::RegisterSpec for Scub20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub20::R`](R) reader structure"]
impl crate::Readable for Scub20Spec {}
#[doc = "`write(|w| ..)` method takes [`scub20::W`](W) writer structure"]
impl crate::Writable for Scub20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB20 to value 0"]
impl crate::Resettable for Scub20Spec {}
