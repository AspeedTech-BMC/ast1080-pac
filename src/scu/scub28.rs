#[doc = "Register `SCUB28` reader"]
pub type R = crate::R<Scub28Spec>;
#[doc = "Register `SCUB28` writer"]
pub type W = crate::W<Scub28Spec>;
#[doc = "Field `SCUSWPUF10` reader - SCU_SW_PUF_10"]
pub type Scuswpuf10R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_10"]
    #[inline(always)]
    pub fn scuswpuf10(&self) -> Scuswpuf10R {
        Scuswpuf10R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`scub28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub28Spec;
impl crate::RegisterSpec for Scub28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub28::R`](R) reader structure"]
impl crate::Readable for Scub28Spec {}
#[doc = "`write(|w| ..)` method takes [`scub28::W`](W) writer structure"]
impl crate::Writable for Scub28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB28 to value 0"]
impl crate::Resettable for Scub28Spec {}
