#[doc = "Register `SCUB54` reader"]
pub type R = crate::R<Scub54Spec>;
#[doc = "Register `SCUB54` writer"]
pub type W = crate::W<Scub54Spec>;
#[doc = "Field `SCUSWPUF21` reader - SCU_SW_PUF_21"]
pub type Scuswpuf21R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_21"]
    #[inline(always)]
    pub fn scuswpuf21(&self) -> Scuswpuf21R {
        Scuswpuf21R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`scub54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub54Spec;
impl crate::RegisterSpec for Scub54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub54::R`](R) reader structure"]
impl crate::Readable for Scub54Spec {}
#[doc = "`write(|w| ..)` method takes [`scub54::W`](W) writer structure"]
impl crate::Writable for Scub54Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB54 to value 0"]
impl crate::Resettable for Scub54Spec {}
