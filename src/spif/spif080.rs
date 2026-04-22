#[doc = "Register `SPIF080` reader"]
pub type R = crate::R<Spif080Spec>;
#[doc = "Register `SPIF080` writer"]
pub type W = crate::W<Spif080Spec>;
#[doc = "Field `WTABLE00` reader - WTABLE00"]
pub type Wtable00R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE00` writer - WTABLE00"]
pub type Wtable00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE00"]
    #[inline(always)]
    pub fn wtable00(&self) -> Wtable00R {
        Wtable00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE00"]
    #[inline(always)]
    pub fn wtable00(&mut self) -> Wtable00W<Spif080Spec> {
        Wtable00W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE00\n\nYou can [`read`](crate::Reg::read) this register and get [`spif080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif080Spec;
impl crate::RegisterSpec for Spif080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif080::R`](R) reader structure"]
impl crate::Readable for Spif080Spec {}
#[doc = "`write(|w| ..)` method takes [`spif080::W`](W) writer structure"]
impl crate::Writable for Spif080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF080 to value 0"]
impl crate::Resettable for Spif080Spec {}
