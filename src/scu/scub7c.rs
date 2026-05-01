#[doc = "Register `SCUB7C` reader"]
pub type R = crate::R<Scub7cSpec>;
#[doc = "Register `SCUB7C` writer"]
pub type W = crate::W<Scub7cSpec>;
#[doc = "Field `SCUSWPUF31` reader - SCU_SW_PUF_31"]
pub type Scuswpuf31R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_31"]
    #[inline(always)]
    pub fn scuswpuf31(&self) -> Scuswpuf31R {
        Scuswpuf31R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`scub7c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub7c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub7cSpec;
impl crate::RegisterSpec for Scub7cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub7c::R`](R) reader structure"]
impl crate::Readable for Scub7cSpec {}
#[doc = "`write(|w| ..)` method takes [`scub7c::W`](W) writer structure"]
impl crate::Writable for Scub7cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB7C to value 0"]
impl crate::Resettable for Scub7cSpec {}
