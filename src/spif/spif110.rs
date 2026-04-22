#[doc = "Register `SPIF110` reader"]
pub type R = crate::R<Spif110Spec>;
#[doc = "Register `SPIF110` writer"]
pub type W = crate::W<Spif110Spec>;
#[doc = "Field `ADDRCTL04` reader - ADDR_CTL04"]
pub type Addrctl04R = crate::FieldReader;
#[doc = "Field `ADDRCTL04` writer - ADDR_CTL04"]
pub type Addrctl04W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL04"]
    #[inline(always)]
    pub fn addrctl04(&self) -> Addrctl04R {
        Addrctl04R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL04"]
    #[inline(always)]
    pub fn addrctl04(&mut self) -> Addrctl04W<Spif110Spec> {
        Addrctl04W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL04\n\nYou can [`read`](crate::Reg::read) this register and get [`spif110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif110Spec;
impl crate::RegisterSpec for Spif110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif110::R`](R) reader structure"]
impl crate::Readable for Spif110Spec {}
#[doc = "`write(|w| ..)` method takes [`spif110::W`](W) writer structure"]
impl crate::Writable for Spif110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF110 to value 0"]
impl crate::Resettable for Spif110Spec {}
