#[doc = "Register `SPIF208` reader"]
pub type R = crate::R<Spif208Spec>;
#[doc = "Register `SPIF208` writer"]
pub type W = crate::W<Spif208Spec>;
#[doc = "Field `ADDRLBND02` reader - ADDR_LBND02"]
pub type Addrlbnd02R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND02` writer - ADDR_LBND02"]
pub type Addrlbnd02W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND02` reader - ADDR_UBND02"]
pub type Addrubnd02R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND02` writer - ADDR_UBND02"]
pub type Addrubnd02W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND02"]
    #[inline(always)]
    pub fn addrlbnd02(&self) -> Addrlbnd02R {
        Addrlbnd02R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND02"]
    #[inline(always)]
    pub fn addrubnd02(&self) -> Addrubnd02R {
        Addrubnd02R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND02"]
    #[inline(always)]
    pub fn addrlbnd02(&mut self) -> Addrlbnd02W<Spif208Spec> {
        Addrlbnd02W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND02"]
    #[inline(always)]
    pub fn addrubnd02(&mut self) -> Addrubnd02W<Spif208Spec> {
        Addrubnd02W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND02\n\nYou can [`read`](crate::Reg::read) this register and get [`spif208::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif208::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif208Spec;
impl crate::RegisterSpec for Spif208Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif208::R`](R) reader structure"]
impl crate::Readable for Spif208Spec {}
#[doc = "`write(|w| ..)` method takes [`spif208::W`](W) writer structure"]
impl crate::Writable for Spif208Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF208 to value 0"]
impl crate::Resettable for Spif208Spec {}
