#[doc = "Register `TIMER00C` reader"]
pub type R = crate::R<Timer00cSpec>;
#[doc = "Register `TIMER00C` writer"]
pub type W = crate::W<Timer00cSpec>;
#[doc = "Field `SecondaryMatchReg` reader - Secondary match register"]
pub type SecondaryMatchRegR = crate::FieldReader<u32>;
#[doc = "Field `SecondaryMatchReg` writer - Secondary match register"]
pub type SecondaryMatchRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secondary match register"]
    #[inline(always)]
    pub fn secondary_match_reg(&self) -> SecondaryMatchRegR {
        SecondaryMatchRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secondary match register"]
    #[inline(always)]
    pub fn secondary_match_reg(&mut self) -> SecondaryMatchRegW<Timer00cSpec> {
        SecondaryMatchRegW::new(self, 0)
    }
}
#[doc = "Counter Second Matching Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer00cSpec;
impl crate::RegisterSpec for Timer00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer00c::R`](R) reader structure"]
impl crate::Readable for Timer00cSpec {}
#[doc = "`write(|w| ..)` method takes [`timer00c::W`](W) writer structure"]
impl crate::Writable for Timer00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER00C to value 0"]
impl crate::Resettable for Timer00cSpec {}
