#[doc = "Register `SPIF018` reader"]
pub type R = crate::R<Spif018Spec>;
#[doc = "Register `SPIF018` writer"]
pub type W = crate::W<Spif018Spec>;
#[doc = "Field `CS2BASE` reader - CS2_BASE"]
pub type Cs2baseR = crate::FieldReader<u16>;
#[doc = "Field `CS2BASE` writer - CS2_BASE"]
pub type Cs2baseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CS2_BASE"]
    #[inline(always)]
    pub fn cs2base(&self) -> Cs2baseR {
        Cs2baseR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CS2_BASE"]
    #[inline(always)]
    pub fn cs2base(&mut self) -> Cs2baseW<Spif018Spec> {
        Cs2baseW::new(self, 0)
    }
}
#[doc = "SPIF\\_CSBASE2\n\nYou can [`read`](crate::Reg::read) this register and get [`spif018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif018Spec;
impl crate::RegisterSpec for Spif018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif018::R`](R) reader structure"]
impl crate::Readable for Spif018Spec {}
#[doc = "`write(|w| ..)` method takes [`spif018::W`](W) writer structure"]
impl crate::Writable for Spif018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF018 to value 0"]
impl crate::Resettable for Spif018Spec {}
