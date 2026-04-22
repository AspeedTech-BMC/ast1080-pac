#[doc = "Register `SPIF130` reader"]
pub type R = crate::R<Spif130Spec>;
#[doc = "Register `SPIF130` writer"]
pub type W = crate::W<Spif130Spec>;
#[doc = "Field `ADDRCTL12` reader - ADDR_CTL12"]
pub type Addrctl12R = crate::FieldReader;
#[doc = "Field `ADDRCTL12` writer - ADDR_CTL12"]
pub type Addrctl12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL12"]
    #[inline(always)]
    pub fn addrctl12(&self) -> Addrctl12R {
        Addrctl12R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL12"]
    #[inline(always)]
    pub fn addrctl12(&mut self) -> Addrctl12W<Spif130Spec> {
        Addrctl12W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL12\n\nYou can [`read`](crate::Reg::read) this register and get [`spif130::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif130::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif130Spec;
impl crate::RegisterSpec for Spif130Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif130::R`](R) reader structure"]
impl crate::Readable for Spif130Spec {}
#[doc = "`write(|w| ..)` method takes [`spif130::W`](W) writer structure"]
impl crate::Writable for Spif130Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF130 to value 0"]
impl crate::Resettable for Spif130Spec {}
