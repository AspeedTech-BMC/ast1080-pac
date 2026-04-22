#[doc = "Register `SPIF0E0` reader"]
pub type R = crate::R<Spif0e0Spec>;
#[doc = "Register `SPIF0E0` writer"]
pub type W = crate::W<Spif0e0Spec>;
#[doc = "Field `WTABLE24` reader - WTABLE24"]
pub type Wtable24R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE24` writer - WTABLE24"]
pub type Wtable24W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE24"]
    #[inline(always)]
    pub fn wtable24(&self) -> Wtable24R {
        Wtable24R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE24"]
    #[inline(always)]
    pub fn wtable24(&mut self) -> Wtable24W<Spif0e0Spec> {
        Wtable24W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE24\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0e0Spec;
impl crate::RegisterSpec for Spif0e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0e0::R`](R) reader structure"]
impl crate::Readable for Spif0e0Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0e0::W`](W) writer structure"]
impl crate::Writable for Spif0e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0E0 to value 0"]
impl crate::Resettable for Spif0e0Spec {}
