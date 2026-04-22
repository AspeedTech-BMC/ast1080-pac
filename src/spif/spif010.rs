#[doc = "Register `SPIF010` reader"]
pub type R = crate::R<Spif010Spec>;
#[doc = "Register `SPIF010` writer"]
pub type W = crate::W<Spif010Spec>;
#[doc = "Field `CS0BASE` reader - CS0_BASE"]
pub type Cs0baseR = crate::FieldReader<u16>;
#[doc = "Field `CS0BASE` writer - CS0_BASE"]
pub type Cs0baseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CS0_BASE"]
    #[inline(always)]
    pub fn cs0base(&self) -> Cs0baseR {
        Cs0baseR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CS0_BASE"]
    #[inline(always)]
    pub fn cs0base(&mut self) -> Cs0baseW<Spif010Spec> {
        Cs0baseW::new(self, 0)
    }
}
#[doc = "SPIF\\_CSBASE0\n\nYou can [`read`](crate::Reg::read) this register and get [`spif010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif010Spec;
impl crate::RegisterSpec for Spif010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif010::R`](R) reader structure"]
impl crate::Readable for Spif010Spec {}
#[doc = "`write(|w| ..)` method takes [`spif010::W`](W) writer structure"]
impl crate::Writable for Spif010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF010 to value 0"]
impl crate::Resettable for Spif010Spec {}
