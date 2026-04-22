#[doc = "Register `SPIF230` reader"]
pub type R = crate::R<Spif230Spec>;
#[doc = "Register `SPIF230` writer"]
pub type W = crate::W<Spif230Spec>;
#[doc = "Field `ADDRLBND12` reader - ADDR_LBND12"]
pub type Addrlbnd12R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND12` writer - ADDR_LBND12"]
pub type Addrlbnd12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND12` reader - ADDR_UBND12"]
pub type Addrubnd12R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND12` writer - ADDR_UBND12"]
pub type Addrubnd12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND12"]
    #[inline(always)]
    pub fn addrlbnd12(&self) -> Addrlbnd12R {
        Addrlbnd12R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND12"]
    #[inline(always)]
    pub fn addrubnd12(&self) -> Addrubnd12R {
        Addrubnd12R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND12"]
    #[inline(always)]
    pub fn addrlbnd12(&mut self) -> Addrlbnd12W<Spif230Spec> {
        Addrlbnd12W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND12"]
    #[inline(always)]
    pub fn addrubnd12(&mut self) -> Addrubnd12W<Spif230Spec> {
        Addrubnd12W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND12\n\nYou can [`read`](crate::Reg::read) this register and get [`spif230::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif230::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif230Spec;
impl crate::RegisterSpec for Spif230Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif230::R`](R) reader structure"]
impl crate::Readable for Spif230Spec {}
#[doc = "`write(|w| ..)` method takes [`spif230::W`](W) writer structure"]
impl crate::Writable for Spif230Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF230 to value 0"]
impl crate::Resettable for Spif230Spec {}
