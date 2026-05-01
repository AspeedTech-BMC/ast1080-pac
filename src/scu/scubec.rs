#[doc = "Register `SCUBEC` reader"]
pub type R = crate::R<ScubecSpec>;
#[doc = "Register `SCUBEC` writer"]
pub type W = crate::W<ScubecSpec>;
#[doc = "Field `SCUHWPUF27` reader - SCU_HW_PUF_27"]
pub type Scuhwpuf27R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_27"]
    #[inline(always)]
    pub fn scuhwpuf27(&self) -> Scuhwpuf27R {
        Scuhwpuf27R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`scubec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScubecSpec;
impl crate::RegisterSpec for ScubecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubec::R`](R) reader structure"]
impl crate::Readable for ScubecSpec {}
#[doc = "`write(|w| ..)` method takes [`scubec::W`](W) writer structure"]
impl crate::Writable for ScubecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBEC to value 0"]
impl crate::Resettable for ScubecSpec {}
