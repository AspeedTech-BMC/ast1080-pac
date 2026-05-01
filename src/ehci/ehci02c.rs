#[doc = "Register `EHCI02C` reader"]
pub type R = crate::R<Ehci02cSpec>;
#[doc = "Register `EHCI02C` writer"]
pub type W = crate::W<Ehci02cSpec>;
#[doc = "Field `FrameIndex` reader - Frame Index"]
pub type FrameIndexR = crate::FieldReader<u16>;
#[doc = "Field `FrameIndex` writer - Frame Index"]
pub type FrameIndexW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    pub fn frame_index(&self) -> FrameIndexR {
        FrameIndexR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    pub fn frame_index(&mut self) -> FrameIndexW<Ehci02cSpec> {
        FrameIndexW::new(self, 0)
    }
}
#[doc = "Frame Index Register (FRINDEX)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci02cSpec;
impl crate::RegisterSpec for Ehci02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci02c::R`](R) reader structure"]
impl crate::Readable for Ehci02cSpec {}
#[doc = "`write(|w| ..)` method takes [`ehci02c::W`](W) writer structure"]
impl crate::Writable for Ehci02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI02C to value 0"]
impl crate::Resettable for Ehci02cSpec {}
