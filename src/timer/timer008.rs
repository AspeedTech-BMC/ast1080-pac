#[doc = "Register `TIMER008` reader"]
pub type R = crate::R<Timer008Spec>;
#[doc = "Register `TIMER008` writer"]
pub type W = crate::W<Timer008Spec>;
#[doc = "Field `FirstSetMatchReg` reader - First set match register"]
pub type FirstSetMatchRegR = crate::FieldReader<u32>;
#[doc = "Field `FirstSetMatchReg` writer - First set match register"]
pub type FirstSetMatchRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - First set match register"]
    #[inline(always)]
    pub fn first_set_match_reg(&self) -> FirstSetMatchRegR {
        FirstSetMatchRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - First set match register"]
    #[inline(always)]
    pub fn first_set_match_reg(&mut self) -> FirstSetMatchRegW<Timer008Spec> {
        FirstSetMatchRegW::new(self, 0)
    }
}
#[doc = "Counter First Matching Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer008Spec;
impl crate::RegisterSpec for Timer008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer008::R`](R) reader structure"]
impl crate::Readable for Timer008Spec {}
#[doc = "`write(|w| ..)` method takes [`timer008::W`](W) writer structure"]
impl crate::Writable for Timer008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER008 to value 0"]
impl crate::Resettable for Timer008Spec {}
