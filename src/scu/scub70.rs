#[doc = "Register `SCUB70` reader"]
pub type R = crate::R<Scub70Spec>;
#[doc = "Register `SCUB70` writer"]
pub type W = crate::W<Scub70Spec>;
#[doc = "Field `SCUSWPUF28` reader - SCU_SW_PUF_28"]
pub type Scuswpuf28R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_28"]
    #[inline(always)]
    pub fn scuswpuf28(&self) -> Scuswpuf28R {
        Scuswpuf28R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`scub70::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub70::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub70Spec;
impl crate::RegisterSpec for Scub70Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub70::R`](R) reader structure"]
impl crate::Readable for Scub70Spec {}
#[doc = "`write(|w| ..)` method takes [`scub70::W`](W) writer structure"]
impl crate::Writable for Scub70Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB70 to value 0"]
impl crate::Resettable for Scub70Spec {}
