#[doc = "Register `SPIF218` reader"]
pub type R = crate::R<Spif218Spec>;
#[doc = "Register `SPIF218` writer"]
pub type W = crate::W<Spif218Spec>;
#[doc = "Field `ADDRLBND06` reader - ADDR_LBND06"]
pub type Addrlbnd06R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND06` writer - ADDR_LBND06"]
pub type Addrlbnd06W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND06` reader - ADDR_UBND06"]
pub type Addrubnd06R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND06` writer - ADDR_UBND06"]
pub type Addrubnd06W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND06"]
    #[inline(always)]
    pub fn addrlbnd06(&self) -> Addrlbnd06R {
        Addrlbnd06R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND06"]
    #[inline(always)]
    pub fn addrubnd06(&self) -> Addrubnd06R {
        Addrubnd06R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND06"]
    #[inline(always)]
    pub fn addrlbnd06(&mut self) -> Addrlbnd06W<Spif218Spec> {
        Addrlbnd06W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND06"]
    #[inline(always)]
    pub fn addrubnd06(&mut self) -> Addrubnd06W<Spif218Spec> {
        Addrubnd06W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND06\n\nYou can [`read`](crate::Reg::read) this register and get [`spif218::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif218::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif218Spec;
impl crate::RegisterSpec for Spif218Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif218::R`](R) reader structure"]
impl crate::Readable for Spif218Spec {}
#[doc = "`write(|w| ..)` method takes [`spif218::W`](W) writer structure"]
impl crate::Writable for Spif218Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF218 to value 0"]
impl crate::Resettable for Spif218Spec {}
