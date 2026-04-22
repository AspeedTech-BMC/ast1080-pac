#[doc = "Register `SPIF014` reader"]
pub type R = crate::R<Spif014Spec>;
#[doc = "Register `SPIF014` writer"]
pub type W = crate::W<Spif014Spec>;
#[doc = "Field `CS1BASE` reader - CS1_BASE"]
pub type Cs1baseR = crate::FieldReader<u16>;
#[doc = "Field `CS1BASE` writer - CS1_BASE"]
pub type Cs1baseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CS1_BASE"]
    #[inline(always)]
    pub fn cs1base(&self) -> Cs1baseR {
        Cs1baseR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CS1_BASE"]
    #[inline(always)]
    pub fn cs1base(&mut self) -> Cs1baseW<Spif014Spec> {
        Cs1baseW::new(self, 0)
    }
}
#[doc = "SPIF\\_CSBASE1\n\nYou can [`read`](crate::Reg::read) this register and get [`spif014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif014Spec;
impl crate::RegisterSpec for Spif014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif014::R`](R) reader structure"]
impl crate::Readable for Spif014Spec {}
#[doc = "`write(|w| ..)` method takes [`spif014::W`](W) writer structure"]
impl crate::Writable for Spif014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF014 to value 0"]
impl crate::Resettable for Spif014Spec {}
