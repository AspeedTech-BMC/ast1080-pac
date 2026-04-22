#[doc = "Register `SPIF104` reader"]
pub type R = crate::R<Spif104Spec>;
#[doc = "Register `SPIF104` writer"]
pub type W = crate::W<Spif104Spec>;
#[doc = "Field `ADDRCTL01` reader - ADDR_CTL01"]
pub type Addrctl01R = crate::FieldReader;
#[doc = "Field `ADDRCTL01` writer - ADDR_CTL01"]
pub type Addrctl01W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL01"]
    #[inline(always)]
    pub fn addrctl01(&self) -> Addrctl01R {
        Addrctl01R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL01"]
    #[inline(always)]
    pub fn addrctl01(&mut self) -> Addrctl01W<Spif104Spec> {
        Addrctl01W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL01\n\nYou can [`read`](crate::Reg::read) this register and get [`spif104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif104Spec;
impl crate::RegisterSpec for Spif104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif104::R`](R) reader structure"]
impl crate::Readable for Spif104Spec {}
#[doc = "`write(|w| ..)` method takes [`spif104::W`](W) writer structure"]
impl crate::Writable for Spif104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF104 to value 0"]
impl crate::Resettable for Spif104Spec {}
