#[doc = "Register `SCUB48` reader"]
pub type R = crate::R<Scub48Spec>;
#[doc = "Register `SCUB48` writer"]
pub type W = crate::W<Scub48Spec>;
#[doc = "Field `SCUSWPUF18` reader - SCU_SW_PUF_18"]
pub type Scuswpuf18R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_18"]
    #[inline(always)]
    pub fn scuswpuf18(&self) -> Scuswpuf18R {
        Scuswpuf18R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`scub48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub48Spec;
impl crate::RegisterSpec for Scub48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub48::R`](R) reader structure"]
impl crate::Readable for Scub48Spec {}
#[doc = "`write(|w| ..)` method takes [`scub48::W`](W) writer structure"]
impl crate::Writable for Scub48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB48 to value 0"]
impl crate::Resettable for Scub48Spec {}
