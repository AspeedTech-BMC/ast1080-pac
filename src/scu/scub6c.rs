#[doc = "Register `SCUB6C` reader"]
pub type R = crate::R<Scub6cSpec>;
#[doc = "Register `SCUB6C` writer"]
pub type W = crate::W<Scub6cSpec>;
#[doc = "Field `SCUSWPUF27` reader - SCU_SW_PUF_27"]
pub type Scuswpuf27R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_SW_PUF_27"]
    #[inline(always)]
    pub fn scuswpuf27(&self) -> Scuswpuf27R {
        Scuswpuf27R::new(self.bits)
    }
}
impl W {}
#[doc = "SW PUF Register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`scub6c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scub6c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scub6cSpec;
impl crate::RegisterSpec for Scub6cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scub6c::R`](R) reader structure"]
impl crate::Readable for Scub6cSpec {}
#[doc = "`write(|w| ..)` method takes [`scub6c::W`](W) writer structure"]
impl crate::Writable for Scub6cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUB6C to value 0"]
impl crate::Resettable for Scub6cSpec {}
