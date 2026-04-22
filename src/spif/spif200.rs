#[doc = "Register `SPIF200` reader"]
pub type R = crate::R<Spif200Spec>;
#[doc = "Register `SPIF200` writer"]
pub type W = crate::W<Spif200Spec>;
#[doc = "Field `ADDRLBND00` reader - ADDR_LBND00"]
pub type Addrlbnd00R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND00` writer - ADDR_LBND00"]
pub type Addrlbnd00W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND00` reader - ADDR_UBND00"]
pub type Addrubnd00R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND00` writer - ADDR_UBND00"]
pub type Addrubnd00W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND00"]
    #[inline(always)]
    pub fn addrlbnd00(&self) -> Addrlbnd00R {
        Addrlbnd00R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND00"]
    #[inline(always)]
    pub fn addrubnd00(&self) -> Addrubnd00R {
        Addrubnd00R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND00"]
    #[inline(always)]
    pub fn addrlbnd00(&mut self) -> Addrlbnd00W<Spif200Spec> {
        Addrlbnd00W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND00"]
    #[inline(always)]
    pub fn addrubnd00(&mut self) -> Addrubnd00W<Spif200Spec> {
        Addrubnd00W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND00\n\nYou can [`read`](crate::Reg::read) this register and get [`spif200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif200Spec;
impl crate::RegisterSpec for Spif200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif200::R`](R) reader structure"]
impl crate::Readable for Spif200Spec {}
#[doc = "`write(|w| ..)` method takes [`spif200::W`](W) writer structure"]
impl crate::Writable for Spif200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF200 to value 0"]
impl crate::Resettable for Spif200Spec {}
