#[doc = "Register `SCUBBC` reader"]
pub type R = crate::R<ScubbcSpec>;
#[doc = "Register `SCUBBC` writer"]
pub type W = crate::W<ScubbcSpec>;
#[doc = "Field `SCUHWPUF15` reader - SCU_HW_PUF_15"]
pub type Scuhwpuf15R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_HW_PUF_15"]
    #[inline(always)]
    pub fn scuhwpuf15(&self) -> Scuhwpuf15R {
        Scuhwpuf15R::new(self.bits)
    }
}
impl W {}
#[doc = "HW PUF Register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`scubbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scubbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScubbcSpec;
impl crate::RegisterSpec for ScubbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scubbc::R`](R) reader structure"]
impl crate::Readable for ScubbcSpec {}
#[doc = "`write(|w| ..)` method takes [`scubbc::W`](W) writer structure"]
impl crate::Writable for ScubbcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUBBC to value 0"]
impl crate::Resettable for ScubbcSpec {}
