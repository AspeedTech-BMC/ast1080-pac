#[doc = "Register `SPIF134` reader"]
pub type R = crate::R<Spif134Spec>;
#[doc = "Register `SPIF134` writer"]
pub type W = crate::W<Spif134Spec>;
#[doc = "Field `ADDRCTL13` reader - ADDR_CTL13"]
pub type Addrctl13R = crate::FieldReader;
#[doc = "Field `ADDRCTL13` writer - ADDR_CTL13"]
pub type Addrctl13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL13"]
    #[inline(always)]
    pub fn addrctl13(&self) -> Addrctl13R {
        Addrctl13R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL13"]
    #[inline(always)]
    pub fn addrctl13(&mut self) -> Addrctl13W<Spif134Spec> {
        Addrctl13W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL13\n\nYou can [`read`](crate::Reg::read) this register and get [`spif134::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif134::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif134Spec;
impl crate::RegisterSpec for Spif134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif134::R`](R) reader structure"]
impl crate::Readable for Spif134Spec {}
#[doc = "`write(|w| ..)` method takes [`spif134::W`](W) writer structure"]
impl crate::Writable for Spif134Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF134 to value 0"]
impl crate::Resettable for Spif134Spec {}
