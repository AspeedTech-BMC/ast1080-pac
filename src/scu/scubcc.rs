#[doc = "Register `SCUBCC` reader"]
pub type R = crate::R<ScubccSpec>;
#[doc = "Register `SCUBCC` writer"]
pub type W = crate::W<ScubccSpec>;
#[doc = "Field `SCUHWPUF19` reader - SCU_HW_PUF_19"]
pub type Scuhwpuf19R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_19"]
    #[inline(always)]
    pub fn scuhwpuf19(&self) -> Scuhwpuf19R {
        Scuhwpuf19R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`scubcc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubcc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScubccSpec;
impl crate::RegisterSpec for ScubccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubcc::R`](R) reader structure"]
impl crate::Readable for ScubccSpec {}
#[doc = "`write(|w| ..)` method takes [`scubcc::W`](W) writer structure"]
impl crate::Writable for ScubccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBCC to value 0"]
impl crate::Resettable for ScubccSpec {}
