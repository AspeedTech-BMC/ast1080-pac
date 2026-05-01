#[doc = "Register `SCUB04` reader"]
pub type R = crate::R<Scub04Spec>;
#[doc = "Register `SCUB04` writer"]
pub type W = crate::W<Scub04Spec>;
#[doc = "Field `SCUSWPUF1` reader - SCU_SW_PUF_1"]
pub type Scuswpuf1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_1"]
    #[inline(always)]
    pub fn scuswpuf1(&self) -> Scuswpuf1R {
        Scuswpuf1R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scub04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub04Spec;
impl crate::RegisterSpec for Scub04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub04::R`](R) reader structure"]
impl crate::Readable for Scub04Spec {}
#[doc = "`write(|w| ..)` method takes [`scub04::W`](W) writer structure"]
impl crate::Writable for Scub04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB04 to value 0"]
impl crate::Resettable for Scub04Spec {}
