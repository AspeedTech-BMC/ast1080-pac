#[doc = "Register `SCUB44` reader"]
pub type R = crate::R<Scub44Spec>;
#[doc = "Register `SCUB44` writer"]
pub type W = crate::W<Scub44Spec>;
#[doc = "Field `SCUSWPUF17` reader - SCU_SW_PUF_17"]
pub type Scuswpuf17R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_17"]
    #[inline(always)]
    pub fn scuswpuf17(&self) -> Scuswpuf17R {
        Scuswpuf17R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`scub44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub44Spec;
impl crate::RegisterSpec for Scub44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub44::R`](R) reader structure"]
impl crate::Readable for Scub44Spec {}
#[doc = "`write(|w| ..)` method takes [`scub44::W`](W) writer structure"]
impl crate::Writable for Scub44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB44 to value 0"]
impl crate::Resettable for Scub44Spec {}
