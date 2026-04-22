#[doc = "Register `SPIF124` reader"]
pub type R = crate::R<Spif124Spec>;
#[doc = "Register `SPIF124` writer"]
pub type W = crate::W<Spif124Spec>;
#[doc = "Field `ADDRCTL09` reader - ADDR_CTL09"]
pub type Addrctl09R = crate::FieldReader;
#[doc = "Field `ADDRCTL09` writer - ADDR_CTL09"]
pub type Addrctl09W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL09"]
    #[inline(always)]
    pub fn addrctl09(&self) -> Addrctl09R {
        Addrctl09R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL09"]
    #[inline(always)]
    pub fn addrctl09(&mut self) -> Addrctl09W<Spif124Spec> {
        Addrctl09W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL09\n\nYou can [`read`](crate::Reg::read) this register and get [`spif124::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif124::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif124Spec;
impl crate::RegisterSpec for Spif124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif124::R`](R) reader structure"]
impl crate::Readable for Spif124Spec {}
#[doc = "`write(|w| ..)` method takes [`spif124::W`](W) writer structure"]
impl crate::Writable for Spif124Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF124 to value 0"]
impl crate::Resettable for Spif124Spec {}
