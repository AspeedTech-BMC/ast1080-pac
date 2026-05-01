#[doc = "Register `AT03C` reader"]
pub type R = crate::R<At03cSpec>;
#[doc = "Register `AT03C` writer"]
pub type W = crate::W<At03cSpec>;
#[doc = "Field `ATGDBGREADY` reader - AT_GD_BG_READY"]
pub type AtgdbgreadyR = crate::BitReader;
#[doc = "Field `ATGDATTACKINFO` reader - AT_GD_ATTACK_INFO"]
pub type AtgdattackinfoR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - AT_GD_BG_READY"]
    #[inline(always)]
    pub fn atgdbgready(&self) -> AtgdbgreadyR {
        AtgdbgreadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - AT_GD_ATTACK_INFO"]
    #[inline(always)]
    pub fn atgdattackinfo(&self) -> AtgdattackinfoR {
        AtgdattackinfoR::new(((self.bits >> 1) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Glitch Detection Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At03cSpec;
impl crate::RegisterSpec for At03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at03c::R`](R) reader structure"]
impl crate::Readable for At03cSpec {}
#[doc = "`write(|w| ..)` method takes [`at03c::W`](W) writer structure"]
impl crate::Writable for At03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT03C to value 0"]
impl crate::Resettable for At03cSpec {}
