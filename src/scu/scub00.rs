#[doc = "Register `SCUB00` reader"]
pub type R = crate::R<Scub00Spec>;
#[doc = "Register `SCUB00` writer"]
pub type W = crate::W<Scub00Spec>;
#[doc = "Field `SCUSWPUF0` reader - SCU_SW_PUF_0"]
pub type Scuswpuf0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_0"]
    #[inline(always)]
    pub fn scuswpuf0(&self) -> Scuswpuf0R {
        Scuswpuf0R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scub00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub00Spec;
impl crate::RegisterSpec for Scub00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub00::R`](R) reader structure"]
impl crate::Readable for Scub00Spec {}
#[doc = "`write(|w| ..)` method takes [`scub00::W`](W) writer structure"]
impl crate::Writable for Scub00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB00 to value 0"]
impl crate::Resettable for Scub00Spec {}
