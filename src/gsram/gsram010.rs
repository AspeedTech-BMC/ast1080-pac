#[doc = "Register `GSRAM010` reader"]
pub type R = crate::R<Gsram010Spec>;
#[doc = "Register `GSRAM010` writer"]
pub type W = crate::W<Gsram010Spec>;
#[doc = "Field `OADDRBASE` reader - OADDR_BASE"]
pub type OaddrbaseR = crate::FieldReader<u16>;
#[doc = "Field `OADDRBASE` writer - OADDR_BASE"]
pub type OaddrbaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - OADDR_BASE"]
    #[inline(always)]
    pub fn oaddrbase(&self) -> OaddrbaseR {
        OaddrbaseR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - OADDR_BASE"]
    #[inline(always)]
    pub fn oaddrbase(&mut self) -> OaddrbaseW<Gsram010Spec> {
        OaddrbaseW::new(self, 0)
    }
}
#[doc = "GSRAM\\_OADDR\\_BASE\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram010Spec;
impl crate::RegisterSpec for Gsram010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram010::R`](R) reader structure"]
impl crate::Readable for Gsram010Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram010::W`](W) writer structure"]
impl crate::Writable for Gsram010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM010 to value 0"]
impl crate::Resettable for Gsram010Spec {}
