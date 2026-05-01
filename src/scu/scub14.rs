#[doc = "Register `SCUB14` reader"]
pub type R = crate::R<Scub14Spec>;
#[doc = "Register `SCUB14` writer"]
pub type W = crate::W<Scub14Spec>;
#[doc = "Field `SCUSWPUF5` reader - SCU_SW_PUF_5"]
pub type Scuswpuf5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_5"]
    #[inline(always)]
    pub fn scuswpuf5(&self) -> Scuswpuf5R {
        Scuswpuf5R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scub14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub14Spec;
impl crate::RegisterSpec for Scub14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub14::R`](R) reader structure"]
impl crate::Readable for Scub14Spec {}
#[doc = "`write(|w| ..)` method takes [`scub14::W`](W) writer structure"]
impl crate::Writable for Scub14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB14 to value 0"]
impl crate::Resettable for Scub14Spec {}
