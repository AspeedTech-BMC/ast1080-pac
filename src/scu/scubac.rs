#[doc = "Register `SCUBAC` reader"]
pub type R = crate::R<ScubacSpec>;
#[doc = "Register `SCUBAC` writer"]
pub type W = crate::W<ScubacSpec>;
#[doc = "Field `SCUHWPUF11` reader - SCU_HW_PUF_11"]
pub type Scuhwpuf11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_11"]
    #[inline(always)]
    pub fn scuhwpuf11(&self) -> Scuhwpuf11R {
        Scuhwpuf11R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`scubac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScubacSpec;
impl crate::RegisterSpec for ScubacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubac::R`](R) reader structure"]
impl crate::Readable for ScubacSpec {}
#[doc = "`write(|w| ..)` method takes [`scubac::W`](W) writer structure"]
impl crate::Writable for ScubacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBAC to value 0"]
impl crate::Resettable for ScubacSpec {}
