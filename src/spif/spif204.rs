#[doc = "Register `SPIF204` reader"]
pub type R = crate::R<Spif204Spec>;
#[doc = "Register `SPIF204` writer"]
pub type W = crate::W<Spif204Spec>;
#[doc = "Field `ADDRLBND01` reader - ADDR_LBND01"]
pub type Addrlbnd01R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND01` writer - ADDR_LBND01"]
pub type Addrlbnd01W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND01` reader - ADDR_UBND01"]
pub type Addrubnd01R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND01` writer - ADDR_UBND01"]
pub type Addrubnd01W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND01"]
    #[inline(always)]
    pub fn addrlbnd01(&self) -> Addrlbnd01R {
        Addrlbnd01R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND01"]
    #[inline(always)]
    pub fn addrubnd01(&self) -> Addrubnd01R {
        Addrubnd01R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND01"]
    #[inline(always)]
    pub fn addrlbnd01(&mut self) -> Addrlbnd01W<Spif204Spec> {
        Addrlbnd01W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND01"]
    #[inline(always)]
    pub fn addrubnd01(&mut self) -> Addrubnd01W<Spif204Spec> {
        Addrubnd01W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND01\n\nYou can [`read`](crate::Reg::read) this register and get [`spif204::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif204Spec;
impl crate::RegisterSpec for Spif204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif204::R`](R) reader structure"]
impl crate::Readable for Spif204Spec {}
#[doc = "`write(|w| ..)` method takes [`spif204::W`](W) writer structure"]
impl crate::Writable for Spif204Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF204 to value 0"]
impl crate::Resettable for Spif204Spec {}
