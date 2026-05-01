#[doc = "Register `HUB40` reader"]
pub type R = crate::R<Hub40Spec>;
#[doc = "Register `HUB40` writer"]
pub type W = crate::W<Hub40Spec>;
#[doc = "Field `SOFCounter` reader - SOF Counter"]
pub type SofcounterR = crate::FieldReader<u32>;
#[doc = "Field `SOFCounter` writer - SOF Counter"]
pub type SofcounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SOF Counter"]
    #[inline(always)]
    pub fn sofcounter(&self) -> SofcounterR {
        SofcounterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SOF Counter"]
    #[inline(always)]
    pub fn sofcounter(&mut self) -> SofcounterW<Hub40Spec> {
        SofcounterW::new(self, 0)
    }
}
#[doc = "SOF Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`hub40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub40Spec;
impl crate::RegisterSpec for Hub40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub40::R`](R) reader structure"]
impl crate::Readable for Hub40Spec {}
#[doc = "`write(|w| ..)` method takes [`hub40::W`](W) writer structure"]
impl crate::Writable for Hub40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB40 to value 0"]
impl crate::Resettable for Hub40Spec {}
