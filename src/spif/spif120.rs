#[doc = "Register `SPIF120` reader"]
pub type R = crate::R<Spif120Spec>;
#[doc = "Register `SPIF120` writer"]
pub type W = crate::W<Spif120Spec>;
#[doc = "Field `ADDRCTL08` reader - ADDR_CTL08"]
pub type Addrctl08R = crate::FieldReader;
#[doc = "Field `ADDRCTL08` writer - ADDR_CTL08"]
pub type Addrctl08W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL08"]
    #[inline(always)]
    pub fn addrctl08(&self) -> Addrctl08R {
        Addrctl08R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL08"]
    #[inline(always)]
    pub fn addrctl08(&mut self) -> Addrctl08W<Spif120Spec> {
        Addrctl08W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL08\n\nYou can [`read`](crate::Reg::read) this register and get [`spif120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif120Spec;
impl crate::RegisterSpec for Spif120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif120::R`](R) reader structure"]
impl crate::Readable for Spif120Spec {}
#[doc = "`write(|w| ..)` method takes [`spif120::W`](W) writer structure"]
impl crate::Writable for Spif120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF120 to value 0"]
impl crate::Resettable for Spif120Spec {}
