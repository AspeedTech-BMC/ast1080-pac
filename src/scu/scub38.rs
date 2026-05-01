#[doc = "Register `SCUB38` reader"]
pub type R = crate::R<Scub38Spec>;
#[doc = "Register `SCUB38` writer"]
pub type W = crate::W<Scub38Spec>;
#[doc = "Field `SCUSWPUF14` reader - SCU_SW_PUF_14"]
pub type Scuswpuf14R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_14"]
    #[inline(always)]
    pub fn scuswpuf14(&self) -> Scuswpuf14R {
        Scuswpuf14R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`scub38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub38Spec;
impl crate::RegisterSpec for Scub38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub38::R`](R) reader structure"]
impl crate::Readable for Scub38Spec {}
#[doc = "`write(|w| ..)` method takes [`scub38::W`](W) writer structure"]
impl crate::Writable for Scub38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB38 to value 0"]
impl crate::Resettable for Scub38Spec {}
