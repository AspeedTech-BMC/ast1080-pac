#[doc = "Register `SPIF108` reader"]
pub type R = crate::R<Spif108Spec>;
#[doc = "Register `SPIF108` writer"]
pub type W = crate::W<Spif108Spec>;
#[doc = "Field `ADDRCTL02` reader - ADDR_CTL02"]
pub type Addrctl02R = crate::FieldReader;
#[doc = "Field `ADDRCTL02` writer - ADDR_CTL02"]
pub type Addrctl02W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL02"]
    #[inline(always)]
    pub fn addrctl02(&self) -> Addrctl02R {
        Addrctl02R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL02"]
    #[inline(always)]
    pub fn addrctl02(&mut self) -> Addrctl02W<Spif108Spec> {
        Addrctl02W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL02\n\nYou can [`read`](crate::Reg::read) this register and get [`spif108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif108Spec;
impl crate::RegisterSpec for Spif108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif108::R`](R) reader structure"]
impl crate::Readable for Spif108Spec {}
#[doc = "`write(|w| ..)` method takes [`spif108::W`](W) writer structure"]
impl crate::Writable for Spif108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF108 to value 0"]
impl crate::Resettable for Spif108Spec {}
