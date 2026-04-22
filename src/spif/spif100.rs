#[doc = "Register `SPIF100` reader"]
pub type R = crate::R<Spif100Spec>;
#[doc = "Register `SPIF100` writer"]
pub type W = crate::W<Spif100Spec>;
#[doc = "Field `ADDRCTL00` reader - ADDR_CTL00"]
pub type Addrctl00R = crate::FieldReader;
#[doc = "Field `ADDRCTL00` writer - ADDR_CTL00"]
pub type Addrctl00W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL00"]
    #[inline(always)]
    pub fn addrctl00(&self) -> Addrctl00R {
        Addrctl00R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL00"]
    #[inline(always)]
    pub fn addrctl00(&mut self) -> Addrctl00W<Spif100Spec> {
        Addrctl00W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL00\n\nYou can [`read`](crate::Reg::read) this register and get [`spif100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif100Spec;
impl crate::RegisterSpec for Spif100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif100::R`](R) reader structure"]
impl crate::Readable for Spif100Spec {}
#[doc = "`write(|w| ..)` method takes [`spif100::W`](W) writer structure"]
impl crate::Writable for Spif100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF100 to value 0"]
impl crate::Resettable for Spif100Spec {}
