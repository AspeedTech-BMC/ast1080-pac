#[doc = "Register `UARTSCR` reader"]
pub type R = crate::R<UartscrSpec>;
#[doc = "Register `UARTSCR` writer"]
pub type W = crate::W<UartscrSpec>;
#[doc = "Field `ScratchBits` reader - Scratch bits"]
pub type ScratchBitsR = crate::FieldReader;
#[doc = "Field `ScratchBits` writer - Scratch bits"]
pub type ScratchBitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Scratch bits"]
    #[inline(always)]
    pub fn scratch_bits(&self) -> ScratchBitsR {
        ScratchBitsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scratch bits"]
    #[inline(always)]
    pub fn scratch_bits(&mut self) -> ScratchBitsW<UartscrSpec> {
        ScratchBitsW::new(self, 0)
    }
}
#[doc = "Scratch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartscrSpec;
impl crate::RegisterSpec for UartscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartscr::R`](R) reader structure"]
impl crate::Readable for UartscrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartscr::W`](W) writer structure"]
impl crate::Writable for UartscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTSCR to value 0"]
impl crate::Resettable for UartscrSpec {}
