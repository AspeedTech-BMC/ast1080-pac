#[doc = "Register `SCUB08` reader"]
pub type R = crate::R<Scub08Spec>;
#[doc = "Register `SCUB08` writer"]
pub type W = crate::W<Scub08Spec>;
#[doc = "Field `SCUSWPUF2` reader - SCU_SW_PUF_2"]
pub type Scuswpuf2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_2"]
    #[inline(always)]
    pub fn scuswpuf2(&self) -> Scuswpuf2R {
        Scuswpuf2R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scub08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub08Spec;
impl crate::RegisterSpec for Scub08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub08::R`](R) reader structure"]
impl crate::Readable for Scub08Spec {}
#[doc = "`write(|w| ..)` method takes [`scub08::W`](W) writer structure"]
impl crate::Writable for Scub08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB08 to value 0"]
impl crate::Resettable for Scub08Spec {}
