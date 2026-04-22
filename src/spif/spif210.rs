#[doc = "Register `SPIF210` reader"]
pub type R = crate::R<Spif210Spec>;
#[doc = "Register `SPIF210` writer"]
pub type W = crate::W<Spif210Spec>;
#[doc = "Field `ADDRLBND04` reader - ADDR_LBND04"]
pub type Addrlbnd04R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND04` writer - ADDR_LBND04"]
pub type Addrlbnd04W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND04` reader - ADDR_UBND04"]
pub type Addrubnd04R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND04` writer - ADDR_UBND04"]
pub type Addrubnd04W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND04"]
    #[inline(always)]
    pub fn addrlbnd04(&self) -> Addrlbnd04R {
        Addrlbnd04R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND04"]
    #[inline(always)]
    pub fn addrubnd04(&self) -> Addrubnd04R {
        Addrubnd04R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND04"]
    #[inline(always)]
    pub fn addrlbnd04(&mut self) -> Addrlbnd04W<Spif210Spec> {
        Addrlbnd04W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND04"]
    #[inline(always)]
    pub fn addrubnd04(&mut self) -> Addrubnd04W<Spif210Spec> {
        Addrubnd04W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND04\n\nYou can [`read`](crate::Reg::read) this register and get [`spif210::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif210::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif210Spec;
impl crate::RegisterSpec for Spif210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif210::R`](R) reader structure"]
impl crate::Readable for Spif210Spec {}
#[doc = "`write(|w| ..)` method takes [`spif210::W`](W) writer structure"]
impl crate::Writable for Spif210Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF210 to value 0"]
impl crate::Resettable for Spif210Spec {}
